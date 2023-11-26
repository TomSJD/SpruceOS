#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(SpruceOS::test_runner)]
#![reexport_test_harness_main = "test_main"]

extern crate alloc;

use alloc::boxed::Box;
use core::panic::PanicInfo;
use bootloader::{BootInfo, entry_point};
use SpruceOS::memory::BootInfoFrameAllocator;
use SpruceOS::println;

entry_point!(kernel_main);

fn kernel_main(boot_info: &'static BootInfo) -> ! {
    use SpruceOS::memory;
    use x86_64::VirtAddr;

    println!("Spruce OS Kernel 0.0.1");

    SpruceOS::init();

    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mut _mapper = unsafe { memory::init(phys_mem_offset) };
    let mut _frame_allocator = unsafe {
        BootInfoFrameAllocator::init(&boot_info.memory_map)
    };

    let x = Box::new(41);

    #[cfg(test)]
    test_main();

    println!("No crash :3");
    SpruceOS::hlt_loop();
}

// Panic method for non test panic calls
#[cfg(not(test))]
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    println!("{}", _info);
    SpruceOS::hlt_loop();
}

// Panic method for test panic calls
#[cfg(test)]
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    SpruceOS::test_panic_handler(_info);
}