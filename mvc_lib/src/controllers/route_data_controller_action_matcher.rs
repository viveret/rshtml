use std::cell::RefCell;
use std::error::Error;
use std::rc::Rc;

use crate::action_results::iaction_result::IActionResult;

use crate::contexts::controller_context::IControllerContext;
use crate::contexts::controller_context::ControllerContext;
use crate::contexts::request_context::RequestContext;

use crate::controllers::controller_actions_map::IControllerActionsMap;

use crate::services::service_collection::IServiceCollection;


pub struct RouteDataControllerActionMatcher {
    actions_map: Rc<dyn IControllerActionsMap>,
    controller_context: Rc<RefCell<ControllerContext>>
}

impl RouteDataControllerActionMatcher {
    pub fn new(
        actions_map: Rc<dyn IControllerActionsMap>,
        controller_context: Rc<RefCell<ControllerContext>>
    ) -> Self {
        Self {
            actions_map: actions_map,
            controller_context: controller_context,
        }
    }

    pub fn process_request(self: &Self, services: &dyn IServiceCollection) -> Result<Option<Box<dyn IActionResult>>, Box<dyn Error>> {
        // Some(self.controller_context.borrow().route_data_result.as_ref().unwrap().clone())
        let actions = self.actions_map.get_actions();
        let route_data = self.controller_context.borrow().get_route_data_result();

        for action in actions {
            // println!("Testing {}", action.get_name());
            
            if !action.is_route_match(self.controller_context.clone())? {
                // println!("Route does not match");
                continue;
            }

            let result = action.invoke(self.controller_context.clone(), services)?;
            match result {
                Some(action_result) => {
                    // println!("Returning result");
                    return Ok(Some(action_result));
                },
                None => {
                    // println!("Trying next path if available");
                },
            }
        }

        // match path {
        //     "/dev" => {
        //         controller_ctx.as_ref().borrow_mut().get_view_data().as_ref().borrow_mut().insert("Layout".to_string(), Rc::new(Box::new("views/shared/_Layout.rs")));
        //         Ok(Some(Box::new(ViewResult::new("views/dev/index.rs".to_string(), Box::new("")))))
        //     },
        //     "/dev/views" | "/dev/views/" => {
        //         let view_renderer = ServiceCollectionExtensions::get_required_single::<dyn IViewRenderer>(services);
        //         let viewModel = Box::new(Rc::new(ViewsViewModel::new(view_renderer.get_all_views(services))));
        //         controller_ctx.as_ref().borrow_mut().get_view_data().as_ref().borrow_mut().insert("Layout".to_string(), Rc::new(Box::new("views/shared/_Layout.rs")));
        //         Ok(Some(Box::new(ViewResult::new("views/dev/views.rs".to_string(), viewModel))))
        //     },
        //     _ if path.starts_with("/dev/views/") && path.len() > "/dev/views/".len() => {
        //         let path = path["/dev/views/".len()..].to_string();
        //         println!("Viewing view at path: {}", path);
        //         let view_renderer = ServiceCollectionExtensions::get_required_single::<dyn IViewRenderer>(services);
        //         let viewModel = Box::new(Rc::new(ViewDetailsViewModel::new(view_renderer.get_view(&path, services))));
        //         controller_ctx.as_ref().borrow_mut().get_view_data().as_ref().borrow_mut().insert("Layout".to_string(), Rc::new(Box::new("views/shared/_Layout.rs")));
        //         Ok(Some(Box::new(ViewResult::new("views/dev/view_details.rs".to_string(), viewModel))))
        //     },
        //     _ => Ok(None),
        // }
        Ok(None)
    }
}