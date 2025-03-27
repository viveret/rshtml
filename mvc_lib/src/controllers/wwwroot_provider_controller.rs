use std::any::Any;
use std::borrow::Cow;
use std::collections::HashMap;
use std::rc::Rc;

use core_macro_lib::IHazAttributes;
use core_macro_lib::IModel;
use core_macro_lib::reflect_attributes;
use core_macro_lib::reflect_methods;
use core_macro_lib::reflect_properties;

use crate::core::type_info::TypeInfo;

use crate::model_binder::imodel_attribute::IAttribute;
use crate::model_binder::ihaz_attributes::IHazAttributes;
use crate::model_binder::imodel::IModel;
use crate::model_binder::imodel_property::IModelProperty;
use crate::model_binder::imodel_method::IModelMethod;
use crate::model_binder::reflected_attribute::ReflectedAttribute;
use crate::model_binder::reflected_method::ReflectedMethod;
use crate::model_binder::reflected_property::ReflectedProperty;
use crate::options::wwwroot_provider_controller_options::IWwwRootProviderControllerOptions;
use crate::services::service_collection::IServiceCollection;
use crate::services::service_collection::ServiceCollectionExtensions;

use crate::controllers::icontroller::IController;
use crate::controller_action_features::controller_action_feature::IControllerActionFeature;
use crate::controller_actions::controller_action::IControllerAction;
use crate::controller_actions::file::ControllerActionFileResult;
use crate::services::wwwroot_provider_service::IWwwRootProviderService;

use super::icontroller_extensions::IControllerExtensions;


// this controller is used to serve static files from the disk.
#[reflect_attributes]
#[reflect_properties]
#[derive(Clone, IHazAttributes, IModel)]
pub struct WwwRootProviderController {
    // the options for the file provider controller.
    options: Rc<dyn IWwwRootProviderControllerOptions>,
    service: Rc<dyn IWwwRootProviderService>,
}

#[reflect_methods]
impl WwwRootProviderController {
    // create a new instance of the controller.
    // options: the options for the file provider controller.
    pub fn new(
        options: Rc<dyn IWwwRootProviderControllerOptions>,
        service: Rc<dyn IWwwRootProviderService>
    ) -> Self {
        Self { 
            options,
            service
        }
    }

    // create a new instance of the controller as a service for a service collection.
    // services: the service collection for the controller.
    // returns: a new instance of the controller in a vector as a service for a service collection.
    pub fn new_service(services: &dyn IServiceCollection) -> Vec<Box<dyn Any>> {
        vec![Box::new(Rc::new(Self::new(
            ServiceCollectionExtensions::get_required_single::<dyn IWwwRootProviderControllerOptions>(services),
            ServiceCollectionExtensions::get_required_single::<dyn IWwwRootProviderService>(services),
        )) as Rc<dyn IController>)]
    }
}

impl IController for WwwRootProviderController {
    fn get_route_area(&self) -> String {
        String::new()
    }

    fn get_type_name(&self) -> &'static str {
        nameof::name_of_type!(WwwRootProviderController)
    }

    fn get_actions(&self) -> Vec<Rc<dyn IControllerAction>> {
        let mapped_paths = self.service.as_ref().get_mapped_paths(true);

        mapped_paths
            .into_iter()
            .map(|x|
                Rc::new(ControllerActionFileResult::new(
                    x.1.into(), x.0.into(), String::default().into(), IControllerExtensions::get_name(self).into(), self.get_route_area(),
                )) as Rc<dyn IControllerAction>
            )
            .collect()
    }

    fn get_features(&self) -> Vec<Rc<dyn IControllerActionFeature>> {
        vec![]
    }
}