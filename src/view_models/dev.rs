use std::rc::Rc;

use mvc_lib::controller_actions::controller_action::IControllerAction;
use mvc_lib::controllers::icontroller::IController;

use mvc_lib::view::iview::IView;

// this is the view model for the index view
pub struct IndexViewModel {
}

impl IndexViewModel {
    // create a new instance of the view model
    pub fn new() -> Self {
        Self { }
    }
}

// this is the view model for the views view
pub struct ViewsViewModel {
    pub views: Vec<Rc<dyn IView>>,
}

impl ViewsViewModel {
    // create a new instance of the view model
    pub fn new(views: Vec<Rc<dyn IView>>) -> Self {
        Self { views: views }
    }
}

// this is the view model for the view details view
pub struct ViewDetailsViewModel {
    pub view: Rc<dyn IView>,
}

impl ViewDetailsViewModel {
    // create a new instance of the view model
    pub fn new(view: Rc<dyn IView>) -> Self {
        Self { view: view }
    }
}

// this is the view model for the routes view
pub struct RoutesViewModel {
    pub routes: Vec<Rc<dyn IControllerAction>>,
}

impl RoutesViewModel {
    // create a new instance of the view model
    pub fn new(routes: Vec<Rc<dyn IControllerAction>>) -> Self {
        Self { routes: routes }
    }
}

// this is the view model for the route details view
pub struct RouteDetailsViewModel {
    pub route: Rc<dyn IControllerAction>,
    pub controller: Rc<dyn IController>,
}

impl RouteDetailsViewModel {
    // create a new instance of the view model
    pub fn new(route: Rc<dyn IControllerAction>, controller: Rc<dyn IController>) -> Self {
        Self { route: route, controller: controller }
    }
}

// this is the view model for the system info view
pub struct SysInfoViewModel {
}

impl SysInfoViewModel {
    // create a new instance of the view model
    pub fn new() -> Self {
        Self {  }
    }
}

pub struct LogViewModel {
    pub supports_read: bool,
    pub logs: Vec<String>,
}

impl LogViewModel {
    pub fn new(supports_read: bool, logs: Vec<String>) -> Self {
        Self { supports_read: supports_read, logs: logs }
    }
}

pub struct LogAddInputModel {
    pub message: String,
    pub level: String,
}

pub struct LogAddViewModel {
    pub supports_read: bool,
}

impl LogAddViewModel {
    pub fn new(supports_read: bool) -> Self {
        Self { supports_read: supports_read }
    }
}

pub struct LogClearViewModel {
    pub supports_clear: bool,
}

impl LogClearViewModel {
    pub fn new(supports_clear: bool) -> Self {
        Self { supports_clear: supports_clear }
    }
}

pub struct PerfLogViewModel {

}

impl PerfLogViewModel {
    pub fn new() -> Self {
        Self { }
    }
}
