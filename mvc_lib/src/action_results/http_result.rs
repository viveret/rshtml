use std::any::Any;
use std::cell::RefCell;
use std::rc::Rc;

use http::StatusCode;

use crate::contexts::request_context::RequestContext;
use crate::contexts::response_context::ResponseContext;
use crate::contexts::controller_context::IControllerContext;
use crate::contexts::controller_context::ControllerContext;
use crate::contexts::view_context::IViewContext;
use crate::contexts::view_context::ViewContext;

use crate::action_results::iaction_result::IActionResult;
use crate::view::view_renderer::IViewRenderer;
use crate::view::rusthtml::html_string::HtmlString;
use crate::view::rusthtml::rusthtml_error::RustHtmlError;

use crate::services::service_collection::{IServiceCollection, ServiceCollectionExtensions};

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

    fn configure_response(self: &Self, controller_ctx: Rc<RefCell<ControllerContext>>, response_ctx: Rc<RefCell<ResponseContext>>, request_ctx: Rc<RequestContext>, services: &dyn IServiceCollection) {
        let mut response = response_ctx.as_ref().borrow_mut();
        response.add_header_string("Location".to_string(), self.redirect_target.clone());
    }
}

// todo: redirect to action