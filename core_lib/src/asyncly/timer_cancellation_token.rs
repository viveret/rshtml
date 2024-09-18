use std::cell::RefCell;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread::sleep;
use std::time::Duration;

use super::icancellation_token::ICancellationToken;


pub struct TimerCancellationToken {
    cancelled_by_timer: Arc<AtomicBool>,
    cancelled_by_other: RefCell<bool>,
    cancelled_reason: RefCell<String>,
    timer: RefCell<Option<std::thread::JoinHandle<()>>>,
}

impl TimerCancellationToken {
    pub fn new(duration: Duration) -> Self {
        let cancelled_by_timer = Arc::new(AtomicBool::new(false));
        let cancelled_by_other = RefCell::new(false);
        let cancelled_reason = RefCell::new(String::new());
        let c2 = cancelled_by_timer.clone();
        let timer = RefCell::new(Some(std::thread::spawn(move || {
            sleep(duration);
            c2.store(true, Ordering::Release);
        })));

        Self {
            cancelled_by_timer,
            cancelled_by_other,
            cancelled_reason,
            timer,
        }
    }

    pub fn wait_for_timer(&self) -> std::io::Result<()> {
        match self.timer.borrow_mut().take() {
            Some(handle) => {
                match handle.join() {
                    Ok(_) => Ok(()),
                    Err(e) => {
                        Err(std::io::Error::new(std::io::ErrorKind::ConnectionAborted, format!("{:?}", e)))
                    },
                }
            }
            None => Ok(()),
        }
    }

    pub fn stop(&self) -> std::io::Result<()> {
        self.cancelled_by_timer.store(true, Ordering::Release);
        self.wait_for_timer()
    }
}

impl ICancellationToken for TimerCancellationToken {
    fn is_cancelled(&self) -> bool {
        self.cancelled_by_timer.load(Ordering::Acquire) ||
        *self.cancelled_by_other.borrow()
    }

    fn cancel(&self) {
        self.cancelled_by_other.replace(true);
    }
    
    fn get_cancelled_reason(&self) -> String {
        if self.cancelled_by_timer.load(Ordering::Acquire) {
            "timer elapsed and expired".to_string()
        } else {
            self.cancelled_reason.borrow().clone()
        }
    }
    
    fn cancel_with_reason(&self, reason: String) {
        self.cancelled_by_other.replace(true);
        *self.cancelled_reason.borrow_mut() = reason;
    }
}