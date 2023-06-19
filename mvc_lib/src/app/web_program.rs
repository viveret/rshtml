use std::cell::RefCell;
use std::rc::Rc;
use std::net::{TcpStream, TcpListener, Shutdown};
use std::option::Option;
use std::vec::Vec;

use crate::app::ihttp_request_pipeline::IHttpRequestPipeline;

use crate::contexts::httpconnection_context::HttpConnectionContext;
use crate::contexts::ihttpconnection_context::IHttpConnectionContext;
use crate::options::http_options::IHttpOptions;

use crate::services::default_services::DefaultServices;
use crate::services::service_collection::IServiceCollection;
use crate::services::service_collection::ServiceCollection;
use crate::services::service_collection::ServiceCollectionExtensions;
use crate::services::service_scope::ServiceScope;

// this is a trait for a class that can be used to configure and start a web program.
pub trait IWebProgram {
    // configure is called by the host to allow the program to configure itself.
    fn configure(self: &mut Self, args: Rc<Vec<String>>);
    
    // configure_services is called by the host to allow the program to add
    // services to the service collection.
    fn configure_services(self: &mut Self);
    
    // start is called by the host to allow the program to start itself.
    fn start(self: &Self, args: Rc<Vec<String>>);

    // main is called by the host to allow the program to configure options, configure services, and start itself.
    fn main(self: &mut Self, args: Rc<Vec<String>>);
}

// this is a struct that implements IWebProgram. it uses a builder pattern to configure itself.
pub struct WebProgram<'a> {
    on_configure_fn: Option<fn(&mut ServiceCollection, Rc<Vec<String>>)>,
    on_configure_services_fn: Option<fn(&mut ServiceCollection)>,
    onstart_fn: Option<fn(&dyn IServiceCollection)>,
    services_builder: RefCell<ServiceCollection<'a>>,
    next_client_connection_id: RefCell<u32>,
}

impl <'a> WebProgram<'a> {
    pub fn new() -> Self {
        Self {
            services_builder: RefCell::new(ServiceCollection::new_root()),
            on_configure_fn: None,
            on_configure_services_fn: None,
            onstart_fn: None,
            next_client_connection_id: RefCell::new(0),
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

    pub fn client_connected(self: &Self, client: Result<TcpStream, std::io::Error>) {
        let next_client_connection_id = self.next_client_connection_id.borrow().clone();
        *self.next_client_connection_id.borrow_mut() += 1;
        match client {
            Ok(stream) => {
                // stream.set_ttl(100).unwrap();
                // stream.set_nodelay(true).unwrap();
                // stream.set_nonblocking(true).unwrap();
                // stream.set_read_timeout(Some(Duration::from_secs(2))).unwrap();
                // stream.set_write_timeout(Some(Duration::from_secs(2))).unwrap();
                self.client_ready(stream, next_client_connection_id);
            },
            Err(e) => {
                panic!("Error: {}", e);
            }
        }
    }

    fn client_ready(&self, stream: TcpStream, connection_id: u32) {
        let connection_context = HttpConnectionContext::new_from_stream(stream, connection_id);

        let self_services = self.services_builder.borrow().clone();
        let connection_services = ServiceCollection::new(ServiceScope::Request, &self_services, self_services.get_root().unwrap_or(&self_services));

        // get the request pipeline from the connection services.
        let request_pipeline = ServiceCollectionExtensions::get_required_single::<dyn IHttpRequestPipeline>(&connection_services);

        // invoke the request pipeline to process the request and get the response.
        match request_pipeline.as_ref().process_request(&connection_context, &connection_services) {
            Ok(_) => {
                // the request was processed successfully, call response written event listeners.
                // todo: call response written event listeners.
            },
            Err(e) => {
                // unhandled error, panic. if the application shouldn't panic, then add middleware to handle the error.
                panic!("unhandled error occurred while processing request: {}", e);
            }
        }

        // flush the stream. this will send the response back to the client.
        match connection_context.flush() {
            Ok(_) => {},
            Err(e) => {
                println!("could not flush stream: {}", e);
            }
        }
        
        // shutdown the stream. this disconnects the client.
        match connection_context.shutdown(Shutdown::Both) {
            Ok(_) => {},
            Err(e) => {
                println!("could not shutdown stream: {}", e);
            }
        }
    }
}

impl <'a> IWebProgram for WebProgram<'a> {
    fn configure(self: &mut Self, args: Rc<Vec<String>>) {
        (self.on_configure_fn.unwrap())(&mut self.services_builder.borrow_mut(), args);
    }
    
    fn configure_services(self: &mut Self) {
        (self.on_configure_services_fn.unwrap())(&mut self.services_builder.borrow_mut());

        DefaultServices::add_http_request_pipeline(&mut self.services_builder.borrow_mut());
    }

    fn start(self: &Self, _args: Rc<Vec<String>>) {
        let services = &self.services_builder.clone().into_inner();
        (self.onstart_fn.unwrap())(services);

        let options = ServiceCollectionExtensions::get_required_single::<dyn IHttpOptions>(services);

        println!("Hosting at {}", options.get_ip_and_port());
        let listener = TcpListener::bind(options.get_ip_and_port()).unwrap();

        for stream in listener.incoming() {
            self.client_connected(stream);
        }
    }

    fn main(self: &mut Self, args: Rc<Vec<String>>) {
        self.configure(args.clone());
        self.configure_services();
        self.start(args);
    }
}