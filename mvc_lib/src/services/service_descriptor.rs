use std::any::Any;
use std::fmt;

use crate::core::type_info::TypeInfo;

use crate::services::service_collection::IServiceCollection;
use crate::services::service_scope::ServiceScope;

// this is a struct that holds the type info and the factory function for a service.
pub struct ServiceDescriptor {
    // this is the type info for the service
    pub type_info: Box<TypeInfo>,
    // this is the factory function for the service
    pub type_factory: Option<fn(&dyn IServiceCollection) -> Vec<Box<dyn Any>>>,
    // this is the factory closure for the service
    pub type_factory_closure: Option<Box<dyn Fn(&dyn IServiceCollection) -> Vec<Box<dyn Any>>>>,
    // this is the scope of the service
    pub scope: ServiceScope,
}

impl ServiceDescriptor {
    // create a new service descriptor with a factory function
    pub fn new(type_info: Box<TypeInfo>, type_factory: fn(&dyn IServiceCollection) -> Vec<Box<dyn Any>>, scope: ServiceScope) -> Self {
        Self { type_info: type_info, type_factory: Some(type_factory), type_factory_closure: None, scope: scope }
    }

    // create a new service descriptor with a closure for the factory function
    pub fn new_closure<T>(type_info: Box<TypeInfo>, type_factory: T, scope: ServiceScope) -> Self
     where T: Fn(&dyn IServiceCollection) -> Vec<Box<dyn Any>> + 'static {
        Self { type_info: type_info, type_factory: None, type_factory_closure: Some(Box::new(type_factory)), scope: scope }
    }

    // pub fn new_singleton<T, TInterface: ?Sized>(type_info: Box<TypeInfo>, item: &'static T) -> Self {
    //     Self { type_info: type_info, type_factory: None, type_factory_closure: Some(Box::new(move |_| -> Vec<Box<dyn Any>> { vec![Box::new(Rc::new(*item))] })), scope: ServiceScope::Singleton }
    // }
}

impl fmt::Display for ServiceDescriptor {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?} - {:?}", self.scope, self.type_info)
    }
}