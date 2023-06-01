use std::any::Any;
use std::borrow::Cow;
use std::error::Error;
use std::rc::Rc;

use http::Method;
use mvc_lib::action_results::iaction_result::IActionResult;
use mvc_lib::contexts::controller_context::{ControllerContext, IControllerContext};
use mvc_lib::controllers::icontroller_extensions::IControllerExtensions;
use mvc_lib::services::service_collection::IServiceCollection;

use mvc_lib::action_results::view_result::ViewResult;

use mvc_lib::controllers::icontroller::IController;

use mvc_lib::controller_action_features::controller_action_feature::IControllerActionFeature;
use mvc_lib::controller_actions::controller_action::IControllerAction;
use mvc_lib::controller_actions::builder::ControllerActionsBuilder;

use crate::view_models::home::IndexViewModel;



// this is the controller for the home page.
pub struct HomeController {

}

impl HomeController {
    // create a new instance of the controller.
    pub fn new() -> Self {
        Self { }
    }

    // create a new instance of the controller as a service for a service collection.
    // services: the collection of available services.
    // returns: a new instance of the controller as a service in a vector.
    pub fn new_service(_services: &dyn IServiceCollection) -> Vec<Box<dyn Any>> {
        vec![Box::new(Rc::new(Self::new()) as Rc<dyn IController>)]
    }

    // this is the index action for the home controller.
    // this is the home page for the site.
    pub fn get_index(controller: &HomeController, _controller_ctx: &dyn IControllerContext, _services: &dyn IServiceCollection) -> Result<Option<Rc<dyn IActionResult>>, Box<dyn Error>> {
        let view_model = Box::new(Rc::new(IndexViewModel::new()));
        Ok(Some(Rc::new(ViewResult::new("views/home/index.rs".to_string(), view_model))))
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
        let actions_builder = ControllerActionsBuilder::new(self);
        let controller_name = Cow::Owned(IControllerExtensions::get_name_ref(self));
        
        actions_builder.add("/")
            .methods(&[Method::GET])
            .set_name("index")
            .set_controller_name(controller_name)
            .set_member_fn(Self::get_index);

        actions_builder.build()
    }

    fn get_features(self: &Self) -> Vec<Rc<dyn IControllerActionFeature>> {
        vec![]
    }

    fn as_any(self: &Self) -> &dyn Any {
        self
    }
}