use std::any::Any;
use std::rc::Rc;


use crate::http::http_body_content::ContentType;
use crate::http::http_body_content::GenericBodyContent;
use crate::http::http_body_content::IBodyContent;
use crate::model_binder::decoders::url_encoded_model_decoder::UrlEncodedDecoder;
use crate::services::service_collection::IServiceCollection;
use crate::services::service_collection::ServiceCollection;
use crate::services::service_collection::ServiceCollectionExtensions;
use crate::services::service_descriptor::ServiceDescriptor;

use super::ihttp_body_format_resolver::IHttpBodyFormatResolver;



// // decodes a stream of bytes from one format to another based on the content type
// pub trait IRequestBodyDecoderService {
//     fn decode(self: &Self, body: Rc<dyn IBodyContent>) -> Rc<dyn IBodyContent>;
//     fn decode_from_raw(self: &Self, content_type: ContentType, content_length: usize, body_raw: &Vec<u8>) -> Rc<dyn IBodyContent>;
// }

// pub struct RequestBodyDecoderService {
//     pub decoders: Vec<Box<dyn IRequestBodyDecoder>>,
// }

// impl RequestBodyDecoderService {
//     pub fn new() -> Self {
//         Self {
//             decoders: vec![
//                 Box::new(UrlEncodedDecoder::new()),
//             ],
//         }
//     }

//     pub fn new_service(services: &dyn IServiceCollection) -> Vec<Box<dyn Any>> {
//         vec![
//             Box::new(Rc::new(Self::new()) as Rc<dyn IRequestBodyDecoderService>)
//         ]
//     }

//     pub fn add_to_services(services: &mut ServiceCollection) {
//         services.add(ServiceDescriptor::new(
//             crate::core::type_info::TypeInfo::rc_of::<dyn IRequestBodyDecoderService>(),
//             Self::new_service,
//             crate::services::service_scope::ServiceScope::Singleton,
//         ));
//     }
// }

// impl IRequestBodyDecoderService for RequestBodyDecoderService {
//     fn decode(self: &Self, body: Rc<dyn IBodyContent>) -> Rc<dyn IBodyContent> {
//         // println!("self.decoders.len: {}", self.decoders.len());
//         let mut body = body;
//         for decoder in &self.decoders {
//             if decoder.matches_content_type(body.get_content_type()) {
//                 // println!("RequestBodyDecoderService::decode: decoding body with decoder: {}", decoder.type_info());
//                 body = decoder.decode(body);
//             }
//         }
//         body
//     }

//     fn decode_from_raw(self: &Self, content_type: ContentType, content_length: usize, body_raw: &Vec<u8>) -> Rc<dyn IBodyContent> {
//         let body = Rc::new(GenericBodyContent::new(
//             content_type,
//             content_length,
//             body_raw.clone()
//         ));
//         self.decode(body)
//     }
// }



// decodes a stream of bytes from one format to another based on the content type
pub trait IHttpBodyFormatService {
    fn decode(self: &Self, body: Rc<dyn IBodyContent>) -> Rc<dyn IBodyContent>;
    fn decode_from_raw(self: &Self, content_type: ContentType, content_length: usize, body_raw: &Vec<u8>) -> Rc<dyn IBodyContent>;

    fn encode(self: &Self, body: Rc<dyn IBodyContent>) -> Rc<dyn IBodyContent>;
    fn encode_from_raw(self: &Self, content_type: ContentType, body: Rc<dyn IBodyContent>) -> &Vec<u8>;
}

pub struct HttpBodyFormatService {
    resolvers: Vec<Rc<dyn IHttpBodyFormatResolver>>,
}

impl HttpBodyFormatService {
    pub fn new(
        resolvers: Vec<Rc<dyn IHttpBodyFormatResolver>>,
    ) -> Self {
        Self {
            resolvers: resolvers,
        }
    }

    pub fn new_service(services: &dyn IServiceCollection) -> Vec<Box<dyn Any>> {
        vec![
            Box::new(Rc::new(Self::new(
                ServiceCollectionExtensions::get_required_multiple::<dyn IHttpBodyFormatResolver>(services),
            )) as Rc<dyn IHttpBodyFormatService>)
        ]
    }

    pub fn add_to_services(services: &mut ServiceCollection) {
        services.add(ServiceDescriptor::new(
            crate::core::type_info::TypeInfo::rc_of::<dyn IHttpBodyFormatService>(),
            Self::new_service,
            crate::services::service_scope::ServiceScope::Singleton,
        ));
    }
}

impl IHttpBodyFormatService for HttpBodyFormatService {
    fn decode(self: &Self, body: Rc<dyn IBodyContent>) -> Rc<dyn IBodyContent> {
        let resolved = self.resolve(body.get_content_type());
        if let Some(resolved) = resolved {
            // println!("self.decoders.len: {}", self.decoders.len());
            let mut body = body;
            for decoder in &self.formats {
                if decoder.matches_content_type(body.get_content_type()) {
                    // println!("RequestBodyDecoderService::decode: decoding body with decoder: {}", decoder.type_info());
                    body = decoder.decode(body);
                }
            }
            body
        } else {
            body
        }
    }

    fn decode_from_raw(self: &Self, content_type: ContentType, content_length: usize, body_raw: &Vec<u8>) -> Rc<dyn IBodyContent> {
        let body = Rc::new(GenericBodyContent::new(
            content_type,
            content_length,
            body_raw.clone()
        ));
        self.decode(body)
    }

    fn encode(self: &Self, body: Rc<dyn IBodyContent>) -> Rc<dyn IBodyContent> {
        todo!()
    }

    fn encode_from_raw(self: &Self, content_type: ContentType, body: Rc<dyn IBodyContent>) -> &Vec<u8> {
        todo!()
    }
}