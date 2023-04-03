use std::any::Any;
use std::cell::RefCell;
use std::error::Error;
use std::rc::Rc;
use std::result::Result;

use crate::contexts::request_context::RequestContext;
use crate::contexts::response_context::ResponseContext;

use crate::controllers::icontroller::IController;
use crate::controllers::icontroller_extensions::IControllerExtensions;

use crate::services::service_collection::IServiceCollection;
use crate::services::service_collection::ServiceCollectionExtensions;


pub trait IRequestHandlerService {
    fn handle_request(self: &Self, request_ctx: Rc<RequestContext>, services: &dyn IServiceCollection) -> Result<Option<Rc<RefCell<ResponseContext>>>, Box<dyn Error>>;
}

pub struct ControllerRequestHandlerService {
    controllers: Vec<Rc<dyn IController>>,
}

impl ControllerRequestHandlerService {
    pub fn new(controllers: Vec<Rc<dyn IController>>) -> Self {
        Self { controllers: controllers }
    }

    pub fn new_service(services: &dyn IServiceCollection) -> Vec<Box<dyn Any>> {
        vec![Box::new(Rc::new(Self::new(
            ServiceCollectionExtensions::get_required_multiple::<dyn IController>(services)
        )) as Rc<dyn IRequestHandlerService>)]
    }

    pub fn get_controllers_in_area(self: &Self, request: Rc<RequestContext>) -> Vec<Rc<dyn IController>> {
        self.controllers
            .iter()
            .filter(|x| {
                match x.get_route_area() {
                    Some(route_area) => request.path.starts_with(&route_area),
                    None => false,
                }
            })
            .map(|x| x.clone())
            .collect()
    }

    pub fn get_controllers(self: &Self, _request: Rc<RequestContext>) -> Vec<Rc<dyn IController>> {
        self.controllers.iter().map(|x| x.clone()).collect()
    }
}

impl IRequestHandlerService for ControllerRequestHandlerService {
    fn handle_request(self: &Self, request_context: Rc<RequestContext>, services: &dyn IServiceCollection) -> Result<Option<Rc<RefCell<ResponseContext>>>, Box<dyn Error>> {
        let response_context = Rc::new(RefCell::new(ResponseContext::new(http::version::Version::HTTP_11, http::StatusCode::NOT_FOUND)));
        for controller in self.controllers.iter() {
            let controller_ctx = IControllerExtensions::create_context(controller.clone(), request_context.clone());
            let has_result = controller.process_request(controller_ctx.clone(), services)?;
            match has_result {
                Some(has_some) => {
                    response_context.as_ref().borrow_mut().status_code = has_some.get_statuscode();
                    has_some.configure_response(controller_ctx.clone(), response_context.clone(), request_context.clone(), services);
                    return Ok(Some(response_context))
                },
                None => { }
            }
        }
        return Ok(Some(response_context))
    }
}