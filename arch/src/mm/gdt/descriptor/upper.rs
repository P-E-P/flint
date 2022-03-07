pub mod offsets {
    pub const BASE_MID: u8 = 0;
    pub const TYPE: u8 = 8;
    pub const S: u8 = 12;
    pub const DPL: u8 = 13;
    pub const P: u8 = 15;
    pub const LIMIT_HIGH: u8 = 16;
    pub const AVL: u8 = 20;
    pub const L: u8 = 21;
    pub const DB: u8 = 22;
    pub const G: u8 = 23;
    pub const BASE_HIGH: u8 = 24;
}

pub mod bits {
    pub const BASE_MID: u32 = 0xff;
    pub const TYPE: u32 = 0xf;
    pub const S: u32 = 0x1;
    pub const DPL: u32 = 0x3;
    pub const P: u32 = 0x1;
    pub const LIMIT_HIGH: u32 = 0xf;
    pub const AVL: u32 = 0x1;
    pub const L: u32 = 0x1;
    pub const DB: u32 = 0x1;
    pub const G: u32 = 0x1;
    pub const BASE_HIGH: u32 = 0xff;
}

pub mod flags {
    use super::{bits, offsets};
    pub const BASE_MID: u32 = offset!(BASE_MID);
    pub const TYPE: u32 = offset!(TYPE);
    pub const S: u32 = offset!(S);
    pub const DPL: u32 = offset!(DPL);
    pub const P: u32 = offset!(P);
    pub const LIMIT_HIGH: u32 = offset!(LIMIT_HIGH);
    pub const AVL: u32 = offset!(AVL);
    pub const L: u32 = offset!(L);
    pub const DB: u32 = offset!(DB);
    pub const G: u32 = offset!(G);
    pub const BASE_HIGH: u32 = offset!(BASE_HIGH);
}

#[repr(C, packed)]
#[derive(Default, Copy, Clone, Debug)]
pub struct Upper(pub u32);

impl Upper {
    pub fn segment_type(self, value: u32) -> Self {
        Self(setbits!(self.0, value, TYPE))
    }

    pub fn descriptor_type(self, value: u32) -> Self {
        Self(setbits!(self.0, value, S))
    }

    pub fn privilege_level(self, value: u32) -> Self {
        Self(setbits!(self.0, value, DPL))
    }

    pub fn present(self, value: u32) -> Self {
        Self(setbits!(self.0, value, P))
    }

    pub fn available(self, value: u32) -> Self {
        Self(setbits!(self.0, value, AVL))
    }

    pub fn ia32e_mode(self, value: u32) -> Self {
        Self(setbits!(self.0, value, L))
    }

    pub fn default_operation_size(self, value: u32) -> Self {
        Self(setbits!(self.0, value, DB))
    }

    pub fn granularity(self, value: u32) -> Self {
        Self(setbits!(self.0, value, G))
    }

    pub fn base_mid(self, value: u32) -> Self {
        Self(setbits!(self.0, value, BASE_MID))
    }

    pub fn base_high(self, value: u32) -> Self {
        Self(setbits!(self.0, value, BASE_HIGH))
    }

    pub fn limit_high(self, value: u32) -> Self {
        Self(setbits!(self.0, value, LIMIT_HIGH))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test_case]
    fn segment_type_data() {
        let value = Upper(0).segment_type(0x1).0;
        assert_eq!(value, 0x100);
    }

    #[test_case]
    fn segment_type_code() {
        let value = Upper(0).segment_type(0x8).0;
        assert_eq!(value, 0x800);
    }

    #[test_case]
    fn segment_type_overflow() {
        let value = Upper(0).segment_type(0xf1).0;
        assert_eq!(value, 0x100);
    }

    #[test_case]
    fn desc_type_system() {
        let value = Upper(0).descriptor_type(0).0;
        assert_eq!(value, 0);
    }

    #[test_case]
    fn desc_type_code_data() {
        let value = Upper(0).descriptor_type(1).0;
        assert_eq!(value, 0x1000);
    }

    #[test_case]
    fn desc_type_overflow() {
        let value = Upper(0).descriptor_type(2).0;
        assert_eq!(value, 0);
    }

    #[test_case]
    fn privilege_level_kernel() {
        let value = Upper(0).privilege_level(0).0;
        assert_eq!(value, 0);
    }

    #[test_case]
    fn privilege_level_userland() {
        let value = Upper(0).privilege_level(3).0;
        assert_eq!(value, 0x6000);
    }

    #[test_case]
    fn privilege_level_overflow() {
        let value = Upper(0).privilege_level(4).0;
        assert_eq!(value, 0x0000);
    }

    #[test_case]
    fn present() {
        let value = Upper(0).present(1).0;
        assert_eq!(value, 0x8000);
    }

    #[test_case]
    fn not_present() {
        let value = Upper(0).present(0).0;
        assert_eq!(value, 0)
    }

    #[test_case]
    fn present_overflow() {
        let value = Upper(0).present(3).0;
        assert_eq!(value, 0x8000);
    }

    #[test_case]
    fn available_on() {
        let value = Upper(0).available(1).0;
        assert_eq!(value, 0x100000);
    }

    #[test_case]
    fn available_off() {
        let value = Upper(0).available(0).0;
        assert_eq!(value, 0x000000);
    }

    #[test_case]
    fn available_overflow() {
        let value = Upper(0).available(2).0;
        assert_eq!(value, 0x000000);
    }

    #[test_case]
    fn ia32e_mode_on() {
        let value = Upper(0).ia32e_mode(1).0;
        assert_eq!(value, 0x200000);
    }

    #[test_case]
    fn ia32e_mode_off() {
        let value = Upper(0).ia32e_mode(0).0;
        assert_eq!(value, 0x000000);
    }

    #[test_case]
    fn ia32e_mode_overflow() {
        let value = Upper(0).ia32e_mode(3).0;
        assert_eq!(value, 0x200000);
    }

    #[test_case]
    fn default_op_size_16() {
        let value = Upper(0).default_operation_size(0).0;
        assert_eq!(value, 0)
    }

    #[test_case]
    fn default_op_size_32() {
        let value = Upper(0).default_operation_size(1).0;
        assert_eq!(value, 0x400000)
    }

    #[test_case]
    fn default_op_size_overflow() {
        let value = Upper(0).default_operation_size(2).0;
        assert_eq!(value, 0)
    }

    #[test_case]
    fn granularity_byte() {
        let value = Upper(0).granularity(0).0;
        assert_eq!(value, 0)
    }

    #[test_case]
    fn granularity_4k() {
        let value = Upper(0).granularity(1).0;
        assert_eq!(value, 0x800000)
    }

    #[test_case]
    fn granularity_overflow() {
        let value = Upper(0).granularity(2).0;
        assert_eq!(value, 0)
    }

    #[test_case]
    fn base_mid() {
        let value = Upper(0).base_mid(42).0;
        assert_eq!(value, 42)
    }

    #[test_case]
    fn base_mid_overflow() {
        let value = Upper(0).base_mid(0x1ff).0;
        assert_eq!(value, 0xff)
    }

    #[test_case]
    fn base_high() {
        let value = Upper(0).base_high(1).0;
        assert_eq!(value, 0x1000000);
    }

    #[test_case]
    fn base_high_overflow() {
        let value = Upper(0).base_high(0x100).0;
        assert_eq!(value, 0);
    }

    #[test_case]
    fn limit_high() {
        let value = Upper(0).limit_high(1).0;
        assert_eq!(value, 0x10000);
    }

    #[test_case]
    fn limit_high_overflow() {
        let value = Upper(0).limit_high(0x10).0;
        assert_eq!(value, 0);
    }
}
