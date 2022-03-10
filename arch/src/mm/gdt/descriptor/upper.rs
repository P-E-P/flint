/// The collection of all segment descriptor's upper part field offsets.
pub mod offsets {
    /// Base address middle part offset (16:23)
    pub const BASE_MID: u8 = 0;
    /// Segment type offset
    pub const TYPE: u8 = 8;
    /// Descriptor type offset
    pub const S: u8 = 12;
    /// Descriptor privilege level offset
    pub const DPL: u8 = 13;
    /// Segment present bit offset
    pub const P: u8 = 15;
    /// Limit high part offset (16:19)
    pub const LIMIT_HIGH: u8 = 16;
    /// Available system bit offset
    pub const AVL: u8 = 20;
    /// 64-bit code segment offset
    pub const L: u8 = 21;
    /// Default operation size bit offset
    pub const DB: u8 = 22;
    /// Granularity bit offset
    pub const G: u8 = 23;
    /// Base address high part offset (24:31)
    pub const BASE_HIGH: u8 = 24;
}

/// The collection of all segment descriptor's upper part bits representation.
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

/// The collection of all segment descriptor's upper part flags.
pub mod flags {
    use super::{bits, offsets};
    /// Base address middle part bitmask (16:23)
    pub const BASE_MID: u32 = offset!(BASE_MID);
    /// Segment type bitmask
    pub const TYPE: u32 = offset!(TYPE);
    /// Descriptor type bitmask
    pub const S: u32 = offset!(S);
    /// Descriptor privilege level bitmask
    pub const DPL: u32 = offset!(DPL);
    /// Segment present bitmask
    pub const P: u32 = offset!(P);
    /// Limit high part bitmask (16:19)
    pub const LIMIT_HIGH: u32 = offset!(LIMIT_HIGH);
    /// Available system bit bitmask
    pub const AVL: u32 = offset!(AVL);
    /// 64-bit code segment bitmask
    pub const L: u32 = offset!(L);
    /// Default operation size bitmask
    pub const DB: u32 = offset!(DB);
    /// Granularity bitmask
    pub const G: u32 = offset!(G);
    /// Base address high part bitmask (24:31)
    pub const BASE_HIGH: u32 = offset!(BASE_HIGH);
}

#[repr(C, packed)]
#[derive(Default, Copy, Clone, Debug)]
pub struct Upper(pub u32);

impl Upper {
    /// Changes the segment type
    /// ```
    /// 31          24 23 22 21 20 19  16 15 14 13 12 11 8 7   0
    /// +-------------+--+--+--+--+------+--+-----+--+----+-----+
    /// |             |  |  |  |  |      |  |XXXXX|  |    |     |
    /// |             |  |  |  |  |      |  |XXXXX|  |    |     |
    /// +-------------+--+--+--+--+------+--+-----+--+----+-----+
    /// ```
    pub fn segment_type(self, value: u32) -> Self {
        Self(setbits!(self.0, value, TYPE))
    }

    /// Changes the bit 12 `S` in the upper part.
    /// ```
    /// 31          24 23 22 21 20 19  16 15 14 13 12 11 8 7   0
    /// +-------------+--+--+--+--+------+--+-----+--+----+-----+
    /// |             |  |  |  |  |      |  |     |XX|    |     |
    /// |             |  |  |  |  |      |  |     |XX|    |     |
    /// +-------------+--+--+--+--+------+--+-----+--+----+-----+
    /// ```
    pub fn descriptor_type(self, value: u32) -> Self {
        Self(setbits!(self.0, value, S))
    }

    /// Changes the bits 13-14 `DPL` in the upper part.
    /// ```
    /// 31          24 23 22 21 20 19  16 15 14 13 12 11 8 7   0
    /// +-------------+--+--+--+--+------+--+-----+--+----+-----+
    /// |             |  |  |  |  |      |  |XXXXX|  |    |     |
    /// |             |  |  |  |  |      |  |XXXXX|  |    |     |
    /// +-------------+--+--+--+--+------+--+-----+--+----+-----+
    /// ```
    pub fn privilege_level(self, value: u32) -> Self {
        Self(setbits!(self.0, value, DPL))
    }

    /// Change the bit 15 `P` in the upper part.
    /// ```
    /// 31          24 23 22 21 20 19  16 15 14 13 12 11 8 7   0
    /// +-------------+--+--+--+--+------+--+-----+--+----+-----+
    /// |             |  |  |  |  |      |XX|     |  |    |     |
    /// |             |  |  |  |  |      |XX|     |  |    |     |
    /// +-------------+--+--+--+--+------+--+-----+--+----+-----+
    /// ```
    pub fn present(self, value: u32) -> Self {
        Self(setbits!(self.0, value, P))
    }

    /// Changes the bit 20 `AVL` in the upper part.
    /// ```
    /// 31          24 23 22 21 20 19  16 15 14 13 12 11 8 7   0
    /// +-------------+--+--+--+--+------+--+-----+--+----+-----+
    /// |             |  |  |  |XX|      |  |     |  |    |     |
    /// |             |  |  |  |XX|      |  |     |  |    |     |
    /// +-------------+--+--+--+--+------+--+-----+--+----+-----+
    /// ```
    pub fn available(self, value: u32) -> Self {
        Self(setbits!(self.0, value, AVL))
    }

    /// Changes the bit 21 `L` in the upper part.
    /// ```
    /// 31          24 23 22 21 20 19  16 15 14 13 12 11 8 7   0
    /// +-------------+--+--+--+--+------+--+-----+--+----+-----+
    /// |             |  |  |XX|  |      |  |     |  |    |     |
    /// |             |  |  |XX|  |      |  |     |  |    |     |
    /// +-------------+--+--+--+--+------+--+-----+--+----+-----+
    /// ```
    pub fn ia32e_mode(self, value: u32) -> Self {
        Self(setbits!(self.0, value, L))
    }

    /// Changes the bit 22 `D/B` in the upper part.
    /// ```
    /// 31          24 23 22 21 20 19  16 15 14 13 12 11 8 7   0
    /// +-------------+--+--+--+--+------+--+-----+--+----+-----+
    /// |             |  |XX|  |  |      |  |     |  |    |     |
    /// |             |  |XX|  |  |      |  |     |  |    |     |
    /// +-------------+--+--+--+--+------+--+-----+--+----+-----+
    /// ```
    pub fn default_operation_size(self, value: u32) -> Self {
        Self(setbits!(self.0, value, DB))
    }

    /// Changes the bit 23 `G` in the upper part.
    /// ```
    /// 31          24 23 22 21 20 19  16 15 14 13 12 11 8 7   0
    /// +-------------+--+--+--+--+------+--+-----+--+----+-----+
    /// |             |XX|  |  |  |      |  |     |  |    |     |
    /// |             |XX|  |  |  |      |  |     |  |    |     |
    /// +-------------+--+--+--+--+------+--+-----+--+----+-----+
    /// ```
    pub fn granularity(self, value: u32) -> Self {
        Self(setbits!(self.0, value, G))
    }

    /// Changes the bits 0-7 in the upper part.
    /// ```
    /// 31          24 23 22 21 20 19  16 15 14 13 12 11 8 7   0
    /// +-------------+--+--+--+--+------+--+-----+--+----+-----+
    /// |             |  |  |  |  |      |  |     |  |    |XXXXX|
    /// |             |  |  |  |  |      |  |     |  |    |XXXXX|
    /// +-------------+--+--+--+--+------+--+-----+--+----+-----+
    /// ```
    pub fn base_mid(self, value: u32) -> Self {
        Self(setbits!(self.0, value, BASE_MID))
    }

    /// Changes the bits 24-31 in the upper part.
    /// ```
    /// 31          24 23 22 21 20 19  16 15 14 13 12 11 8 7   0
    /// +-------------+--+--+--+--+------+--+-----+--+----+-----+
    /// |XXXXXXXXXXXXX|  |  |  |  |      |  |     |  |    |     |
    /// |XXXXXXXXXXXXX|  |  |  |  |      |  |     |  |    |     |
    /// +-------------+--+--+--+--+------+--+-----+--+----+-----+
    /// ```
    pub fn base_high(self, value: u32) -> Self {
        Self(setbits!(self.0, value, BASE_HIGH))
    }

    /// Changes the bits 16-19 in the upper part.
    /// ```
    /// 31          24 23 22 21 20 19  16 15 14 13 12 11 8 7   0
    /// +-------------+--+--+--+--+------+--+-----+--+----+-----+
    /// |             |  |  |  |  |XXXXXX|  |     |  |    |     |
    /// |             |  |  |  |  |XXXXXX|  |     |  |    |     |
    /// +-------------+--+--+--+--+------+--+-----+--+----+-----+
    /// ```
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
