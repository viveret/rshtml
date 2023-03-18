use std::any::Any;
use std::cell::RefCell;
use std::ops::Deref;
use std::rc::Rc;
use std::sync::{Arc, RwLock};

use http::StatusCode;

use crate::core::type_info::TypeInfo;

use crate::contexts::request_context::RequestContext;
use crate::contexts::response_context::ResponseContext;
use crate::contexts::controller_context::IControllerContext;
use crate::contexts::controller_context::ControllerContext;
use crate::contexts::view_context::IViewContext;
use crate::contexts::view_context::ViewContext;

use crate::action_results::iaction_result::IActionResult;
use crate::view::view_renderer::IViewRenderer;

use crate::services::service_collection::{IServiceCollection, ServiceCollectionExtensions};

pub struct ViewResult {
    pub path: String,
    pub model: Option<Rc<Box<dyn Any>>>,
}

impl ViewResult {
    pub fn new(path: String, model: Rc<Box<dyn Any>>) -> Self {
        Self { path: path, model: Some(model) }
    }

    pub fn new_no_model(path: String) -> Self {
        Self { path: path, model: None }
    }

    pub fn new_default_path(model: Rc<Box<dyn Any>>) -> Self {
        Self { path: "".to_string(), model: Some(model) }
    }
}

impl IActionResult for ViewResult {
    fn get_statuscode(self: &Self) -> StatusCode {
        StatusCode::OK
    }

    fn configure_response(self: &Self, controller_ctx: Rc<RefCell<ControllerContext>>, response_ctx: Rc<RefCell<ResponseContext>>, request_ctx: Rc<RequestContext>, services: Arc<RwLock<dyn IServiceCollection>>) {
        let view_renderer = ServiceCollectionExtensions::get_required_single::<dyn IViewRenderer>(services.clone().read().unwrap().deref());
        let layout_view_option = view_renderer.get_layout_view_from_context(controller_ctx.clone(), services.clone());


        // need to fix this to not return response and render directly to vector of bytes, or figure out way to correctly render parts in right order
        // problem is body can only return string to eval statement, and that is escaped for rendering as HTML safely.
        // have to bypass eval return aka render directly to buffer, skipping the delay of rendering the whole view before writing to body response
        let view_render_result = match layout_view_option {
            Some(layout_view) => {
                let body_view = view_renderer.get_view(&self.path, services.clone());
                // println!("Asked for {}, received {}", self.path, body_view.get_path());
                controller_ctx.as_ref().borrow_mut().get_view_data().as_ref().borrow_mut().insert("Body".to_string(), Rc::new(Box::new(body_view.clone())));

                let ctx = Arc::new(RwLock::new(ViewContext::new(layout_view.clone(), self.model.clone(), view_renderer.clone(), controller_ctx.clone(), response_ctx.clone(), request_ctx.clone())));
                view_renderer.render_view(ctx, services)
            },
            None => {
                view_renderer.render_partial(self.path.clone(), self.model.clone(), view_renderer.clone(), controller_ctx.clone(), response_ctx.clone(), request_ctx.clone(), services.clone())
            }
        };

        let mut response = response_ctx.as_ref().borrow_mut();
        response.add_header_str("Content-Type", "text/html");
        match view_render_result {
            Ok(ok_view_result) => {
                response.body.extend_from_slice(ok_view_result.content.as_bytes());
            },
            Err(err) => {
                response.body.extend_from_slice(format!("Error: {}", err).as_bytes());
            }
        }
    }
}