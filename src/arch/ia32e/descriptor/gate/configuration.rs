use super::{Kind, PrivilegeLevel};
use bit_field::BitField;
use core::fmt;

#[derive(Default, Copy, Clone)]
pub struct Configuration(u16);

mod offset {
    pub mod ist {
        pub const LOWER: usize = 0;
        pub const UPPER: usize = 2;
    }

    pub mod kind {
        pub const LOWER: usize = 8;
        pub const UPPER: usize = 11;
    }

    pub mod privilege_level {
        pub const LOWER: usize = 13;
        pub const UPPER: usize = 14;
    }

    pub const PRESENT: usize = 15;
}

impl Configuration {
    pub fn privilege_level(self, level: PrivilegeLevel) -> Self {
        use offset::privilege_level::{LOWER, UPPER};
        Self(
            *self
                .0
                .clone()
                .set_bits(LOWER..=UPPER, u8::from(level).into()),
        )
    }

    pub fn present(self, present: bool) -> Self {
        use offset::PRESENT;
        Self(*self.0.clone().set_bit(PRESENT, present))
    }

    pub fn interrupt_stack_table(self, ist: u8) -> Self {
        use offset::ist::{LOWER, UPPER};
        Self(*self.0.clone().set_bits(LOWER..=UPPER, ist.into()))
    }

    pub fn kind(self, kind: Kind) -> Self {
        use offset::kind::{LOWER, UPPER};
        Self(
            *self
                .0
                .clone()
                .set_bits(LOWER..=UPPER, kind.into()),
        )
    }
}

impl fmt::Display for Configuration {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use offset::{privilege_level, kind, ist, PRESENT};
        write!(
            f,
            "DPL: {}\nPresent {}\nType:{}\nIST:{}",
            self.0.get_bits(privilege_level::LOWER..=privilege_level::UPPER),
            self.0.get_bit(PRESENT),
            self.0.get_bits(kind::LOWER..=kind::UPPER),
            self.0.get_bits(ist::LOWER..=ist::UPPER)
        )
    }
}
