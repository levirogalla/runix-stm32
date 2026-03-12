use crate::{
    drivers,
    exceptions::{CpuState, ExceptionFrame},
    types::{Byte, StackPointer},
    user::temp_idle,
};

/// zeros the exception frame and sets the exception frames pc to a new the idle thread, then edits the cpu's lr so that we return to use
pub unsafe fn tadi_140326a(sp: StackPointer<Byte>) {
    unsafe extern "C" {
        fn enter_user_mode(sp: StackPointer<u32>);
    }
    unsafe {
        // update sp to gave space for the full cpu start we are going to put on the stack
        drivers::rtt::rprintln!("{:?}", *(sp as StackPointer<ExceptionFrame>));
        let sp = ((0x20000000 + 1024 * 96 - 1024 * 48) as StackPointer<CpuState>)
            .offset(-(size_of::<CpuState>() as isize)) as StackPointer<CpuState>;
        drivers::rtt::rprintln!("{:p}", sp);
        sp.write_volatile(CpuState::generate(temp_idle));
        drivers::rtt::rprintln!("{:?}", *(sp as StackPointer<CpuState>));
        enter_user_mode(sp as StackPointer<u32>);
    }
}
