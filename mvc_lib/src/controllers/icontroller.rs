use std::any::Any;
use std::borrow::Cow;
use std::rc::Rc;

use crate::controller_action_features::controller_action_feature::IControllerActionFeature;
use crate::controller_actions::controller_action::IControllerAction;

// this interface represents a routing mechanism for a controller where
// a controller maps to a route area, a controller name, and a set of actions.
// the controller is responsible for mapping the route area, controller name, and actions.
// examples of controllers include:
// - a controller that serves static files from the disk or memory
// - a controller that serves a REST API or a GraphQL API
// - a controller that serves HTML views for a web application
pub trait IController {
    // get the route area of the controller.
    fn get_route_area(self: &Self) -> String;
    // get the type name of the controller.
    fn get_type_name(self: &Self) -> &'static str;
    // get the actions of the controller.
    fn get_actions(self: &Self) -> Vec<Rc<dyn IControllerAction>>;
    // get the features of the controller.
    fn get_features(self: &Self) -> Vec<Rc<dyn IControllerActionFeature>>;
    // get the controller as an Any for downcasting.
    fn as_any(self: &Self) -> &dyn Any;
}