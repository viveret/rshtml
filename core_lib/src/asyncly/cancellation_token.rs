use std::sync::atomic::{AtomicBool, Ordering};

use super::icancellation_token::ICancellationToken;

pub struct CancellationToken {
    cancellation: AtomicBool,
}

impl CancellationToken {
    pub fn new() -> CancellationToken {
        CancellationToken {
            cancellation: AtomicBool::new(false),
        }
    }
}

impl ICancellationToken for CancellationToken {
    fn is_cancelled(&self) -> bool {
        self.cancellation.load(Ordering::Acquire)
    }

    fn cancel(&self) {
        self.cancellation.store(true, Ordering::Release);
    }
}