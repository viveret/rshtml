use std::rc::Rc;

use http::StatusCode;

use crate::contexts::irequest_context::IRequestContext;
use crate::contexts::response_context::IResponseContext;

use crate::action_results::iaction_result::IActionResult;
use crate::model_binder::iviewmodel::IViewModel;
use crate::view::view_renderer::IViewRenderer;
use crate::view::rusthtml::html_string::HtmlString;
use crate::view::rusthtml::rusthtml_error::RustHtmlError;

use crate::services::service_collection::{IServiceCollection, ServiceCollectionExtensions};

// this struct holds the path to the view and the model to be rendered
#[derive(Clone)]
pub struct ViewResult {
    pub path: String,
    pub model: Option<Rc<dyn IViewModel>>,
}

impl ViewResult {
    pub fn new(path: String, model: Rc<dyn IViewModel>) -> Self {
        Self { path: path, model: Some(model) }
    }

    // this function creates a new ViewResult with no model
    pub fn new_no_model(path: String) -> Self {
        Self { path: path, model: None }
    }

    // this function creates a new ViewResult with a specified model and default path
    pub fn new_default_path(model: Rc<dyn IViewModel>) -> Self {
        Self { path: "".to_string(), model: Some(model) }
    }

    // write the view result to the response body
    pub fn write_response(self: &Self, view_render_result: Result<HtmlString, RustHtmlError>, response_context: &dyn IResponseContext) -> std::io::Result<()> {
        response_context.add_header_str("Content-Type", "text/html");
        match view_render_result {
            Ok(ok_view_result) => {
                response_context.get_connection_context().write_line(&ok_view_result.content)?;
            },
            Err(err) => {
                response_context.get_connection_context().write_str(format!("Error: {}", err).as_str())?;
            }
        }
        Ok(())
    }
}

impl IActionResult for ViewResult {
    fn get_statuscode(self: &Self) -> StatusCode {
        StatusCode::OK
    }

    fn configure_response(self: &Self, response_context: &dyn IResponseContext, request_context: &dyn IRequestContext, services: &dyn IServiceCollection) -> Result<(), Rc<dyn std::error::Error>> {
        let view_renderer = ServiceCollectionExtensions::get_required_single::<dyn IViewRenderer>(services);
        let html = view_renderer.render_with_layout_if_specified(&self.path, self.model.clone(), response_context, request_context, services);
        match self.write_response(html, response_context) {
            Ok(_) => Ok(()),
            Err(err) => Err(Rc::new(err)),
        }
    }
}

impl std::fmt::Debug for ViewResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.model {
            Some(model) => write!(f, "ViewResult: path: {}, model: {:?}", self.path, model.as_ref().get_type_info()),
            None => write!(f, "ViewResult: path: {}, model: None", self.path),
        }
    }
}