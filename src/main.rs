#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(SpruceOS::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use SpruceOS::println;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello World!");

    SpruceOS::init();

    #[cfg(test)]
    test_main();

    println!("No crash :3");

    loop {}
}

// Panic method for non test panic calls
#[cfg(not(test))]
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    println!("{}", _info);
    loop {}
}

// Panic method for test panic calls
#[cfg(test)]
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    SpruceOS::test_panic_handler(_info);
}