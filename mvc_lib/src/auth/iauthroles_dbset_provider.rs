use std::any::Any;
use std::rc::Rc;

use crate::entity::idbset::IDbSetAny;
use crate::services::service_scope::ServiceScope;
use crate::services::service_descriptor::ServiceDescriptor;
use crate::services::service_collection::{ServiceCollection, IServiceCollection};

use crate::auth::auth_role_json_file_dbset::AuthRoleJsonFileDbSet;

// this trait is used to get the authroles dbset
pub trait IAuthRolesDbSetProvider {
    // get the authroles dbset
    fn get_authroles_dbset(self: &Self) -> &dyn IDbSetAny;
}

// this struct implements IAuthRolesDbSetProvider
pub struct GenericAuthRolesDbSetProvider {
    authroles_dbset: Box<dyn IDbSetAny>,
}

impl GenericAuthRolesDbSetProvider {
    pub fn new() -> Self {
        Self {
            authroles_dbset: Box::new(AuthRoleJsonFileDbSet::open("data/authrole_dbset.json".to_string()).expect("could not open authrole_dbset.json"))
        }
    }

    pub fn new_service(_services: &dyn IServiceCollection) -> Vec<Box<dyn Any>> {
        vec![Box::new(Rc::new(Self::new()) as Rc<dyn IAuthRolesDbSetProvider>)]
    }

    pub fn add_to_services(services: &mut ServiceCollection) {
        services.add(ServiceDescriptor::new_from::<dyn IAuthRolesDbSetProvider, Self>(Self::new_service, ServiceScope::Singleton));
    }
}

impl IAuthRolesDbSetProvider for GenericAuthRolesDbSetProvider {
    fn get_authroles_dbset(self: &Self) -> &dyn IDbSetAny {
        self.authroles_dbset.as_ref()
    }
}