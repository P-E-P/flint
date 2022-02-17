pub mod dlh;
pub mod dll;
pub mod fcr;
pub mod ier;
pub mod iir;
pub mod lcr;
pub mod lsr;
pub mod mcr;
pub mod msr;
pub mod rbr;
pub mod sr;
pub mod thr;

/// Implementation required for an UART register.
pub trait Register {
    /// The kind of value contained in the register.
    /// In some cases UART registers plain hold data without any prior knowledge
    /// of it's format (eg. data transmission buffers), but sometimes we know
    /// the structure of that data beforehand and we could arrange it using
    /// fields and flags. This type allow us to register the type contained in
    /// the register.
    type Value;
}

/// Implementation required for a readable UART register.
pub trait ReadRegister: Register {
    /// Read a value from the UART register and return it.
    fn read(&self) -> Self::Value;
}

/// Implementation required for a writable UART register.
pub trait WriteRegister: Register {
    /// Write a value to the UART register.
    fn write(&self, value: Self::Value);
}
