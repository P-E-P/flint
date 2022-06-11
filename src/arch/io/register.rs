pub trait Register {
    /// The register's inner value type.
    type Value;
}

/// Implementation required for a readable register.
pub trait ReadRegister: Register {
    /// Read a value from the register and return it.
    ///
    /// # Safety
    ///
    /// Reading from a register might be unsafe depending on the context, double
    /// check your code.
    unsafe fn read(&self) -> Self::Value;
}

/// Implementation required for a writable register.
pub trait WriteRegister: Register {
    /// Write a value to the register.
    ///
    /// # Safety
    ///
    /// Writing to a register might be unsafe depending on the context, double
    /// check your code.
    unsafe fn write(&self, value: Self::Value);
}
