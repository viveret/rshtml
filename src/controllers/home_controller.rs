use std::any::Any;
use std::rc::Rc;

use mvc_lib::services::service_collection::IServiceCollection;

use mvc_lib::action_results::view_result::ViewResult;

use mvc_lib::controllers::icontroller::IController;

use mvc_lib::controller_action_features::controller_action_feature::IControllerActionFeature;
use mvc_lib::controllers::controller_action::IControllerAction;
use mvc_lib::controllers::controller_action::ControllerActionClosure;

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
    fn get_route_area(self: &Self) -> String {
        String::new()
    }

    fn get_type_name(self: &Self) -> &'static str {
        nameof::name_of_type!(HomeController)
    }
    
    fn get_actions(self: &Self) -> Vec<Rc<dyn IControllerAction>> {
        vec![
            Rc::new(ControllerActionClosure::new_default_area(vec![], None, "/".to_string(), "Index".to_string(), self.get_type_name(), |_controller_ctx, _services| {
                let view_model = Box::new(Rc::new(IndexViewModel::new()));
                Ok(Some(Rc::new(ViewResult::new("views/home/index.rs".to_string(), view_model))))
            })),
        ]
    }

    fn get_features(self: &Self) -> Vec<Rc<dyn IControllerActionFeature>> {
        vec![]
    }
}