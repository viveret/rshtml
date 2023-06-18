use std::collections::HashMap;
use std::any::{Any, TypeId};
use std::error::Error;
use std::rc::Rc;

use crate::core::type_info::TypeInfo;

use crate::services::service_scope::ServiceScope;
use crate::services::service_descriptor::ServiceDescriptor;
use crate::services::service_instance::ServiceInstance;


// this interface is used to store services and dependencies in a collection.
// services are described via their type and scope, and when requested, are instantiated.
// dependencies are created when a service is instantiated, and are stored in the collection.
pub trait IServiceCollection: Send + Sync {
    // try to get a service from the collection. if it is not found, return an error.
    fn try_get(&self, type_info: Box<TypeInfo>) -> Result<Vec<Box<dyn Any>>, &str>;
    // get a service from the collection. if it is not found, panic.
    fn get_required(&self, type_info: Box<TypeInfo>) -> Vec<Box<dyn Any>>;

    // get the current scope of the collection
    fn get_current_scope(&self) -> ServiceScope;

    // get the parent scope of the collection
    fn get_parent(&self) -> Option<&dyn IServiceCollection>;
    
    // get the root scope of the collection
    fn get_root(&self) -> Option<&dyn IServiceCollection>;
    
    // get the items in the collection
    fn get_items(&self) -> Vec<Rc<ServiceDescriptor>>;

    // get the singletons in the collection
    fn get_singletons(&self) -> &Vec<Rc<ServiceInstance>>;

    // get the per-request instances in the collection
    fn get_request_instances(&self) -> &Vec<Rc<ServiceInstance>>;

    // find a service descriptor by type info
    fn find_descriptor(self: &Self, type_info: Box<TypeInfo>) -> Vec<Rc<ServiceDescriptor>>;
    // find a service descriptor by type id
    fn find_descriptor_by_id(self: &Self, type_id: TypeId) -> Vec<Rc<ServiceDescriptor>>;
    // try to find a service descriptor by type id
    fn try_find_descriptor_by_id(self: &Self, type_id: TypeId) -> Option<&Vec<Rc<ServiceDescriptor>>>;
}


// generic service collection implementation. 
#[derive(Clone)]
pub struct ServiceCollection<'a> {
    // scope of the current collection
    current_scope: ServiceScope,

    // outer scope of the current collection
    parent: Option<&'a dyn IServiceCollection>,

    // root scope of the current collection
    root: Option<&'a dyn IServiceCollection>,

    // services kept alive within this scope
    items: Vec<Rc<ServiceDescriptor>>,

    // type descriptors kept alive within just this scope
    type_id_to_descriptor: HashMap<TypeId, Vec<Rc<ServiceDescriptor>>>,
    type_id_to_type_info: HashMap<TypeId, Rc<Box<TypeInfo>>>,

    // singletons live in this scope
    singletons: Vec<Rc<ServiceInstance>>,

    // per-request instances live in this scope
    request_instances: Vec<Rc<ServiceInstance>>,
}
unsafe impl <'a> Send for ServiceCollection<'a> {}
unsafe impl <'a> Sync for ServiceCollection<'a> {}

impl <'a> ServiceCollection<'a> {
    // create a new root service collection
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

    // create a new service collection with the given scope, parent, and root
    pub fn new(scope: ServiceScope, parent: &'a (dyn IServiceCollection + 'a), root: &'a (dyn IServiceCollection + 'a)) -> Self {
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

    // add a service descriptor to the collection
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

    // get or instantiate a service for a HTTP request that is kept alive for the duration of the request
    fn get_or_instantiate_for_request(self: &Self, descriptor: &ServiceDescriptor) -> Vec<Box<dyn Any>> {
        self.instantiate(descriptor)
    }

    // get or instantiate a service that is kept alive for the duration of the current scope
    fn get_or_instantiate_singleton(self: &Self, descriptor: &ServiceDescriptor) -> Vec<Box<dyn Any>> {
        self.instantiate(descriptor)
    }

    // instantiate a service
    // descriptor: the service descriptor
    // returns: the instantiated service
    fn instantiate(self: &Self, descriptor: &ServiceDescriptor) -> Vec<Box<dyn Any>> {
        match &descriptor.type_factory {
            Some(regular_fn) => (regular_fn)(self),
            None => {
                match &descriptor.type_factory_closure {
                    Some(closure_fn) => (closure_fn)(self),
                    None => {
                        if let Some(parent) = self.parent {
                            return parent.try_get(descriptor.type_info.clone()).unwrap_or(vec![]);
                        } else {
                            self.get_could_not_find_descriptor_message(descriptor.type_info.clone());
                            panic!("No type factory function available for {}", descriptor)
                        }
                    },
                }
            }
        }
    }
    
    // get a message that a service could not be found for type info
    fn get_could_not_find_descriptor_message(self: &Self, type_info: Box<TypeInfo>) -> String {
        let mut message = String::new();
        message.push_str(format!("Could not get service for type {}", type_info.type_name).as_str());
        message.push_str(format!("Services:").as_str());
        for type_descriptors in self.type_id_to_type_info.values() {
            message.push_str(format!("\t{:?}", type_descriptors).as_str());
        }
        message
    }
    
    // get a message that a service could not be found for a type id
    #[allow(dead_code)]
    fn get_could_not_find_type_id_message(self: &Self, type_id: TypeId) -> String {
        let mut message = String::new();
        message.push_str(format!("Could not get service for type {:?}", type_id).as_str());
        message.push_str(format!("Services:").as_str());
        for type_descriptors in self.type_id_to_type_info.values() {
            message.push_str(format!("\t{:?}", type_descriptors).as_str());
        }
        message
    }
    // might need methods for creating derived service collections with instantiated non-singleton scoped services
}

impl <'a> IServiceCollection for ServiceCollection<'a> {
    // should implement get_at_index, contains, index_of, insert, clear, remove, make_readonly
    // if it is possible to add extension methods, should add them as helpers.
    // for example, add_web_response_handlers (convert web response to string for HTTP)
    // add_localization, add_logging, add_file_providers

    fn try_get(&self, type_info: Box<TypeInfo>) -> Result<Vec<Box<dyn Any>>, &str> {
        let descriptors = self.find_descriptor(type_info);
        Ok(descriptors.iter().cloned().map(|descriptor| {
            match descriptor.scope {
                ServiceScope::AlwaysNew |
                ServiceScope::Area |
                ServiceScope::Controller |
                ServiceScope::Scope |
                ServiceScope::Host => self.instantiate(&descriptor),
                ServiceScope::Request => self.get_or_instantiate_for_request(&descriptor),
                ServiceScope::Singleton => self.get_or_instantiate_singleton(&descriptor),
            }
        }).flatten().collect())
    }

    fn get_required(&self, type_info: Box<TypeInfo>) -> Vec<Box<dyn Any>> {
        let try_get_self = self.try_get(type_info.clone());
        match try_get_self {
            Ok(x) => x,
            Err(_) => {
                let try_get_parent = match self.parent {
                    Some(parent) => parent.try_get(type_info.clone()),
                    None => Err(""),
                };

                match try_get_parent {
                    Ok(x) => x,
                    Err(_) => {
                        panic!("type not found: {}", type_info);
                    },
                }
            },
        }
    }

    fn get_current_scope(&self) -> ServiceScope {
        self.current_scope
    }

    fn get_parent(&self) -> Option<&dyn IServiceCollection> {
        self.parent.clone()
    }

    fn get_root(&self) -> Option<&dyn IServiceCollection> {
        self.root.clone()
    }
    
    fn get_items(&self) -> Vec<Rc<ServiceDescriptor>> {
        if let Some(parent) = self.parent {
            self.items
                .iter()
                .chain(parent.get_items().iter())
                .cloned()
                .collect::<Vec<Rc<ServiceDescriptor>>>()
        } else {
            self.items.clone()
        }
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
            None => {
                vec![]
            },
        }
        .iter()
        .cloned()
        .chain(
            match self.parent {
                Some(parent) => parent.find_descriptor(type_info),
                None => vec![],
            }.iter().cloned()
        )
        .collect()
    }

    fn find_descriptor_by_id(self: &Self, type_id: TypeId) -> Vec<Rc<ServiceDescriptor>> {
        match self.try_find_descriptor_by_id(type_id) {
            Some(descriptor) => descriptor.clone(),
            None => { vec![] }, // panic!("{}", self.get_could_not_find_type_id_message(type_id))
        }
        .iter()
        .cloned()
        .chain(
            match self.parent {
                Some(parent) => parent.try_find_descriptor_by_id(type_id),
                None => { None },
            }
            .unwrap_or(&vec![])
            .iter()
            .cloned()
        )
        .collect()
    }

    fn try_find_descriptor_by_id(self: &Self, type_id: TypeId) -> Option<&Vec<Rc<ServiceDescriptor>>> {
        if let Some(descriptor) = self.type_id_to_descriptor.get(&type_id) {
            Some(descriptor)
        } else {
            match self.parent {
                Some(parent) => parent.try_find_descriptor_by_id(type_id),
                None => { None },
            }
        }
    }
}

// extension methods for IServiceCollection (leave empty)
pub struct ServiceCollectionExtensions {}

// extension methods for IServiceCollection
impl ServiceCollectionExtensions {
    // try to get a service from the collection. if it is not found, return an error.
    pub fn try_get_single<T: 'static + ?Sized>(services: &dyn IServiceCollection) -> Result<Option<Rc<T>>, Rc<dyn Error>> {
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

    // get a service from the collection. if it is not found, panic.
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
            .first()
            .unwrap()
            .clone()
    }

    // get a service from the collection. if it is not found, panic.
    pub fn format_error_could_not_downcast<T: ?Sized + 'static>(services: &dyn IServiceCollection, x: TypeId) -> String {
        let type_info = TypeInfo::rc_of::<T>();
        let _expected_descriptor = services.find_descriptor(type_info.clone());
        let found_descriptor = &services.try_find_descriptor_by_id(x);
        let found_name = match found_descriptor { Some(d) => &d.get(0).unwrap().type_info.type_name, None => "" };

        format!("could not downcast Box<dyn Any> ({:?}) to {:?} (known = true, found = {:?})", x, type_info.type_name, found_name)
    }

    // try to get multiple services from the collection. if it is not found, return an error.
    pub fn try_get_multiple<T: 'static + ?Sized>(services: &dyn IServiceCollection) -> Result<Vec<Rc<T>>, &str> {
        Ok(Self::get_required_multiple::<T>(services))
    }

    // get multiple services from the collection. if it is not found, panic.
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

    // get multiple services from the collection. if it is not found, panic.
    pub fn get_required_one_or_more<T: 'static + ?Sized>(services: &dyn IServiceCollection) -> Vec<Rc<T>> {
        let found = Self::get_required_multiple::<T>(services);
        if found.len() == 0 {
            println!("Available services ({}):\n\t{}", services.get_items().len(), services.get_items().iter().map(|x| x.to_string()).collect::<Vec<String>>().join("\n\t"));
            let type_info = TypeInfo::rc_of::<T>();
            panic!("No services found for type {}", type_info.type_name);
        } else {
            found
        }
    }
}