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
    use SpruceOS::memory::{self, BootInfoFrameAllocator};
    use SpruceOS::allocator;
    use x86_64::VirtAddr;

    println!("Spruce Kernel 0.0.1");

    SpruceOS::init();

    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mut mapper = unsafe { memory::init(phys_mem_offset) };
    let mut frame_allocator = unsafe {
        BootInfoFrameAllocator::init(&boot_info.memory_map)
    };

    allocator::init_heap(&mut mapper, &mut frame_allocator).expect("Heap initialization failed");

    let heap_value = Box::new(41);
    println!("heap_value at {heap_value:?}");

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