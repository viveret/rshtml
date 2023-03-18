use std::vec::Vec;

pub trait BackgroundService {
    fn run(self: &Self, args: Vec<String>);
}