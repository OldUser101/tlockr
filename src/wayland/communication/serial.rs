use std::sync::atomic::{AtomicU32, Ordering};

static SERIAL_COUNTER: AtomicU32 = AtomicU32::new(1);

pub fn next_serial() -> u32 {
    SERIAL_COUNTER.fetch_add(1, Ordering::SeqCst)
}
