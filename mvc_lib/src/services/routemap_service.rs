use std::any::Any;
use std::rc::Rc;

use crate::contexts::request_context::RequestContext;

use crate::controllers::icontroller::IController;
use crate::controllers::controller_actions_map::ControllerActionsMap;

use crate::services::service_collection::{ IServiceCollection, ServiceCollectionExtensions };

pub trait IRouteMapService {
    fn get_mapper(self: &Self) -> Rc<ControllerActionsMap>;
}

pub struct RouteMapService {
    controllers: Vec<Rc<dyn IController>>,
    mapper: Rc<ControllerActionsMap>,
}

impl RouteMapService {
    pub fn new(controllers: Vec<Rc<dyn IController>>) -> Self {
        Self {
            controllers: controllers.clone(),
            mapper: Rc::new(ControllerActionsMap::from_controllers(controllers)),
        }
    }

    pub fn new_service(services: &dyn IServiceCollection) -> Vec<Box<dyn Any>> {
        vec![Box::new(Rc::new(Self::new(
            ServiceCollectionExtensions::get_required_multiple::<dyn IController>(services)
        )) as Rc<dyn IRouteMapService>)]
    }

    pub fn get_controllers_in_area(self: &Self, request: Rc<RequestContext>) -> Vec<Rc<dyn IController>> {
        self.controllers
            .iter()
            .filter(|x| request.path.starts_with(&x.get_route_area()))
            .map(|x| x.clone())
            .collect()
    }

    pub fn get_controllers(self: &Self, _request: Rc<RequestContext>) -> Vec<Rc<dyn IController>> {
        self.controllers.iter().map(|x| x.clone()).collect()
    }
}

impl IRouteMapService for RouteMapService {
    fn get_mapper(self: &Self) -> Rc<ControllerActionsMap> {
        self.mapper.clone()
    }
}