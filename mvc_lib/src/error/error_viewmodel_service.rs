use std::any::Any;
use std::collections::HashMap;
use std::error::Error;
use std::rc::Rc;

use core_macro_lib::{IModel, IViewModel, reflect_attributes, reflect_properties};
use core_macro_lib::IHazAttributes;
use core_macro_lib::reflect_methods;
use crate::core::type_info::TypeInfo;

use crate::model_binder::imodel::IModel;
use crate::model_binder::iviewmodel::IViewModel;
use crate::model_binder::imodel_attribute::IAttribute;
use crate::model_binder::imodel_method::IModelMethod;
use crate::model_binder::imodel_property::IModelProperty;
use crate::model_binder::ihaz_attributes::IHazAttributes;
use crate::model_binder::reflected_attribute::ReflectedAttribute;
use crate::model_binder::reflected_method::ReflectedMethod;
use crate::model_binder::reflected_property::ReflectedProperty;
use crate::services::service_collection::{IServiceCollection, ServiceCollection};
use crate::services::service_descriptor::ServiceDescriptor;
use crate::services::service_scope::ServiceScope;


pub trait IErrorViewModelService {
    fn create_error_viewmodel(self: &Self, error: Rc<dyn Error>) -> Rc<dyn IViewModel>;
}

pub struct ErrorViewModelService {

}

impl ErrorViewModelService {
    pub fn new() -> Self {
        Self {

        }
    }

    pub fn new_service(_services: &dyn IServiceCollection) -> Vec<Box<dyn Any>> {
        vec![Box::new(Rc::new(Self::new()) as Rc<dyn IErrorViewModelService>)]
    }

    pub fn add_to_services(services: &mut ServiceCollection) {
        services.add(ServiceDescriptor::new_from::<dyn IErrorViewModelService, Self>(Self::new_service, ServiceScope::Singleton));
    }
}

impl IErrorViewModelService for ErrorViewModelService {
    fn create_error_viewmodel(self: &Self, error: Rc<dyn Error>) -> Rc<dyn IViewModel> {
        Rc::new(BasicErrorViewModel::new(error)) as Rc<dyn IViewModel>
    }
}


#[derive(Clone, Debug, IHazAttributes, IModel, IViewModel)]
#[reflect_attributes]
#[reflect_properties]
pub struct BasicErrorViewModel {
    pub error: Rc<dyn Error>
}

#[reflect_methods]
impl BasicErrorViewModel {
    pub fn new(error: Rc<dyn Error>) -> Self {
        Self {
            error: error
        }
    }
}
