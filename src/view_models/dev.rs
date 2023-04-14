use std::rc::Rc;

use mvc_lib::controller_actions::controller_action::IControllerAction;
use mvc_lib::controllers::icontroller::IController;

use mvc_lib::view::iview::IView;

pub struct IndexViewModel {
}

impl IndexViewModel {
    pub fn new() -> Self {
        Self { }
    }
}


pub struct ViewsViewModel {
    pub views: Vec<Rc<dyn IView>>,
}

impl ViewsViewModel {
    pub fn new(views: Vec<Rc<dyn IView>>) -> Self {
        Self { views: views }
    }
}

pub struct ViewDetailsViewModel {
    pub view: Rc<dyn IView>,
}

impl ViewDetailsViewModel {
    pub fn new(view: Rc<dyn IView>) -> Self {
        Self { view: view }
    }
}


pub struct RoutesViewModel {
    pub routes: Vec<Rc<dyn IControllerAction>>,
}

impl RoutesViewModel {
    pub fn new(routes: Vec<Rc<dyn IControllerAction>>) -> Self {
        Self { routes: routes }
    }
}

pub struct RouteDetailsViewModel {
    pub route: Rc<dyn IControllerAction>,
    pub controller: Rc<dyn IController>,
}

impl RouteDetailsViewModel {
    pub fn new(route: Rc<dyn IControllerAction>, controller: Rc<dyn IController>) -> Self {
        Self { route: route, controller: controller }
    }
}


pub struct SysInfoViewModel {
}

impl SysInfoViewModel {
    pub fn new() -> Self {
        Self {  }
    }
}