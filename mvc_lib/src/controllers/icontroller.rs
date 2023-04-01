use std::cell::RefCell;
use std::error::Error;
use std::result::Result;
use std::rc::Rc;

use crate::action_results::iaction_result::IActionResult;
use crate::contexts::request_context::RequestContext;
use crate::contexts::controller_context::ControllerContext;

use crate::services::service_collection::IServiceCollection;


pub trait IController {
    fn process_request(self: &Self, controller_ctx: Rc<RefCell<ControllerContext>>, request_ctx: Rc<RequestContext>, services: &dyn IServiceCollection) -> Result<Option<Box<dyn IActionResult>>, Box<dyn Error>>;

    fn get_route_area(self: &Self) -> Option<String>;
}