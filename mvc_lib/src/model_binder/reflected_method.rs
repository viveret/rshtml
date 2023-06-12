use std::rc::Rc;

use crate::core::type_info::TypeInfo;

use super::imodel_attribute::IAttribute;
use super::imodel_property::IModelProperty;
use super::imodel_method::IModelMethod;



pub struct ReflectedMethod {
    pub attributes: Vec<Rc<dyn IAttribute>>,
    pub visibility: String,
    pub name: String,
    pub generics: Vec<Rc<dyn IModelProperty>>,
    pub parameters: Vec<Rc<dyn IModelProperty>>,
    pub return_type: Box<TypeInfo>,
}

impl ReflectedMethod {
    pub fn new(
        attributes: Vec<Rc<dyn IAttribute>>,
        visibility: String,
        name: String,
        generics: Vec<Rc<dyn IModelProperty>>,
        parameters: Vec<Rc<dyn IModelProperty>>,
        return_type: Box<TypeInfo>
    ) -> Self {
        Self {
            attributes: attributes,
            visibility: visibility,
            name: name,
            generics: generics,
            parameters: parameters,
            return_type: return_type,
        }
    }
}

impl IModelMethod for ReflectedMethod {
    fn get_name(&self) -> String {
        self.name.clone()
    }

    fn get_return_type(&self) -> Box<TypeInfo> {
        self.return_type.clone()
    }

    fn get_arguments(&self) -> Vec<Rc<dyn IModelProperty>> {
        self.parameters.clone()
    }

    fn get_argument(&self, name: &str) -> Option<Rc<dyn IModelProperty>> {
        self.parameters.iter().find(|x| x.get_name() == name).map(|x| x.clone())
    }

    fn to_string(&self) -> String {
        format!("{}: {}", self.get_name(), self.get_return_type().to_string())
    }

    fn get_type_info(&self) -> Box<TypeInfo> {
        Box::new(TypeInfo::of::<Self>())
    }

    fn get_attributes(&self) -> Vec<Rc<dyn IAttribute>> {
        self.attributes.clone()
    }

    fn get_attribute(&self, typeinfo: &TypeInfo) -> Option<Rc<dyn IAttribute>> {
        self.get_attributes().iter().find(|x| x.get_type_info().as_ref().is_compatible_with(typeinfo)).map(|x| x.clone())
    }

    fn get_visibility(&self) -> String {
        self.visibility.clone()
    }
}