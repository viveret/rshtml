use std::any::Any;
use std::rc::Rc;

use crate::core::type_info::TypeInfo;

use crate::services::service_collection::IServiceCollection;
use crate::services::service_scope::ServiceScope;

pub struct ServiceDescriptor {
    pub type_info: Box<TypeInfo>,
    pub type_factory: Option<fn(&dyn IServiceCollection) -> Vec<Rc<dyn Any>>>,
    pub type_factory_closure: Option<Box<dyn Fn(&dyn IServiceCollection) -> Vec<Rc<dyn Any>>>>,
    pub scope: ServiceScope,
}

impl ServiceDescriptor {
    pub fn new(type_info: Box<TypeInfo>, type_factory: fn(&dyn IServiceCollection) -> Vec<Rc<dyn Any>>, scope: ServiceScope) -> Self {
        Self { type_info: type_info, type_factory: Some(type_factory), type_factory_closure: None, scope: scope }
    }

    pub fn new_closure<T>(type_info: Box<TypeInfo>, type_factory: T, scope: ServiceScope) -> Self
     where T: Fn(&dyn IServiceCollection) -> Vec<Rc<dyn Any>> + 'static {
        Self { type_info: type_info, type_factory: None, type_factory_closure: Some(Box::new(type_factory)), scope: scope }
    }
}
