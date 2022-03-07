#![no_std]
pub mod io;
pub mod mm;

use core::arch::asm;

trait InOut {
    unsafe fn in_reg(address: u16) -> Self;
    unsafe fn out_reg(address: u16, value: Self);
}

impl InOut for u8 {
    unsafe fn in_reg(address: u16) -> Self {
        let result: u8;
        asm!("in al, dx",
            in("dx") address,
            out("al") result,
            options(nomem, nostack)
        );
        result
    }

    unsafe fn out_reg(address: u16, value: Self) {
        asm!("out dx, al",
            in("dx") address,
            in("al") value,
            options(nomem, nostack)
        );
    }
}

impl InOut for u16 {
    unsafe fn in_reg(address: u16) -> Self {
        let result: u16;
        asm!("in ax, dx",
            in("dx") address,
            out("ax") result,
            options(nomem, nostack)
        );
        result
    }

    unsafe fn out_reg(address: u16, value: Self) {
        asm!("out dx, ax",
            in("dx") address,
            in("ax") value,
            options(nomem, nostack));
    }
}

impl InOut for u32 {
    unsafe fn in_reg(address: u16) -> Self {
        let result: u32;
        asm!("in eax, dx",
            out("eax") result,
            in("dx") address,
            options(nomem, nostack)
        );
        result
    }

    unsafe fn out_reg(address: u16, value: Self) {
        asm!("out dx, eax",
            in("dx") address,
            in("eax") value,
            options(nomem, nostack)
        );
    }
}

pub fn pause() {
    unsafe {
        asm!("pause");
    }
}
