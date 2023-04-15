


use std::{rc::Rc, any::Any};

use crate::{entity::idbset::IDbSetAny, services::{service_collection::{ServiceCollection, IServiceCollection}, service_descriptor::ServiceDescriptor, service_scope::ServiceScope}, core::type_info::TypeInfo};

use crate::auth::auth_role_json_file_dbset::AuthRoleJsonFileDbSet;


pub trait IAuthRolesDbSetProvider {
    fn get_authroles_dbset(self: &Self) -> &dyn IDbSetAny;
}

pub struct GenericAuthRolesDbSetProvider {
    authroles_dbset: Box<dyn IDbSetAny>,
}

impl GenericAuthRolesDbSetProvider {
    pub fn new() -> Self {
        Self {
            authroles_dbset: Box::new(AuthRoleJsonFileDbSet::new("data/authrole_dbset.json".to_string()))
        }
    }

    pub fn new_service(services: &dyn IServiceCollection) -> Vec<Box<dyn Any>> {
        vec![Box::new(Rc::new(Self::new()) as Rc<dyn IAuthRolesDbSetProvider>)]
    }

    pub fn add_to_services(services: &mut ServiceCollection) {
        services.add(ServiceDescriptor::new(TypeInfo::rc_of::<dyn IAuthRolesDbSetProvider>(), Self::new_service, ServiceScope::Singleton));
    }
}

impl IAuthRolesDbSetProvider for GenericAuthRolesDbSetProvider {
    fn get_authroles_dbset(self: &Self) -> &dyn IDbSetAny {
        self.authroles_dbset.as_ref()
    }
}