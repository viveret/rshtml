use std::vec::Vec;

use crate::app::background_service::BackgroundService;

pub trait WebApiService {

}

impl BackgroundService for dyn WebApiService {
    fn run(&self, _args: Vec<String>) {
        
    }
}