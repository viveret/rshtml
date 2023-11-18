use std::cell::RefCell;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread::sleep;
use std::time::Duration;

use super::icancellation_token::ICancellationToken;


pub struct TimerCancellationToken {
    cancelled: Arc<AtomicBool>,
    timer: RefCell<Option<std::thread::JoinHandle<()>>>,
}

impl TimerCancellationToken {
    pub fn new(duration: Duration) -> Self {
        let cancelled = Arc::new(AtomicBool::new(false));
        let c2 = cancelled.clone();
        let timer = RefCell::new(Some(std::thread::spawn(move || {
            sleep(duration);
            c2.store(true, Ordering::Release);
        })));

        Self {
            cancelled,
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
        self.cancelled.store(true, Ordering::Release);
        self.wait_for_timer()
    }
}

impl ICancellationToken for TimerCancellationToken {
    fn is_cancelled(&self) -> bool {
        self.cancelled.load(Ordering::Acquire)
    }

    fn cancel(&self) {
        self.cancelled.store(true, Ordering::Release);
    }
}