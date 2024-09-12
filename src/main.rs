#![no_std]
#![no_main]

fn main() {
}

use core::panic::PanicInfo;

/// Function called to handle panics
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    loop {}
}

