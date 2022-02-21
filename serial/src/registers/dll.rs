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

/// A structure containing the informations to identify a
/// [`DivisorLatchLowByte`] register along some utility values.
pub struct DivisorLatchLowByte {
    /// The port address of the [`DivisorLatchLowByte`].
    pub address: u16,
    /// A [`LineControlRegister`] from the same serial device to control the
    /// `DLAB` value.
    pub lcr: LineControlRegister,
}

impl Register for DivisorLatchLowByte {
    type Value = u8;
}

impl ReadRegister for DivisorLatchLowByte {
    fn read(&self) -> Self::Value {
        self.lcr.set_dlab(true);
        let result = Port::new(self.address).read();
        self.lcr.set_dlab(false);
        result
    }
}

impl WriteRegister for DivisorLatchLowByte {
    fn write(&self, value: Self::Value) {
        self.lcr.set_dlab(true);
        Port::new(self.address).write(value);
        self.lcr.set_dlab(false);
    }
}
