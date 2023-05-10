use std::any::Any;
use std::borrow::Cow;
use std::rc::Rc;

use crate::controller_action_features::controller_action_feature::IControllerActionFeature;
use crate::controller_actions::controller_action::IControllerAction;

pub trait IController {
    fn get_route_area(self: &Self) -> String;
    
    fn get_type_name(self: &Self) -> &'static str;
    
    fn get_controller_name(self: &Self) -> Cow<'static, str>;

    fn get_actions(self: &Self) -> Vec<Rc<dyn IControllerAction>>;

    fn get_features(self: &Self) -> Vec<Rc<dyn IControllerActionFeature>>;

    fn as_any(self: &Self) -> &dyn Any;
}