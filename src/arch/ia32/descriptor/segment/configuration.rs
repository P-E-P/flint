//! A module containing the implementation and tests for the [`Configuration`]
//! structure.
use super::{DefaultOperationSize, Granularity};
use bit_field::BitField;
use core::fmt;

/// A structure representing the bits 16 to 24 from a
/// [SegmentDescriptor](super::SegmentDescriptor) structure.
///
/// It gathers the following fields:
/// - Segment limit bits 16 to 19
/// - Available bit (AVL)
/// - 64 bit mode bit (L)
/// - Default operation size bit (D/B)
/// - Granularity bit (G)
#[derive(Default, Copy, Clone)]
pub struct Configuration(u8);

/// The set of all field offsets for the [`Configuration`] structure.
mod offset {
    /// Offset of the AVL bit within the [Configuration](super::Configuration) structure.
    pub const AVL: usize = 4;

    /// Offset of the 64bit code segment (L) bit within the
    /// [Configuration](super::Configuration) structure.
    pub const L: usize = 5;

    /// Offset of the default operation size (D/B) bit within the
    /// [Configuration](super::Configuration) structure.
    pub const D_B: usize = 6;

    /// Offset of the granularity (G) bit within the
    /// [Configuration](super::Configuration) structure.
    pub const G: usize = 7;
}

impl Configuration {
    /// Get the bits (19:16) of the limit value stored in the configuration
    /// field.
    pub fn get_limit(&self) -> u8 {
        self.0.get_bits(..4)
    }

    /// Change a [`Configuration`]'s limit bits (19:16) to a new value.
    ///
    /// # Arguments
    ///
    /// * `limit` - The limit value bits to set in the structure.
    ///
    /// # Panics
    ///
    /// This method will panic if the given limit value is greater than 15
    /// because [`Configuration`] structure hold only bits 16 to 19 of the
    /// segment descriptor structure.
    pub fn limit(self, limit: u8) -> Self {
        Self(*self.0.clone().set_bits(..4, limit))
    }

    /// Change a [`Configuration`]'s available bit value.
    ///
    /// # Arguments
    ///
    /// * `value` - The desired bit value, a value of `true` will store a `1`,
    /// `false` will store the bit `0`.
    pub fn available(self, value: bool) -> Self {
        Self(*self.0.clone().set_bit(offset::AVL, value))
    }

    /// Change a [`Configuration`]'s current mode to 32 bits or 64 bits.
    ///
    /// # Arguments
    ///
    /// * `mode` - The desired mode, `true` for 64 bits and `false` for 32 bits.
    ///
    /// # Note
    ///
    /// This method will overwrite any previous call to
    /// [default_operation_size](Configuration#method.default_operation_size)
    /// if the mode is set to 64bits as it required the bit `D/B` set to `0`
    /// (cf. Intel Volume III 3.4.5).
    pub fn ia32e_mode(self, mode: bool) -> Self {
        let mut result = self.0;
        //If L-bit is set, then D-bit must be cleared
        // cf. Intel 3.4.5 "L (64 bit code segment) flag"
        if mode {
            result.set_bit(offset::D_B, false);
        }
        result.set_bit(offset::L, mode);
        Self(result)
    }

    /// Change a [`Configuration`]'s default operation size.
    ///
    /// # Arguments
    ///
    /// * `size` - The desired mode operation size.
    pub fn default_operation_size(self, size: DefaultOperationSize) -> Self {
        Self(*self.0.clone().set_bit(offset::D_B, size.into()))
    }

    /// Change a [`Configuration`]'s granularity.
    ///
    /// # Arguments
    ///
    /// * `granularity` - The desired granularity.
    pub fn granularity(self, granularity: Granularity) -> Self {
        Self(*self.0.clone().set_bit(offset::G, granularity.into()))
    }
}

impl fmt::Display for Configuration {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "AVL: {}\n64-bit code segment: {}\nDefault operation size: {}\nGranularity: {}",
            self.0.get_bit(offset::AVL),
            self.0.get_bit(offset::L),
            self.0.get_bit(offset::D_B),
            self.0.get_bit(offset::G)
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test_case]
    fn default_values() {
        let conf = Configuration::default();
        assert_eq!(conf.0, 0);
    }

    #[test_case]
    fn limit_bits() {
        let limit = 12;
        let conf = Configuration::default().limit(limit);
        assert_eq!(conf.0.get_bits(0..4), limit);
    }

    #[test_case]
    fn limit_getter() {
        let limit = 0xf;
        let conf = Configuration::default().limit(limit);
        assert_eq!(conf.get_limit(), limit);
    }

    #[test_case]
    fn available() {
        let conf = Configuration::default().available(true);
        assert_eq!(conf.0.get_bit(4), true);
    }

    #[test_case]
    fn not_available() {
        let conf = Configuration::default().available(false);
        assert_eq!(conf.0.get_bit(4), false);
    }

    #[test_case]
    fn ia32e_mode() {
        let conf = Configuration::default().ia32e_mode(true);
        assert_eq!(conf.0.get_bit(5), true);
        // Intel 3.4.5/L (64-bit code segment) flag
        // > If L-bit is set, then D-bit must be cleared.
        assert_eq!(conf.0.get_bit(6), false);
    }

    #[test_case]
    fn db_16bits() {
        use DefaultOperationSize::Segment16Bits;
        let conf = Configuration::default().default_operation_size(Segment16Bits);
        assert_eq!(conf.0.get_bit(6), false);
    }

    #[test_case]
    fn db_32bits() {
        use DefaultOperationSize::Segment32Bits;
        let conf = Configuration::default().default_operation_size(Segment32Bits);
        assert_eq!(conf.0.get_bit(6), true);
    }

    #[test_case]
    fn granularity_byte() {
        use Granularity::Byte;
        let conf = Configuration::default().granularity(Byte);
        assert_eq!(conf.0.get_bit(7), false);
    }

    #[test_case]
    fn granularity_4kbyte() {
        use Granularity::FourKByte;
        let conf = Configuration::default().granularity(FourKByte);
        assert_eq!(conf.0.get_bit(7), true);
    }
}
