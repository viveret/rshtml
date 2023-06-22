use std::rc::Rc;

use http::HeaderMap;
use http::status::StatusCode;
use mockall::automock;

use crate::http::http_body_content::ContentType;
use crate::http::ihttp_body_stream_format::IHttpBodyStreamFormat;

use super::itcpconnection_context::ITcpConnectionContext;



#[automock]
pub trait IHttpConnectionContext {
    fn get_connection_id(&self) -> u32;
    fn get_tcp_context(&self) -> &dyn ITcpConnectionContext;
    // fn get_stream(&self) -> &RefCell<Rc<dyn ITcpStreamWrapper>>;
    fn add_stream_decoders(&self, decoders: &[Rc<dyn IHttpBodyStreamFormat>], content_type: &ContentType);

    fn get_has_started_writing(&self) -> bool;

    fn set_pending_status_code(&self, status_code: StatusCode);
    fn set_pending_status_message(&self, status_message: String);
    fn get_pending_status_code(&self) -> StatusCode;
    fn get_pending_status_message(&self) -> String;

    fn add_header_string(&self, name: String, value: String);
    fn add_header_str(&self, name: &str, value: &str);
    fn set_header_string(&self, name: String, value: String);
    fn set_header_str(&self, name: &str, value: &str);
    fn get_pending_headers(&self) -> HeaderMap;
    fn get_pending_header(&self, name: &str) -> Option<String>;

    fn begin_reading(&self) -> std::io::Result<()>;
    fn end_reading(self: &Self) -> std::io::Result<()>;
    fn end_reading_begin_writing(&self) -> std::io::Result<()>;
    fn begin_writing(&self) -> std::io::Result<()>;
    fn end_writing(self: &Self) -> std::io::Result<()>;

    fn write(&self, b: &[u8]) -> std::io::Result<usize>;
    fn write_str(&self, b: &str) -> std::io::Result<usize>;
    fn write_line(&self, b: &String) -> std::io::Result<usize>;
    fn flush(&self) -> std::io::Result<()>;

    fn read_bytes(&self) -> std::io::Result<Vec<u8>>;
    fn read(&self, b: &mut [u8]) -> std::io::Result<usize>;
    fn read_line(&self) -> std::io::Result<String>;
}
