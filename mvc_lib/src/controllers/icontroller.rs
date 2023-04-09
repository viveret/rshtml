use std::rc::Rc;

use crate::controllers::controller_action::IControllerAction;
use crate::controllers::controller_action::IControllerActionFeature;

pub trait IController {
    fn get_route_area(self: &Self) -> String;
    
    fn get_name(self: &Self) -> String;

    fn get_actions(self: &Self) -> Vec<Rc<dyn IControllerAction>>;

    fn get_features(self: &Self) -> Vec<Rc<dyn IControllerActionFeature>>;
}