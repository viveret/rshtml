use std::any::Any;
use std::rc::Rc;

// this trait abstracts the wwwroot provider controller options.
pub trait IWwwRootProviderControllerOptions {
}

// this struct implements IWwwRootProviderControllerOptions.
#[derive(Debug, Clone)]
pub struct WwwRootProviderControllerOptions {
}

impl WwwRootProviderControllerOptions {
    pub fn new(
    ) -> Self {
        Self {}
    }

    // create a new WwwRootProviderControllerOptions struct with default values.
    pub fn new_defaults() -> Self {
        Self {}
    }

    // create a new WwwRootProviderControllerOptions struct as a service.
    // returns: a new WwwRootProviderControllerOptions struct as a service.
    pub fn new_service(
    ) -> Box<dyn Any> {
        Box::new(Rc::new(Self::new()) as Rc<dyn IWwwRootProviderControllerOptions>)
    }

    // create a new WwwRootProviderControllerOptions struct as a service with default values.
    pub fn new_service_defaults() -> Box<dyn Any> {
        Box::new(Rc::new(Self::new_defaults()) as Rc<dyn IWwwRootProviderControllerOptions>)
    }
}

impl IWwwRootProviderControllerOptions for WwwRootProviderControllerOptions {
}
