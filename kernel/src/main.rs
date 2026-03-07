#![no_std]
#![no_main]
mod boot;

use core::panic::PanicInfo;
use rtt_target;

#[unsafe(no_mangle)]
unsafe fn main() -> ! {
    loop {}
}


#[panic_handler]
fn panic_handler(_info: &PanicInfo) -> ! {
    loop { }
}