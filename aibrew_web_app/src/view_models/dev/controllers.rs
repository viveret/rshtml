use std::borrow::Cow;
use std::collections::HashMap;
use std::rc::Rc;

use core_macro_lib::{IModel, IViewModel, IHazAttributes, reflect_properties, reflect_attributes, reflect_methods};

use mvc_lib::core::type_info::TypeInfo;
use mvc_lib::controllers::icontroller::IController;
use mvc_lib::model_binder::imodel::IModel;
use mvc_lib::model_binder::iviewmodel::IViewModel;
use mvc_lib::model_binder::imodel_attribute::IAttribute;
use mvc_lib::model_binder::ihaz_attributes::IHazAttributes;
use mvc_lib::model_binder::imodel_method::IModelMethod;
use mvc_lib::model_binder::imodel_property::IModelProperty;
use mvc_lib::model_binder::reflected_attribute::ReflectedAttribute;
use mvc_lib::model_binder::reflected_property::ReflectedProperty;
use mvc_lib::model_binder::reflected_method::ReflectedMethod;


#[derive(Clone, Debug, IHazAttributes, IModel, IViewModel)]
#[reflect_attributes]
#[reflect_properties]
pub struct ControllerDetailsViewModel {
    pub name: Cow<'static, str>,
    pub actions: Vec<(Cow<'static, str>, Cow<'static, str>)>,
    pub features: Vec<String>,
    pub attributes: Vec<String>,
    pub properties: Vec<(String,String)>,
    pub methods: Vec<(String,String,String,String)>,
}

#[reflect_methods]
impl ControllerDetailsViewModel {
    pub fn new(
        name: Cow<'static, str>,
        actions: Vec<(Cow<'static, str>, Cow<'static, str>)>,
        features: Vec<String>,
        attributes: Vec<String>,
        properties: Vec<(String,String)>,
        methods: Vec<(String,String,String,String)>,
    ) -> Self {
        Self {
            name: name,
            actions: actions,
            features: features,
            attributes: attributes,
            properties: properties,
            methods: methods,
        }
    }
}

#[derive(Clone, Debug, IHazAttributes, IModel, IViewModel)]
#[reflect_attributes]
#[reflect_properties]
pub struct ControllersViewModel {
    pub controllers: Vec<ControllerDetailsViewModel>,
}

#[reflect_methods]
impl ControllersViewModel {
    pub fn new(controllers: Vec<ControllerDetailsViewModel>) -> Self {
        Self {
            controllers: controllers,
        }
    }

    pub fn from_controllers(get_controllers: Vec<std::rc::Rc<dyn IController>>) -> Self {
        let mut controllers = vec![];
        for controller in get_controllers {
            let mut actions = vec![];
            for action in controller.get_actions() {
                actions.push((action.get_name(), action.get_path().to_cow_str()));
            }
            let mut attrs = vec![];
            for attr in controller.get_attributes() {
                attrs.push(attr.to_string());
            }
            let mut properties = vec![];
            for prop in controller.get_properties() {
                properties.push((prop.0.clone(), prop.1.get_return_type().map(|x| x.to_string()).unwrap_or("void".to_string())));
            }
            let mut methods = vec![];
            for method in controller.get_methods() {
                methods.push((
                    method.1.get_attributes().iter().map(|x| x.to_string()).collect::<Vec<String>>().join(" "),
                    method.0.clone(), 
                    method.1.get_arguments().iter().map(|p| p.get_return_type().map(|x| x.to_string()).unwrap_or("void".to_string())).collect::<Vec<String>>().join(", "),
                    method.1.get_return_type().map(|x| x.to_string()).unwrap_or("void".to_string())
                ));
            }
            controllers.push(ControllerDetailsViewModel::new(controller.get_type_name().into(), actions, vec![], attrs, properties, methods));
        }
        Self::new(controllers)
    }
}