use core::{cell::RefCell, sync::atomic::AtomicUsize};

use critical_section::{CriticalSection, Mutex};

use crate::constants::MAX_THREADS;

#[derive(Clone, Copy)]
pub struct ThreadContext {
    sp: *const u32,
}

pub struct ThreadPool(Mutex<RefCell<[Option<ThreadContext>; MAX_THREADS]>>);
unsafe impl Sync for ThreadPool {} // this is safe because the inner is wrapped in a mutex
// all accesses to the thread pool go through these function, I think I could also add a bit mask to quickly see which spots are available in the thread pool
impl ThreadPool {
    pub const fn new(pool: [Option<ThreadContext>; MAX_THREADS]) -> Self {
        Self(Mutex::new(RefCell::new(pool)))
    }
    /// Linear searches for the first free slot starting from the beginning, returns None if no space was found
    pub fn add(&self, cs: CriticalSection<'_>, thread: ThreadContext) -> Option<()> {
        for slot in self.0.borrow_ref_mut(cs).iter_mut() {
            if slot.is_none() {
                *slot = Some(thread);
                return Some(());
            }
        }
        None
    }

    pub fn peek<R>(
        &self,
        cs: CriticalSection<'_>,
        f: impl FnOnce(&[Option<ThreadContext>]) -> R,
    ) -> R {
        let borrow = self.0.borrow_ref(cs);
        f(borrow.as_slice())
    }

    pub fn remove() {
        todo!("thread pool remove")
    }
}

// this scheduler trait would allows as to have multiple different schedulers for the same pool of threads, all of them could have different behaviors, one scheduler does not need to care about another schedulers actions because we can ensure that all behavior is valid since adds and remove go through the controlled thread pool
pub trait Scheduler {
    fn init(threads: &'static ThreadPool) -> Self;
    // get the next thread to run
    fn next(&self) -> Option<ThreadContext>;
    // schedule the current thread
    fn sched(&self, thread: ThreadContext);
}

// =================================================================================================
// Logic for Schedulers
// =================================================================================================

pub struct RoundRobinScheduler {
    threads: &'static ThreadPool,
    state: AtomicUsize,
}

impl Scheduler for RoundRobinScheduler {
    fn init(threads: &'static ThreadPool) -> Self {
        Self {
            threads,
            state: AtomicUsize::new(0),
        }
    }

    fn next(&self) -> Option<ThreadContext> {
        critical_section::with(|cs| {
            self.threads.peek(cs, |pool| {
                let current_index = self.state.load(core::sync::atomic::Ordering::SeqCst);
                pool[current_index..]
                    .iter()
                    .chain(&pool[..current_index])
                    .find_map(|slot| *slot)
            })
        })
    }
    fn sched(&self, thread: ThreadContext) {
        critical_section::with(|cs| {
            self.threads.add(cs, thread);
        })
    }
}
