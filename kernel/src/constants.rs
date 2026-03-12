// ------------------- 0x20018000
// kernel Stack
// -------------------
// n user stacks
// -------------------
// heap
// ------------------- 0x20000000

// Since cortex m4 doesn't have paging or virtual memory we need to think of a different solution to ensure that user stacks can work with enough ram
// if we assign each thread fixed space in ram, then we will be severely limiting each threads ram space.
// I think a solution to this to use and std card to to make "swap memory" we will have maybe 2 or 3 stacks and 2 or 3 heaps in memory at and given time, these will be contiguous chunks. E.G.:

// ----
// kernel stack
// ----
// user stack (active)
// ----
// user stack (inactive)
// ----
// user heap (active)
// ----
// user heap (inactive)
// ----

// the m4 mpu can protect 8 regions, for this we only need 2, 1 for kernel, 1 for inactive memory, now the active thread cannot access anything else. TODO shared memory and ipc
// now when we switch threads we will switch to the currently in active memory, adjust mpu protections (hopefully that works on the fly), and prefetch the next thread to run. Obviously this will double the time it may take for a thread to be run if it becomes unblocked but I think this okay. If dma is involved, hopefully the sd card can fill the inactive stack while the active stack runs so that there is no delay for the next switch
// actually, I think we can dynamically choose how much stuff is sent to the disk because apps need to ask for heap and we can see how much is left from the pool to give them. so if no apps are asking for heap, we could have like all of our stacks in memory, but then if a thread asks for some heap mem, then we might have to start putting things on disk
// I guess this is pretty much paging except instead of pages its per thread memory chunks

pub const MAX_THREADS: usize = 0x40;
pub const KERNEL_STACK_START: *const usize = 0x20018000 as *const usize;
pub const STACK_SIZE: usize = 0x18000;
pub const THREAD_STACK_SIZE: usize = 0x800; // 2K stack by default this means the maximum space this would take up in static memory is 128K bytes
