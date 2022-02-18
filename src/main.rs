#![no_std]
#![no_main]

use core::panic::PanicInfo;

mod vga;
#[macro_use]
mod klog;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    log::error!("Kernel Panic!:\n{}", info);
    loop {}
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    klog::init().ok();

    loop {}
}
