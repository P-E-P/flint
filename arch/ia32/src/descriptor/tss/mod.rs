use super::segment::lower::Lower;
use super::{split_address, split_limit, Granularity, PrivilegeLevel};
use upper::Upper;

mod upper;

#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct TssDescriptor {
    upper: Upper,
    lower: Lower,
}

impl TssDescriptor {
    pub fn new(base: u32, limit: u32) -> Self {
        let (base_high, base_mid, base_low) = split_address(base);
        let (limit_high, limit_low) = split_limit(limit);

        TssDescriptor {
            lower: Lower::default().base_low(base_low).limit_low(limit_low),
            upper: Upper::default()
                .base_high(base_high)
                .base_mid(base_mid)
                .limit_high(limit_high)
                .present(1),
        }
    }

    pub fn busy(self, state: bool) -> Self {
        Self {
            upper: self.upper.busy(state.into()),
            ..self
        }
    }

    pub fn available(self, avl: bool) -> Self {
        Self {
            upper: self.upper.available(avl.into()),
            ..self
        }
    }

    pub fn privilege_level(self, level: PrivilegeLevel) -> Self {
        Self {
            upper: self.upper.privilege_level(level as u32),
            ..self
        }
    }

    pub fn present(self, present: bool) -> Self {
        Self {
            upper: self.upper.present(present.into()),
            ..self
        }
    }

    pub fn granularity(self, granularity: Granularity) -> Self {
        Self {
            upper: self.upper.granularity(granularity as u32),
            ..self
        }
    }
}
