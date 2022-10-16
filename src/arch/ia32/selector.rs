use crate::arch::ia32::PrivilegeLevel;

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
