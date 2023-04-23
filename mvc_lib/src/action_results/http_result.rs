use std::rc::Rc;

use http::StatusCode;

use crate::contexts::irequest_context::IRequestContext;
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

    fn configure_response(self: &Self, _controller_ctx: Rc<ControllerContext>, response_ctx: Rc<ResponseContext>, _request_ctx: Rc<dyn IRequestContext>, _services: &dyn IServiceCollection) {
        response_ctx.add_header_string("Location".to_string(), self.redirect_target.clone());
    }
}

// todo: redirect to action