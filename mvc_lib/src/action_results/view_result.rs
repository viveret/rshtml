use std::rc::Rc;

use http::StatusCode;

use crate::contexts::irequest_context::IRequestContext;
use crate::contexts::response_context::IResponseContext;
use crate::contexts::controller_context::IControllerContext;

use crate::action_results::iaction_result::IActionResult;
use crate::model_binder::imodel::IModel;
use crate::view::view_renderer::IViewRenderer;
use crate::view::rusthtml::html_string::HtmlString;
use crate::view::rusthtml::rusthtml_error::RustHtmlError;

use crate::services::service_collection::{IServiceCollection, ServiceCollectionExtensions};

// this struct holds the path to the view and the model to be rendered
pub struct ViewResult<T: IModel> {
    pub path: String,
    pub model: Option<Box<T>>,
}

impl<T: IModel> ViewResult<T> {
    pub fn new(path: String, model: Box<T>) -> Self {
        Self { path: path, model: Some(model) }
    }

    // this function creates a new ViewResult with no model
    pub fn new_no_model(path: String) -> Self {
        Self { path: path, model: None }
    }

    // this function creates a new ViewResult with a specified model and default path
    pub fn new_default_path(model: Box<T>) -> Self {
        Self { path: "".to_string(), model: Some(model) }
    }

    // write the view result to the response body
    pub fn write_response(self: &Self, view_render_result: Result<HtmlString, RustHtmlError>, response_context: &dyn IResponseContext) {
        response_context.add_header_str("Content-Type", "text/html");
        match view_render_result {
            Ok(ok_view_result) => {
                response_context.get_connection_context().write_line(&ok_view_result.content).unwrap();
            },
            Err(err) => {
                response_context.get_connection_context().write_str(format!("Error: {}", err).as_str()).unwrap();
            }
        }
    }
}

impl <T: 'static + IModel + Clone> IActionResult for ViewResult<T> {
    fn get_statuscode(self: &Self) -> StatusCode {
        StatusCode::OK
    }

    fn configure_response(self: &Self, controller_ctx: &dyn IControllerContext, response_context: &dyn IResponseContext, _request_context: &dyn IRequestContext, services: &dyn IServiceCollection) {
        let view_renderer = ServiceCollectionExtensions::get_required_single::<dyn IViewRenderer>(services);
        let model = Some(Rc::new(*(self.model.as_ref().unwrap().clone())) as Rc<dyn IModel>);
        let html = view_renderer.render_with_layout_if_specified(&self.path, model, controller_ctx, response_context.clone(), services);
        self.write_response(html, response_context)
    }
}