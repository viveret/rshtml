use std::collections::HashMap;
use std::rc::Rc;

use core_macro_lib::{reflect_attributes, reflect_properties, reflect_methods};
use core_macro_lib::IHazAttributes;
use core_macro_lib::IModel;

use crate::contexts::irequest_context::IRequestContext;
use crate::http::http_body_content::ContentType;
use crate::core::type_info::TypeInfo;
use crate::core::query_string::QueryString;

use super::imodel_attribute::IAttribute;
use super::ihaz_attributes::IHazAttributes;
use super::imodel::IModel;
use super::imodel_property::IModelProperty;
use crate::model_binder::imodel_method::IModelMethod;
use crate::model_binder::reflected_attribute::ReflectedAttribute;
use crate::model_binder::reflected_property::ReflectedProperty;
use crate::model_binder::reflected_method::ReflectedMethod;


// this struct is used to parse the body content of a form url encoded request.
// it is used by the FormUrlEncodedBinder.
#[reflect_attributes]
#[reflect_properties]
#[derive(Clone, Debug, IHazAttributes, IModel)]
pub struct UrlEncodedModel(pub QueryString);

#[reflect_methods]
impl UrlEncodedModel {
    // parse the body content of a form url encoded request.
    // body_content: the body content to parse.
    // returns: the parsed body content.
    pub fn parse_body(_: ContentType, body_bytes: &Vec<u8>) -> Self {
        let body_content = std::str::from_utf8(body_bytes).unwrap();
        Self::new(body_content)
    }

    pub fn new(body: &str) -> Self {
        Self(QueryString::parse(body))
    }

    pub fn new_from_body(content_type: ContentType, body: &dyn IRequestContext) -> Self {
        let generic = body.get_connection_context();
        // read all of body (TODO: implement in ITcpStreamWrapper so that we can read all of the body in one call and reuse the function)
        let mut body_bytes = Vec::new();
        loop {
            let mut buffer = [0; 1024];
            match generic.read(&mut buffer) {
                Ok(0) => break,
                Ok(n) => {
                    body_bytes.extend_from_slice(&buffer[0..n]);
                    if n < 1024 {
                        break;
                    }
                }
                Err(e) => {
                    println!("Error reading body: {:?}", e);
                    break;
                }
            }
        }

        Self::parse_body(content_type, &body_bytes)
    }
}

// impl IBodyContent for UrlEncodedModel {
//     fn get_content_type(self: &Self) -> ContentType {
//         ContentType {
//             mime_type: "application/x-www-form-urlencoded".to_string(),
//             options: "".to_string(),
//         }
//     }

//     fn get_content_length(self: &Self) -> usize {
//         self.0.to_string().len()
//     }

//     fn get_self_type(self: &Self) -> ContentType {
//         ContentType {
//             mime_type: "application/x-www-form-urlencoded".to_string(),
//             options: "".to_string(),
//         }
//     }

//     fn data(self: &Self) -> &dyn std::any::Any {
//         &self.0
//     }

//     fn to_string(self: &Self) -> String {
//         format!("UrlEncodedModel: {:?}", self.0)
//     }
// }