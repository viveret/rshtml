use std::any::Any;
use std::rc::Rc;

use http::StatusCode;

use crate::contexts::irequest_context::IRequestContext;
use crate::contexts::response_context::ResponseContext;
use crate::contexts::controller_context::ControllerContext;

use crate::action_results::iaction_result::IActionResult;
use crate::view::view_renderer::IViewRenderer;
use crate::view::rusthtml::html_string::HtmlString;
use crate::view::rusthtml::rusthtml_error::RustHtmlError;

use crate::services::service_collection::{IServiceCollection, ServiceCollectionExtensions};

// this struct holds the path to the view and the model to be rendered
pub struct ViewResult {
    pub path: String,
    pub model: Rc<Option<Box<dyn Any>>>,
}

impl ViewResult {
    pub fn new(path: String, model: Box<dyn Any>) -> Self {
        Self { path: path, model: Rc::new(Some(model)) }
    }

    // this function creates a new ViewResult with no model
    pub fn new_no_model(path: String) -> Self {
        Self { path: path, model: Rc::new(None) }
    }

    // this function creates a new ViewResult with a specified model and default path
    pub fn new_default_path(model: Box<dyn Any>) -> Self {
        Self { path: "".to_string(), model: Rc::new(Some(model)) }
    }

    // write the view result to the response body
    pub fn write_response(self: &Self, view_render_result: Result<HtmlString, RustHtmlError>, response_ctx: Rc<ResponseContext>) {
        response_ctx.add_header_str("Content-Type", "text/html");
        match view_render_result {
            Ok(ok_view_result) => {
                response_ctx.body.borrow_mut().extend_from_slice(ok_view_result.content.as_bytes());
            },
            Err(err) => {
                response_ctx.body.borrow_mut().extend_from_slice(format!("Error: {}", err).as_bytes());
            }
        }
    }
}

impl IActionResult for ViewResult {
    fn get_statuscode(self: &Self) -> StatusCode {
        StatusCode::OK
    }

    fn configure_response(self: &Self, controller_ctx: Rc<ControllerContext>, response_ctx: Rc<ResponseContext>, _request_ctx: Rc<dyn IRequestContext>, services: &dyn IServiceCollection) {
        let view_renderer = ServiceCollectionExtensions::get_required_single::<dyn IViewRenderer>(services);
        let html = view_renderer.render_with_layout_if_specified(&self.path, self.model.clone(), controller_ctx.clone(), response_ctx.clone(), services);
        self.write_response(html, response_ctx)
    }
}