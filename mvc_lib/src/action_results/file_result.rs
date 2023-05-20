use std::fs::File;
use std::rc::Rc;
use std::io::Read;
use std::path::Path;

use http::StatusCode;

use crate::contexts::irequest_context::IRequestContext;
use crate::contexts::response_context::ResponseContext;
use crate::contexts::controller_context::ControllerContext;

use crate::action_results::iaction_result::IActionResult;

use crate::services::service_collection::IServiceCollection;

// this is a struct that holds the file path and the content type
pub struct FileResult {
    pub path: String,
    pub content_type: String,
}

impl FileResult {
    // create a new FileResult. If the content type is not specified, it will be determined by the extension.
    // if the extension is not recognized, it will be set to text/plain.
    // path: String - the path to the file
    // content_type: Option<String> - the content type of the file
    // returns: FileResult - the FileResult
    pub fn new(path: String, content_type: Option<String>) -> Self {
        Self { path: path.clone(), content_type: content_type.unwrap_or(Self::extension_to_content_type(&path).to_string()) }
    }

    // this function takes a path and returns the content type based on the extension
    // if the extension is not recognized, it returns text/plain.
    // this function is used by the constructor to set the content type if it is not specified.
    // this function is also used by the static file provider to set the content type of the response.
    // path: &String - the path to the file
    // returns: mime::Mime - the content type
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

    fn configure_response(self: &Self, _controller_ctx: Rc<ControllerContext>, response_ctx: Rc<ResponseContext>, _request_ctx: Rc<dyn IRequestContext>, _services: &dyn IServiceCollection) {
        match File::open(self.path.clone()) {
            Ok(f) => {
                response_ctx.add_header_str("Content-Type", &self.content_type);
                let mut reader = std::io::BufReader::new(f);
                match reader.read_to_end(&mut response_ctx.body.borrow_mut()) {
                    Ok(num_read) => {
                        // println!("Wrote {}, {} bytes", self.path, num_read);
                        response_ctx.status_code.replace(StatusCode::OK);
                    },
                    Err(error) => {
                        // println!("Error reading file: {}", error);
                        response_ctx.status_code.replace(StatusCode::NOT_FOUND);
                    }
                }
            },
            Err(error) => {
                // println!("Error opening file: {}", error);
                response_ctx.status_code.replace(StatusCode::NOT_FOUND);
            }
        }
    }
}