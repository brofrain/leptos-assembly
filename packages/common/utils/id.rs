use std::sync::atomic::{AtomicUsize, Ordering};

pub type Usize = usize;

static LAST_USIZE_ID: AtomicUsize = AtomicUsize::new(0);

pub fn runtime_usize() -> Usize {
    let id = LAST_USIZE_ID.load(Ordering::Acquire) + 1;
    LAST_USIZE_ID.store(id, Ordering::Release);
    id
}
