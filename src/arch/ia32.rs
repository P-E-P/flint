use core::arch::asm;

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
