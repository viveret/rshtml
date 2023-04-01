use std::any::Any;
use std::cell::RefCell;
use std::error::Error;
use std::rc::Rc;
use std::result::Result;

use crate::contexts::request_context::RequestContext;
use crate::contexts::response_context::ResponseContext;
use crate::contexts::controller_context::ControllerContext;

use crate::controllers::icontroller::IController;
use crate::services::service_collection::IServiceCollection;
use crate::services::service_collection::ServiceCollectionExtensions;


pub trait IRequestHandlerService {
    fn handle_request(self: &Self, request_ctx: Rc<RequestContext>, services: &dyn IServiceCollection) -> Result<Option<Rc<RefCell<ResponseContext>>>, Box<dyn Error>>;
}

pub struct ControllerRequestHandlerService {
    controllers: Vec<Rc<dyn IController>>,
    // areas: Vec<String>,
}

impl ControllerRequestHandlerService {
    pub fn new(controllers: Vec<Rc<dyn IController>>) -> Self {
        // let knownAreas = controllers
        //                     .iter()
        //                     .map(|x| x.get_route_area())
        //                     .filter(|x| {
        //                         match x.get_route_area() {
        //                             Some(route_area) => true,
        //                             None => false,
        //                         }
        //                     })
        //                     .map(|x| x.clone())
        //                     .collect();
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
    fn handle_request(self: &Self, request_ctx: Rc<RequestContext>, services: &dyn IServiceCollection) -> Result<Option<Rc<RefCell<ResponseContext>>>, Box<dyn Error>> {
        let response_ctx = Rc::new(RefCell::new(ResponseContext::new(http::version::Version::HTTP_11, http::StatusCode::NOT_FOUND)));
        let controller_ctx = Rc::new(RefCell::new(ControllerContext::new(None)));
        for controller in self.controllers.iter() {
            controller_ctx.as_ref().borrow_mut().controller = Some(controller.clone());
            let has_result = controller.process_request(controller_ctx.clone(), request_ctx.clone(), services)?;
            match has_result {
                Some(has_some) => {
                    response_ctx.as_ref().borrow_mut().status_code = has_some.get_statuscode();
                    has_some.configure_response(controller_ctx.clone(), response_ctx.clone(), request_ctx.clone(), services);
                    return Ok(Some(response_ctx))
                },
                None => { }
            }
        }
        return Ok(Some(response_ctx))
    }
}