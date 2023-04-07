use std::any::Any;
use std::cell::RefCell;
use std::error::Error;
use std::result::Result;
use std::rc::Rc;

use mvc_lib::services::service_collection::IServiceCollection;
use mvc_lib::services::service_collection::ServiceCollectionExtensions;

use mvc_lib::contexts::controller_context::IControllerContext;
use mvc_lib::contexts::controller_context::ControllerContext;

use mvc_lib::action_results::view_result::ViewResult;
use mvc_lib::action_results::iaction_result::IActionResult;
use mvc_lib::action_results::http_result::HttpRedirectResult;

use mvc_lib::controllers::icontroller::IController;
use mvc_lib::controllers::icontroller_extensions::IControllerExtensions;
use mvc_lib::controllers::controller_actions_map::IControllerAction;
use mvc_lib::controllers::controller_actions_map::ControllerActionClosure;

use mvc_lib::view::view_renderer::IViewRenderer;

use crate::view_models::dev::IndexViewModel;
use crate::view_models::dev::ViewsViewModel;
use crate::view_models::dev::ViewDetailsViewModel;
use crate::view_models::dev::SysInfoViewModel;


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
            Box::new(ControllerActionClosure::new("/dev", "Index", self.get_name(), self.get_route_area(), |_controller_ctx, _services| {
                let view_model = Box::new(Rc::new(IndexViewModel::new()));
                Ok(Some(Box::new(ViewResult::new("views/dev/index.rs".to_string(), view_model))))
            })),
            Box::new(ControllerActionClosure::new("/dev/views", "Views", self.get_name(), self.get_route_area(), |_controller_ctx, services| {
                let view_renderer = ServiceCollectionExtensions::get_required_single::<dyn IViewRenderer>(services);
                let view_model = Box::new(Rc::new(ViewsViewModel::new(view_renderer.get_all_views(services))));
                Ok(Some(Box::new(ViewResult::new("views/dev/views.rs".to_string(), view_model))))
            })),
            Box::new(ControllerActionClosure::new("/dev/views/..", "ViewDetails", self.get_name(), self.get_route_area(), |controller_ctx, services| {
                let request_context = controller_ctx.borrow().get_request_context();
                let path = &request_context.path.as_str()["/dev/views/".len()..];

                if path.len() == 0 {
                    return Ok(Some(Box::new(HttpRedirectResult::new("/dev/views".to_string()))))
                }

                println!("Viewing view at path: {:?}", path);
                let view_renderer = ServiceCollectionExtensions::get_required_single::<dyn IViewRenderer>(services);
                let view_model = Box::new(Rc::new(ViewDetailsViewModel::new(view_renderer.get_view(&path.to_string(), services))));
                return Ok(Some(Box::new(ViewResult::new("views/dev/view_details.rs".to_string(), view_model))));
            })),
            Box::new(ControllerActionClosure::new("/dev/sysinfo", "SysInfo", self.get_name(), self.get_route_area(), |_controller_ctx, _services| {
                let view_model = Box::new(Rc::new(SysInfoViewModel::new()));
                Ok(Some(Box::new(ViewResult::new("views/dev/sysinfo.rs".to_string(), view_model))))
            })),
        ]
    }
}