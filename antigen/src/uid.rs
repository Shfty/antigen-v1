use std::sync::atomic::{AtomicUsize, Ordering};

pub type UID = usize;

pub fn new() -> UID {
    static COUNTER: AtomicUsize = AtomicUsize::new(1);
    COUNTER.fetch_add(1, Ordering::Relaxed)
}
