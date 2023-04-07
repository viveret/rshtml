use std::any::Any;
use std::cell::RefCell;
use std::error::Error;
use std::result::Result;
use std::rc::Rc;

use crate::services::service_collection::IServiceCollection;
use crate::services::service_collection::ServiceCollectionExtensions;

use crate::contexts::controller_context::IControllerContext;
use crate::contexts::controller_context::ControllerContext;

use crate::action_results::file_result::FileResult;
use crate::action_results::iaction_result::IActionResult;

use crate::controllers::icontroller::IController;
use crate::controllers::controller_actions_map::IControllerAction;

use crate::options::file_provider_controller_options::IFileProviderControllerOptions;


pub struct FileProviderController {
    pub options: Rc<dyn IFileProviderControllerOptions>,
}

impl FileProviderController {
    pub fn new(options: Rc<dyn IFileProviderControllerOptions>) -> Self {
        Self
        { 
            options: options
        }
    }

    pub fn new_service(services: &dyn IServiceCollection) -> Vec<Box<dyn Any>> {
        vec![Box::new(Rc::new(Self::new(
            ServiceCollectionExtensions::get_required_single::<dyn IFileProviderControllerOptions>(services)
        )) as Rc<dyn IController>)]
    }
}

impl IController for FileProviderController {
    fn get_route_area(self: &Self) -> &'static str {
        ""
    }

    fn get_name(self: &Self) -> &'static str {
        "FileProvider"
    }

    fn process_request(self: &Self, controller_ctx: Rc<RefCell<ControllerContext>>, _services: &dyn IServiceCollection) -> Result<Option<Box<dyn IActionResult>>, Box<dyn Error>> {
        let find_path = self.options.get_file(*controller_ctx.borrow().get_request_context().path.clone());
        match find_path {
            Some(path) => Ok(Some(Box::new(FileResult::new(path, None)))),
            None => Ok(None),
        }
    }

    fn get_actions(self: &Self) -> Vec<Box<dyn IControllerAction>> {
        vec![]
    }
}