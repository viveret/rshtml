use std::fmt;
use std::rc::Rc;

use crate::controllers::icontroller::IController;
use crate::controllers::icontroller_extensions::IControllerExtensions;
use crate::controller_actions::controller_action::IControllerAction;


// this trait is used to map controllers and actions.
// it is used to get controllers and actions by name.
pub trait IControllerActionsMap {
    // get a string representation of the controller actions map.
    fn to_string(self: &Self) -> String;

    // get all the actions in the controller actions map.
    fn get_all_actions(self: &Self) -> Vec<Rc<dyn IControllerAction>>;

    // get all the controllers in the controller actions map.
    fn get_controllers(self: &Self) -> Vec<Rc<dyn IController>>;
    
    // get the controller with the given name.
    // name: the name of the controller.
    // returns: the controller with the given name.
    fn get_controller(self: &Self, name: String) -> Rc<dyn IController>;

    // get the action with the given path.
    // path: the path of the action.
    // returns: the action with the given path.
    fn get_action_at_area_controller_action_path(self: &Self, path: String) -> Rc<dyn IControllerAction>;

    // get the action with the given name values.
    // action_name: the name of the action.
    // controller_name: the name of the controller.
    // area_name: the name of the area.
    // returns: the action with the given name values.
    fn get_action(self: &Self, action_name: &str, controller_name: &str, area_name: &str) -> Rc<dyn IControllerAction>;
}

// this struct is used to map controllers and actions and implement the IControllerActionsMap trait.
pub struct ControllerActionsMap {
    // the controllers in the map.
    pub controllers: Vec<Rc<dyn IController>>,
    // the actions in the map for all the controllers.
    pub actions: Vec<Rc<dyn IControllerAction>>,
}

impl ControllerActionsMap {
    // create a new instance of the controller actions map.
    // controllers: the controllers in the map.
    // actions: the actions in the map for all the controllers.
    pub fn new(
        controllers: Vec<Rc<dyn IController>>,
        actions: Vec<Rc<dyn IControllerAction>>,
    ) -> Self {
        Self {
            controllers: controllers,
            actions: actions,
        }
    }

    // create a new instance of the controller actions map from a list of controllers.
    // controllers: the controllers in the map.
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
            .filter(|x| x.get_type_name() == name || IControllerExtensions::get_name(x.as_ref()) == name)
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
            .filter(|x| x.get_path().is_equivalent_to(&path))
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

    fn get_action(self: &Self, action_name: &str, controller_name: &str, area_name: &str) -> Rc<dyn IControllerAction> {
        self.actions
            .iter()
            .filter(|x| x.get_name() == action_name && x.get_controller_name() == controller_name && x.get_area_name() == area_name)
            .take(1)
            .cloned()
            .collect::<Vec<Rc<dyn IControllerAction>>>()
            .first()
            .expect(format!("Could not find controller action with name {} in controller {} in area {}", action_name, controller_name, area_name).as_str())
            .clone()
    }
}

impl fmt::Display for ControllerActionsMap {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for action in self.actions.iter() {
            write!(f, "{} -> {}\n", action.get_path(), action.get_route_pattern().to_string())?;
        }
        write!(f, "\n")
    }
}
