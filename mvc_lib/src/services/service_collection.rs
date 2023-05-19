use std::collections::HashMap;
use std::any::{Any, TypeId};
use std::rc::Rc;

use crate::core::type_info::TypeInfo;

use crate::services::service_scope::ServiceScope;
use crate::services::service_descriptor::ServiceDescriptor;
use crate::services::service_instance::ServiceInstance;


// this interface is used to store services and dependencies in a collection.
// services are described via their type and scope, and when requested, are instantiated.
// dependencies are created when a service is instantiated, and are stored in the collection.
pub trait IServiceCollection: Send + Sync {
    fn try_get(&self, type_info: Box<TypeInfo>) -> Result<Vec<Box<dyn Any>>, &str>;
    fn get_required(&self, type_info: Box<TypeInfo>) -> Vec<Box<dyn Any>>;

    fn get_current_scope(&self) -> ServiceScope;
    fn get_parent(&self) -> Option<Rc<dyn IServiceCollection>>;
    fn get_root(&self) -> Option<Rc<dyn IServiceCollection>>;
    
    fn get_items(&self) -> &Vec<Rc<ServiceDescriptor>>;
    fn get_singletons(&self) -> &Vec<Rc<ServiceInstance>>;
    fn get_request_instances(&self) -> &Vec<Rc<ServiceInstance>>;

    fn find_descriptor(self: &Self, type_info: Box<TypeInfo>) -> Vec<Rc<ServiceDescriptor>>;
    fn find_descriptor_by_id(self: &Self, type_id: TypeId) -> Vec<Rc<ServiceDescriptor>>;
    fn try_find_descriptor_by_id(self: &Self, type_id: TypeId) -> Option<&Vec<Rc<ServiceDescriptor>>>;
}


// generic service collection implementation. 
#[derive(Clone)]
pub struct ServiceCollection {
    current_scope: ServiceScope,
    parent: Option<Rc<dyn IServiceCollection>>,
    root: Option<Rc<dyn IServiceCollection>>,
    items: Vec<Rc<ServiceDescriptor>>,
    type_id_to_descriptor: HashMap<TypeId, Vec<Rc<ServiceDescriptor>>>,
    type_id_to_type_info: HashMap<TypeId, Rc<Box<TypeInfo>>>,
    singletons: Vec<Rc<ServiceInstance>>,
    request_instances: Vec<Rc<ServiceInstance>>,
}
unsafe impl Send for ServiceCollection {}
unsafe impl Sync for ServiceCollection {}

impl ServiceCollection {
    pub fn new_root() -> Self {
        Self { 
            current_scope: ServiceScope::Singleton,
            items: Vec::new(),
            type_id_to_descriptor: HashMap::new(),
            type_id_to_type_info: HashMap::new(),
            request_instances: Vec::new(),
            singletons: Vec::new(),
            parent: None,
            root: None,
        }
    }

    pub fn new(scope: ServiceScope, parent: Rc<ServiceCollection>, root: Rc<ServiceCollection>) -> Self {
        Self {
            current_scope: scope,
            parent: Some(parent),
            root: Some(root),
            items: Vec::new(),
            type_id_to_descriptor: HashMap::new(),
            type_id_to_type_info: HashMap::new(),
            request_instances: Vec::new(),
            singletons: Vec::new(),
        }
    }

    pub fn add(self: &mut Self, item: ServiceDescriptor) -> &mut Self {
        let boxed_item = Rc::new(item);
        self.items.push(boxed_item.clone());
        match self.type_id_to_descriptor.get_mut(&boxed_item.type_info.type_id) {
            Some(add_to) => {
                add_to.push(boxed_item);
            },
            None => {
                self.type_id_to_descriptor.insert(boxed_item.type_info.type_id, vec![boxed_item.clone()]);
                self.type_id_to_type_info.insert(boxed_item.type_info.type_id, Rc::new(boxed_item.type_info.clone()));
            }
        }
        self
    }

    // pub fn add_instance<T, TInterface: ?Sized>(self: &mut Self, type_info: Box<TypeInfo>, item: &'static T) {
    //     self.add(ServiceDescriptor::new_singleton::<T, TInterface>(type_info, item));
    // }

    #[allow(dead_code)]
    fn get_or_instantiate(self: &Self, descriptor: &ServiceDescriptor) -> Vec<Box<dyn Any>> {
        self.instantiate(descriptor)
    }

    fn get_or_instantiate_for_request(self: &Self, descriptor: &ServiceDescriptor) -> Vec<Box<dyn Any>> {
        self.instantiate(descriptor)
    }

    fn get_or_instantiate_singleton(self: &Self, descriptor: &ServiceDescriptor) -> Vec<Box<dyn Any>> {
        self.instantiate(descriptor)
    }

    fn instantiate(self: &Self, descriptor: &ServiceDescriptor) -> Vec<Box<dyn Any>> {
        match &descriptor.type_factory {
            Some(regular_fn) => (regular_fn)(self),
            None => {
                match &descriptor.type_factory_closure {
                    Some(closure_fn) => (closure_fn)(self),
                    None => {
                        self.print_could_not_find_descriptor(descriptor.type_info.clone());
                        panic!("No type factory function available for {}", descriptor)
                    },
                }
            }
        }
    }
    
    fn print_could_not_find_descriptor(self: &Self, type_info: Box<TypeInfo>) {
        println!("Could not get service for type {}", type_info.type_name);
        println!("Services:");
        for type_descriptors in self.type_id_to_type_info.values() {
            println!("\t{:?}", type_descriptors);
        }
    }
    
    fn print_could_not_find_type_id(self: &Self, type_id: TypeId) {
        println!("Could not get service for type {:?}", type_id);
        println!("Services:");
        for type_descriptors in self.type_id_to_type_info.values() {
            println!("\t{:?}", type_descriptors);
        }
    }
    // might need methods for creating derived service collections with instantiated non-singleton scoped services
}


impl IServiceCollection for ServiceCollection {
    // should implement get_at_index, contains, index_of, insert, clear, remove, make_readonly
    // if it is possible to add extension methods, should add them as helpers.
    // for example, add_web_response_handlers (convert web response to string for HTTP)
    // add_localization, add_logging, add_file_providers

    fn try_get(&self, type_info: Box<TypeInfo>) -> Result<Vec<Box<dyn Any>>, &str> {
        let descriptors = self.find_descriptor(type_info);
        Ok(descriptors.iter().cloned().map(|descriptor| {
            match descriptor.scope {
                ServiceScope::AlwaysNew => self.instantiate(&descriptor),
                ServiceScope::Request => self.get_or_instantiate_for_request(&descriptor),
                ServiceScope::Singleton => self.get_or_instantiate_singleton(&descriptor),
            }
        }).flatten().collect())
    }

    fn get_required(&self, type_info: Box<TypeInfo>) -> Vec<Box<dyn Any>> {
        self.try_get(type_info.clone()).expect(&format!("type not found: {}", type_info).to_string())
    }

    fn get_current_scope(&self) -> ServiceScope {
        self.current_scope
    }

    fn get_parent(&self) -> Option<Rc<dyn IServiceCollection>> {
        self.parent.clone()
    }

    fn get_root(&self) -> Option<Rc<dyn IServiceCollection>> {
        self.root.clone()
    }
    
    fn get_items(&self) -> &Vec<Rc<ServiceDescriptor>> {
        &self.items
    }

    fn get_singletons(&self) -> &Vec<Rc<ServiceInstance>> {
        &self.singletons
    }

    fn get_request_instances(&self) -> &Vec<Rc<ServiceInstance>> {
        &self.request_instances
    }

    fn find_descriptor(self: &Self, type_info: Box<TypeInfo>) -> Vec<Rc<ServiceDescriptor>> {
        match self.type_id_to_descriptor.get(&type_info.type_id) {
            Some(descriptor) => descriptor.clone(),
            None => { self.print_could_not_find_descriptor(type_info); panic!() },
        }
    }

    fn find_descriptor_by_id(self: &Self, type_id: TypeId) -> Vec<Rc<ServiceDescriptor>> {
        match self.try_find_descriptor_by_id(type_id) {
            Some(descriptor) => descriptor.clone(),
            None => { panic!("self.print_could_not_find_descriptor(type_id)") },
        }
    }

    fn try_find_descriptor_by_id(self: &Self, type_id: TypeId) -> Option<&Vec<Rc<ServiceDescriptor>>> {
        self.type_id_to_descriptor.get(&type_id)
    }
}

pub struct ServiceCollectionExtensions {

}

// extension methods for IServiceCollection
impl ServiceCollectionExtensions {
    pub fn try_get_single<T: 'static + ?Sized>(services: &dyn IServiceCollection) -> Result<Option<Rc<T>>, &str> {
        let type_info = TypeInfo::rc_of::<T>();
        Ok(services
            .try_get(type_info)
            .unwrap_or(vec![])
            .iter()
            .map(|x| x.downcast_ref::<Rc<T>>().expect("could not downcast Any to Rc<T>"))
            .take(1)
            .map(|x| x.clone())
            .collect::<Vec<Rc<T>>>()
            .first().cloned()
        )
    }

    pub fn get_required_single<T: 'static + ?Sized>(services: &dyn IServiceCollection) -> Rc<T> {
        let type_info = TypeInfo::rc_of::<T>();
        services
            .try_get(type_info)
            .unwrap_or(vec![])
            .iter()
            .map(|x| x.downcast_ref::<Rc<T>>().expect(Self::format_error_could_not_downcast::<T>(services, x.type_id()).as_str()))
            .take(1)
            .map(|x| x.clone())
            .collect::<Vec<Rc<T>>>()
            .first().unwrap().clone()
    }

    pub fn format_error_could_not_downcast<T: ?Sized + 'static>(services: &dyn IServiceCollection, x: TypeId) -> String {
        let type_info = TypeInfo::rc_of::<T>();
        let _expected_descriptor = services.find_descriptor(type_info.clone());
        let found_descriptor = &services.try_find_descriptor_by_id(x);
        let found_name = match found_descriptor { Some(d) => &d.get(0).unwrap().type_info.type_name, None => "" };

        format!("could not downcast Box<dyn Any> ({:?}) to {:?} (known = true, found = {:?})", x, type_info.type_name, found_name)
    }

    pub fn try_get_multiple<T: 'static>(_services: &dyn IServiceCollection) -> Result<Vec<Box<dyn Any>>, &str> {
        Err("a")
    }

    pub fn get_required_multiple<T: 'static + ?Sized>(services: &dyn IServiceCollection) -> Vec<Rc<T>> {
        let type_info = TypeInfo::rc_of::<T>();
        services
            .try_get(type_info)
            .unwrap_or(vec![])
            .iter()
            .map(|x| x.downcast_ref::<Rc<T>>().expect("could not downcast Any to T"))
            .map(|x| x.clone())
            .collect::<Vec<Rc<T>>>()
    }
}