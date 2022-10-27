//! A module containing the implementation and tests for the [`Configuration`]
//! structure.
use super::Granularity;
use crate::utils::bitfield::*;
use core::fmt;

/// A structure representing the bits 16 to 23 from a
/// [TssDescriptor](super::TssDescriptor) structure.
///
/// It gather the following fields:
/// - Segment limit bits 16 to 19
/// - Available bit (AVL)
/// - Granularity bit (G)
#[derive(Default, Copy, Clone)]
pub struct Configuration(u8);

/// The set of all field offsets for the [`Configuration`] structure.
mod offset {
    /// Offset of the available (AVL) bit within the
    /// [Configuration](super::Configuration) structure.
    pub const AVL: usize = 4;
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
        Self(self.0.set_bits(..4, limit))
    }

    /// Change a [`Configuration`]'s available bit value.
    ///
    /// # Arguments
    ///
    /// * `value` - The desired bit value, a value of `true` will store a `1`,
    /// `false` will store the bit `0`.
    pub fn available(self, value: bool) -> Self {
        Self(self.0.set_bit(offset::AVL, value))
    }

    /// Change a [`Configuration`]'s granularity.
    ///
    /// # Arguments
    ///
    /// * `granularity` - The desired granularity.
    pub fn granularity(self, granularity: Granularity) -> Self {
        Self(self.0.set_bit(offset::G, granularity.into()))
    }
}

impl fmt::Display for Configuration {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "AVL: {}\nGranularity: {}",
            self.0.get_bit(offset::AVL),
            self.0.get_bit(offset::G)
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test_case]
    fn structure_size() {
        use core::mem::size_of;
        assert_eq!(size_of::<Configuration>(), 1);
    }

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
