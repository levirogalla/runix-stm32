use core::arch::global_asm;

/// Parse cfg attributes inside a global_asm call.
macro_rules! cfg_global_asm {
    {@inner, [$($x:tt)*], } => {
        global_asm!{$($x)*}
    };
    (@inner, [$($x:tt)*], #[cfg($meta:meta)] $asm:literal, $($rest:tt)*) => {
        #[cfg($meta)]
        cfg_global_asm!{@inner, [$($x)* $asm,], $($rest)*}
        #[cfg(not($meta))]
        cfg_global_asm!{@inner, [$($x)*], $($rest)*}
    };
    {@inner, [$($x:tt)*], $asm:literal, $($rest:tt)*} => {
        cfg_global_asm!{@inner, [$($x)* $asm,], $($rest)*}
    };
    {$($asms:tt)*} => {
        cfg_global_asm!{@inner, [], $($asms)*}
    };
}

// This reset vector is the initial entry point after a system reset.
// Calls an optional user-provided __pre_init and then initialises RAM.
// If the target has an FPU, it is enabled.
// Finally jumps to the user main function.
cfg_global_asm! {
    ".cfi_sections .debug_frame
     .section .Reset, \"ax\"
     .global Reset
     .type Reset,%function
     .thumb_func",
    ".cfi_startproc
     Reset:",

    // If enabled, initialise the SP. This is normally initialised by the CPU itself or by a
    // bootloader, but some debuggers fail to set it when resetting the target, leading to
    // stack corruptions.
    #[cfg(feature = "set-sp")]
    "ldr r0, =_stack_start
     msr msp, r0",

    // If enabled, initialise VTOR to the start of the vector table. This is normally initialised
    // by a bootloader when the non-reset value is required, but some bootloaders do not set it,
    // leading to frustrating issues where everything seems to work but interrupts are never
    // handled. The VTOR register is optional on ARMv6-M, but when not present is RAZ,WI and
    // therefore safe to write to.
    #[cfg(feature = "set-vtor")]
    "ldr r0, =0xe000ed08
     ldr r1, =__vector_table
     str r1, [r0]",

    // If enabled, initialize RAM with zeros. This is not usually required, but might be necessary
    // to properly initialize checksum-based memory integrity measures on safety-critical hardware.
    #[cfg(feature = "zero-init-ram")]
    "ldr r0, =_ram_start
     ldr r1, =_ram_end
     movs r2, #0
     0:
     cmp r1, r0
     beq 1f
     stm r0!, {{r2}}
     b 0b
     1:",

    // Initialise .bss memory. `__sbss` and `__ebss` come from the linker script.
    #[cfg(not(feature = "zero-init-ram"))]
    "ldr r0, =__sbss
     ldr r1, =__ebss
     movs r2, #0
     0:
     cmp r1, r0
     beq 1f
     stm r0!, {{r2}}
     b 0b
     1:",

    // If enabled, paint stack/heap RAM with 0xcccccccc.
    // `__sheap` and `_stack_start` come from the linker script.
    #[cfg(feature = "paint-stack")]
    "ldr r0, =__sheap
     ldr r1, =_stack_start
     ldr r2, =0xcccccccc // This must match STACK_PAINT_VALUE
     0:
     cmp r1, r0
     beq 1f
     stm r0!, {{r2}}
     b 0b
     1:",

    // Initialise .data memory. `__sdata`, `__sidata`, and `__edata` come from the linker script.
    "ldr r0, =__sdata
     ldr r1, =__edata
     ldr r2, =__sidata
     0:
     cmp r1, r0
     beq 1f
     ldm r2!, {{r3}}
     stm r0!, {{r3}}
     b 0b
     1:",

    // Potentially enable an FPU.
    // SCB.CPACR is 0xE000_ED88.
    // We enable access to CP10 and CP11 from priviliged and unprivileged mode.
    #[cfg(has_fpu)]
    "ldr r0, =0xE000ED88
     ldr r1, =(0b1111 << 20)
     ldr r2, [r0]
     orr r2, r2, r1
     str r2, [r0]
     dsb
     isb",

    // Jump to user main function.
    // `bl` is used for the extended range, but the user main function should not return,
    // so trap on any unexpected return.
    "bl main
     udf #0",

    ".cfi_endproc
     .size Reset, . - Reset",
}

