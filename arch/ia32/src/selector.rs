use crate::PrivilegeLevel;

pub struct SegmentSelector(u16);

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
