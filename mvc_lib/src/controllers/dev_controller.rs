use std::any::Any;
use std::cell::RefCell;
use std::error::Error;
use std::result::Result;
use std::rc::Rc;

use crate::services::service_collection::IServiceCollection;
use crate::services::service_collection::ServiceCollectionExtensions;

use crate::contexts::request_context::RequestContext;
use crate::contexts::controller_context::IControllerContext;
use crate::contexts::controller_context::ControllerContext;

use crate::action_results::view_result::ViewResult;
use crate::action_results::iaction_result::IActionResult;
use crate::controllers::icontroller::IController;

use crate::view::view_renderer::IViewRenderer;
use crate::view_models::dev::ViewsViewModel;

pub struct DevController {

}

impl DevController {
    pub fn new() -> Self {
        Self { }
    }

    pub fn new_service(_services: &dyn IServiceCollection) -> Vec<Box<dyn Any>> {
        vec![Box::new(Rc::new(Self::new()) as Rc<dyn IController>)]
    }
}

impl IController for DevController {
    fn get_route_area(self: &Self) -> Option<String> {
        None
    }

    fn process_request(self: &Self, controller_ctx: Rc<RefCell<ControllerContext>>, request_ctx: Rc<RequestContext>, services: &dyn IServiceCollection) -> Result<Option<Box<dyn IActionResult>>, Box<dyn Error>> {
        match request_ctx.path.as_str() {
            "/dev" => {
                controller_ctx.as_ref().borrow_mut().get_view_data().as_ref().borrow_mut().insert("Layout".to_string(), Rc::new(Box::new("views/shared/_Layout.rs")));
                Ok(Some(Box::new(ViewResult::new("views/dev/index.rs".to_string(), Box::new("")))))
            },
            "/dev/views" => {
                let view_renderer = ServiceCollectionExtensions::get_required_single::<dyn IViewRenderer>(services);
                let viewModel = Box::new(Rc::new(ViewsViewModel::new(view_renderer.get_all_views(services))));
                controller_ctx.as_ref().borrow_mut().get_view_data().as_ref().borrow_mut().insert("Layout".to_string(), Rc::new(Box::new("views/shared/_Layout.rs")));
                Ok(Some(Box::new(ViewResult::new("views/dev/views.rs".to_string(), viewModel))))
            },
            _ => Ok(None),
        }
    }
}