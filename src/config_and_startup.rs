use std::any::Any;
use std::borrow::Cow;
use std::rc::Rc;

use phf::phf_map;

use mvc_lib::auth::iauthroles_dbset_provider::GenericAuthRolesDbSetProvider;

use mvc_lib::core::type_info::TypeInfo;

use mvc_lib::controllers::icontroller::IController;

use mvc_lib::controller_action_features::local_host_only::LocalHostOnlyControllerActionFeatureMiddleware;
use mvc_lib::middleware::authorize_controller_action_middleware::AuthorizeControllerActionFeatureMiddleware;

use mvc_lib::services::service_collection::{IServiceCollection, ServiceCollection};
use mvc_lib::services::service_scope::ServiceScope;
use mvc_lib::services::service_descriptor::ServiceDescriptor;
use mvc_lib::services::default_services::{*};
use mvc_lib::services::authorization_service::AuthorizationService;

use mvc_lib::options::http_options::{IHttpOptions, HttpOptions};
use mvc_lib::options::file_provider_controller_options::{IFileProviderControllerOptions, FileProviderControllerOptions};
use mvc_lib::options::logging_services_options::{ ILogHttpRequestsOptions, LogHttpRequestsOptions };

use mvc_lib::view::iview::IView;

use crate::view_models::dev::log_add::LogAddInputModelBinder;
use crate::views::authroles::index::view_authroles_index;
use crate::views::authroles::add::view_authroles_add;
use crate::views::dev::index::view_dev_index;
use crate::views::dev::log::view_dev_log;
use crate::views::dev::log_add::view_dev_log_add;
use crate::views::dev::log_clear::view_dev_log_clear;
use crate::views::dev::perf_log::view_dev_perf_log;
use crate::views::dev::views::view_dev_views;
use crate::views::dev::view_details::view_dev_view_details;
use crate::views::dev::controllers::view_dev_controllers;
use crate::views::dev::controller_details::view_dev_controller_details;
use crate::views::dev::routes::view_dev_routes;
use crate::views::dev::route_details::view_dev_route_details;
use crate::views::dev::sysinfo::view_dev_sysinfo;
use crate::views::home::index::view_home_index;
use crate::views::learn::index::view_learn_index;
use crate::views::learn::details::view_learn_details;
use crate::views::shared::_layout::view_shared__layout;

use crate::controllers::home_controller::HomeController;
use crate::controllers::learn_controller::LearnController;
use crate::controllers::dev_controller::DevController;
use crate::controllers::authroles_controller::AuthRolesController;


// add views to the service collection. Eventually this will be done automatically.
// services: the service collection to add the views to.
pub fn add_views(services: &mut ServiceCollection) {
    fn new_dev_views_service(_services: &dyn IServiceCollection) -> Vec<Box<dyn Any>> {
        vec![
            view_authroles_index::new_service(),
            view_authroles_add::new_service(),
            view_dev_index::new_service(),
            view_dev_log::new_service(),
            view_dev_log_add::new_service(),
            view_dev_log_clear::new_service(),
            view_dev_perf_log::new_service(),
            view_dev_views::new_service(),
            view_dev_view_details::new_service(),
            view_dev_controllers::new_service(),
            view_dev_controller_details::new_service(),
            view_dev_routes::new_service(),
            view_dev_route_details::new_service(),
            view_dev_sysinfo::new_service(),
            view_home_index::new_service(),
            view_learn_index::new_service(),
            view_learn_details::new_service(),
            view_shared__layout::new_service(),
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

// this is called when the program is configuring options (before it is started).
// services: the service collection to add options to.
// args: the command line arguments.
pub fn on_configure(services: &mut ServiceCollection, _args: Rc<Vec<String>>) -> () {
    services.add(ServiceDescriptor::new_closure(TypeInfo::rc_of::<dyn IHttpOptions>(), |_| vec![Box::new(Rc::new(HTTP_OPTIONS.clone()) as Rc<dyn IHttpOptions>)], ServiceScope::Singleton));
    services.add(ServiceDescriptor::new_closure(TypeInfo::rc_of::<dyn IFileProviderControllerOptions>(), |_| vec![Box::new(Rc::new(FILE_PROVIDER_OPTIONS.clone()) as Rc<dyn IFileProviderControllerOptions>)], ServiceScope::Singleton));

    // services.add_instance::<HttpOptions, dyn IHttpOptions>(TypeInfo::rc_of::<dyn IHttpOptions>(), &HTTP_OPTIONS);
    // services.add_instance::<FileProviderControllerOptions, dyn IFileProviderControllerOptions>(TypeInfo::rc_of::<dyn IFileProviderControllerOptions>(), &FILE_PROVIDER_OPTIONS);

    services.add(ServiceDescriptor::new_closure(TypeInfo::rc_of::<dyn ILogHttpRequestsOptions>(), |_| vec![Box::new(Rc::new(LogHttpRequestsOptions {
        // log_request: true,
        // log_response: true,
        log_request: false,
        log_response: false,
        // log_request_headers: true,
        // log_response_headers: true,
        // log_request_cookies: true,
        // log_response_cookies: true,
        log_request_headers: false,
        log_response_headers: false,
        log_request_cookies: false,
        log_response_cookies: false,
    }) as Rc<dyn ILogHttpRequestsOptions>)], ServiceScope::Singleton));
}

// add controllers to the service collection. Eventually this will be done automatically.
// services: the service collection to add the controllers to.
pub fn add_controllers(services: &mut ServiceCollection) {
    services.add(ServiceDescriptor::new(TypeInfo::rc_of::<dyn IController>(), HomeController::new_service, ServiceScope::Singleton));
    services.add(ServiceDescriptor::new(TypeInfo::rc_of::<dyn IController>(), LearnController::new_service, ServiceScope::Singleton));
    services.add(ServiceDescriptor::new(TypeInfo::rc_of::<dyn IController>(), DevController::new_service, ServiceScope::Singleton));
    services.add(ServiceDescriptor::new(TypeInfo::rc_of::<dyn IController>(), AuthRolesController::new_service, ServiceScope::Singleton));
}

// this is called when the program is configuring services (before it is started).
// services: the service collection to add services to.
pub fn on_configure_services(services: &mut ServiceCollection) -> () {
    DefaultServices::add_logging(services);
    DefaultServices::add_performance_logging(services);
    DefaultServices::add_file_provider(services);
    DefaultServices::add_error_handling(services);

    GenericAuthRolesDbSetProvider::add_to_services(services);

    AuthorizationService::add_to_services(services);

    DefaultServices::add_request_decoders(services);
    DefaultServices::add_response_encoders(services);

    LogAddInputModelBinder::add_to_services(services);
    DefaultServices::add_model_validators(services);

    add_views(services);
    add_controllers(services);
    DefaultServices::add_controllers(services);
    DefaultServices::add_default_request_middleware(services);
    DefaultServices::add_routing(services);

    // configure middleware
    DefaultServices::use_error_handling(services);
    DefaultServices::use_routing(services);
    DefaultServices::use_request_decoders(services);
    DefaultServices::use_response_encoders(services);
    DefaultServices::use_model_validation(services);


    AuthorizeControllerActionFeatureMiddleware::add_to_services(services);
    LocalHostOnlyControllerActionFeatureMiddleware::add_to_services(services);

    DefaultServices::add_execute_controller_action(services);
}

// this is called when the program is starting (after it is configured).
// services: the service collection to get services from.
pub fn onstart(_services: &dyn IServiceCollection) -> () {
    // let request = Rc::new(Request::builder()
    //                 .uri("https://www.rust-lang.org/")
    //                 .header("User-Agent", "awesome/1.0")
    //                 .body(Vec::new())
    //                 .unwrap());
}