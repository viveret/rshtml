use std::any::Any;
use std::cell::RefCell;
use std::error::Error;
use std::result::Result;
use std::rc::Rc;
use std::sync::{Arc, RwLock};

use crate::services::service_collection::IServiceCollection;
use crate::services::service_collection::ServiceCollectionExtensions;

use crate::contexts::request_context::RequestContext;
use crate::contexts::controller_context::ControllerContext;

use crate::action_results::file_result::FileResult;
use crate::action_results::iaction_result::IActionResult;
use crate::controllers::icontroller::IController;

use crate::options::file_provider_controller_options::IFileProviderControllerOptions;


pub struct FileProviderController {
    pub options: Rc<Box<dyn IFileProviderControllerOptions>>,
}

impl FileProviderController {
    pub fn new(options: Rc<Box<dyn IFileProviderControllerOptions>>) -> Self {
        Self
        { 
            options: options
        }
    }

    pub fn new_service(services: &dyn IServiceCollection) -> Vec<Rc<dyn Any>> {
        vec![Rc::new(Box::new(Self::new(
            ServiceCollectionExtensions::get_required_single::<dyn IFileProviderControllerOptions>(services)
        )) as Box<dyn IController>)]
    }
}

impl IController for FileProviderController {
    fn get_route_area(self: &Self) -> Option<String> {
        None
    }

    fn process_request(self: &Self, _controller_ctx: Rc<RefCell<ControllerContext>>, request_ctx: Rc<RequestContext>, services: Arc<RwLock<dyn IServiceCollection>>) -> Result<Option<Box<dyn IActionResult>>, Box<dyn Error>> {
        let find_path = self.options.get_file(*request_ctx.path.clone());
        match find_path {
            Some(path) => Ok(Some(Box::new(FileResult::new(path, None)))),
            None => Ok(None),
        }
    }
}