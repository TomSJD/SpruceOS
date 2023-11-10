#![no_std]
#![no_main]

mod vga_buffer;

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

static HELLO: &[u8] = b"Hello, SpruceOS!";

#[no_mangle]
pub extern "C" fn _start() -> ! {
    vga_buffer::test_print();

    loop {}
}
