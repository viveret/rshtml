use std::any::Any;
use std::cell::RefCell;
use std::error::Error;
use std::rc::Rc;

use crate::action_results::http_result::HttpRedirectResult;
use crate::contexts::irequest_context::IRequestContext;
use crate::contexts::response_context::IResponseContext;
use crate::services::request_middleware_service::IRequestMiddlewareService;
use crate::services::request_middleware_service::MiddlewareResult;
use crate::services::service_collection::IServiceCollection;




// this trait is for a middleware service that redirects HTTP requests to HTTPS.
pub trait IRedirectHttpsMiddlewareService: IRequestMiddlewareService {
    fn redirect_to_https(self: &Self, response_context: &dyn IResponseContext, request_context: &dyn IRequestContext) -> Result<MiddlewareResult, Box<dyn Error>>;
}

// define the middleware service that redirects HTTP requests to HTTPS.
pub struct RedirectHttpsMiddlewareService {
    // the next middleware in the pipeline
    next: RefCell<Option<Rc<dyn IRequestMiddlewareService>>>,
}

impl RedirectHttpsMiddlewareService {
    pub fn new() -> Self {
        Self { next: RefCell::new(None) }
    }

    // this is the function that will be called by the service collection to create a new instance of the middleware
    pub fn new_service(_services: &dyn IServiceCollection) -> Vec<Box<dyn Any>> {
        vec![Box::new(Rc::new(Self::new()) as Rc<dyn IRequestMiddlewareService>)]
    }
}

impl IRedirectHttpsMiddlewareService for RedirectHttpsMiddlewareService {
    fn redirect_to_https(self: &Self, response_context: &dyn IResponseContext, request_context: &dyn IRequestContext) -> Result<MiddlewareResult, Box<dyn Error>> {
        let mut url = request_context.get_url().clone();
        if let Err(_) = url.set_scheme("https") {
            return Err(Box::new(std::io::Error::new(std::io::ErrorKind::Other, format!("Error setting scheme to https"))));
        }
        HttpRedirectResult::config_response(response_context, url.to_string());
        Ok(MiddlewareResult::OkBreak)
    }
}

impl IRequestMiddlewareService for RedirectHttpsMiddlewareService {
    fn set_next(self: &Self, next: Option<Rc<dyn IRequestMiddlewareService>>) {
        self.next.replace(next);
    }

    fn handle_request(self: &Self, response_context: &dyn IResponseContext, request_context: &dyn IRequestContext, services: &dyn IServiceCollection) -> Result<MiddlewareResult, Box<dyn Error>> {
        if request_context.get_scheme() == "http" {
            return self.redirect_to_https(response_context, request_context);
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
}