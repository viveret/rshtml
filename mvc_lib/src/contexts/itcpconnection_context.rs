use std::rc::Rc;

use mockall::automock;

use crate::http::http_body_content::ContentType;
use crate::http::ihttp_body_stream_format::IHttpBodyStreamFormat;


// this trait represents a TCP connection.
#[automock]
pub trait ITcpConnectionContext {
    // returns a string representation of the connection context.
    fn to_string(self: &Self) -> String;

    // get the remote address of the connection.
    fn get_remote_addr(self: &Self) -> std::net::SocketAddr;

    fn get_connection_id(&self) -> u32;

    fn add_stream_decoders(&self, decoders: &[Rc<dyn IHttpBodyStreamFormat>], content_type: &ContentType);

    fn shutdown(&self, how: std::net::Shutdown) -> std::io::Result<()>;

    fn flush(&self) -> std::io::Result<()>;

    fn write(&self, b: &[u8]) -> std::io::Result<usize>;

    fn write_line(&self, b: &String) -> std::io::Result<usize>;

    fn read(&self, b: &mut [u8]) -> std::io::Result<usize>;

    fn read_line(&self) -> std::io::Result<String>;
}