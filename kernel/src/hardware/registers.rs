use core::arch::asm;

use kernel_macros::{Reg, SafeGet, SafeSet};

// #[derive(SafeGet)]
// #[reg(size=u32, name="_pc")]
// struct Pc;

#[derive(Reg, SafeGet, SafeSet)]
#[reg(size=i32, name="te")]
struct Pc;

// impl crate::hardware::registers::SafeGetReg for Pc {
//     fn get() -> Self::Type {
//         let pc: Self::Type;
//         unsafe {
//             asm!("mov {}, pc", out(reg) pc);
//         }
//         pc
//     }
// }

// unsafe {
//     asm!("mov {}, pc", out(reg) pc);
// }

pub trait Reg {
    type Type;
}
pub trait SafeGetReg: Reg {
    #[inline(always)]
    fn get() -> Self::Type;
}

pub trait UnsafeGetReg: Reg {
    unsafe fn get() -> Self::Type;
}

pub trait SafeSetReg: Reg {
    fn set(val: Self::Type);
}

pub trait UnsafeSetReg: Reg {
    unsafe fn set(val: Self::Type);
}
