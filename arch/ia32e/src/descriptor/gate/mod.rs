use core::fmt;
use ia32::descriptor::gate::PrivilegeLevel;
use lower::Lower;
use upper::Upper;

mod lower;
mod upper;

#[repr(u8)]
pub enum Kind {
    Interrupt = 0xe,
    Trap = 0xf,
}

#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct Gate {
    /// Upper 32bits of the gate are reserved.
    reserved: u32,
    /// Procedure's entry point offset high bits.
    offset_high: u32,
    ///  Segment selector bits (32:64).
    upper: upper::Upper,
    /// Segment selector bits (0:31).
    lower: lower::Lower,
}

impl Gate {
    /// Creates a new interrupt/trap [`Gate`] from a given offset and segment
    /// selector.
    pub fn new(offset: u64, segment_selector: u16) -> Self {
        let offset_high = u32::try_from(offset >> 32).unwrap();
        let offset_mid = u32::try_from((offset >> 16) & 0xffff).unwrap();
        let offset_low = u32::try_from(offset & 0xffff).unwrap();

        Self {
            reserved: Default::default(),
            offset_high,
            upper: Upper::default().offset_mid(offset_mid),
            lower: Lower::default()
                .offset_low(offset_low)
                .segment_selector(segment_selector.into()),
        }
    }

    /// Set or clear the presence bit of the [`Gate`].
    pub fn present(self, value: bool) -> Self {
        Self {
            upper: self.upper.present(value.into()),
            ..self
        }
    }

    /// Set the type of [`Gate`].
    pub fn kind(self, kind: Kind) -> Self {
        Self {
            upper: self.upper.kind(kind as u32),
            ..self
        }
    }

    /// Set the privilege level of the [`Gate`] descriptor.
    pub fn privilege_level(self, level: PrivilegeLevel) -> Self {
        Self {
            upper: self.upper.privilege_level(level as u32),
            ..self
        }
    }

    /// Set the  interrupt stack table.
    pub fn interrupt_stack_table(self, ist: u8) -> Self {
        Self {
            upper: self.upper.interrupt_stack_table(ist.into()),
            ..self
        }
    }
}

impl fmt::Display for Gate {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let reserved = self.reserved;
        let offset_high = self.offset_high;
        let upper = self.upper;
        let lower = self.lower;
        write!(
            f,
            "{:08X?};{:08X?};{:08X?};{:08X?}",
            reserved, offset_high, upper, lower
        )
    }
}
