use std::any::Any;
use std::borrow::Cow;
use std::rc::Rc;

use glob::glob;

use mvc_lib::action_results::http_result::HttpRedirectResult;

use mvc_lib::controllers::icontroller_extensions::IControllerExtensions;
use mvc_lib::services::service_collection::IServiceCollection;

use mvc_lib::action_results::view_result::ViewResult;

use mvc_lib::controllers::icontroller::IController;

use mvc_lib::controller_action_features::controller_action_feature::IControllerActionFeature;
use mvc_lib::controller_actions::controller_action::IControllerAction;
use mvc_lib::controller_actions::closure::ControllerActionClosure;

use crate::view_models::learn::IndexViewModel;
use crate::view_models::learn::DetailsViewModel;


// this is the controller for the learn section of the site.
pub struct LearnController {

}

impl LearnController {
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
}

impl IController for LearnController {
    fn get_route_area(self: &Self) -> String {
        String::new()
    }

    fn get_type_name(self: &Self) -> &'static str {
        nameof::name_of_type!(LearnController)
    }

    fn get_actions(self: &Self) -> Vec<Rc<dyn IControllerAction>> {
        let controller_name = IControllerExtensions::get_name_ref(self);
        vec![
            Rc::new(ControllerActionClosure::new_default_area_not_validated(vec![], None, "/learn".to_string(), "index".to_string(), Cow::Owned(controller_name.clone()), &|_controller_ctx, _services| {
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
                Ok(Some(Rc::new(ViewResult::new("views/learn/index.rs".to_string(), view_model))))
            })),
            Rc::new(ControllerActionClosure::new_default_area_validated(vec![], None, "/learn/..".to_string(), "details".to_string(), Cow::Owned(controller_name), Rc::new(|model, controller_ctx, _services| {
                let request_context = controller_ctx.get_request_context();
                let path = &request_context.get_path()["/learn/".len()..];

                if path.len() == 0 {
                    return Ok(Some(Rc::new(HttpRedirectResult::new("/learn".to_string()))))
                }

                let view_model = Box::new(Rc::new(DetailsViewModel::new(format!("docs/learn/{}.md", path))));
                Ok(Some(Rc::new(ViewResult::new("views/learn/details.rs".to_string(), view_model))))
            }))),
        ]
    }

    fn get_features(self: &Self) -> Vec<Rc<dyn IControllerActionFeature>> {
        vec![]
    }

    fn as_any(self: &Self) -> &dyn Any {
        self
    }
}