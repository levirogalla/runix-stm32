use critical_section::{set_impl, Impl, RawRestoreState};

struct CriticalSection;
set_impl!(CriticalSection);

unsafe impl Impl for CriticalSection {
    unsafe fn acquire() -> RawRestoreState {
        let primask: u32;
        unsafe {
            core::arch::asm!(
                "mrs {0}, PRIMASK",  // Read PRIMASK
                "cpsid i",           // Disable interrupts
                out(reg) primask
            );
        }
        (primask & (1 << 0)) != (1 << 0)
    }

    unsafe fn release(primask: RawRestoreState) {
        if primask {
            unsafe { core::arch::asm!("cpsie i") }; // Enable interrupts
        }
    }
}
