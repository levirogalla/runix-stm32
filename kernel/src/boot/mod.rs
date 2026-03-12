// # Developer notes
//
// - `link_section` is used to place symbols in specific places of the final binary. The names used
// here will appear in the linker script (`link.x`) in conjunction with the `KEEP` command.

/// The 32-bit value the stack is painted with before the program runs.
// Note: keep this value in-sync with the start-up assembly code, as we can't
// use const values in `global_asm!` yet.
#[cfg(feature = "paint-stack")]
pub const STACK_PAINT_VALUE: u32 = 0xcccc_cccc;

// bring startup code into scope to make symbols globally available
mod start;

// We export this static with an informative name so that if an application attempts to link
// two copies of cortex-m-rt together, linking will fail. We also declare a links key in
// Cargo.toml which is the more modern way to solve the same problem, but we have to keep
// __ONCE__ around to prevent linking with versions before the links key was added.
#[unsafe(export_name = "error: cortex-m-rt appears more than once in the dependency graph")]
pub static __ONCE__: () = ();

/// Returns a pointer to the start of the heap
///
/// The returned pointer is guaranteed to be 4-byte aligned.
#[inline]
pub fn heap_start() -> *mut u32 {
    unsafe extern "C" {
        static mut __sheap: u32;
    }

    core::ptr::addr_of_mut!(__sheap)
}

// Entry point is Reset.
#[unsafe(link_section = ".vector_table.reset_vector")]
#[unsafe(no_mangle)]
pub static __RESET_VECTOR: unsafe extern "C" fn() -> ! = Reset;

#[unsafe(link_section = ".HardFault.default")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn HardFault_() -> ! {
    #[allow(clippy::empty_loop)]
    loop {}
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn DefaultHandler_() -> ! {
    #[allow(clippy::empty_loop)]
    loop {}
}

unsafe extern "C" {
    // defined earlier in the assembly
    fn Reset() -> !;

    fn NonMaskableInt();

    fn HardFault();

    fn MemoryManagement();

    fn BusFault();

    fn UsageFault();

    fn SVCall();

    fn DebugMonitor();

    fn PendSV();

    fn SysTick();
}

#[repr(C)]
pub union Vector {
    handler: unsafe extern "C" fn(),
    reserved: usize,
}

#[unsafe(link_section = ".vector_table.exceptions")]
#[unsafe(no_mangle)]
pub static __EXCEPTIONS: [Vector; 14] = [
    // Exception 2: Non Maskable Interrupt.
    Vector {
        handler: NonMaskableInt,
    },
    // Exception 3: Hard Fault Interrupt.
    Vector { handler: HardFault },
    // Exception 4: Memory Management Interrupt [not on Cortex-M0 variants].
    Vector {
        handler: MemoryManagement,
    },
    // Exception 5: Bus Fault Interrupt [not on Cortex-M0 variants].
    Vector { handler: BusFault },
    // Exception 6: Usage Fault Interrupt [not on Cortex-M0 variants].
    Vector {
        handler: UsageFault,
    },
    // Exception 7: Secure Fault Interrupt [only on Armv8-M].
    Vector { reserved: 0 },
    // 8-10: Reserved
    Vector { reserved: 0 },
    Vector { reserved: 0 },
    Vector { reserved: 0 },
    // Exception 11: SV Call Interrupt.
    Vector { handler: SVCall },
    // Exception 12: Debug Monitor Interrupt [not on Cortex-M0 variants].
    Vector {
        handler: DebugMonitor,
    },
    // 13: Reserved
    Vector { reserved: 0 },
    // Exception 14: Pend SV Interrupt [not on Cortex-M0 variants].
    Vector { handler: PendSV },
    // Exception 15: System Tick Interrupt.
    Vector { handler: SysTick },
];

// If we are not targeting a specific device we bind all the potential device specific interrupts
// to the default handler
#[cfg(all(any(not(feature = "device"), test)))]
#[doc(hidden)]
#[unsafe(link_section = ".vector_table.interrupts")]
#[unsafe(no_mangle)]
pub static __INTERRUPTS: [unsafe extern "C" fn(); 240] = [{
    unsafe extern "C" {
        fn DefaultHandler();
    }

    DefaultHandler
}; 240];
