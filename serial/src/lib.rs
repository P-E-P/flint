#![no_std]

use core::fmt;
use io;

use registers::{
    dll::DivisorLatchLowByte,
    dlh::DivisorLatchHighByte,
    fcr::{self, FifoControl, FifoControlRegister},
    ier::{self, InterruptEnable, InterruptEnableRegister},
    iir::InterruptIdentificationRegister,
    lcr::{self, LineControl, LineControlRegister},
    lsr::LineStatusRegister,
    mcr::ModemControlRegister,
    msr::ModemStatusRegister,
    rbr::ReceiverBuffer,
    sr::ScratchRegister,
    thr::TransmitterHoldingBuffer,
    ReadRegister, WriteRegister,
};

pub mod registers;

type ComPort = usize;

/// UART IO port 1 address
pub const COM1: ComPort = 0x3F8;
/// UART IO port 2 address
pub const COM2: ComPort = 0x2F8;
/// UART IO port 3 address
pub const COM3: ComPort = 0x3E8;
/// UART IO port 4 address
pub const COM4: ComPort = 0x2E8;

/// An option containing the default serial to use for communication.
static mut DEFAULT: Option<Serial> = None;

/// Retrieve a mutable reference to the default serial, initialize it before
/// returning it in case it was not set previously.
pub fn get_default() -> &'static mut Serial {
    unsafe {
        if DEFAULT.is_none() {
            DEFAULT = Some(Serial::default());
        }
        DEFAULT.as_mut().unwrap()
    }
}

/// A structure representing an UART device accessible through a given IO port.
pub struct Serial {
    /// The UART port address
    com_port: ComPort,
}

impl Default for Serial {
    /// Return the default UART configuration which is:
    /// - Port [`COM1`]
    /// - Baud rate of 38400
    /// - 8 bits word length
    /// - No parity
    /// - Interrupt trigger level of 14
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
    /// Create a new Serial structure from a COM port address.
    ///
    /// # Note
    /// This constructor does not initialize anything nor configure the UART
    /// in any way.
    pub fn new(com_port: ComPort) -> Self {
        Serial { com_port }
    }

    /// Set the transfer speed of an UART by setting it's DLL and DLH registers.
    pub fn set_baud_rate(&self, baud_rate: usize) {
        let dlv = 115200 / baud_rate;
        self.divisor_latch_low_byte().write((dlv & 0xff) as u8);
        self.divisor_latch_high_byte().write((dlv >> 8) as u8);
    }

    /// Determine whether data can be written safely to the data buffer register
    /// of the UART.
    fn can_write(&self) -> bool {
        let register = self.line_status_register().read();
        register.empty_data_holding_registers() && register.empty_transmitter_holding_register()
    }

    /// Write a byte to the serial bus of a given Serial structure. Waits for
    /// the UART to be ready before sending it.
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

    /// Write a string to the data buffer of the [`Serial`]. This function will
    /// wait for the UART to be ready before sending any byte.
    ///
    /// # Arguments
    ///
    /// * `s` - The string to write.
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

    /// Get a [`LineStatusRegister`] handle from the serial port.
    pub fn line_status_register(&self) -> LineStatusRegister {
        LineStatusRegister::from(self.com_port)
    }

    /// Get a [`LineControlRegister`] handle from the serial port.
    pub fn line_control_register(&self) -> LineControlRegister {
        LineControlRegister::from(self.com_port)
    }

    /// Get a [`TransmitterHoldingBuffer`] handle from the serial port.
    pub fn transmitter_holding_buffer(&self) -> TransmitterHoldingBuffer {
        TransmitterHoldingBuffer::from(self.com_port)
    }

    /// Get a [`ReceiverBuffer`] handle from the serial port.
    pub fn receiver_buffer(&self) -> ReceiverBuffer {
        ReceiverBuffer::from(self.com_port)
    }

    /// Get a [`DivisorLatchLowByte`] handle from the serial port.
    pub fn divisor_latch_low_byte(&self) -> DivisorLatchLowByte {
        DivisorLatchLowByte {
            address: self.com_port as u16,
            lcr: self.line_control_register(),
        }
    }

    /// Get a [`DivisorLatchHighByte`] handle from the serial port.
    pub fn divisor_latch_high_byte(&self) -> DivisorLatchHighByte {
        DivisorLatchHighByte {
            address: self.com_port as u16 + 1,
            lcr: self.line_control_register(),
        }
    }

    /// Get a [`FifoControlRegister`] handle from the serial port.
    pub fn fifo_control_register(&self) -> FifoControlRegister {
        FifoControlRegister::from(self.com_port)
    }

    /// Get an [`InterruptEnableRegister`] handle from the serial port.
    pub fn interrupt_enable_register(&self) -> InterruptEnableRegister {
        InterruptEnableRegister::from(self.com_port)
    }

    /// Get an [`InterruptIdentificationRegister`] handle from the serial port.
    pub fn interrupt_identification_register(&self) -> InterruptIdentificationRegister {
        InterruptIdentificationRegister::from(self.com_port)
    }

    /// Get a [`ModemStatusRegister`] handle from the serial port.
    pub fn modem_status_register(&self) -> ModemStatusRegister {
        ModemStatusRegister::from(self.com_port)
    }

    /// Get a [`ModemControlRegister`] handle from the serial port.
    pub fn model_control_register(&self) -> ModemControlRegister {
        ModemControlRegister::from(self.com_port)
    }

    /// Get a [`ScratchRegister`] handle from the serial port.
    pub fn scratch_register(&self) -> ScratchRegister {
        ScratchRegister::from(self.com_port)
    }
}

impl fmt::Write for Serial {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.write_string(s);
        Ok(())
    }
}
