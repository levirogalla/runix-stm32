// =================================================================================================
// Software Interrupts
// =================================================================================================
mod syscalls;

use core::fmt;

use crate::{
    drivers,
    types::{Byte, StackPointer},
};

#[inline(never)]
#[unsafe(no_mangle)]
// must me extern c because rust needs to restore the correct registers before leaving
extern "C" fn syscall_entry(sp: StackPointer<ExceptionFrame>, mode: Mode) {
    // sp is the sp right when the exception happened and all registers were saved to the stack
    // safe: we can dereference it because this is the exact pointer that was in either PSP, or MSP which the cpu manages, see entry.s, it is passed directly to this function
    let stack_frame = unsafe { *sp };
    // safe: we know that stack_frame is safe so this operation is defined as safe by arm docs
    let svc = unsafe { *(stack_frame.pc as *const u8).offset(-2) }; // arm docs: svc_number = ((char *)svc_args[6])[-2];
    match (mode, svc) {
        // kernel syscalls
        (Mode::Kernel, 0) => {
            // return to first user thread
            drivers::rtt::rprintln!("here");
            unsafe {
                syscalls::tadi_140326a(sp as StackPointer<Byte>);
            }
        }
        (Mode::Kernel, _) => {
            drivers::rtt::rprintln!("svc number of '{}' for kernel is not implemented")
        }

        // user syscalls
        (Mode::User, 0) => {
            todo!("user syscall");
        }
        (Mode::User, _) => {
            drivers::rtt::rprintln!("svc number of '{}' for user is not implemented")
        }
    }
}

#[inline(never)]
#[unsafe(no_mangle)]
extern "C" fn pendsv_entry() {
    // context switching needs to be here
}

#[inline(never)]
#[unsafe(no_mangle)]
extern "C" fn HardFault(registers: &ExceptionFrame) {
    drivers::rtt::rprintln!("HardFault: {:?}", registers);
    loop {}
}

// =================================================================================================
// Faults
// =================================================================================================

// =================================================================================================
// Hardware Interrupts
// =================================================================================================

// =================================================================================================
// Util
// =================================================================================================
#[repr(C)]
#[derive(Debug)]
#[allow(dead_code)]
enum Mode {
    Kernel = 0,
    User = 1,
}

/// Registers stacked (pushed onto the stack) during an exception.
#[derive(Clone, Copy)]
#[repr(C)]
pub struct ExceptionFrame {
    r0: usize,
    r1: usize,
    r2: usize,
    r3: usize,
    r12: usize,
    lr: usize,
    pc: usize,
    xpsr: usize,
}

impl ExceptionFrame {
    /// Returns the value of (general purpose) register 0.
    #[inline(always)]
    pub fn r0(&self) -> usize {
        self.r0
    }

    /// Returns the value of (general purpose) register 1.
    #[inline(always)]
    pub fn r1(&self) -> usize {
        self.r1
    }

    /// Returns the value of (general purpose) register 2.
    #[inline(always)]
    pub fn r2(&self) -> usize {
        self.r2
    }

    /// Returns the value of (general purpose) register 3.
    #[inline(always)]
    pub fn r3(&self) -> usize {
        self.r3
    }

    /// Returns the value of (general purpose) register 12.
    #[inline(always)]
    pub fn r12(&self) -> usize {
        self.r12
    }

    /// Returns the value of the Link Register.
    #[inline(always)]
    pub fn lr(&self) -> usize {
        self.lr
    }

    /// Returns the value of the Program Counter.
    #[inline(always)]
    pub fn pc(&self) -> usize {
        self.pc
    }

    /// Returns the value of the Program Status Register.
    #[inline(always)]
    pub fn xpsr(&self) -> usize {
        self.xpsr
    }

    /// Sets the stacked value of (general purpose) register 0.
    ///
    /// # Safety
    ///
    /// This affects the `r0` register of the preempted code, which must not rely on it getting
    /// restored to its previous value.
    #[inline(always)]
    pub unsafe fn set_r0(&mut self, value: usize) {
        self.r0 = value;
    }

    /// Sets the stacked value of (general purpose) register 1.
    ///
    /// # Safety
    ///
    /// This affects the `r1` register of the preempted code, which must not rely on it getting
    /// restored to its previous value.
    #[inline(always)]
    pub unsafe fn set_r1(&mut self, value: usize) {
        self.r1 = value;
    }

    /// Sets the stacked value of (general purpose) register 2.
    ///
    /// # Safety
    ///
    /// This affects the `r2` register of the preempted code, which must not rely on it getting
    /// restored to its previous value.
    #[inline(always)]
    pub unsafe fn set_r2(&mut self, value: usize) {
        self.r2 = value;
    }

    /// Sets the stacked value of (general purpose) register 3.
    ///
    /// # Safety
    ///
    /// This affects the `r3` register of the preempted code, which must not rely on it getting
    /// restored to its previous value.
    #[inline(always)]
    pub unsafe fn set_r3(&mut self, value: usize) {
        self.r3 = value;
    }

    /// Sets the stacked value of (general purpose) register 12.
    ///
    /// # Safety
    ///
    /// This affects the `r12` register of the preempted code, which must not rely on it getting
    /// restored to its previous value.
    #[inline(always)]
    pub unsafe fn set_r12(&mut self, value: usize) {
        self.r12 = value;
    }

    /// Sets the stacked value of the Link Register.
    ///
    /// # Safety
    ///
    /// This affects the `lr` register of the preempted code, which must not rely on it getting
    /// restored to its previous value.
    #[inline(always)]
    pub unsafe fn set_lr(&mut self, value: usize) {
        self.lr = value;
    }

    /// Sets the stacked value of the Program Counter.
    ///
    /// # Safety
    ///
    /// This affects the `pc` register of the preempted code, which must not rely on it getting
    /// restored to its previous value.
    #[inline(always)]
    pub unsafe fn set_pc(&mut self, value: usize) {
        self.pc = value;
    }

    /// Sets the stacked value of the Program Status Register.
    ///
    /// # Safety
    ///
    /// This affects the `xPSR` registers (`IPSR`, `APSR`, and `EPSR`) of the preempted code, which
    /// must not rely on them getting restored to their previous value.
    #[inline(always)]
    pub unsafe fn set_xpsr(&mut self, value: usize) {
        self.xpsr = value;
    }
}

impl fmt::Debug for ExceptionFrame {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        struct Hex(usize);
        impl fmt::Debug for Hex {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                write!(f, "0x{:08x}", self.0)
            }
        }
        f.debug_struct("ExceptionFrame")
            .field("r0", &Hex(self.r0))
            .field("r1", &Hex(self.r1))
            .field("r2", &Hex(self.r2))
            .field("r3", &Hex(self.r3))
            .field("r12", &Hex(self.r12))
            .field("lr", &Hex(self.lr))
            .field("pc", &Hex(self.pc))
            .field("xpsr", &Hex(self.xpsr))
            .finish()
    }
}

#[derive(Clone, Debug)]
#[repr(C)]
pub struct CpuState {
    r4: usize,
    r5: usize,
    r6: usize,
    r7: usize,
    r8: usize,
    r9: usize,
    r10: usize,
    r11: usize,
    exception_frame: ExceptionFrame,
}

impl CpuState {
    pub fn generate(func: fn() -> !) -> Self {
        Self {
            exception_frame: ExceptionFrame {
                r0: 0,
                r1: 1,
                r2: 2,
                r3: 3,
                r12: 12,
                lr: 0,
                pc: func as usize,
                xpsr: 1 << 24, // we need to set the thumb state bit otherwise it will hardfault
            },
            r4: 4,
            r5: 5,
            r6: 6,
            r7: 7,
            r8: 8,
            r9: 9,
            r10: 10,
            r11: 11,
        }
    }
}
