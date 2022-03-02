pub mod offsets {
    pub const LIMIT_LOW: u8 = 0;
    pub const BASE_LOW: u8 = 16;
}

pub mod bits {
    pub const LIMIT_LOW: u32 = 0xffff;
    pub const BASE_LOW: u32 = 0xffff;
}

pub mod flags {
    use super::{bits, offsets};
    pub const LIMIT_LOW: u32 = offset!(LIMIT_LOW);
    pub const BASE_LOW: u32 = offset!(BASE_LOW);
}

#[repr(C, packed)]
#[derive(Default, Copy, Clone)]
pub struct Lower(pub u32);

impl Lower {
    pub fn limit_low(&self, limit_low: u32) -> Self {
        Self(setbits!(self.0, limit_low, LIMIT_LOW))
    }

    pub fn base_low(&self, base_low: u32) -> Self {
        Self(setbits!(self.0, base_low, BASE_LOW))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test_case]
    fn limit_low_null() {
        let value = Lower(0).limit_low(0).0;
        assert_eq!(value, 0);
    }

    #[test_case]
    fn limit_low() {
        let value = Lower(0).limit_low(0xff).0;
        assert_eq!(value, 0xff);
    }

    #[test_case]
    fn base_low_null() {
        let value = Lower(0).base_low(0).0;
        assert_eq!(value, 0);
    }

    #[test_case]
    fn base_low() {
        let value = Lower(0).base_low(0xff).0;
        assert_eq!(value, 0xff0000);
    }
}
