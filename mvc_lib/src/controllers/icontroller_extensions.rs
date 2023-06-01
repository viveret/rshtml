use std::rc::Rc;

use crate::controllers::icontroller::IController;

use crate::contexts::controller_context::{ControllerContext, IControllerContext};
use crate::contexts::irequest_context::IRequestContext;


// extension methods for IController
pub struct IControllerExtensions {}

impl IControllerExtensions {
    // creates a new instance of the controller context for the given controller and request context.
    // controller: the controller.
    // request_context: the request context.
    // returns: a new controller context.
    pub fn create_context<'a>(
        controller: Rc<dyn IController>,
        request_context: &'a dyn IRequestContext,
    ) -> ControllerContext {
        ControllerContext::new(controller, request_context)
    }

    // gets the name of the controller without the "Controller" suffix.
    // controller: the controller.
    // returns: the name of the controller without the "Controller" suffix.
    pub fn get_name_ref(controller: &dyn IController) -> String {
        let type_name = controller.get_type_name();
        type_name[..type_name.len() - "Controller".len()].to_string()
    }

    // gets the name of the controller without the "Controller" suffix.
    // controller: the controller.
    // returns: the name of the controller without the "Controller" suffix.
    pub fn get_name(controller: Rc<dyn IController>) -> String {
        let type_name = controller.get_type_name();
        type_name[..type_name.len() - "Controller".len()].to_string()
    }
}