#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(SpruceOS::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use bootloader::{BootInfo, entry_point};
use SpruceOS::memory::translate_addr;
use SpruceOS::println;

entry_point!(kernel_main);

fn kernel_main(boot_info: &'static BootInfo) -> ! {
    use SpruceOS::memory;
    use x86_64::{structures::paging::Translate, VirtAddr};

    println!("Spruce OS Kernel 0.0.1");

    SpruceOS::init();

    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mapper = unsafe { memory::init(phys_mem_offset) };

    let addresses = [0xb8000, 0x201008, 0x0100_0020_1a10, boot_info.physical_memory_offset,];

    for &address in &addresses {
        let virt = VirtAddr::new(address);
        let phys = mapper.translate_addr(virt);
        println!("{virt:?} -> {phys:?}");
    }

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