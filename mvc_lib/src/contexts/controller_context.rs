use std::any::Any;
use std::cell::RefCell;
use std::collections::HashMap;
use std::error::Error;
use std::rc::Rc;

use crate::action_results::iaction_result::IActionResult;

use crate::contexts::irequest_context::IRequestContext;

use crate::controllers::icontroller::IController;
use crate::controllers::icontroller_extensions::IControllerExtensions;

use crate::core::type_info::TypeInfo;
use crate::model_binder::imodel::IModel;
use crate::model_binder::imodelbinder_service::IModelBinderService;
use crate::model_binder::model_validation_result::ModelValidationResult;
use crate::routing::route_data::RouteData;
use crate::services::service_collection::{IServiceCollection, ServiceCollectionExtensions};

// this trait represents a controller context which is used to invoke a controller action.
// a controller context is created for each controller that is created.
pub trait IControllerContext {
    // get the request context for the controller context.
    fn get_request_context(self: &Self) -> &dyn IRequestContext;
    // get the context data for the controller context.
    fn get_context_data(self: &Self) -> HashMap<String, Rc<Box<dyn Any>>>;
    // get the view data for the controller context.
    fn get_view_data(self: &Self) -> HashMap<String, String>;
    // get the controller for the controller context.
    fn get_controller(self: &Self) -> Rc<dyn IController>;
    // get the route data for the controller context.
    fn get_route_data_result(self: &Self) -> Result<Box<RouteData>, Box<dyn Error>>;

    // get the action result for the controller context.
    fn get_action_result(self: &Self) -> Option<Rc<dyn IActionResult>>;
    // set the action result for the controller context.
    fn set_action_result(self: &Self, action_result: Option<Rc<dyn IActionResult>>);

    // get a string from the context.
    fn get_string(self: &Self, key: String) -> String;
    // get a string from the context.
    fn get_str(self: &Self, key: &str) -> String;
    
    // insert a string into the context data.
    fn insert_string(self: &Self, key: String, value: String) -> String;
    // insert a string into the context data.
    fn insert_str(self: &Self, key: &str, value: String) -> String;

    // bind the action model type to the request context body content.
    // fn bind_model(self: &Self, type_info: TypeInfo, services: &dyn IServiceCollection) -> Result<(), Rc<dyn Error>>;

    // fn set_model(self: &Self, model: Rc<dyn IModel>);
    // fn get_model(self: &Self, expected_type: &TypeInfo) -> Option<Rc<dyn IModel>>;
    //fn bind_model<T: 'static>(self: &Self, services: &dyn IServiceCollection) -> Result<Rc<T>, Rc<dyn Error>>;
}

// this struct implements IControllerContext.
pub struct ControllerContext<'a> {
    // the request context for the controller context.
    pub request_context: &'a dyn IRequestContext,
    // the context data for the controller context.
    pub context_data: RefCell<HashMap<String, Rc<Box<dyn Any>>>>,
    // the view data for the controller context.
    pub view_data: RefCell<HashMap<String, String>>,
    // the controller for the controller context.
    pub controller: Rc<dyn IController>,
    // the action result for the controller context.
    pub action_result: RefCell<Option<Rc<dyn IActionResult>>>,
}

impl <'a> ControllerContext<'a> {
    // create a new controller context.
    // controller: the controller for the controller context.
    // request_context: the request context for the controller context.
    pub fn new(
        controller: Rc<dyn IController>,
        request_context: &'a dyn IRequestContext
    ) -> Self {
        Self {
            request_context: request_context,
            context_data: RefCell::new(HashMap::new()),
            view_data: RefCell::new(HashMap::new()),
            controller: controller.clone(),
            action_result: RefCell::new(None),
        }
    }

    // parse the route data from the controller context.
    // returns: the route data for the controller context or an error if the route data could not be parsed.
    pub fn parse_route_data(
        self: &Self,
    ) -> Result<Box<RouteData>, Box<dyn Error>> {
        let mut route_data = RouteData::new();
        
        let mut area_name = self.get_str("AreaName");
        if area_name.len() == 0 {
            area_name = self.controller.get_route_area().to_string();
        }
        route_data.map.insert("area".to_string(), area_name);
        
        let mut controller_name = self.get_str("ControllerName");
        if controller_name.len() == 0 {
            controller_name = IControllerExtensions::get_name(self.controller.clone());
        }
        route_data.map.insert("controller".to_string(), controller_name);

        let action_name = self.get_str("ActionName");
        route_data.map.insert("action".to_string(), action_name);

        Ok(Box::new(route_data))
    }
}

impl <'a> IControllerContext for ControllerContext<'a> {
    fn get_request_context(self: &Self) -> &dyn IRequestContext {
        self.request_context
    }

    fn get_context_data(self: &Self) -> HashMap<String, Rc<Box<dyn Any>>> {
        self.context_data.borrow().clone()
    }

    fn get_view_data(self: &Self) -> HashMap<String, String> {
        self.view_data.borrow().clone()
    }

    fn get_controller(self: &Self) -> Rc<dyn IController> {
        self.controller.clone()
    }

    fn get_route_data_result(self: &Self) -> Result<Box<RouteData>, Box<dyn Error>> {
        self.parse_route_data()
    }

    fn get_string(self: &Self, key: String) -> String {
        match self.view_data.borrow().get(&key) {
            Some(s) => s.clone(),
            None => {
                self.request_context.get_string(key)
            },
        }
    }

    fn get_str(self: &Self, key: &str) -> String {
        self.get_string(key.to_string())
    }
    
    fn insert_string(self: &Self, key: String, value: String) -> String {
        self.view_data.borrow_mut().insert(key, value.clone());
        value
    }

    fn insert_str(self: &Self, key: &str, value: String) -> String {
        self.insert_string(key.to_string(), value)
    }

    fn get_action_result(self: &Self) -> Option<Rc<dyn IActionResult>> {
        match self.action_result.borrow().clone() {
            Some(action_result) => Some(action_result),
            None => None,
        }
    }

    fn set_action_result(self: &Self, action_result: Option<Rc<dyn IActionResult>>) {
        self.action_result.replace(action_result);
    }

    // fn bind_model(self: &Self, type_info: TypeInfo, services: &dyn IServiceCollection) -> Result<(), Rc<dyn Error>> {
    //     if let Some(body) = self.request_context.get_body_content() {
    //         let model_binder_service = ServiceCollectionExtensions::get_required_single::<dyn IModelBinderService>(services);
    //         match model_binder_service.bind_model(self.request_context, &type_info) {
    //             ModelValidationResult::Ok(model) => {
    //                 self.set_model(model);
    //                 Ok(())
    //             },
    //             ModelValidationResult::OkNone => {
    //                 Ok(())
    //             },
    //             ModelValidationResult::ModelError(property, error) => {
    //                 Err(error)
    //             },
    //             ModelValidationResult::PropertyError(x, t, error) => {
    //                 Err(error)
    //             }
    //         }
    //     } else {
    //         Ok(())
    //     }
    // }

    // fn get_model(self: &Self, expected_type: &TypeInfo) -> Option<Rc<dyn IModel>> {
    //     self.model.borrow().clone()
    // }

    // fn set_model(self: &Self, model: Rc<dyn IModel>) {
    //     self.model.borrow_mut().replace(model);
    // }

    // new_fn: fn() -> Box<dyn Any>
    // fn bind_model<T: 'static>(self: &Self, services: &dyn IServiceCollection) -> Result<Rc<T>, Rc<dyn Error>> {
    //     let binder_resolver = ServiceCollectionExtensions::get_required_single::<dyn IModelBinderResolver>(services);
    //     let binder = binder_resolver.resolve_for_content_type(self.request_context).unwrap();
    //     let model = binder.bind_model(self.request_context);
    //     println!("binder: {:?}", binder.type_info());
    //     match model {
    //         ModelValidationResult::Ok(model) => {
    //             let model = model.downcast::<T>().unwrap();
    //             Ok(model)
    //         },
    //         ModelValidationResult::OkNone => {
    //             panic!("ModelValidationResult::OkNone")
    //         },
    //         ModelValidationResult::ModelError(err, err2) => {
    //             Err(err2)
    //         },
    //         ModelValidationResult::PropertyError(a, err, err2) => {
    //             Err(err2)
    //         },
    //     }
    // }

}