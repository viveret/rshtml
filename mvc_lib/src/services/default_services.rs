use crate::core::type_info::TypeInfo;

use crate::app::http_request_pipeline::{ IHttpRequestPipeline, HttpRequestPipeline };

use crate::diagnostics::logging::log_http_requests::LogHttpRequestsMiddleware;
use crate::diagnostics::logging::logging_service::{LoggingService, ILoggingService};
use crate::diagnostics::performance::iperformance_logger_service::IPerformanceLoggerService;
use crate::diagnostics::performance::performance_logger_service::PerformanceLoggerService;
use crate::http::http_body_format_resolver::HttpBodyFormatResolver;
use crate::http::http_body_format_service::HttpBodyFormatService;
use crate::http::request_decoder_middleware::RequestDecoderMiddleware;
use crate::http::response_encoder_middleware::ResponseEncoderMiddleware;
use crate::model_binder::decoders::url_encoded_model_decoder::UrlEncodedDecoder;
use crate::model_binder::decoders::json_decoder::JsonDecoder;
use crate::model_binder::model_binder_middleware::ModelBinderMiddleware;
use crate::model_binder::model_binder_resolver::ModelBinderResolver;
use crate::model_binder::model_serializer_resolver::ModelEncoderResolver;
use crate::services::service_descriptor::ServiceDescriptor;
use crate::services::service_scope::ServiceScope;
use crate::services::file_provider_service::{IFileProviderService, FileProviderService };
use crate::services::service_collection::{ ServiceCollection };
use crate::services::request_middleware_service::IRequestMiddlewareService;
use crate::services::controller_action_execute_service::ControllerActionExecuteService;
use crate::services::routing_service::RoutingService;
use crate::services::routemap_service::{ IRouteMapService, RouteMapService };

use crate::view::view_renderer::{ IViewRenderer, ViewRenderer };

use crate::controllers::icontroller::IController;
use crate::controllers::file_provider_controller::FileProviderController;
use crate::model_binder::view_model_binder_resolver::ViewModelBinderResolver;



// this is a struct that holds the default services for the framework.
// these services are added to the service collection by default.
// the services can be removed from the service collection and replaced with custom services.
// default services include:
// - file provider service
// - controllers
// - routing
// - request decoders
// - response encoders
// - request pipeline
// - model validators
pub struct DefaultServices {}

impl DefaultServices {
    // add the default logging services to the service collection.
    pub fn add_logging(services: &mut ServiceCollection) {
        // services.add(ServiceDescriptor::new(TypeInfo::rc_of::<dyn ILogHttpRequestsOptions>(), LogHttpRequestsOptions::new_service, ServiceScope::Singleton));
        services.add(ServiceDescriptor::new(TypeInfo::rc_of::<dyn ILoggingService>(), LoggingService::new_service, ServiceScope::Singleton));
    }

    // add the default performance logging services to the service collection.
    pub fn add_performance_logging(services: &mut ServiceCollection) {
        // services.add(ServiceDescriptor::new(TypeInfo::rc_of::<dyn ILogHttpRequestsOptions>(), LogHttpRequestsOptions::new_service, ServiceScope::Singleton));
        services.add(ServiceDescriptor::new(TypeInfo::rc_of::<dyn IPerformanceLoggerService>(), PerformanceLoggerService::new_service, ServiceScope::Singleton));
    }

    // add the default file provider services to the service collection.
    pub fn add_file_provider(services: &mut ServiceCollection) {
        services.add(ServiceDescriptor::new(TypeInfo::rc_of::<dyn IFileProviderService>(), FileProviderService::new_service, ServiceScope::Singleton));
    }

    // add the default controllers to the service collection.
    pub fn add_controllers(services: &mut ServiceCollection) {
        services.add(ServiceDescriptor::new(TypeInfo::rc_of::<dyn IController>(), FileProviderController::new_service, ServiceScope::Singleton));
    }

    // add the default request middleware services to the service collection.
    pub fn add_default_request_middleware(services: &mut ServiceCollection) {
        services.add(ServiceDescriptor::new(TypeInfo::rc_of::<dyn IRequestMiddlewareService>(), LogHttpRequestsMiddleware::new_service, ServiceScope::Singleton));
    }

    // add the default routing services to the service collection.
    pub fn add_routing(services: &mut ServiceCollection) {
        // the actual map of actions
        services.add(ServiceDescriptor::new(TypeInfo::rc_of::<dyn IRouteMapService>(), RouteMapService::new_service, ServiceScope::Singleton));

        // the thing that uses map
        services.add(ServiceDescriptor::new(TypeInfo::rc_of::<dyn IRequestMiddlewareService>(), RoutingService::new_service, ServiceScope::Singleton));
    }

    // add the default controller action execution services to the service collection.
    pub fn add_execute_controller_action(services: &mut ServiceCollection) {
        // the thing that executes the request controller action from routing and other middleware
        services.add(ServiceDescriptor::new(TypeInfo::rc_of::<dyn IRequestMiddlewareService>(), ControllerActionExecuteService::new_service, ServiceScope::Singleton));
    }

    // add the default request pipeline services to the service collection.
    pub fn add_request_decoders(services: &mut ServiceCollection) {
        HttpBodyFormatResolver::add_to_services(services);
        HttpBodyFormatService::add_to_services(services);

        UrlEncodedDecoder::add_to_services(services);
        JsonDecoder::add_to_services(services);
    }

    // add the default response encoder services to the service collection.
    pub fn add_response_encoders(services: &mut ServiceCollection) {
        services.add(ServiceDescriptor::new(TypeInfo::rc_of::<dyn IViewRenderer>(), ViewRenderer::new_service, ServiceScope::Singleton));
        ModelEncoderResolver::add_to_services(services);
    }

    // add the default request decoder services to the service collection.
    pub fn use_request_decoders(services: &mut ServiceCollection) {
        RequestDecoderMiddleware::add_to_services(services);
    }

    // add the default response encoding middleware to the service collection.
    pub fn use_response_encoders(services: &mut ServiceCollection) {
        ResponseEncoderMiddleware::add_to_services(services);
    }

    // add the default http request pipeline services to the service collection.
    pub fn add_http_request_pipeline(services: &mut ServiceCollection) {
        services.add(ServiceDescriptor::new(TypeInfo::rc_of::<dyn IHttpRequestPipeline>(), HttpRequestPipeline::new_service, ServiceScope::Request));
    }

    // add the default model validator services to the service collection.
    pub fn add_model_validators(services: &mut ServiceCollection) {
        // add factories / binders / validators to services
        // FormUrlEncodedBinder::add_to_services(services);
        
        // add resolvers
        ModelBinderResolver::add_to_services(services);
        ViewModelBinderResolver::add_to_services(services);
    }

    // add the default model validation middleware to the service collection.
    pub fn use_model_validation(services: &mut ServiceCollection) {
        ModelBinderMiddleware::add_to_services(services);
    }
}