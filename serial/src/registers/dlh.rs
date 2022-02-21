//! A module containing the operations accessible for a [`DivisorLatchHighByte`]
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

/// The offset of the [`DivisorLatchHighByte`] relatively to the UART's base
/// address.
pub const DLH_OFFSET: u16 = 1;

/// A structure containing the informations to identify a
/// [`DivisorLatchHighByte`] register along some utility values.
pub struct DivisorLatchHighByte {
    /// The port address of the [`DivisorLatchHighByte`].
    address: u16,
    /// A [`LineControlRegister`] from the same serial device to control the
    /// `DLAB` value.
    lcr: LineControlRegister,
}

impl DivisorLatchHighByte {
    pub fn new(address: u16, lcr: LineControlRegister) -> Self {
        DivisorLatchHighByte { address, lcr }
    }

    pub fn from_com(com: u16, lcr: LineControlRegister) -> Self {
        DivisorLatchHighByte::new(com + DLH_OFFSET, lcr)
    }
}

impl Register for DivisorLatchHighByte {
    type Value = u8;
}

impl WriteRegister for DivisorLatchHighByte {
    fn write(&self, value: Self::Value) {
        self.lcr.set_dlab(true);
        Port::<u8>::new(self.address).write(value);
        self.lcr.set_dlab(false);
    }
}

impl ReadRegister for DivisorLatchHighByte {
    fn read(&self) -> Self::Value {
        self.lcr.set_dlab(true);
        let result = Port::<u8>::new(self.address).read();
        self.lcr.set_dlab(false);
        result
    }
}
