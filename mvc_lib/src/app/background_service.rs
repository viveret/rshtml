use std::vec::Vec;

// trait for a background service that can be run in a separate thread than the main thread
pub trait BackgroundService {
    // run the background service
    fn run(self: &Self, args: Vec<String>);
}