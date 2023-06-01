// use std::rc::Rc;

// use crate::http::http_body_content::{ContentType, IBodyContent};
// use crate::core::query_string::QueryString;


// pub struct UrlEncodedBodyContent {
//     pub content_length: usize,
//     pub form_data: QueryString,
// }

// impl UrlEncodedBodyContent {
//     pub fn is_match(content_type: &ContentType) -> bool {
//         content_type.mime_type == "application/x-www-form-urlencoded"
//     }

//     pub fn new(body: Rc<dyn IBodyContent>) -> Self {
//         if let Some(generic) = body.data().downcast_ref::<Vec<u8>>() {
//             // return Self::parse_body(generic.get_content_type(), &generic.body_raw);

//              //     // form_urlencoded::parse(&body).map(|x| (x.0.to_string(), x.1.to_string())).collect()
//             let form_data = QueryString::parse(std::str::from_utf8(generic).unwrap());
//             Self {
//                 content_length: body.get_content_length(),
//                 form_data,
//             }
//         } else {
//             panic!("UrlEncodedBodyContent::new: data is not a Vec<u8>");
//         }
//     }
// }

// impl IBodyContent for UrlEncodedBodyContent {
//     fn get_content_type(self: &Self) -> ContentType {
//         ContentType {
//             mime_type: "application/x-www-form-urlencoded".to_string(),
//             options: "".to_string(),
//         }
//     }

//     fn get_content_length(self: &Self) -> usize {
//         self.content_length
//     }

//     fn get_self_type(self: &Self) -> ContentType {
//         ContentType {
//             mime_type: "application/x-www-form-urlencoded".to_string(),
//             options: "".to_string(),
//         }
//     }

//     fn data(self: &Self) -> &dyn std::any::Any {
//         &self.form_data
//     }

//     fn to_string(self: &Self) -> String {
//         format!("UrlEncodedBodyContent: {:?}", self.form_data)
//     }
// }