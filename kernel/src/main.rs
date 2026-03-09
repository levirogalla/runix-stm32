#![no_std]
#![no_main]
mod boot;
mod critical_section;
mod drivers;
mod exceptions;
mod hardware;
mod user_api;

use core::{arch::asm, panic::PanicInfo};

#[unsafe(no_mangle)]
unsafe fn main() -> ! {
    drivers::rtt::rtt_init_print!();
    loop {
        unsafe { asm!("svc #1") }
    }
}

#[panic_handler]
fn panic_handler(_info: &PanicInfo) -> ! {
    loop {}
}
