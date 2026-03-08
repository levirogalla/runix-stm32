use core::arch::asm;

use kernel_macros::{Reg, SafeRead, SafeWrite, UnsafeRead};

// #[derive(SafeRead)]
// #[reg(size=u32, name="_pc")]
// struct Pc;

#[derive(Reg, UnsafeRead, SafeRead)]
struct Pc;

// impl crate::hardware::registers::SafeReadReg for Pc {
//     fn read() -> Self::Type {
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
pub trait UnsafeReadReg: Reg {
    unsafe fn read_raw() -> Self::Type;
}

pub trait SafeReadReg: UnsafeReadReg {
    fn read() -> Self::Type;
}

pub trait UnsafeWriteReg: Reg {
    unsafe fn write(val: Self::Type);
}

pub trait SafeWriteReg: UnsafeWriteReg {
    fn write_raw(val: Self::Type);
}

