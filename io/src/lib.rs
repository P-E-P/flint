#![no_std]
use core::arch::asm;

pub unsafe fn outb(value: u8, port: u16) {
    asm!("out dx, al",
    in("dx") port,
    in("al") value,
    options(nomem, nostack)
    );
}

pub unsafe fn inb(port: u16) -> u8 {
    let result: u8;
    asm!("in al, dx",
    in("dx") port,
    out("al") result,
    options(nomem, nostack)
    );
    result
}

pub unsafe fn outw(value: u16, port: u16) {
    asm!("out dx, ax",
        in("dx") port,
        in("ax") value,
        options(nomem, nostack));
}

pub unsafe fn inw(port: u16) -> u16 {
    let result: u16;
    asm!("in ax, dx",
        in("dx") port,
        out("ax") result,
        options(nomem, nostack)
    );
    result
}

pub fn pause() {
    unsafe {
        asm!("pause");
    }
}
