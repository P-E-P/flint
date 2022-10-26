use super::register::{ReadRegister, Register, WriteRegister};
use crate::arch::InOut;
use core::marker::PhantomData;

pub struct Port<T> {
    address: u16,
    phantom: PhantomData<T>,
}

impl<T> Port<T> {
    pub const fn new(address: u16) -> Self {
        Port {
            address,
            phantom: PhantomData,
        }
    }
}

impl<T: InOut> Register for Port<T> {
    type Value = T;
}

impl<T: InOut> ReadRegister for Port<T> {
    unsafe fn read(&self) -> Self::Value {
        InOut::in_reg(self.address)
    }
}

impl<T: InOut> WriteRegister for Port<T> {
    unsafe fn write(&self, value: Self::Value) {
        InOut::out_reg(self.address, value)
    }
}
