use std::any::Any;
use std::rc::Rc;

use crate::services::service_descriptor::ServiceDescriptor;


pub struct ServiceInstance {
    pub descriptor: ServiceDescriptor,
    pub instance: Rc<dyn Any>,
}