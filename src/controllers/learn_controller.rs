use std::any::Any;
use std::cell::RefCell;
use std::error::Error;
use std::result::Result;
use std::rc::Rc;

use glob::glob;

use mvc_lib::action_results::http_result::HttpRedirectResult;

use mvc_lib::services::service_collection::IServiceCollection;

use mvc_lib::contexts::controller_context::IControllerContext;
use mvc_lib::contexts::controller_context::ControllerContext;

use mvc_lib::action_results::view_result::ViewResult;
use mvc_lib::action_results::iaction_result::IActionResult;

use mvc_lib::controllers::icontroller::IController;
use mvc_lib::controllers::icontroller_extensions::IControllerExtensions;
use mvc_lib::controllers::controller_actions_map::IControllerAction;
use mvc_lib::controllers::controller_actions_map::ControllerActionClosure;

use crate::view_models::learn::IndexViewModel;
use crate::view_models::learn::DetailsViewModel;

pub struct LearnController {

}

impl LearnController {
    pub fn new() -> Self {
        Self { }
    }

    pub fn new_service(_services: &dyn IServiceCollection) -> Vec<Box<dyn Any>> {
        vec![Box::new(Rc::new(Self::new()) as Rc<dyn IController>)]
    }
}

impl IController for LearnController {
    fn get_route_area(self: &Self) -> &'static str {
        ""
    }

    fn get_name(self: &Self) -> &'static str {
        "Learn"
    }

    fn process_request(self: &Self, controller_context: Rc<RefCell<ControllerContext>>, services: &dyn IServiceCollection) -> Result<Option<Box<dyn IActionResult>>, Box<dyn Error>> {
        IControllerExtensions::process_mvc_request(controller_context.clone(), services)
    }
    
    fn get_actions(self: &Self) -> Vec<Box<dyn IControllerAction>> {
        vec![
            Box::new(ControllerActionClosure::new_default_area("/learn", "Index", self.get_name(), |_controller_ctx, _services| {
                let learn_docs: Vec<String> = glob("docs/learn/**/*.md")
                    .expect("Failed to read glob pattern")
                    .map(|path_to_string| {
                        let p = path_to_string.unwrap();
                        let path = p.as_path().to_str().unwrap();
                        let s = &path["docs/learn/".len()..path.len() - 3];// remove extension ".md"
                        s.to_string()
                    })
                    .filter(|x| x.as_str() != "README")
                    .collect();

                let view_model = Box::new(Rc::new(IndexViewModel::new(learn_docs)));
                Ok(Some(Box::new(ViewResult::new("views/learn/index.rs".to_string(), view_model))))
            })),
            Box::new(ControllerActionClosure::new_default_area("/learn/..", "Details", self.get_name(), |controller_ctx, _services| {
                let request_context = controller_ctx.borrow().get_request_context();
                let path = &request_context.path.as_str()["/learn/".len()..];

                if path.len() == 0 {
                    return Ok(Some(Box::new(HttpRedirectResult::new("/learn".to_string()))))
                }

                let view_model = Box::new(Rc::new(DetailsViewModel::new(format!("docs/learn/{}.md", path))));
                Ok(Some(Box::new(ViewResult::new("views/learn/details.rs".to_string(), view_model))))
            })),
        ]
    }
}