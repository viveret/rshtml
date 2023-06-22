use std::any::Any;
use std::cell::RefCell;
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


pub struct MockIModel {
    pub attributes: RefCell<Vec<Rc<dyn IAttribute>>>,
    pub properties: RefCell<HashMap<String, Rc<dyn IModelProperty>>>,
    pub methods: RefCell<HashMap<String, Rc<dyn IModelMethod>>>,
    pub type_info: RefCell<Option<Box<TypeInfo>>>,
    pub underlying_value: RefCell<Option<Rc<dyn Any>>>,
}

impl MockIModel {
    pub fn new() -> Self {
        Self {
            attributes: RefCell::new(Vec::new()),
            properties: RefCell::new(HashMap::new()),
            methods: RefCell::new(HashMap::new()),
            type_info: RefCell::new(None),
            underlying_value: RefCell::new(None),
        }
    }

    pub fn object(&self) -> MockIModelObject {
        MockIModelObject::new(
            self.attributes.borrow().clone(),
            self.properties.borrow().clone(),
            self.methods.borrow().clone(),
            self.type_info.borrow().clone(),
            self.underlying_value.borrow().clone(),
        )
    }
}

pub struct MockIModelObject {
    attributes: Vec<Rc<dyn IAttribute>>,
    properties: HashMap<String, Rc<dyn IModelProperty>>,
    methods: HashMap<String, Rc<dyn IModelMethod>>,
    type_info: Option<Box<TypeInfo>>,
    underlying_value: Option<Rc<dyn Any>>,
}

impl MockIModelObject {
    pub fn new(
        attributes: Vec<Rc<dyn IAttribute>>,
        properties: HashMap<String, Rc<dyn IModelProperty>>,
        methods: HashMap<String, Rc<dyn IModelMethod>>,
        type_info: Option<Box<TypeInfo>>,
        underlying_value: Option<Rc<dyn Any>>,
    ) -> Self {
        Self {
            attributes: attributes,
            properties: properties,
            methods: methods,
            type_info: type_info,
            underlying_value: underlying_value,
        }
    }

    pub fn default() -> Self {
        Self {
            attributes: Vec::new(),
            properties: HashMap::new(),
            methods: HashMap::new(),
            type_info: None,
            underlying_value: None,
        }
    }
}

impl IHazAttributes for MockIModelObject {
    fn get_attributes(&self) -> Vec<Rc<dyn IAttribute>> {
        self.attributes.clone()
    }

    fn get_attribute(&self,typeinfo: &TypeInfo) -> Option<Rc<dyn IAttribute>> {
        self.attributes.iter().find(|a| a.get_type_info().unwrap().is_compatible_with(typeinfo)).map(|a| a.clone())
    }
}

impl IModel for MockIModelObject {
    fn get_properties(&self) -> HashMap<String, Rc<dyn IModelProperty>> {
        self.properties.clone()
    }

    fn get_property(&self, name: &str) -> Option<Rc<dyn IModelProperty>> {
        self.properties.get(name).map(|p| p.clone())
    }

    fn get_type_info(&self) -> Box<TypeInfo> {
        if let Some(type_info) = &self.type_info {
            type_info.clone()
        } else {
            Box::new(TypeInfo::of::<MockIModelObject>())
        }
    }

    // similar to as any, but returns the value contained in the model struct instead of the model container itself.
    fn get_underlying_value(&self) -> &dyn Any {
        // if let Some(v) = self.underlying_value {
        //     v.as_ref()
        // } else {
            self
        // }
    }

    // string representation of the model, not used for binding or serialization / deserialization.
    fn to_string(&self) -> String {
        format!("MockIModelObject: {} attributes, {} properties, {} methods", self.attributes.len(), self.properties.len(), self.methods.len())
    }

    fn get_methods(&self) -> HashMap<String, Rc<dyn IModelMethod>> {
        self.methods.clone()
    }

    fn get_method(&self, name: &str) -> Option<Rc<dyn IModelMethod>> {
        self.methods.get(name).map(|m| m.clone())
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}