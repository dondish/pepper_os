#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(pepper_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use pepper_os::{println};

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    pepper_os::test_panic_handler(info)
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Starting Pepper OS...");

    pepper_os::init();

    #[cfg(test)]
    test_main();

    println!("Successfully Loaded OS!");

    loop {}
}