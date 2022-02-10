#![no_std]
#![no_main]
use core::panic::PanicInfo;

use serial::Serial;

mod vga;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

static WELCOME_MESSAGE: &[u8] = b"Booting flint...";
static LOAD: &[u8] = b"|/-\\";

#[no_mangle]
pub extern "C" fn _start() -> ! {
    let vga_buffer = 0xb8000 as *mut u8;

    let serial = Serial::new(serial::COM1);
    serial.initialize();
    serial.write_string("Testing serial\nEven on multiple lines");

    for (i, &byte) in WELCOME_MESSAGE.iter().enumerate() {
        unsafe {
            *vga_buffer.offset(i as isize * 2) = byte;
            *vga_buffer.offset(i as isize * 2 + 1) = 0xb;
        }
    }

    vga::text::print_something();

    for i in 0.. {
        unsafe {
            *vga_buffer.offset(WELCOME_MESSAGE.len() as isize * 2) = LOAD[i % LOAD.len()] as u8;
            *vga_buffer.offset(WELCOME_MESSAGE.len() as isize * 2 + 1) = 0xf;
        }
    }
    loop {}
}
