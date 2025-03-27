use std::any::Any;
use std::cell::RefCell;
use std::error::Error;
use std::rc::Rc;

use crate::action_results::http_result::OkResult;
use crate::contexts::irequest_context::IRequestContext;
use crate::contexts::iresponse_context::IResponseContext;
use crate::core::type_info::TypeInfo;

use crate::services::request_middleware_service::IRequestMiddlewareService;
use crate::services::request_middleware_service::MiddlewareResult;

use crate::services::service_collection::IServiceCollection;
use crate::services::service_collection::ServiceCollection;
use crate::services::service_collection::ServiceCollectionExtensions;
use crate::services::service_descriptor::ServiceDescriptor;
use crate::services::service_scope::ServiceScope;
use crate::services::wwwroot_provider_service::IWwwRootProviderService;


// this middleware is used to authorize a controller action.
pub struct WwwRootBrowserMiddleware {
    file_service: Rc<dyn IWwwRootProviderService>,
    next: RefCell<Option<Rc<dyn IRequestMiddlewareService>>>
}

impl WwwRootBrowserMiddleware {
    // create a new instance of the middleware.
    // file_service - the file list service. this is used to get the directories and files.
    // returns the new instance of the middleware.
    pub fn new(file_service: Rc<dyn IWwwRootProviderService>) -> Self {
        Self { file_service: file_service, next: RefCell::new(None) }
    }

    // this is the function that will be called by the service collection to create a new instance of the middleware
    // services - the service collection
    // returns a vector containing the new instance of the middleware.
    pub fn new_service(services: &dyn IServiceCollection) -> Vec<Box<dyn Any>> {
        vec![Box::new(Rc::new(Self::new(
            ServiceCollectionExtensions::get_required_single::<dyn IWwwRootProviderService>(services)
        )) as Rc<dyn IRequestMiddlewareService>)]
    }
    
    // this is called by the application to add the middleware to the service collection
    // services - the service collection
    // returns nothing
    pub fn add_to_services(services: &mut ServiceCollection) {
        services.add(ServiceDescriptor::new(TypeInfo::rc_of::<dyn IRequestMiddlewareService>(), Self::new_service, ServiceScope::Singleton));
    }
}

impl IRequestMiddlewareService for WwwRootBrowserMiddleware {
    fn set_next(&self, next: Option<Rc<dyn IRequestMiddlewareService>>) {
        self.next.replace(next);
    }

    fn handle_request(&self, response_context: &dyn IResponseContext, request_context: &dyn IRequestContext, services: &dyn IServiceCollection) -> Result<MiddlewareResult, Rc<dyn Error>> {
        let url = request_context.get_url();
        let url = url.path();
        let base_path = "/wwwroot-browser/";
        if url.starts_with(base_path) {
            let path_name = url.split_at(base_path.len()).1;
            if path_name.is_empty() || path_name.chars().all(|c| c.is_alphanumeric() || c == '/' || c == '.') {
                if !path_name.contains("../") { // do not serve upward requests
                    let search_path = path_name;
                    match self.file_service.list(search_path) {
                        Ok(entries) => {
                            println!("Listing entries in {}", search_path);
                            response_context.set_action_result(Some(Rc::new(OkResult::new(entries.join("\n")))));
                            return Ok(MiddlewareResult::OkBreak); // short circuit middleware
                        },
                        Err(e) => {
                            println!("File browser denied {}, error: {:?}", search_path, e);
                            response_context.set_status_code(http::StatusCode::NOT_FOUND);
                            return Ok(MiddlewareResult::OkBreak); // short circuit middleware
                        },
                    }
                }
            }
        }

        if let Some(next) = self.next.borrow().as_ref() {
            let next_response = next.handle_request(response_context, request_context, services)?;

            match next_response {
                MiddlewareResult::OkBreak => {
                    return Ok(MiddlewareResult::OkBreak); // short circuit middleware
                },
                _ => { }
            }
        }

        Ok(MiddlewareResult::OkContinue)
    }

    fn get_type_info(&self) -> Box<TypeInfo> {
        Box::new(TypeInfo::of::<WwwRootBrowserMiddleware>())
    }
}