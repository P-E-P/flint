#![no_std]
#![no_main]
use core::panic::PanicInfo;

mod vga;
#[macro_use]
mod klog;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

static WELCOME_MESSAGE: &[u8] = b"Booting flint...";
static LOAD: &[u8] = b"|/-\\";

#[no_mangle]
pub extern "C" fn _start() -> ! {
    let vga_buffer = 0xb8000 as *mut u8;

    klog::init().ok();

    log::info!("Information message");
    log::error!("Error message");

    vga::text::print_something();

    for i in 0.. {
        unsafe {
            *vga_buffer.offset(79 as isize * 2) = LOAD[i % LOAD.len()] as u8;
            *vga_buffer.offset(79 as isize * 2 + 1) = 0xf;
        }
    }
    loop {}
}
