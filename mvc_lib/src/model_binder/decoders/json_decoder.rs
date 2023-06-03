use std::any::Any;
use std::rc::Rc;

use crate::contexts::irequest_context::IRequestContext;
use crate::core::type_info::TypeInfo;
use crate::model_binder::imodel::IModel;
use crate::model_binder::imodel_binder::IModelBinder;
use crate::model_binder::model_validation_result::ModelValidationResult;
use crate::services::service_descriptor::ServiceDescriptor;
use crate::services::service_collection::{IServiceCollection, ServiceCollection};
use crate::services::service_scope::ServiceScope;


// this struct is used to decode the view model from the request body.
pub struct JsonDecoder {

}

impl JsonDecoder {
    // creates a new instance of JsonDecoder.
    pub fn new() -> Self {
        Self {
        }
    }

    // creates a new instance of JsonDecoder as a service from the given IServiceCollection.
    // services: the IServiceCollection to create the JsonDecoder from.
    // returns: a Vec of Box<dyn Any> containing the JsonDecoder as a service.
    pub fn new_service(_services: &dyn IServiceCollection) -> Vec<Box<dyn Any>> {
        vec![Box::new(Rc::new(Self::new(
        )) as Rc<dyn IModelBinder>)]
    }

    // adds the JsonDecoder to the given IServiceCollection.
    // services: the IServiceCollection to add the JsonDecoder to.
    pub fn add_to_services(services: &mut ServiceCollection) {
        services.add(ServiceDescriptor::new(TypeInfo::rc_of::<dyn IModelBinder>(), Self::new_service, ServiceScope::Singleton));
    }
}

impl IModelBinder for JsonDecoder {
    fn matches(self: &Self, request_context: &dyn IRequestContext) -> bool {
        false
    }

    fn bind_model(self: &Self, request_context: &dyn IRequestContext) -> ModelValidationResult<Rc<dyn IModel>> {
        todo!()

        // let mut body_bytes = vec![];
        // if let Some(content_length) = found_content_length {
        //     // iterate for body bytes, at most content_length bytes
        //     loop {
        //         let mut buf = [0; 1];
        //         let read_result = request_context.get_connection_context().mut_body_stream().borrow_mut().read(&mut buf);

        //         if read_result.is_err() {
        //             println!("Could not read http body: {}", read_result.err().unwrap());
        //             break;
        //         }

        //         let read_result = read_result.unwrap();

        //         if read_result == 0 {
        //             println!("Could not read http body: no bytes read");
        //             break;
        //         }

        //         body_bytes.push(buf[0]);

        //         if body_bytes.len() >= content_length {
        //             break;
        //         }
        //     }

        //     if body_bytes.len() != content_length {
        //         println!("Could not read http body: expected {} bytes, but read {} bytes", content_length, body_bytes.len());
        //         body_bytes.clear();
        //     } else {
        //         println!("Read {} bytes from http body which matched content-length header", body_bytes.len());
        //         println!("Preview: {:?}", String::from_utf8_lossy(&body_bytes[0..std::cmp::min(100, body_bytes.len())]));
        //     }
        // }

        // found_content_length, found_content_type, &body_bytes
        // if body_bytes.len() > 0 {
        //     self.model_binder_service.resolve_for_request(request_context);
        // }
        // let body = Self::decode_body(found_content_length, found_content_type, request_bytes, body_content_decoder_service);
    }

    fn type_info(self: &Self) -> Box<TypeInfo> {
        TypeInfo::rc_of::<Self>()
    }
}