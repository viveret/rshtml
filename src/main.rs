use std::any::Any;
use std::borrow::Cow;
use std::rc::Rc;
use std::env;

use phf::phf_map;

extern crate mvc_lib;

use mvc_lib::core::type_info::TypeInfo;

use mvc_lib::app::web_program::{IWebProgram, WebProgram};

use mvc_lib::services::service_collection::{IServiceCollection, ServiceCollection};
use mvc_lib::services::service_scope::ServiceScope;
use mvc_lib::services::service_descriptor::ServiceDescriptor;
use mvc_lib::services::default_services::{*};

use mvc_lib::options::http_options::{IHttpOptions, HttpOptions};
use mvc_lib::options::file_provider_controller_options::{IFileProviderControllerOptions, FileProviderControllerOptions};

use mvc_lib::view::iview::IView;

pub mod views;

use views::dev::index::view_dev_index;
use views::dev::views::view_dev_views;
use views::dev::view_details::view_dev_view_details;
use views::dev::sysinfo::view_dev_sysinfo;
use views::home::index::view_home_index;
use views::shared::_Layout::view_shared__layout;



pub fn add_views(services: &mut ServiceCollection) {
    fn new_dev_views_service(services: &dyn IServiceCollection) -> Vec<Box<dyn Any>> {
        vec![
            Box::new(Rc::new(view_dev_index::new()) as Rc<dyn IView>) as Box<dyn Any>,
            Box::new(Rc::new(view_dev_views::new()) as Rc<dyn IView>) as Box<dyn Any>,
            Box::new(Rc::new(view_dev_view_details::new()) as Rc<dyn IView>) as Box<dyn Any>,
            Box::new(Rc::new(view_dev_sysinfo::new()) as Rc<dyn IView>) as Box<dyn Any>,
            Box::new(Rc::new(view_home_index::new()) as Rc<dyn IView>) as Box<dyn Any>,
            Box::new(Rc::new(view_shared__layout::new()) as Rc<dyn IView>) as Box<dyn Any>,
        ]
    }
    services.add(ServiceDescriptor::new(TypeInfo::rc_of::<dyn IView>(), new_dev_views_service, ServiceScope::Singleton));
}

static HTTP_OPTIONS: HttpOptions = HttpOptions { ip: Cow::Borrowed("127.0.0.1"), port: 8080, port_https: 8181 };
const SERVING_PATHS: [&'static str; 1] = ["wwwroot/"];
static SERVING_FILES: phf::Map<&'static str, &'static str> = phf_map! {
    "/stacks.min.css" => "ts/node_modules/@stackoverflow/stacks/dist/css/stacks.min.css",
    "/stacks.css" => "ts/node_modules/@stackoverflow/stacks/dist/css/stacks.css",
};
static FILE_PROVIDER_OPTIONS: FileProviderControllerOptions = FileProviderControllerOptions { serving_directories: &SERVING_PATHS, serving_files: &SERVING_FILES };

fn on_configure(services: &mut ServiceCollection, _args: Rc<Vec<String>>) -> () {
    services.add(ServiceDescriptor::new_closure(TypeInfo::rc_of::<dyn IHttpOptions>(), |x| vec![Box::new(Rc::new(HTTP_OPTIONS.clone()) as Rc<dyn IHttpOptions>)], ServiceScope::Singleton));
    services.add(ServiceDescriptor::new_closure(TypeInfo::rc_of::<dyn IFileProviderControllerOptions>(), |_| vec![Box::new(Rc::new(FILE_PROVIDER_OPTIONS.clone()) as Rc<dyn IFileProviderControllerOptions>)], ServiceScope::Singleton));

    // services.add_instance::<HttpOptions, dyn IHttpOptions>(TypeInfo::rc_of::<dyn IHttpOptions>(), &HTTP_OPTIONS);
    // services.add_instance::<FileProviderControllerOptions, dyn IFileProviderControllerOptions>(TypeInfo::rc_of::<dyn IFileProviderControllerOptions>(), &FILE_PROVIDER_OPTIONS);
}

fn on_configure_services(services: &mut ServiceCollection) -> () {
    DefaultServices::add_logging(services);
    DefaultServices::add_file_provider(services);

    // these can be added in any order, the HTTP request pipeline will decide usage
    add_views(services);
    DefaultServices::add_controllers(services);
    DefaultServices::add_request_handlers(services);
    DefaultServices::add_request_decoders(services);
    DefaultServices::add_response_encoders(services);

    // must be added after the pipeline parts
    DefaultServices::add_http_request_pipeline(services);
}

fn onstart(_services: &dyn IServiceCollection) -> () {
    // let request = Rc::new(Request::builder()
    //                 .uri("https://www.rust-lang.org/")
    //                 .header("User-Agent", "awesome/1.0")
    //                 .body(Vec::new())
    //                 .unwrap());
}

fn main() {
    println!("Hello world");
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
    prog.configure(args.clone());
    prog.configure_services();
    prog.start(args.clone());
}