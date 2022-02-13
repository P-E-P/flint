#![no_std]

use io;
use core::fmt;
use registers::{
    dlh::DivisorLatchHighByte, dll::DivisorLatchLowByte, lsr::LineStatusRegister,
    rbr::ReceiverBuffer, thr::TransmitterHoldingBuffer, ReadRegister, Register, WriteRegister,
};

use registers::fcr::{self, FifoControl, FifoControlRegister};
use registers::ier::{self, InterruptEnable, InterruptEnableRegister};
use registers::iir::{self, InterruptIdentification, InterruptIdentificationRegister};
use registers::lcr::{self, LineControl, LineControlRegister};

mod registers;

type ComPort = usize;

pub const COM1: ComPort = 0x3F8;
pub const COM2: ComPort = 0x2F8;
pub const COM3: ComPort = 0x3E8;
pub const COM4: ComPort = 0x2E8;

pub struct Serial {
    com_port: ComPort,
}

impl Default for Serial {
    fn default() -> Self {
        let result = Serial::new(COM1);
        result.set_baud_rate(38400);
        // 8 bit length, no parity
        result.line_control_register().write(LineControl(
            lcr::flags::WordLengthBits::Eight as u8
                | lcr::flags::StopBit::OneStop as u8
                | lcr::flags::Parity::NoParity as u8,
        ));
        // Enable FIFO, clear, 14 bits
        result.fifo_control_register().write(FifoControl(
            fcr::flags::TriggerLevel::Itl14 as u8
                | fcr::flags::ENABLE_FIFOS
                | fcr::flags::CLEAR_TRANSMIT_FIFO
                | fcr::flags::CLEAR_RECEIVE_FIFO,
        ));
        // Enable interrupts
        result.interrupt_enable_register().write(InterruptEnable(
            ier::flags::RECEIVED_DATA_AVAILABLE_INTERRUPT,
        ));
        result
    }
}

impl Serial {
    pub fn new(com_port: ComPort) -> Self {
        Serial { com_port }

    }

    pub fn set_baud_rate(&self, baud_rate: usize) {
        let dlv = 115200 / baud_rate;
        self.divisor_latch_low_byte().write((dlv & 0xff) as u8);
        self.divisor_latch_high_byte().write((dlv >> 8) as u8);
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
    pub fn write_byte(&self, byte: u8) {
        while !self.can_write() {
            io::pause();
        }
        self.transmitter_holding_buffer().write(byte);
    }

    pub fn write_string(&self, s: &str) {
        for byte in s.bytes() {
            match byte {
                b'\n' => {
                    self.write_byte(b'\r');
                    self.write_byte(b'\n');
                }
                _ => self.write_byte(byte),
            }
        }
    }

    /// Get a line status register handle from the serial port.
    fn line_status_register(&self) -> LineStatusRegister {
        LineStatusRegister::from(self.com_port)
    }

    /// Get a line control register handle from the serial port.
    fn line_control_register(&self) -> LineControlRegister {
        LineControlRegister::from(self.com_port)
    }

    /// Get a transmitter holding buffer handle from the serial port.
    fn transmitter_holding_buffer(&self) -> TransmitterHoldingBuffer {
        TransmitterHoldingBuffer::from(self.com_port)
    }

    /// Get a receiver buffer handle from the serial port.
    fn receiver_buffer(&self) -> ReceiverBuffer {
        ReceiverBuffer::from(self.com_port)
    }

    /// Get a dll handle from the serial port.
    fn divisor_latch_low_byte(&self) -> DivisorLatchLowByte {
        DivisorLatchLowByte {
            address: self.com_port as u16,
            lcr: self.line_control_register(),
        }
    }

    /// Get a dlh handle from the serial port.
    fn divisor_latch_high_byte(&self) -> DivisorLatchHighByte {
        DivisorLatchHighByte {
            address: self.com_port as u16 + 1,
            lcr: self.line_control_register(),
        }
    }

    /// Get a fifo control register handle from the serial port.
    fn fifo_control_register(&self) -> FifoControlRegister {
        FifoControlRegister {
            address: self.com_port as u16 + 2,
        }
    }

    /// Get an interrupt enable register handle from the serial port.
    fn interrupt_enable_register(&self) -> InterruptEnableRegister {
        InterruptEnableRegister {
            address: self.com_port as u16 + 1,
        }
    }

    /// Get an interrupt identification register handle from the serial port.
    fn interrupt_identification_register(&self) -> InterruptIdentificationRegister {
        InterruptIdentificationRegister {
            address: self.com_port as u16 + 2,
        }
    }
}

impl fmt::Write for Serial {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.write_string(s);
        Ok(())
    }
}
