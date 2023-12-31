#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(SpruceOS::test_runner)]
#![reexport_test_harness_main = "test_main"]

extern crate alloc;

use core::panic::PanicInfo;
use bootloader::{BootInfo, entry_point};
use SpruceOS::println;
use SpruceOS::task::{Task, keyboard, executor::Executor};
use SpruceOS::cli::basic_cli::CommandLineInterface;
// Experimental Graphics Mode
/*
use vga::colors::Color16;
use vga::writers::{Graphics640x480x16, GraphicsWriter, PrimitiveDrawing};

 */

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

    /*
    let mode = Graphics640x480x16::new();
    mode.set_mode();
    mode.clear_screen(Color16::Black);
    mode.draw_line((80, 60), (80, 420), Color16::White);
    mode.draw_line((80, 60), (540, 60), Color16::White);
    mode.draw_line((80, 420), (540, 420), Color16::White);
    mode.draw_line((540, 420), (540, 60), Color16::White);
    mode.draw_line((80, 90), (540, 90), Color16::White);

     */

    #[cfg(test)]
    test_main();

    let mut executor = Executor::new();
    executor.spawn(Task::new(example_task()));
    //executor.spawn(Task::new(keyboard::print_keypresses()));
    executor.spawn(Task::new(keyboard::handle_keys()));
    executor.run();
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

async fn async_number() -> u32 {
    69
}

async fn example_task() {
    let number = async_number().await;
    println!("async number: {}", number);
}