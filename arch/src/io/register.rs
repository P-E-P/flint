pub trait Register {
    /// The register's inner value type.
    type Value;
}

/// Implementation required for a readable register.
pub trait ReadRegister: Register {
    /// Read a value from the register and return it.
    unsafe fn read(&self) -> Self::Value;
}

/// Implementation required for a writable register.
pub trait WriteRegister: Register {
    /// Write a value to the register.
    unsafe fn write(&self, value: Self::Value);
}
