use std::rc::Rc;

use crate::controllers::icontroller::IController;

use crate::contexts::controller_context::ControllerContext;
use crate::contexts::request_context::RequestContext;


pub struct IControllerExtensions {
    
}

impl IControllerExtensions {
    pub fn create_context(
        controller: Rc<dyn IController>,
        request_context: Rc<RequestContext>
    ) -> Rc<ControllerContext> {
        Rc::new(ControllerContext::new(controller, request_context))
    }
}