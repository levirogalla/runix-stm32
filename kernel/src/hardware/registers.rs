use core::arch::asm;

use kernel_macros::{Reg, SafeRead, UnsafeRead, UnsafeWrite};

// =================================================================================================
// General Purpose Registers
// =================================================================================================
#[derive(Reg, UnsafeRead, UnsafeWrite)]
pub struct R0;
#[derive(Reg, UnsafeRead, UnsafeWrite)]
pub struct R1;
#[derive(Reg, UnsafeRead, UnsafeWrite)]
pub struct R2;
#[derive(Reg, UnsafeRead, UnsafeWrite)]
pub struct R3;
#[derive(Reg, UnsafeRead, UnsafeWrite)]
pub struct R4;
#[derive(Reg, UnsafeRead, UnsafeWrite)]
pub struct R5;
#[derive(Reg, UnsafeRead, UnsafeWrite)]
pub struct R6;
#[derive(Reg, UnsafeRead, UnsafeWrite)]
pub struct R7;
#[derive(Reg, UnsafeRead, UnsafeWrite)]
pub struct R8;
#[derive(Reg, UnsafeRead, UnsafeWrite)]
pub struct R9;
#[derive(Reg, UnsafeRead, UnsafeWrite)]
pub struct R10;
#[derive(Reg, UnsafeRead, UnsafeWrite)]
pub struct R11;
#[derive(Reg, UnsafeRead, UnsafeWrite)]
pub struct R12;

// =================================================================================================
// Special Registers
// =================================================================================================
#[derive(Reg, UnsafeRead)]
pub struct Sp;
#[derive(Reg, UnsafeRead)]
#[reg(special)]
pub struct Msp;
#[derive(Reg, UnsafeRead)]
#[reg(special)]
pub struct Psp;
#[derive(Reg, UnsafeRead, UnsafeWrite)]
pub struct Lr;
#[derive(Reg, UnsafeRead)]
pub struct Pc;
#[derive(Reg, UnsafeRead)]
#[reg(special, name = "XPSR")]
pub struct Xpsr;

// =================================================================================================
// Mask Registers
// =================================================================================================

#[derive(Reg, UnsafeRead, SafeRead)]
#[reg(special, name = "PRIMASK")]
pub struct Primask;

impl Primask {
    pub fn enabled() -> bool {
        let bits = Self::read();
        (bits & 0b1) != 0b1
    }
    pub fn enable() {
        unsafe { asm!("cpsie i") }
    }
    pub fn disable() {
        unsafe { asm!("cpsid i") }
    }
}

#[derive(Reg, UnsafeRead)]
#[reg(special, name = "BASEPRI")]
pub struct Basepri;

#[derive(Reg, UnsafeRead)]
#[reg(special, name = "FAULTMASK")]
pub struct Faultmask;

pub trait Reg {
    type Size;
}
pub trait UnsafeReadReg: Reg {
    unsafe fn read_raw() -> Self::Size;
}

pub trait SafeReadReg: UnsafeReadReg {
    fn read() -> Self::Size;
}

pub trait UnsafeWriteReg: Reg {
    unsafe fn write_raw(val: Self::Size);
}

pub trait SafeWriteReg: UnsafeWriteReg {
    fn write(val: Self::Size);
}
