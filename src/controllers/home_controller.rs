use std::any::Any;
use std::cell::RefCell;
use std::error::Error;
use std::result::Result;
use std::rc::Rc;

use mvc_lib::services::service_collection::IServiceCollection;

use mvc_lib::contexts::controller_context::ControllerContext;

use mvc_lib::action_results::view_result::ViewResult;
use mvc_lib::action_results::iaction_result::IActionResult;

use mvc_lib::controllers::icontroller::IController;
use mvc_lib::controllers::icontroller_extensions::IControllerExtensions;
use mvc_lib::controllers::controller_actions_map::IControllerAction;
use mvc_lib::controllers::controller_actions_map::ControllerActionClosure;

use crate::view_models::home::IndexViewModel;


pub struct HomeController {

}

impl HomeController {
    pub fn new() -> Self {
        Self { }
    }

    pub fn new_service(_services: &dyn IServiceCollection) -> Vec<Box<dyn Any>> {
        vec![Box::new(Rc::new(Self::new()) as Rc<dyn IController>)]
    }
}

impl IController for HomeController {
    fn get_route_area(self: &Self) -> &'static str {
        ""
    }

    fn get_name(self: &Self) -> &'static str {
        "Home"
    }

    fn process_request(self: &Self, controller_context: Rc<RefCell<ControllerContext>>, services: &dyn IServiceCollection) -> Result<Option<Box<dyn IActionResult>>, Box<dyn Error>> {
        IControllerExtensions::process_mvc_request(controller_context.clone(), services)
    }
    
    fn get_actions(self: &Self) -> Vec<Box<dyn IControllerAction>> {
        vec![
            Box::new(ControllerActionClosure::new_default_area("/", "Index", self.get_name(), |_controller_ctx, _services| {
                let view_model = Box::new(Rc::new(IndexViewModel::new()));
                Ok(Some(Box::new(ViewResult::new("views/home/index.rs".to_string(), view_model))))
            })),
        ]
    }
}