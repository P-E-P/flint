use core::arch::asm;

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
