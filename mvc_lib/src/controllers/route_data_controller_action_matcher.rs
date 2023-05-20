use std::error::Error;
use std::rc::Rc;

use crate::contexts::irequest_context::IRequestContext;
use crate::contexts::response_context::ResponseContext;

use crate::controller_actions::controller_action::IControllerAction;
use crate::controllers::controller_actions_map::IControllerActionsMap;

use crate::services::service_collection::IServiceCollection;


// this struct is used to match a request to a controller action.
// matching is done by comparing the request path to the controller action path.
pub struct RouteDataControllerActionMatcher {
    // the controller actions map.
    actions_map: Rc<dyn IControllerActionsMap>,
}

impl RouteDataControllerActionMatcher {
    // create a new instance of the matcher.
    // actions_map: the controller actions map.
    pub fn new(
        actions_map: Rc<dyn IControllerActionsMap>,
    ) -> Self {
        Self {
            actions_map: actions_map,
        }
    }

    // create a new instance of the matcher as a service for a service collection.
    // request_context: the request context for the controller action.
    // response_ctx: the response context for the controller action.
    // services: the service collection for the controller action.
    // returns: the controller action or an error.
    pub fn get_action_for_request(self: &Self, request_context: Rc<dyn IRequestContext>, response_context: Rc<ResponseContext>, services: &dyn IServiceCollection) -> Result<Option<Rc<dyn IControllerAction>>, Box<dyn Error>> {
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