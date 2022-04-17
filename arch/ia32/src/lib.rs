#![no_std]

use core::arch::asm;

/// Offset the bits of the given identifier by it's offset.
#[macro_export]
macro_rules! offset {
    ($n: ident) => {
        bits::$n << offsets::$n
    };
}

/// Clear the flag bits `n` from a number `s` and set those bits to the new
/// `v` value.
#[macro_export]
macro_rules! setbits {
    ($s: expr, $v: expr, $n: ident) => {
        // - Ensure the value does not overflow
        // - Clear previous flag
        // - Set new flag
        ($s & !flags::$n) | (($v & bits::$n) << offsets::$n)
    };
}

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
