use super::imodel::IModel;


// this trait represents a view model parsed from body content.
// it can also be used to hold the unparsed body content itself.
pub trait IViewModel: IModel {

}


pub struct MockIViewModelObject {
    mock_model_object: super::imodel::MockIModelObject,
}

impl IViewModel for MockIViewModelObject {
}

impl IModel for MockIViewModelObject {
    fn get_properties(&self) -> std::collections::HashMap<String, std::rc::Rc<dyn super::imodel_property::IModelProperty>> {
        self.mock_model_object.get_properties()
    }

    fn get_property(&self, name: &str) -> Option<std::rc::Rc<dyn super::imodel_property::IModelProperty>> {
        self.mock_model_object.get_property(name)
    }

    fn get_methods(&self) -> std::collections::HashMap<String, std::rc::Rc<dyn super::imodel_method::IModelMethod>> {
        todo!()
    }

    fn get_method(&self, name: &str) -> Option<std::rc::Rc<dyn super::imodel_method::IModelMethod>> {
        todo!()
    }

    fn get_type_info(&self) -> Box<crate::core::type_info::TypeInfo> {
        todo!()
    }

    fn get_underlying_value(&self) -> &dyn std::any::Any {
        todo!()
    }

    fn as_any(&self) -> &dyn std::any::Any {
        todo!()
    }

    fn to_string(&self) -> String {
        todo!()
    }
}

pub struct MockIViewModel {
}

impl MockIViewModel {
    pub fn new() -> Self {
        Self {
        }
    }

    pub fn object() -> MockIViewModelObject {
        MockIViewModelObject::new()
    }
}
