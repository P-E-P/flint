//! A module containing the operations accessible for a [`DivisorLatchLowByte`]
//! register.
//!
//! # Note
//!
//! This module assume that the `DLAB` value is always set to false as the most
//! used registers require an unset `DLAB` bit. It will therefore set the `DLAB`
//! value to `false` after modifying the register's value.

use super::lcr::LineControlRegister;
use arch::io::{
    port::Port,
    register::{ReadRegister, Register, WriteRegister},
};

/// The offset of the [`DivisorLatchLowByte`] relatively to the UART's base
/// address.
pub const DLL_OFFSET: u16 = 0;

/// A structure containing the informations to identify a
/// [`DivisorLatchLowByte`] register along some utility values.
pub struct DivisorLatchLowByte {
    /// The port of the [`DivisorLatchLowByte`].
    port: Port<u8>,
    /// A [`LineControlRegister`] from the same serial device to control the
    /// `DLAB` value.
    lcr: LineControlRegister,
}

impl DivisorLatchLowByte {
    pub fn new(address: u16, lcr: LineControlRegister) -> Self {
        DivisorLatchLowByte {
            port: Port::new(address),
            lcr,
        }
    }

    pub fn from_com(com: u16, lcr: LineControlRegister) -> Self {
        DivisorLatchLowByte::new(com + DLL_OFFSET, lcr)
    }
}

impl Register for DivisorLatchLowByte {
    type Value = u8;
}

impl ReadRegister for DivisorLatchLowByte {
    /// Get the divisor latch low byte from the UART's register.
    ///
    /// # Safety
    ///
    /// This function sets the divisor latch access bit for it's own
    /// operations before clearing it.
    unsafe fn read(&self) -> Self::Value {
        self.lcr.set_dlab(true);
        let result = self.port.read();
        self.lcr.set_dlab(false);
        result
    }
}

impl WriteRegister for DivisorLatchLowByte {
    /// Set the divisor latch low byte in the UART's register.
    ///
    /// # Safety
    ///
    /// This function sets the divisor latch access bit for it's own
    /// operations before clearing it.
    unsafe fn write(&self, value: Self::Value) {
        self.lcr.set_dlab(true);
        self.port.write(value);
        self.lcr.set_dlab(false);
    }
}
