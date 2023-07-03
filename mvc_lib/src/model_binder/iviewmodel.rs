use std::collections::HashMap;
use std::rc::Rc;


use crate::core::type_info::TypeInfo;

use super::ihaz_attributes::IHazAttributes;
use super::imodel::IModel;
use super::imodel_attribute::IAttribute;


// this trait represents a view model parsed from body content.
// it can also be used to hold the unparsed body content itself.
pub trait IViewModel: IModel {

}


pub struct MockIViewModelObject {
}

impl MockIViewModelObject {
    pub fn new() -> Self {
        Self {
        }
    }
}

impl IHazAttributes for MockIViewModelObject {
    fn get_attributes(&self) -> Vec<Rc<dyn super::imodel_attribute::IAttribute>> {
        vec![]
    }

    fn get_attribute(&self, _typeinfo: &crate::core::type_info::TypeInfo) -> Option<Rc<dyn IAttribute>> {
        None
    }
}

impl IViewModel for MockIViewModelObject {
}

impl IModel for MockIViewModelObject {
    fn get_properties(&self) -> std::collections::HashMap<String, std::rc::Rc<dyn super::imodel_property::IModelProperty>> {
        HashMap::new()
        // self.mock_model_object.get_properties()
    }

    fn get_property(&self, _name: &str) -> Option<std::rc::Rc<dyn super::imodel_property::IModelProperty>> {
        None
        // self.mock_model_object.get_property(name)
    }

    fn get_methods(&self) -> std::collections::HashMap<String, std::rc::Rc<dyn super::imodel_method::IModelMethod>> {
        HashMap::new()
    }

    fn get_method(&self, _name: &str) -> Option<std::rc::Rc<dyn super::imodel_method::IModelMethod>> {
        None
    }

    fn get_type_info(&self) -> Box<crate::core::type_info::TypeInfo> {
        Box::new(TypeInfo::of::<MockIViewModelObject>())
    }

    fn get_underlying_value(&self) -> &dyn std::any::Any {
        todo!()
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn to_string(&self) -> String {
        nameof::name_of_type!(MockIViewModelObject).to_string()
    }
}

pub struct MockIViewModel {
}

impl MockIViewModel {
    pub fn new() -> Self {
        Self {
        }
    }

    pub fn object(&self) -> MockIViewModelObject {
        MockIViewModelObject::new()
    }
}
