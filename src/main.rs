extern crate mvc_lib;

pub mod controllers;
pub mod helpers;
pub mod views;
pub mod view_models;
pub mod config_and_startup;

use std::rc::Rc;
use std::env;

use mvc_lib::app::web_program::{ IWebProgram, WebProgram };
use config_and_startup::{ on_configure, on_configure_services, onstart };


fn main() {
    let args: Rc<Vec<String>> = Rc::new(env::args().collect());
    
    let mut prog = WebProgram::new();
    prog
        // program configuration
        .on_configure(on_configure)
        // service / dependency injection configuration
        .on_configure_services(on_configure_services)
        // thread on start configuration
        // (last line starts listening for incoming HTTTP requests)
        .on_start(onstart);

    // Now do configuration and start web app
    prog.main(args.clone());
}