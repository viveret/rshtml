use std::fs::File;
use std::cell::RefCell;
use std::rc::Rc;
use std::sync::{Arc, RwLock};
use std::io::Read;
use std::path::Path;

use http::StatusCode;

use crate::contexts::request_context::RequestContext;
use crate::contexts::response_context::ResponseContext;
use crate::contexts::controller_context::ControllerContext;

use crate::action_results::iaction_result::IActionResult;

use crate::services::service_collection::IServiceCollection;

pub struct FileResult {
    pub path: String,
    pub content_type: String,
}

impl FileResult {
    pub fn new(path: String, content_type: Option<String>) -> Self {
        Self { path: path.clone(), content_type: content_type.unwrap_or(Self::extension_to_content_type(&path).to_string()) }
    }

    pub fn extension_to_content_type(path: &String) -> mime::Mime {
        match Path::new(&path)
            .extension()
            .and_then(std::ffi::OsStr::to_str)
            .expect("Could not get extension") {
                "png" => mime::IMAGE_PNG,
                "jpg" => mime::IMAGE_JPEG,
                "bmp" => mime::IMAGE_BMP,
                "gif" => mime::IMAGE_GIF,
                "svg" => mime::IMAGE_SVG,
                "css" => mime::TEXT_CSS,
                "js" => mime::TEXT_JAVASCRIPT,
                "json" => mime::APPLICATION_JSON,
                "html" => mime::TEXT_HTML,
                _ => mime::TEXT_PLAIN,
            }
    }
}

impl IActionResult for FileResult {
    fn get_statuscode(self: &Self) -> StatusCode {
        StatusCode::OK
    }

    fn configure_response(self: &Self, _controller_ctx: Rc<RefCell<ControllerContext>>, response_ctx: Rc<RefCell<ResponseContext>>, _request_ctx: Rc<RequestContext>, _services: Arc<RwLock<dyn IServiceCollection>>) {
        let mut response = response_ctx.as_ref().borrow_mut();
        match File::open(self.path.clone()) {
            Ok(f) => {
                response.add_header_str("Content-Type", &self.content_type);
                let mut reader = std::io::BufReader::new(f);
                match reader.read_to_end(&mut response.body) {
                    Ok(num_read) => {
                        println!("Wrote {}, {} bytes", self.path, num_read);
                        response.status_code = StatusCode::OK;
                    },
                    Err(error) => {
                        println!("Error reading file: {}", error);
                        response.status_code = StatusCode::NOT_FOUND;
                    }
                }
            },
            Err(error) => {
                println!("Error opening file: {}", error);
                response.status_code = StatusCode::NOT_FOUND;
            }
        }
    }
}