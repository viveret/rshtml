use std::any::Any;
use std::collections::HashMap;
use std::rc::Rc;

use crate::core::type_info::TypeInfo;

use super::imodel_attribute::IAttribute;
use super::ihaz_attributes::IHazAttributes;
use super::imodel_method::IModelMethod;
use super::imodel_property::IModelProperty;


pub trait IModel: IHazAttributes {
    // properties must be public and annotated with #[imodel_property] to be reflected.
    fn get_properties(&self) -> HashMap<String, Rc<dyn IModelProperty>>;
    fn get_property(&self, name: &str) -> Option<Rc<dyn IModelProperty>>;

    // methods must be public and annotated with #[imodel_method] to be reflected.
    fn get_methods(&self) -> HashMap<String, Rc<dyn IModelMethod>>;
    fn get_method(&self, name: &str) -> Option<Rc<dyn IModelMethod>>;

    fn get_type_info(&self) -> Box<TypeInfo>;

    // similar to as any, but returns the value contained in the model struct instead of the model container itself if they are different.
    fn get_underlying_value(&self) -> &dyn Any;

    // returns a reference to the model class instance as a dyn Any.
    fn as_any(&self) -> &dyn Any;

    // string representation of the model, not used for binding or serialization / deserialization.
    fn to_string(&self) -> String;
}

#[derive(Clone)]
pub struct AnyIModel {
    pub model: Rc<dyn IModel>,
}

impl AnyIModel {
    pub fn new(model: Rc<dyn IModel>) -> Self {
        Self {
            model: model,
        }
    }
}

impl IHazAttributes for AnyIModel {
    fn get_attributes(&self) -> Vec<Rc<dyn IAttribute>> {
        self.model.get_attributes()
    }
    fn get_attribute(&self, typeinfo: &TypeInfo) -> Option<Rc<dyn IAttribute>> {
        self.model.get_attribute(typeinfo)
    }
}

impl IModel for AnyIModel {
    fn get_properties(&self) -> HashMap<String, Rc<dyn IModelProperty>> {
        self.model.get_properties()
    }
    fn get_property(&self, name: &str) -> Option<Rc<dyn IModelProperty>> {
        self.model.get_property(name)
    }

    fn get_type_info(&self) -> Box<TypeInfo> {
        self.model.get_type_info()
    }

    // similar to as any, but returns the value contained in the model struct instead of the model container itself.
    fn get_underlying_value(&self) -> &dyn Any {
        self.model.get_underlying_value()
    }

    // string representation of the model, not used for binding or serialization / deserialization.
    fn to_string(&self) -> String {
        self.model.to_string()
    }

    fn get_methods(&self) -> HashMap<String, Rc<dyn IModelMethod>> {
        self.model.get_methods()
    }

    fn get_method(&self, name: &str) -> Option<Rc<dyn IModelMethod>> {
        self.model.get_method(name)
    }

    fn as_any(&self) -> &dyn Any {
        self.model.as_any()
    }
}