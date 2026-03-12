.syntax unified //This lets us use C like comments!
.cpu cortex-m4 //Guess what this does
.thumb //Practically this only matters to the CPU, but it ensures that the correct types of instructions get included

.global SVCall
.thumb_func
SVCall:
	tst lr, 4 //TeST the 3rd bit in LR (4 is 0b1000, so its 3rd bit is 1)
	ite eq // tbh i don't really understand this instruction, but it does a condtional check to see if we are using msp or psp, then passes it to my function
	mrseq r0, msp // opposite
	mrsne r0, psp // do mrs command if test was not equal
    ite eq
    moveq r1, #1 // in mode enum 1 means users mode
    movne r1, #0 // in Mode enum 0 means kernel mode
    //  syscall_entry(sp, mode); we pass sp so that we can load the exception frame in the handler
    b syscall_entry // we also don't need to save the scratch registers because the rust compiler knows that they must be restored when it exists the syscall (or better said, we tell it to using extern "C")

.global PendSV
.thumb_func
PendSV: // assume the user was running in thread mode, so we can use psp

    // save current threads registers to the stack
    mrs r0, psp // buffer psp into r0 for the next insturction
    stmdb r0!, {r4-r11} // store the registers r4-r11 on the stack, this is needed so they can be restored later, will store lowest register at the lowest address
    msr psp, r0 // update the process stack pointer with the new value


    // call rust function to run the more complex logic of finding the next task/thread to run
    bl pendsv_entry // bl because we want to comeback

    // restore next threads state, the scratch registers will be automatically restored by the return instruction
    mrs r0, psp

.global enter_user_mode
.thumb_func
enter_user_mode:
    ldmia r0!, {r4-r11} //Load the registers from the stack
    msr psp, r0 //Set the process stack pointer to the value in r0, the first argument
    mov lr, #0xFFFFFFFD //Set the return address to the thread mode
    bx lr //Return to the thread mode
