use std::rc::Rc;
use std::net::{TcpStream, TcpListener};
use std::io::{Write, BufReader, BufRead};
use std::option::Option;
use std::vec::Vec;

use crate::app::http_request_pipeline::IHttpRequestPipeline;

use crate::contexts::connection_context::{ IConnectionContext, ConnectionContext };

use crate::options::http_options::IHttpOptions;

use crate::services::default_services::DefaultServices;
use crate::services::service_collection::IServiceCollection;
use crate::services::service_collection::ServiceCollection;
use crate::services::service_collection::ServiceCollectionExtensions;

// this is a trait for a class that can be used to configure and start a web program.
pub trait IWebProgram {
    // configure is called by the host to allow the program to configure itself.
    fn configure(self: &mut Self, args: Rc<Vec<String>>);
    
    // configure_services is called by the host to allow the program to add
    // services to the service collection.
    fn configure_services(self: &mut Self);
    
    // start is called by the host to allow the program to start itself.
    fn start(self: &Self, args: Rc<Vec<String>>);

    // get_services is called by the host to get the service collection.
    fn get_services(self: &Self) -> &dyn IServiceCollection;
}

// this is a struct that implements IWebProgram. it uses a builder pattern to configure itself.
pub struct WebProgram {
    on_configure_fn: Option<fn(&mut ServiceCollection, Rc<Vec<String>>)>,
    on_configure_services_fn: Option<fn(&mut ServiceCollection)>,
    onstart_fn: Option<fn(&dyn IServiceCollection)>,
    services: ServiceCollection,
}

impl WebProgram {
    pub fn new() -> Self {
        Self {
            services: ServiceCollection::new_root(),
            on_configure_fn: None,
            on_configure_services_fn: None,
            onstart_fn: None,
        }
    }

    pub fn on_configure(self: &mut Self, on_configure_fn: fn(&mut ServiceCollection, Rc<Vec<String>>)) -> &mut Self {
        self.on_configure_fn = Some(on_configure_fn);
        self
    }
    
    pub fn on_configure_services(self: &mut Self, on_configure_services_fn: fn(&mut ServiceCollection)) -> &mut Self {
        self.on_configure_services_fn = Some(on_configure_services_fn);
        self
    }

    pub fn on_start(self: &mut Self, onstart_fn: fn(&dyn IServiceCollection)) -> &mut Self {
        self.onstart_fn = Some(onstart_fn);
        self
    }

    pub fn client_connected(self: &Self, mut stream: TcpStream) {
        let buf_reader = BufReader::new(&mut stream);

        let mut request_headers: Vec<String> = buf_reader
            .lines()
            .map(|line| line.unwrap())
            .take_while(|x| !x.is_empty() )
            .collect();

        if request_headers.len() == 0 {
            println!("Could not read http header");
            return;
        }
        let http_header: String = request_headers.remove(0);

        let request_bytes = Vec::<u8>::new();
        let request_bytes_boxed = Box::new(request_bytes);

        let connection_context = Rc::new(ConnectionContext::new(stream.peer_addr().unwrap())) as Rc<dyn IConnectionContext>;

        let request_pipeline = ServiceCollectionExtensions::get_required_single::<dyn IHttpRequestPipeline>(&self.services);

        let response = request_pipeline.process_request(http_header, request_headers, request_bytes_boxed, connection_context, &self.services).expect("could not process request");
        stream.write_all(&response).expect("could not write response");
    }
}

impl IWebProgram for WebProgram {
    fn configure(self: &mut Self, args: Rc<Vec<String>>) {
        (self.on_configure_fn.unwrap())(&mut self.services, args);
    }
    
    fn configure_services(self: &mut Self) {
        (self.on_configure_services_fn.unwrap())(&mut self.services);

        DefaultServices::add_http_request_pipeline(&mut self.services);
    }

    fn start(self: &Self, _args: Rc<Vec<String>>) {
        (self.onstart_fn.unwrap())(&self.services);

        let options = ServiceCollectionExtensions::get_required_single::<dyn IHttpOptions>(&self.services);

        println!("Hosting at {}", options.get_ip_and_port());
        let listener = TcpListener::bind(options.get_ip_and_port()).unwrap();

        for stream in listener.incoming() {
            self.client_connected(stream.unwrap());
        }
    }

    fn get_services(self: &Self) -> &dyn IServiceCollection {
        &self.services
    }
}