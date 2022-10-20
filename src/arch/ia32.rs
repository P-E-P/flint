use core::arch::asm;
use core::fmt;
use core::fmt::Display;

pub mod address;
pub mod descriptor;
pub mod interrupts;
pub mod mm;
pub mod selector;
pub mod task;

#[repr(u8)]
pub enum PrivilegeLevel {
    Kernel = 0,
    Unused1 = 1,
    Unused2 = 2,
    Userland = 3,
}

impl From<PrivilegeLevel> for u8 {
    fn from(value: PrivilegeLevel) -> Self {
        value as u8
    }
}

impl From<u8> for PrivilegeLevel {
    fn from(value: u8) -> Self {
        match value {
            0 => PrivilegeLevel::Kernel,
            1 => PrivilegeLevel::Unused1,
            2 => PrivilegeLevel::Unused2,
            3 => PrivilegeLevel::Userland,
            _ => panic!("Unexpected value for PrivilegeLevel"),
        }
    }
}

impl Display for PrivilegeLevel {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                PrivilegeLevel::Kernel => "Kernel",
                PrivilegeLevel::Unused1 | PrivilegeLevel::Unused2 => "Unused",
                PrivilegeLevel::Userland => "Userland",
            }
        )
    }
}

pub fn pause() {
    unsafe {
        asm!("pause");
    }
}

pub fn halt() {
    unsafe {
        asm!("hlt");
    }
}
