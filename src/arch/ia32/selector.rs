use crate::arch::ia32::PrivilegeLevel;
use crate::utils::bitfield::{BitGetter, ConstBitGetter};
use core::fmt;
use core::fmt::Display;

#[derive(Debug, Clone, Copy)]
pub struct SegmentSelector(u16);

impl SegmentSelector {
    /// Create a new null [`SegmentSelector`].
    pub const fn const_default() -> Self {
        SegmentSelector(0)
    }
}

impl Default for SegmentSelector {
    fn default() -> Self {
        Self::const_default()
    }
}

#[repr(u8)]
pub enum TableIndicator {
    GDT = 0,
    LDT = 1,
}

impl From<bool> for TableIndicator {
    fn from(value: bool) -> Self {
        match value {
            false => TableIndicator::GDT,
            true => TableIndicator::LDT,
        }
    }
}

impl Display for TableIndicator {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                TableIndicator::GDT => "GDT",
                TableIndicator::LDT => "LDT",
            }
        )
    }
}

impl From<SegmentSelector> for u16 {
    fn from(value: SegmentSelector) -> Self {
        value.0
    }
}

impl From<SegmentSelector> for u32 {
    fn from(value: SegmentSelector) -> Self {
        value.0.into()
    }
}

impl SegmentSelector {
    pub fn new(index: u16, ti: TableIndicator, rpl: PrivilegeLevel) -> Self {
        SegmentSelector(index << 3 | ((ti as u16) << 2) | (rpl as u16))
    }
}

impl fmt::Display for SegmentSelector {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let ti: TableIndicator = self.0.get_bit(2).into();
        let index: u16 = self.0 >> 3;
        let rpl: PrivilegeLevel = (self.0.get_bits(0..1) as u8).into();
        write!(
            f,
            "Index: {}\nTable Indicator: {}\nPrivilege Level: {}",
            index, ti, rpl
        )
    }
}
