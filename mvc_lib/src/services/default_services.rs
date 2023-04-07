use std::rc::Rc;
use std::any::Any;

use crate::core::type_info::TypeInfo;

use crate::app::http_request_pipeline::IHttpRequestPipeline;
use crate::app::http_request_pipeline::HttpRequestPipeline;

use crate::services::service_collection::IServiceCollection;
use crate::services::file_provider_service::FileProviderService;
use crate::services::file_provider_service::IFileProviderService;
use crate::services::service_collection::ServiceCollection;
use crate::services::service_descriptor::ServiceDescriptor;
use crate::services::service_scope::ServiceScope;
use crate::services::request_handler_service::{IRequestHandlerService, ControllerRequestHandlerService};

// use crate::view::rust_html_view::RustHtmlView;
use crate::view::view_renderer::IViewRenderer;
use crate::view::view_renderer::ViewRenderer;

use crate::controllers::icontroller::IController;
use crate::controllers::file_provider_controller::FileProviderController;

pub struct DefaultServices {
}

struct PrintLnLogger {
    pub lines_written: u32,
}

impl PrintLnLogger {
    pub fn new() -> Self {
        Self { lines_written: 0 }
    }
    pub fn new_service(_services: &dyn IServiceCollection) -> Vec<Box<dyn Any>> {
        vec![Box::new(Rc::new(PrintLnLogger::new()))]
    }
    #[allow(dead_code)]
    pub fn info(self: &mut Self, message: String) {
        println!("{}", message);
        self.lines_written += 1;
    }
}

impl DefaultServices {
    pub fn add_logging(services: &mut ServiceCollection) {
        services.add(ServiceDescriptor::new(TypeInfo::rc_of::<PrintLnLogger>(), PrintLnLogger::new_service, ServiceScope::Singleton));
    }

    pub fn add_file_provider(services: &mut ServiceCollection) {
        services.add(ServiceDescriptor::new(TypeInfo::rc_of::<dyn IFileProviderService>(), FileProviderService::new_service, ServiceScope::Singleton));
    }

    pub fn add_controllers(services: &mut ServiceCollection) {
        services.add(ServiceDescriptor::new(TypeInfo::rc_of::<dyn IController>(), FileProviderController::new_service, ServiceScope::Singleton));
    }

    pub fn add_request_handlers(services: &mut ServiceCollection) {
        services.add(ServiceDescriptor::new(TypeInfo::rc_of::<dyn IRequestHandlerService>(), ControllerRequestHandlerService::new_service, ServiceScope::Singleton));
    }

    pub fn add_request_decoders(services: &mut ServiceCollection) {
        services.add(ServiceDescriptor::new(TypeInfo::rc_of::<PrintLnLogger>(), PrintLnLogger::new_service, ServiceScope::Singleton));
    }

    pub fn add_response_encoders(services: &mut ServiceCollection) {
        services.add(ServiceDescriptor::new(TypeInfo::rc_of::<dyn IViewRenderer>(), ViewRenderer::new_service, ServiceScope::Singleton));
    }

    pub fn add_http_request_pipeline(services: &mut ServiceCollection) {
        services.add(ServiceDescriptor::new(TypeInfo::rc_of::<dyn IHttpRequestPipeline>(), HttpRequestPipeline::new_service, ServiceScope::Request));
    }
}