use std::rc::Rc;

use crate::controllers::icontroller::IController;
use crate::controllers::icontroller_extensions::IControllerExtensions;
use crate::controller_actions::controller_action::IControllerAction;

pub trait IControllerActionsMap {
    fn to_string(self: &Self) -> String;

    fn get_all_actions(self: &Self) -> Vec<Rc<dyn IControllerAction>>;
    fn get_controllers(self: &Self) -> Vec<Rc<dyn IController>>;
    
    fn get_controller(self: &Self, name: String) -> Rc<dyn IController>;
    fn get_action_at_area_controller_action_path(self: &Self, path: String) -> Rc<dyn IControllerAction>;
}

pub struct ControllerActionsMap {
    pub controllers: Vec<Rc<dyn IController>>,
    pub actions: Vec<Rc<dyn IControllerAction>>,
}

impl ControllerActionsMap {
    pub fn new(
        controllers: Vec<Rc<dyn IController>>,
        actions: Vec<Rc<dyn IControllerAction>>,
    ) -> Self {
        Self {
            controllers: controllers,
            actions: actions,
        }
    }

    pub fn from_controllers(controllers: Vec<Rc<dyn IController>>) -> Self {
        Self::new(
            controllers.clone(),
            controllers
                .iter()
                .map(|x| x.get_actions())
                .flatten()
                .collect(),
        )
    }
}

impl IControllerActionsMap for ControllerActionsMap  {
    fn get_all_actions(self: &Self) -> Vec<Rc<dyn IControllerAction>> {
        self.actions.clone()
    }
    
    fn get_controllers(self: &Self) -> Vec<Rc<dyn IController>> {
        self.controllers.clone()
    }
    
    fn get_controller(self: &Self, name: String) -> Rc<dyn IController> {
        if name.len() == 0 {
            panic!("name.len() == 0");
        }

        self.controllers
            .iter()
            .filter(|x| x.get_type_name() == name || IControllerExtensions::get_name(x.clone().clone()) == name)
            .take(1)
            .cloned()
            .collect::<Vec<Rc<dyn IController>>>()
            .first()
            .expect(format!("Could not find controller {}", name).as_str())
            .clone()
    }
    
    fn get_action_at_area_controller_action_path(self: &Self, path: String) -> Rc<dyn IControllerAction> {
        self.actions
            .iter()
            .filter(|x| x.get_path() == path)
            .take(1)
            .cloned()
            .collect::<Vec<Rc<dyn IControllerAction>>>()
            .first()
            .expect(format!("Could not find controller action at path {}", path).as_str())
            .clone()
    }

    fn to_string(self: &Self) -> String {
        let mut s = String::new();
        s.push_str(&format!("count: {}\n", self.actions.len()));
        for (i, x) in self.actions.iter().enumerate() {
            s.push_str(&format!("\t{}) {}\n", i + 1, x.to_string()));
        }
        s
    }
}
