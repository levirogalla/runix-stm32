
// =================================================================================================
// Static Memory
// =================================================================================================

use critical_section::Mutex;

use crate::{constants::MAX_THREADS, state::ThreadContext};

// unsafe impl Sync for Mutex<[Option<ThreadContext>; MAX_THREADS]> {};


// =================================================================================================
// Make allocator so that we can use the heap
// =================================================================================================

extern crate alloc;

// this is todo for now, but looks to be pretty straight forward, just need to implement the GlobalAlloc trait and set a const variable to a struct that implements it
