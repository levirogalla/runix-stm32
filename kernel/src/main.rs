#![no_std]
#![no_main]

mod boot; // brings start code into compilation

mod constants;
mod critical_section;
mod drivers;
mod exceptions;
mod hardware;
mod mem;
mod sched;
mod types;
mod user;

use core::{arch::asm, panic::PanicInfo};

#[unsafe(no_mangle)]
unsafe fn main() -> ! {
    drivers::rtt::rtt_init_print!();
    unsafe { asm!("svc #0") };
    loop {}
    // unsafe { asm!("mov r0, #0", "mov r3, #3", "mov r12, #10", "svc #0") }
    // unsafe { asm!("mov r0, #0", "mov r3, #3", "mov r12, #10", "svc #1") }
    // unsafe { asm!("mov r0, #0", "mov r3, #3", "mov r12, #10", "svc #2") }
    // loop {
    // }
}

#[panic_handler]
fn panic_handler(_info: &PanicInfo) -> ! {
    loop {}
}
