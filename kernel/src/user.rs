//! Test user thread functions

use crate::drivers;

pub fn temp_idle() -> ! {
    loop {
        drivers::rtt::rprintln!("idle");
    }
}
