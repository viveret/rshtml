use std::any::Any;
use std::rc::Rc;

use crate::contexts::irequest_context::IRequestContext;
use crate::core::type_info::TypeInfo;

use super::imodel::IModel;
use super::model_validation_result::ModelValidationResult;




pub trait IModelBinderService {
    fn bind_model(&self, request_context: &dyn IRequestContext, model_type: &TypeInfo) -> ModelValidationResult<Rc<dyn IModel>>;
}