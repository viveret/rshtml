use std::any::Any;
use std::cell::RefCell;
use std::error::Error;
use std::rc::Rc;
use std::result::Result;

use crate::contexts::request_context::RequestContext;
use crate::contexts::response_context::ResponseContext;

use crate::controllers::route_data_controller_action_matcher::RouteDataControllerActionMatcher;

use crate::services::service_collection::{ IServiceCollection, ServiceCollectionExtensions };
use crate::services::request_middleware_service::{ IRequestMiddlewareService, MiddlewareResult };
use crate::services::routemap_service::IRouteMapService;

pub struct RoutingService {
    routemap: Rc<dyn IRouteMapService>,
    next: RefCell<Option<Rc<dyn IRequestMiddlewareService>>>
}

impl RoutingService {
    pub fn new(routemap: Rc<dyn IRouteMapService>) -> Self {
        Self { next: RefCell::new(None), routemap: routemap }
    }

    pub fn new_service(services: &dyn IServiceCollection) -> Vec<Box<dyn Any>> {
        vec![Box::new(Rc::new(Self::new(
            ServiceCollectionExtensions::get_required_single::<dyn IRouteMapService>(services)
        )) as Rc<dyn IRequestMiddlewareService>)]
    }
}

impl IRequestMiddlewareService for RoutingService {
    fn set_next(self: &Self, next: Option<Rc<dyn IRequestMiddlewareService>>) {
        self.next.replace(next);
    }

    fn handle_request(self: &Self, request_context: Rc<RequestContext>, response_context: Rc<ResponseContext>, services: &dyn IServiceCollection) -> Result<MiddlewareResult, Box<dyn Error>> {
        let route_matcher = RouteDataControllerActionMatcher::new(self.routemap.get_mapper().clone());
        let action_option = route_matcher.get_action_for_request(request_context.clone(), response_context.clone(), services)?;

        if let Some(action) = action_option {
            let mut controller_name = action.get_controller_name().to_string();
            if controller_name.ends_with("Controller") {
                controller_name = controller_name[..controller_name.len() - "Controller".len()].to_string();
            }
            
            request_context.route_data.borrow_mut().map.insert("ActionName".to_string(), action.get_name());
            request_context.route_data.borrow_mut().map.insert("ControllerName".to_string(), controller_name);
            request_context.route_data.borrow_mut().map.insert("AreaName".to_string(), action.get_area_name());
            request_context.controller_action.replace(Some(action.clone()));
        } else {
            // 404 not found
            // panic!("404 not found");
        }
                
        if let Some(next) = self.next.borrow().as_ref() {
            next.handle_request(request_context.clone(), response_context.clone(), services)
        } else {
            Ok(MiddlewareResult::OkContinue)
        }
    }
}