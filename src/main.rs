#![no_std]
#![no_main]

use core::panic::PanicInfo;
mod vga_buffer;

/// This function is called on panic.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    println!("{}", _info);
    loop {}
}

static HELLO: &str = "Oded Shapira\nCyber";

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("{}", HELLO);

    loop {}
}