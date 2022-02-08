use core::arch::asm;

pub unsafe fn outb(value: u8, port: u16) {
    asm!("outb {v},{p:x}",
        v = in(reg_byte) value,
        p = in(reg) port);
}

pub unsafe fn inb(port: u16) -> u8 {
    let result: u8;
    asm!("inb {p:x},{v}",
        v = out(reg_byte) result,
        p = in(reg) port);
    result
}

pub unsafe fn outw(value: u16, port: u16) {
    asm!("outw {v:x},{p:x}",
        v = in(reg) value,
        p = in(reg) port);
}

pub unsafe fn inw(port: u16) -> u16 {
    let result: u16;
    asm!("inw {p:x},{v:x}",
        v = out(reg) result,
        p = in(reg) port);
    result
}

pub fn pause() {
    unsafe {
        asm!("pause");
    }
}
