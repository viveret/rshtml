use std::any::Any;
use std::cell::RefCell;
use std::error::Error;
use std::result::Result;
use std::rc::Rc;

use crate::services::service_collection::IServiceCollection;
use crate::services::service_collection::ServiceCollectionExtensions;

use crate::contexts::controller_context::IControllerContext;
use crate::contexts::controller_context::ControllerContext;

use crate::action_results::view_result::ViewResult;
use crate::action_results::iaction_result::IActionResult;

use crate::controllers::icontroller::IController;
use crate::controllers::icontroller_extensions::IControllerExtensions;
use crate::controllers::controller_actions_map::IControllerAction;
use crate::controllers::controller_actions_map::IControllerActionsMap;
use crate::controllers::controller_actions_map::ControllerActionsMap;
use crate::controllers::controller_actions_map::ControllerActionClosure;

use crate::routing::route_data::RouteData;

use crate::view::view_renderer::IViewRenderer;

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
            Box::new(ControllerActionClosure::new_default_area("/", "Index", self.get_name(), |controller_ctx, services| {
                let view_renderer = ServiceCollectionExtensions::get_required_single::<dyn IViewRenderer>(services);
                let viewModel = Box::new(Rc::new(IndexViewModel::new()));
                Ok(Some(Box::new(ViewResult::new("views/home/index.rs".to_string(), viewModel))))
            })),
        ]
    }
}