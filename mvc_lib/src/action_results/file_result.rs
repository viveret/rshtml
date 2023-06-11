use std::fs::File;
use std::io::Read;
use std::path::Path;

use http::StatusCode;

use crate::contexts::irequest_context::IRequestContext;
use crate::contexts::response_context::IResponseContext;
use crate::contexts::controller_context::IControllerContext;

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

    fn configure_response(self: &Self, _controller_ctx: &dyn IControllerContext, response_context: &dyn IResponseContext, _request_context: &dyn IRequestContext, _services: &dyn IServiceCollection) {
        match File::open(self.path.clone()) {
            Ok(f) => {
                response_context.set_status_code(StatusCode::OK);
                response_context.add_header_str("Content-Type", &self.content_type);
                let mut reader = std::io::BufReader::new(f);
                let body_stream = response_context.get_connection_context();
                let mut buffer = [0; 4096];
                loop {
                    let num_read = reader.read(&mut buffer).unwrap();
                    if num_read == 0 {
                        break;
                    }
                    match body_stream.write(&buffer[0..num_read]) {
                        Ok(_) => {},
                        Err(_error) => break,
                    }
                }
            },
            Err(_error) => {
                // println!("Error opening file: {}", error);
                response_context.set_status_code(StatusCode::NOT_FOUND);
            }
        }
    }
}