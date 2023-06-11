use std::rc::Rc;

use crate::core::type_info::TypeInfo;

use super::imodel_property::IModelProperty;
use super::imodel_method::IModelMethod;



pub struct ReflectedMethod {
    pub name: String,
    pub visibility: String,
    pub return_type: Box<TypeInfo>,
    pub parameters: Vec<Rc<dyn IModelProperty>>,
}

impl ReflectedMethod {
    pub fn new(
        name: String,
        visibility: String,
        parameters: Vec<Rc<dyn IModelProperty>>,
        return_type: Box<TypeInfo>
    ) -> Self {
        Self {
            name: name,
            visibility: visibility,
            return_type: return_type,
            parameters: parameters,
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
        todo!()
    }

    fn to_string(&self) -> String {
        todo!()
    }

    fn get_type_info(&self) -> Box<TypeInfo> {
        todo!()
    }

    fn get_attributes(&self) -> Vec<std::rc::Rc<dyn super::imodel_attribute::IAttribute>> {
        todo!()
    }

    fn get_attribute(&self, typeinfo: &crate::core::type_info::TypeInfo) -> Option<std::rc::Rc<dyn super::imodel_attribute::IAttribute>> {
        todo!()
    }

    fn get_visibility(&self) -> String {
        self.visibility.clone()
    }
}