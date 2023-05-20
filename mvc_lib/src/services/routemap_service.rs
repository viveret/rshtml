use std::any::Any;
use std::rc::Rc;

use crate::contexts::irequest_context::IRequestContext;

use crate::controllers::icontroller::IController;
use crate::controllers::controller_actions_map::ControllerActionsMap;

use crate::services::service_collection::{ IServiceCollection, ServiceCollectionExtensions };

// this is the service that handles route mapping.
pub trait IRouteMapService {
    // gets the mapper.
    fn get_mapper(self: &Self) -> Rc<ControllerActionsMap>;
}

// implementation of the route map service.
pub struct RouteMapService {
    // known controllers used to build the mapper from actions.
    controllers: Vec<Rc<dyn IController>>,
    // the mapper.
    mapper: Rc<ControllerActionsMap>,
}

impl RouteMapService {
    // creates a new instance of the route map service.
    // controllers: the controllers to use to build the mapper.
    pub fn new(controllers: Vec<Rc<dyn IController>>) -> Self {
        Self {
            controllers: controllers.clone(),
            mapper: Rc::new(ControllerActionsMap::from_controllers(controllers)),
        }
    }

    // creates the route map service as a service.
    // services: the service collection.
    // returns a vector containing the new instance of the service.
    pub fn new_service(services: &dyn IServiceCollection) -> Vec<Box<dyn Any>> {
        vec![Box::new(Rc::new(Self::new(
            ServiceCollectionExtensions::get_required_multiple::<dyn IController>(services)
        )) as Rc<dyn IRouteMapService>)]
    }

    // gets the controllers in the area of the request.
    // request: the request context.
    // returns a vector containing the controllers in the area of the request.
    pub fn get_controllers_in_area(self: &Self, request: Rc<dyn IRequestContext>) -> Vec<Rc<dyn IController>> {
        self.controllers
            .iter()
            .filter(|x| request.get_path().starts_with(&x.get_route_area()))
            .map(|x| x.clone())
            .collect()
    }

    // gets the controllers.
    // request: the request context.
    // returns a vector containing the controllers.
    pub fn get_controllers(self: &Self, _request: Rc<dyn IRequestContext>) -> Vec<Rc<dyn IController>> {
        self.controllers.iter().map(|x| x.clone()).collect()
    }
}

impl IRouteMapService for RouteMapService {
    fn get_mapper(self: &Self) -> Rc<ControllerActionsMap> {
        self.mapper.clone()
    }
}