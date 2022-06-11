use core::arch::asm;
pub use crate::arch::ia32::{halt, pause, PrivilegeLevel};

pub mod descriptor;
pub mod interrupts;
pub mod mm;
pub mod selector;

/// Read a 8bits value from the chose IO port address.
///
/// # Safety
///
/// Values taken from IO port may come from external devices, check their
/// documentation to ensure the meaning of conveyed values.
pub unsafe fn in_byte(address: u16) -> u8 {
    let result: u8;
    asm!("in al, dx",
        in("dx") address,
        out("al") result,
        options(nomem, nostack)
    );
    result
}

/// Write a 8bits value at the chosen IO port address.
///
/// # Safety
///
/// This function can be used to communicate with external devices, ill-formed
/// value may break those devices.
pub unsafe fn out_byte(address: u16, value: u8) {
    asm!("out dx, al",
        in("dx") address,
        in("al") value,
        options(nomem, nostack)
    );
}

/// Read a 16bits value from the chose IO port address.
///
/// # Safety
///
/// Values taken from IO port may come from external devices, check their
/// documentation to ensure the meaning of conveyed values.
pub unsafe fn in_word(address: u16) -> u16 {
    let result: u16;
    asm!("in ax, dx",
        in("dx") address,
        out("ax") result,
        options(nomem, nostack)
    );
    result
}

/// Write a 16bits value at the chosen IO port address.
///
/// # Safety
///
/// This function can be used to communicate with external devices, ill-formed
/// value may break those devices.
pub unsafe fn out_word(address: u16, value: u16) {
    asm!("out dx, ax",
            in("dx") address,
            in("ax") value,
            options(nomem, nostack));
}

/// Read a 32bits value from the chose IO port address.
///
/// # Safety
///
/// Values taken from IO port may come from external devices, check their
/// documentation to ensure the meaning of conveyed values.
pub unsafe fn in_double_word(address: u16) -> u32 {
    let result: u32;
    asm!("in eax, dx",
        out("eax") result,
        in("dx") address,
        options(nomem, nostack)
    );
    result
}

/// Write a 32bits value at the chosen IO port address.
///
/// # Safety
///
/// This function can be used to communicate with external devices, ill-formed
/// value may break those devices.
pub unsafe fn out_double_word(address: u16, value: u32) {
    asm!("out dx, eax",
        in("dx") address,
        in("eax") value,
        options(nomem, nostack)
    );
}
