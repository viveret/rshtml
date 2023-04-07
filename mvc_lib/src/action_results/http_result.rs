use std::cell::RefCell;
use std::rc::Rc;

use http::StatusCode;

use crate::contexts::request_context::RequestContext;
use crate::contexts::response_context::ResponseContext;
use crate::contexts::controller_context::ControllerContext;

use crate::action_results::iaction_result::IActionResult;

use crate::services::service_collection::IServiceCollection;

pub struct HttpRedirectResult {
    pub redirect_target: String,
}

impl HttpRedirectResult {
    pub fn new(redirect_target: String) -> Self {
        Self { redirect_target: redirect_target }
    }
}

impl IActionResult for HttpRedirectResult {
    fn get_statuscode(self: &Self) -> StatusCode {
        StatusCode::TEMPORARY_REDIRECT
    }

    fn configure_response(self: &Self, _controller_ctx: Rc<RefCell<ControllerContext>>, response_ctx: Rc<RefCell<ResponseContext>>, _request_ctx: Rc<RequestContext>, _services: &dyn IServiceCollection) {
        let mut response = response_ctx.as_ref().borrow_mut();
        response.add_header_string("Location".to_string(), self.redirect_target.clone());
    }
}

// todo: redirect to action