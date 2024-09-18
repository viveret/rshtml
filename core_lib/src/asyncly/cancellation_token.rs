use std::{cell::RefCell, sync::atomic::{AtomicBool, Ordering}};

use super::icancellation_token::ICancellationToken;

pub struct CancellationToken {
    cancellation: AtomicBool,
    cancellation_reason: RefCell<String>
}

impl CancellationToken {
    pub fn new() -> CancellationToken {
        CancellationToken {
            cancellation: AtomicBool::new(false),
            cancellation_reason: RefCell::new(String::new())
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
    
    fn cancel_with_reason(&self, reason: String) {
        self.cancellation.store(true, Ordering::Release);
        self.cancellation_reason.replace(reason);
    }
    
    fn get_cancelled_reason(&self) -> String {
        self.cancellation_reason.borrow().clone()
    }
}