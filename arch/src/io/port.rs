use super::register::{ReadRegister, Register, WriteRegister};
use crate::InOut;
use core::marker::PhantomData;

pub struct Port<T> {
    address: u16,
    phantom: PhantomData<T>,
}

impl<T> Port<T> {
    pub fn new(address: u16) -> Self {
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
    fn read(&self) -> Self::Value {
        unsafe { InOut::in_reg(self.address) }
    }
}

impl<T: InOut> WriteRegister for Port<T> {
    fn write(&self, value: Self::Value) {
        unsafe { InOut::out_reg(self.address, value) }
    }
}
