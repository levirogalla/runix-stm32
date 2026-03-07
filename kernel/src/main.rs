#![no_std]
#![no_main]

use core::panic::PanicInfo;
use cortex_m_rt::entry;

#[entry]
fn main() -> ! {
    loop {}
}


#[panic_handler]
fn panic_handler(_info: &PanicInfo) -> ! {
    loop { }
}