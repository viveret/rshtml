use crate::core::type_info::TypeInfo;

use crate::app::http_request_pipeline::{ IHttpRequestPipeline, HttpRequestPipeline };

use crate::services::service_descriptor::ServiceDescriptor;
use crate::services::service_scope::ServiceScope;
use crate::services::file_provider_service::{IFileProviderService, FileProviderService };
use crate::services::service_collection::{ ServiceCollection };
use crate::services::request_middleware_service::IRequestMiddlewareService;
use crate::services::controller_action_execute_service::ControllerActionExecuteService;
use crate::services::routing_service::RoutingService;
use crate::services::routemap_service::{ IRouteMapService, RouteMapService };
use crate::services::logging_services::LogHttpRequestsMiddleware;

use crate::view::view_renderer::{ IViewRenderer, ViewRenderer };

use crate::controllers::icontroller::IController;
use crate::controllers::file_provider_controller::FileProviderController;

pub struct DefaultServices {
}

impl DefaultServices {
    pub fn add_file_provider(services: &mut ServiceCollection) {
        services.add(ServiceDescriptor::new(TypeInfo::rc_of::<dyn IFileProviderService>(), FileProviderService::new_service, ServiceScope::Singleton));
    }

    pub fn add_controllers(services: &mut ServiceCollection) {
        services.add(ServiceDescriptor::new(TypeInfo::rc_of::<dyn IController>(), FileProviderController::new_service, ServiceScope::Singleton));
    }

    pub fn add_default_request_middleware(services: &mut ServiceCollection) {
        services.add(ServiceDescriptor::new(TypeInfo::rc_of::<dyn IRequestMiddlewareService>(), LogHttpRequestsMiddleware::new_service, ServiceScope::Singleton));
    }

    pub fn add_routing(services: &mut ServiceCollection) {
        // the actual map of actions
        services.add(ServiceDescriptor::new(TypeInfo::rc_of::<dyn IRouteMapService>(), RouteMapService::new_service, ServiceScope::Singleton));

        // the thing that uses map
        services.add(ServiceDescriptor::new(TypeInfo::rc_of::<dyn IRequestMiddlewareService>(), RoutingService::new_service, ServiceScope::Singleton));
    }

    pub fn add_execute_controller_action(services: &mut ServiceCollection) {
        // the thing that executes the request controller action from routing and other middleware
        services.add(ServiceDescriptor::new(TypeInfo::rc_of::<dyn IRequestMiddlewareService>(), ControllerActionExecuteService::new_service, ServiceScope::Singleton));
    }

    pub fn add_request_decoders(services: &mut ServiceCollection) {
        // services.add(ServiceDescriptor::new(TypeInfo::rc_of::<PrintLnLogger>(), PrintLnLogger::new_service, ServiceScope::Singleton));
    }

    pub fn add_response_encoders(services: &mut ServiceCollection) {
        services.add(ServiceDescriptor::new(TypeInfo::rc_of::<dyn IViewRenderer>(), ViewRenderer::new_service, ServiceScope::Singleton));
    }

    pub fn add_http_request_pipeline(services: &mut ServiceCollection) {
        services.add(ServiceDescriptor::new(TypeInfo::rc_of::<dyn IHttpRequestPipeline>(), HttpRequestPipeline::new_service, ServiceScope::Request));
    }
}