#![no_std]
#![no_main]
mod boot;
mod drivers;
mod critical_section;
mod hardware;

use core::panic::PanicInfo;

#[unsafe(no_mangle)]
unsafe fn main() -> ! {
    drivers::rtt::rtt_init_print!();
    loop {
        drivers::rtt::rprintln!("tes");
    }
}


#[panic_handler]
fn panic_handler(_info: &PanicInfo) -> ! {
    loop { }
}