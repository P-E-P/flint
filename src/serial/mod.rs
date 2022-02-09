use crate::io;
use core::fmt;
use registers::{
    lcr::LineControlRegister, lsr::LineStatusRegister, rbr::ReceiverBuffer,
    thr::TransmitterHoldingBuffer, ReadRegister, Register,
};

mod registers;

type ComPort = usize;

const COM1: ComPort = 0x3F8;
const COM2: ComPort = 0x2F8;
const COM3: ComPort = 0x3E8;
const COM4: ComPort = 0x2E8;

struct DivisorLatchValue(usize);

impl DivisorLatchValue {
    fn new(baud_rate: usize) -> Self {
        DivisorLatchValue(115200 / baud_rate)
    }
}

struct Serial {
    com_port: ComPort,
}

impl Serial {
    pub fn new(com_port: ComPort) -> Self {
        Serial { com_port }
    }

    /// Determine whether the current serial port can be used to write data on
    /// the serial bus.
    fn can_write(&self) -> bool {
        let register = self.line_status_register().read();
        register.empty_data_holding_registers() && register.empty_transmitter_holding_register()
    }

    /// Function to write a byte to a given serial bus.
    ///
    /// # Arguments
    ///
    /// * `byte` - The value to write on the serial bus.
    pub fn write_byte(&mut self, byte: u8) {
        while !self.can_write() {
            io::pause();
        }
        todo!("Implement the end of this function")
    }

    pub fn write_string(&mut self, s: &str) {
        for byte in s.bytes() {
            self.write_byte(byte);
        }
    }

    /// Get a line status register handle from the serial port.
    fn line_status_register(&self) -> LineStatusRegister {
        LineStatusRegister {
            address: self.com_port as u16 + 5,
        }
    }

    /// Get a line control register handle from the serial port.
    fn line_control_register(&self) -> LineControlRegister {
        LineControlRegister {
            address: self.com_port as u16 + 3,
        }
    }

    /// Get a transmitter holding buffer handle from the serial port.
    fn transmitter_holding_buffer(&self) -> TransmitterHoldingBuffer {
        TransmitterHoldingBuffer {
            address: self.com_port as u16,
            lcr: self.line_control_register(),
        }
    }

    /// Get a receiver buffer handle from the serial port.
    fn receiver_buffer(&self) -> ReceiverBuffer {
        ReceiverBuffer {
            address: self.com_port as u16,
            lcr: self.line_control_register(),
        }
    }
}

impl fmt::Write for Serial {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.write_string(s);
        Ok(())
    }
}
