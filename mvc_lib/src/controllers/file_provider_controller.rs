use std::any::Any;
use std::rc::Rc;

use crate::services::service_collection::IServiceCollection;
use crate::services::service_collection::ServiceCollectionExtensions;

use crate::controllers::icontroller::IController;
use crate::controller_action_features::controller_action_feature::IControllerActionFeature;
use crate::controllers::controller_action::{ IControllerAction, ControllerActionFileResult };

use crate::options::file_provider_controller_options::IFileProviderControllerOptions;


pub struct FileProviderController {
    pub options: Rc<dyn IFileProviderControllerOptions>,
}

impl FileProviderController {
    pub fn new(options: Rc<dyn IFileProviderControllerOptions>) -> Self {
        Self
        { 
            options: options
        }
    }

    pub fn new_service(services: &dyn IServiceCollection) -> Vec<Box<dyn Any>> {
        vec![Box::new(Rc::new(Self::new(
            ServiceCollectionExtensions::get_required_single::<dyn IFileProviderControllerOptions>(services)
        )) as Rc<dyn IController>)]
    }
}

impl IController for FileProviderController {
    fn get_route_area(self: &Self) -> String {
        String::new()
    }

    fn get_type_name(self: &Self) -> &'static str {
        nameof::name_of_type!(FileProviderController)
    }

    fn get_actions(self: &Self) -> Vec<Rc<dyn IControllerAction>> {
        self.options.as_ref()
            .get_mapped_paths(true)
            .iter()
            .map(|x|
                Rc::new(ControllerActionFileResult::new(
                    x.1.clone(), x.0.clone(), String::new(), self.get_type_name(), self.get_route_area(),
                )) as Rc<dyn IControllerAction>
            )
            .collect()
    }

    fn get_features(self: &Self) -> Vec<Rc<dyn IControllerActionFeature>> {
        vec![]
    }
}