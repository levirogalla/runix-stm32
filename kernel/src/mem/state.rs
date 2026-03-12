pub mod threads {
    // make new scope for threads since SendThreadContext is only safe in this scope

    use crate::constants::MAX_THREADS;
    use crate::sched::ThreadPool;

    static THREADS: ThreadPool = ThreadPool::new([const { None }; MAX_THREADS]);

    // round robin scheduler
}
