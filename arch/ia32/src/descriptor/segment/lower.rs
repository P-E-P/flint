pub mod offsets {
    /// Limit low part offset (0:15)
    pub const LIMIT_LOW: u8 = 0;
    /// Base address low part offset (0:15)
    pub const BASE_LOW: u8 = 16;
}

pub mod bits {
    pub const LIMIT_LOW: u32 = 0xffff;
    pub const BASE_LOW: u32 = 0xffff;
}

pub mod flags {
    use super::{bits, offsets};
    /// Limit low part bitmask (0:15)
    pub const LIMIT_LOW: u32 = offset!(LIMIT_LOW);
    /// Base address low part bitmask (0:15)
    pub const BASE_LOW: u32 = offset!(BASE_LOW);
}

#[must_use]
#[repr(C, packed)]
#[derive(Default, Copy, Clone, Debug)]
pub struct Lower(pub u32);

impl Lower {
    /// Changes bits 0-15 in the lower part. Those bits correspond to the
    /// bits 0:15 of the segment limit.
    pub fn limit_low(&self, limit_low: u32) -> Self {
        Self(setbits!(self.0, limit_low, LIMIT_LOW))
    }

    /// Changes bits 16-31 in the lower part. This bits correspond to the
    /// bits 0:15 of the segment base address.
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
