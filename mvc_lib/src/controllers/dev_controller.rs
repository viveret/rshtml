use std::any::Any;
use std::cell::RefCell;
use std::error::Error;
use std::result::Result;
use std::rc::Rc;
use std::ops::Deref;

use crate::services::service_collection::IServiceCollection;
use crate::services::service_collection::ServiceCollectionExtensions;

use crate::contexts::controller_context::IControllerContext;
use crate::contexts::controller_context::ControllerContext;

use crate::action_results::view_result::ViewResult;
use crate::action_results::iaction_result::IActionResult;
use crate::action_results::http_result::HttpRedirectResult;

use crate::controllers::icontroller::IController;
use crate::controllers::icontroller_extensions::IControllerExtensions;
use crate::controllers::controller_actions_map::IControllerActionsMap;
use crate::controllers::controller_actions_map::IControllerAction;
use crate::controllers::controller_actions_map::ControllerActionsMap;
use crate::controllers::controller_actions_map::ControllerActionClosure;

use crate::view::view_renderer::IViewRenderer;

use crate::view_models::dev::IndexViewModel;
use crate::view_models::dev::ViewsViewModel;
use crate::view_models::dev::ViewDetailsViewModel;
use crate::view_models::dev::SysInfoViewModel;

use crate::routing::route_data::RouteData;


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
    fn get_route_area(self: &Self) -> &'static str {
        ""
    }

    fn get_name(self: &Self) -> &'static str {
        "Dev"
    }

    fn process_request(self: &Self, controller_context: Rc<RefCell<ControllerContext>>, services: &dyn IServiceCollection) -> Result<Option<Box<dyn IActionResult>>, Box<dyn Error>> {
        IControllerExtensions::process_mvc_request(controller_context.clone(), services)
    }
    
    fn get_actions(self: &Self) -> Vec<Box<dyn IControllerAction>> {
        vec![
            Box::new(ControllerActionClosure::new("/dev", "Index", self.get_name(), self.get_route_area(), |controller_ctx, services| {
                let view_renderer = ServiceCollectionExtensions::get_required_single::<dyn IViewRenderer>(services);
                let viewModel = Box::new(Rc::new(IndexViewModel::new()));
                // controller_ctx.as_ref().borrow_mut().get_view_data().as_ref().borrow_mut().insert("Layout".to_string(), Rc::new(Box::new("views/shared/_Layout.rs")));
                Ok(Some(Box::new(ViewResult::new("views/dev/index.rs".to_string(), viewModel))))
            })),
            Box::new(ControllerActionClosure::new("/dev/views", "Views", self.get_name(), self.get_route_area(), |controller_ctx, services| {
                let view_renderer = ServiceCollectionExtensions::get_required_single::<dyn IViewRenderer>(services);
                let viewModel = Box::new(Rc::new(ViewsViewModel::new(view_renderer.get_all_views(services))));
                // controller_ctx.as_ref().borrow_mut().get_view_data().as_ref().borrow_mut().insert("Layout".to_string(), Rc::new(Box::new("views/shared/_Layout.rs")));
                Ok(Some(Box::new(ViewResult::new("views/dev/views.rs".to_string(), viewModel))))
            })),
            Box::new(ControllerActionClosure::new("/dev/views/..", "ViewDetails", self.get_name(), self.get_route_area(), |controller_ctx, services| {
                let request_context = controller_ctx.borrow().get_request_context();
                let path = &request_context.path.as_str()["/dev/views/".len()..];

                if path.len() == 0 {
                    return Ok(Some(Box::new(HttpRedirectResult::new("/dev/views".to_string()))))
                }

                println!("Viewing view at path: {:?}", path);
                let view_renderer = ServiceCollectionExtensions::get_required_single::<dyn IViewRenderer>(services);
                let viewModel = Box::new(Rc::new(ViewDetailsViewModel::new(view_renderer.get_view(&path.to_string(), services))));
                // controller_ctx.as_ref().borrow_mut().get_view_data().as_ref().borrow_mut().insert("Layout".to_string(), Rc::new(Box::new("views/shared/_Layout.rs")));
                return Ok(Some(Box::new(ViewResult::new("views/dev/view_details.rs".to_string(), viewModel))));
            })),
            Box::new(ControllerActionClosure::new("/dev/sysinfo", "SysInfo", self.get_name(), self.get_route_area(), |controller_ctx, services| {
                let viewModel = Box::new(Rc::new(SysInfoViewModel::new()));
                Ok(Some(Box::new(ViewResult::new("views/dev/sysinfo.rs".to_string(), viewModel))))
            })),
        ]
    }
}