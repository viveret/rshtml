use std::error::Error;
use std::rc::Rc;

use crate::contexts::request_context::RequestContext;
use crate::contexts::response_context::ResponseContext;

use crate::controllers::controller_action::IControllerAction;
use crate::controllers::controller_actions_map::IControllerActionsMap;

use crate::services::service_collection::IServiceCollection;


pub struct RouteDataControllerActionMatcher {
    actions_map: Rc<dyn IControllerActionsMap>,
}

impl RouteDataControllerActionMatcher {
    pub fn new(
        actions_map: Rc<dyn IControllerActionsMap>,
    ) -> Self {
        Self {
            actions_map: actions_map,
        }
    }

    pub fn get_action_for_request(self: &Self, request_context: Rc<RequestContext>, response_context: Rc<ResponseContext>, services: &dyn IServiceCollection) -> Result<Option<Rc<dyn IControllerAction>>, Box<dyn Error>> {
        let all_actions = self.actions_map.get_all_actions();
        let actions: Vec<Rc<dyn IControllerAction>> = all_actions
            .iter()
            .filter(|x|
                match x.is_route_match(request_context.clone()) {
                    Ok(is_match) => is_match,
                    Err(e) => false,
                }
            )
            .cloned()
            .collect();

        match actions.len() {
            0 => {
                Ok(None)
                // panic!("No routes found for {}", request_context.as_ref().path);
            },
            1 => {
                Ok(Some(actions.first().unwrap().clone()))
            },
            _ => {
                panic!("Ambiguous routes: {:?}", actions.iter().map(|x| x.get_name()).collect::<Vec<String>>());
            }
        }
    }
}