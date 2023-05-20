use std::any::Any;
use std::rc::Rc;

use crate::services::service_descriptor::ServiceDescriptor;

// the service instance struct holds the service descriptor and the instance of the service.
pub struct ServiceInstance {
    // descriptor for the service
    pub descriptor: ServiceDescriptor,
    // instance of the service
    pub instance: Rc<dyn Any>,
}