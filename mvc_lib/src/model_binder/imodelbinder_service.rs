use crate::contexts::irequest_context::IRequestContext;
use crate::core::type_info::TypeInfo;

use super::imodel::AnyIModel;
use super::model_validation_result::ModelValidationResult;


// service for binding models.
pub trait IModelBinderService {
    fn bind_model(&self, request_context: &dyn IRequestContext, model_type: &TypeInfo) -> ModelValidationResult<AnyIModel>;
}