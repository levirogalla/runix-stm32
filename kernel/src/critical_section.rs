use critical_section::{Impl, RawRestoreState, set_impl};

use crate::hardware::registers;

struct CriticalSection;
set_impl!(CriticalSection);

unsafe impl Impl for CriticalSection {
    unsafe fn acquire() -> RawRestoreState {
        let state = registers::Primask::enabled();
        registers::Primask::disable();
        state
    }

    unsafe fn release(primask: RawRestoreState) {
        if primask {
            registers::Primask::enable();
        }
    }
}
