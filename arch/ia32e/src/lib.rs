#![no_std]

use core::arch::asm;

pub mod mm;
pub mod interrupts;
pub mod descriptor;

pub unsafe fn in_byte(address: u16) -> u8 {
    let result: u8;
    asm!("in al, dx",
        in("dx") address,
        out("al") result,
        options(nomem, nostack)
    );
    result
}

pub unsafe fn out_byte(address: u16, value: u8) {
    asm!("out dx, al",
        in("dx") address,
        in("al") value,
        options(nomem, nostack)
    );
}

pub unsafe fn in_word(address: u16) -> u16 {
    let result: u16;
    asm!("in ax, dx",
        in("dx") address,
        out("ax") result,
        options(nomem, nostack)
    );
    result
}

pub unsafe fn out_word(address: u16, value: u16) {
    asm!("out dx, ax",
            in("dx") address,
            in("ax") value,
            options(nomem, nostack));
}

pub unsafe fn in_double_word(address: u16) -> u32 {
    let result: u32;
    asm!("in eax, dx",
        out("eax") result,
        in("dx") address,
        options(nomem, nostack)
    );
    result
}

pub unsafe fn out_double_word(address: u16, value: u32) {
    asm!("out dx, eax",
        in("dx") address,
        in("eax") value,
        options(nomem, nostack)
    );
}

pub fn pause() {
    unsafe {
        asm!("pause");
    }
}
