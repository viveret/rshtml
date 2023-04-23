use std::rc::Rc;

use crate::controllers::icontroller::IController;

use crate::contexts::controller_context::ControllerContext;
use crate::contexts::irequest_context::IRequestContext;


pub struct IControllerExtensions {
    
}

impl IControllerExtensions {
    pub fn create_context(
        controller: Rc<dyn IController>,
        request_context: Rc<dyn IRequestContext>
    ) -> Rc<ControllerContext> {
        Rc::new(ControllerContext::new(controller, request_context))
    }

    pub fn get_name(controller: Rc<dyn IController>) -> String {
        let type_name = controller.get_type_name();
        type_name[..type_name.len() - "Controller".len()].to_string()
    }
}