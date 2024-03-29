use core::ops::{Bound, Range, RangeBounds};

#[const_trait]
pub trait ConstBitGetter: Sized {
    const TYPE_SIZE: usize = core::mem::size_of::<Self>() * 8;

    /// Return a boolean representing the state of the idx'th bit.
    ///
    /// # Arguments
    ///
    /// * `idx` - The index of the bit, 0 being the rightmost one.
    ///
    /// # Panics
    ///
    /// This method will panics if the given index is out of range.
    fn get_bit(self, idx: usize) -> bool;

    /// Return a value representing the state of a range of bits.
    ///
    /// # Arguments
    ///
    /// * `start` - The range's starting offset.
    /// * `end` - The range's excluded ending offset.
    ///
    /// # Panics
    ///
    /// This method will panics if the given range is invalid.
    fn get_range(self, start: usize, end: usize) -> Self;
}

#[const_trait]
pub trait ConstBitSetter: Sized {
    /// Set the idx'th bit.
    ///
    /// # Arguments
    ///
    /// * `idx` - The index of the bit, 0 being the rightmost one.
    /// * `value` - A boolean representing the desired state of the bit.
    ///
    /// # Panics
    ///
    /// This method will panics if the given index is out of range.
    fn set_bit(self, idx: usize, value: bool) -> Self;

    /// Set a range of bits to the given value.
    ///
    /// # Arguments
    ///
    /// * `start` - The range's starting offset.
    /// * `end` - The range's excluded ending offset.
    ///
    /// # Panics
    ///
    /// This method will panics if the given range is invalid.
    fn set_range(self, start: usize, end: usize, value: Self) -> Self;
}

macro_rules! impl_constbitgetter {

    (for $($t:ty),+) => {
        $(impl const ConstBitGetter for $t {

            fn get_bit(self, idx: usize) -> bool {
                if idx >= Self::TYPE_SIZE {
                    panic!("Index out of range for bit fields");
                }

                (self & (1 << idx)) != 0
            }


            fn get_range(self, start: usize, end: usize) -> Self {
                if start >= Self::TYPE_SIZE || end > Self::TYPE_SIZE {
                    panic!("Range out of range for bit fields");
                }
                if start >= end {
                    panic!("End bound should be greater than lower one for bit fields.");
                }

                // Shift away high bits then lower ones
                let val = self << (Self::TYPE_SIZE - end);
                val >> ((Self::TYPE_SIZE - end) + start)
            }
        })*
    }
}

macro_rules! impl_constbitsetter {
    (for $($t:ty),+) => {
        $(impl const ConstBitSetter for $t {

            fn set_bit(self, idx: usize, value: bool) -> Self {
                if idx >= Self::TYPE_SIZE {
                    panic!("Index out of range for bit fields");
                }

                if value { self | (1 << idx) } else { self & (!(1 << idx)) }
            }


            fn set_range(self, start: usize, end: usize, value: Self) -> Self {
                if start >= Self::TYPE_SIZE || end > Self::TYPE_SIZE {
                    panic!("Range out of range for bit fields");
                }
                if start >= end {
                    panic!("End bound should be greater than lower one for bit fields.");
                }
                if (end - start) < Self::TYPE_SIZE && (value >> (end - start)) > 0 {
                    panic!("Value bigger than range.");
                }

                let mask: Self = !(!0 << (Self::TYPE_SIZE - end) >>
                                    (Self::TYPE_SIZE - end) >>
                                    start << start);
                (self & mask) | (value << start)
            }
        })*
    }
}

/// Trait for getting subsets of integers.
pub trait BitGetter: ConstBitGetter {
    /// Return a value representing the state of a range of bits.
    ///
    /// # Arguments
    ///
    /// * `range` - The range of bits, starting from 0 for the rightmost one.
    ///
    /// # Panics
    ///
    /// This method will panics if the given range is invalid.
    fn get_bits<T: RangeBounds<usize>>(self, range: T) -> Self;
}

/// Trait for setting subsets of integers.
pub trait BitField: ConstBitSetter + BitGetter {
    /// Set a range of bits to the given value.
    ///
    /// # Arguments
    ///
    /// * `range` - The range of bits, starting from 0 for the rightmost one.
    /// * `value` - An integer representing the desired state of the bits.
    ///
    /// # Panics
    ///
    /// This method will panics if the given range is invalid.
    /// Value must also not be greater in terms of bits than the given range.
    fn set_bits<T: RangeBounds<usize>>(self, range: T, value: Self) -> Self;
}

macro_rules! impl_bitgetter {
    (for $($t:ty),+) => {
        $(impl BitGetter for $t {

            fn get_bits<T: RangeBounds<usize>>(self, range: T) -> Self {
                let range = to_regular_range(&range, Self::TYPE_SIZE);

                self.get_range(range.start, range.end)
            }

        })*
    }
}

macro_rules! impl_bitfield {
    (for $($t:ty),+) => {
        $(impl BitField for $t {

            fn set_bits<T: RangeBounds<usize>>(self, range: T, value: Self) -> Self {
                let range = to_regular_range(&range, Self::TYPE_SIZE);

                self.set_range(range.start, range.end, value)
            }

        })*
    }
}

impl_constbitgetter!(for u8, u16, u32, u64, u128);
impl_constbitsetter!(for u8, u16, u32, u64, u128);
impl_bitgetter!(for u8, u16, u32, u64, u128);
impl_bitfield!(for u8, u16, u32, u64, u128);

fn to_regular_range<T: RangeBounds<usize>>(range: &T, maximun: usize) -> Range<usize> {
    let start = match range.start_bound() {
        Bound::Excluded(&value) => value + 1,
        Bound::Included(&value) => value,
        Bound::Unbounded => 0,
    };
    let end = match range.end_bound() {
        Bound::Excluded(&value) => value,
        Bound::Included(&value) => value + 1,
        Bound::Unbounded => maximun,
    };

    start..end
}

#[cfg(test)]
mod range_conversion_tests {
    use super::*;

    #[test_case]
    fn simple_range_start() {
        let range = 1..2;
        assert_eq!(to_regular_range(&range, 5).start, 1)
    }

    #[test_case]
    fn simple_range_stop() {
        let range = 1..2;
        assert_eq!(to_regular_range(&range, 5).end, 2)
    }

    #[test_case]
    fn range_stop() {
        let range = 1..=2;
        assert_eq!(to_regular_range(&range, 5).end, 3)
    }
}

#[cfg(test)]
mod get_tests {
    use super::*;

    #[test_case]
    fn get_bit_0_set_u16() {
        let val: u16 = 1;
        assert_eq!(val.get_bit(0), true);
    }

    #[test_case]
    fn get_bit_0_unset_u8() {
        let val: u8 = !0 - 1;
        assert_eq!(val.get_bit(0), false);
    }

    #[test_case]
    fn get_bit_15_set_u16() {
        let val: u16 = 0x8000;
        assert_eq!(val.get_bit(15), true);
    }

    #[test_case]
    fn get_bits_full_u16() {
        let val: u16 = 0xdead;
        assert_eq!(val.get_bits(0..16), 0xdead);
    }

    #[test_case]
    fn get_bits_low_half_u32() {
        let val: u32 = 0xdeadbeef;
        assert_eq!(val.get_bits(0..16), 0xbeef);
    }

    #[test_case]
    fn get_bits_high_half_u32() {
        let val: u32 = 0xdeadbeef;
        assert_eq!(val.get_bits(16..=31), 0xdead);
    }

    #[test_case]
    fn get_bits_one_u8() {
        let val: u8 = 0b10101010;
        assert_eq!(val.get_bits(1..2), 1);
    }

    #[test_case]
    fn get_bits_unbounded_u64() {
        let val: u64 = 0xdeadbeefcafeface;
        assert_eq!(val.get_bits(0..), val);
    }
}

#[cfg(test)]
mod set_tests {
    use super::*;

    #[test_case]
    fn set_bit_0_u16() {
        let val: u16 = 0;
        assert_eq!(val.set_bit(0, true), 1);
    }

    #[test_case]
    fn unset_bit_15_u16() {
        let val: u16 = 0x8000;
        assert_eq!(val.set_bit(15, false), 0);
    }

    #[test_case]
    fn set_bits_full_included_u16() {
        let val: u16 = 0;
        assert_eq!(val.set_bits(0..=15, 0xfeaf), 0xfeaf);
    }

    #[test_case]
    fn set_bits_full_excluded_u16() {
        let val: u16 = 0;
        assert_eq!(val.set_bits(0..16, 0xfeaf), 0xfeaf);
    }

    #[test_case]
    fn set_bits_full_unbounded_u32() {
        let val: u32 = 0;
        assert_eq!(val.set_bits(0.., 0xfeafbade), 0xfeafbade);
    }

    #[test_case]
    fn unset_bits_u8() {
        let val: u8 = 0b11111111;
        assert_eq!(val.set_bits(4..=6, 0), 0b10001111);
    }

    #[test_case]
    fn set_bits_u32() {
        let val: u32 = 0xabcdefab;
        assert_eq!(val.set_bits(8..24, 0xcafe), 0xabcafeab);
    }
}
