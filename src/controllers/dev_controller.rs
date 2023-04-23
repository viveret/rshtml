use std::any::Any;
use std::borrow::Cow;
use std::rc::Rc;

use mvc_lib::services::routemap_service::IRouteMapService;
use mvc_lib::services::service_collection::IServiceCollection;
use mvc_lib::services::service_collection::ServiceCollectionExtensions;

use mvc_lib::contexts::controller_context::IControllerContext;

use mvc_lib::action_results::view_result::ViewResult;
use mvc_lib::action_results::http_result::HttpRedirectResult;

use mvc_lib::controllers::icontroller::IController;

use mvc_lib::controller_action_features::controller_action_feature::IControllerActionFeature;
use mvc_lib::controller_actions::controller_action::IControllerAction;
use mvc_lib::controller_actions::closure::ControllerActionClosure;

use mvc_lib::controllers::controller_actions_map::IControllerActionsMap;

use mvc_lib::controller_action_features::local_host_only::LocalHostOnlyControllerActionFeature;
use mvc_lib::controller_action_features::authorize::AuthorizeControllerActionFeature;

use mvc_lib::view::view_renderer::IViewRenderer;

use crate::view_models::dev::{ IndexViewModel, ViewsViewModel, ViewDetailsViewModel, RoutesViewModel, RouteDetailsViewModel, SysInfoViewModel };


pub struct DevController {

}

impl DevController {
    pub fn new() -> Self {
        Self { }
    }

    pub fn new_service(_services: &dyn IServiceCollection) -> Vec<Box<dyn Any>> {
        vec![Box::new(Rc::new(Self::new()) as Rc<dyn IController>)]
    }
}

impl IController for DevController {
    fn get_route_area(self: &Self) -> String {
        String::new()
    }

    fn get_type_name(self: &Self) -> &'static str {
        nameof::name_of_type!(DevController)
    }

    fn get_controller_name(self: &Self) -> Cow<'static, str> {
        Cow::Borrowed(nameof::name_of_type!(DevController))
    }
    
    fn get_actions(self: &Self) -> Vec<Rc<dyn IControllerAction>> {
        vec![
            Rc::new(ControllerActionClosure::new_validated(vec![], None, "/dev".to_string(), "Index".to_string(), self.get_type_name(), self.get_route_area(), |_controller_ctx, _services| {
                let view_model = Box::new(Rc::new(IndexViewModel::new()));
                Ok(Some(Rc::new(ViewResult::new("views/dev/index.rs".to_string(), view_model))))
            })),
            Rc::new(ControllerActionClosure::new_validated(vec![], None, "/dev/views".to_string(), "Views".to_string(), self.get_type_name(), self.get_route_area(), |_controller_ctx, services| {
                let view_renderer = ServiceCollectionExtensions::get_required_single::<dyn IViewRenderer>(services);
                let view_model = Box::new(Rc::new(ViewsViewModel::new(view_renderer.get_all_views(services))));
                Ok(Some(Rc::new(ViewResult::new("views/dev/views.rs".to_string(), view_model))))
            })),
            Rc::new(ControllerActionClosure::new_validated(vec![], None, "/dev/views/..".to_string(), "ViewDetails".to_string(), self.get_type_name(), self.get_route_area(), |controller_ctx, services| {
                let request_context = controller_ctx.get_request_context();

                let path = &request_context.get_path()["/dev/views/".len()..];

                if path.len() == 0 {
                    return Ok(Some(Rc::new(HttpRedirectResult::new("/dev/views".to_string()))))
                }

                println!("Viewing view at path: {:?}", path);
                let view_renderer = ServiceCollectionExtensions::get_required_single::<dyn IViewRenderer>(services);
                let view_model = Box::new(Rc::new(ViewDetailsViewModel::new(view_renderer.get_view(&path.to_string(), services))));
                return Ok(Some(Rc::new(ViewResult::new("views/dev/view_details.rs".to_string(), view_model))));
            })),
            Rc::new(ControllerActionClosure::new_validated(vec![], None, "/dev/routes".to_string(), "Routes".to_string(), self.get_type_name(), self.get_route_area(), |_controller_ctx, services| {
                let routes = ServiceCollectionExtensions::get_required_single::<dyn IRouteMapService>(services);
                let view_model = Box::new(Rc::new(RoutesViewModel::new(routes.as_ref().get_mapper().as_ref().get_all_actions())));
                Ok(Some(Rc::new(ViewResult::new("views/dev/routes.rs".to_string(), view_model))))
            })),
            Rc::new(ControllerActionClosure::new_validated(vec![], None, "/dev/routes/..".to_string(), "RouteDetails".to_string(), self.get_type_name(), self.get_route_area(), |controller_ctx, services| {
                let request_context = controller_ctx.get_request_context();
                
                let path = &request_context.get_path()["/dev/routes/".len()..];

                if path.len() == 0 {
                    return Ok(Some(Rc::new(HttpRedirectResult::new("/dev/routes".to_string()))))
                }

                println!("Viewing route at path: {:?}", path);
                let routes = ServiceCollectionExtensions::get_required_single::<dyn IRouteMapService>(services);
                let route = routes.as_ref().get_mapper().as_ref().get_action_at_area_controller_action_path(path.to_string());
                let controller = routes.as_ref().get_mapper().get_controller(route.get_controller_name().to_string());

                let view_model = Box::new(Rc::new(RouteDetailsViewModel::new(route, controller)));
                return Ok(Some(Rc::new(ViewResult::new("views/dev/route_details.rs".to_string(), view_model))));
            })),
            Rc::new(ControllerActionClosure::new_validated(vec![], None, "/dev/sysinfo".to_string(), "SysInfo".to_string(), self.get_type_name(), self.get_route_area(), |_controller_ctx, _services| {
                let view_model = Box::new(Rc::new(SysInfoViewModel::new()));
                Ok(Some(Rc::new(ViewResult::new("views/dev/sysinfo.rs".to_string(), view_model))))
            })),
        ]
    }

    fn get_features(self: &Self) -> Vec<Rc<dyn IControllerActionFeature>> {
        vec![
            AuthorizeControllerActionFeature::new_service_parse("admin,dev,owner".to_string(), None),
            LocalHostOnlyControllerActionFeature::new_service()
        ]
    }
}