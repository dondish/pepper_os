#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(pepper_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

extern crate alloc;

use core::panic::PanicInfo;
use bootloader::{BootInfo, entry_point};
use pepper_os::{println, memory::{self, BootInfoFrameAllocator}, allocator};
use alloc::boxed::Box;
use x86_64::VirtAddr;

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    pepper_os::hlt_loop();
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    pepper_os::test_panic_handler(info)
}


entry_point!(kernel_main);

fn kernel_main(boot_info: &'static BootInfo) -> ! {
    println!("Starting Pepper OS...");

    pepper_os::init();

    let physical_memory_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mut mapper = unsafe { memory::init(physical_memory_offset) };
    let mut frame_allocator = unsafe { BootInfoFrameAllocator::init(&boot_info.memory_map) };

    allocator::init_heap(&mut mapper, &mut frame_allocator).expect("Could not initialize heap");

    let heap_value = Box::new(41);
    println!("heap_value at {:p} {}", heap_value, heap_value);


    #[cfg(test)]
    test_main();

    println!("Successfully Loaded OS!");

    pepper_os::hlt_loop();
}