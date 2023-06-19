use std::cell::RefCell;
use std::rc::Rc;
use std::vec;

use http::{HeaderName, HeaderValue, HeaderMap};
use http::status::StatusCode;

use crate::core::buffered_tcpstream::BufferedTcpStream;
use crate::http::http_body_content::ContentType;
use crate::http::ihttp_body_stream_format::IHttpBodyStreamFormat;

use super::connection_context::ConnectionContext;
use super::iconnection_context::IConnectionContext;
use super::ihttpconnection_context::IHttpConnectionContext;


// this struct implements IHttpConnectionContext and represents a HTTP connection.
pub struct HttpConnectionContext {
    pub tcp_connection_context: Rc<dyn IConnectionContext>,
    pub has_started_writing: RefCell<bool>,

    pub pending_http_version: RefCell<http::Version>,
    pub pending_status_code: RefCell<Option<StatusCode>>,
    pub pending_status_message: RefCell<Option<String>>,
    pub pending_headers: RefCell<HeaderMap>,
}

impl HttpConnectionContext {
    pub fn new(connection_context: Rc<dyn IConnectionContext>) -> Self {
        Self {
            tcp_connection_context: connection_context,
            has_started_writing: RefCell::new(false),
            pending_http_version: RefCell::new(http::Version::HTTP_11),
            pending_status_code: RefCell::new(None),
            pending_status_message: RefCell::new(None),
            pending_headers: RefCell::new(HeaderMap::new()),
        }
    }

    pub fn new_from_stream(stream: std::net::TcpStream, connection_id: u32) -> HttpConnectionContext {
        Self::new(Rc::new(ConnectionContext::new_from_stream(BufferedTcpStream::new_from_tcp(stream), connection_id)))
    }

    pub fn shutdown(&self, how: std::net::Shutdown) -> std::io::Result<()> {
        self.tcp_connection_context.shutdown(how)
    }
}

impl IHttpConnectionContext for HttpConnectionContext {
    fn get_tcp_context(&self) -> &dyn IConnectionContext {
        self.tcp_connection_context.as_ref()
    }
    
    fn flush(&self) -> std::io::Result<()> {
        self.tcp_connection_context.flush()
    }

    fn write(&self, b: &[u8]) -> std::io::Result<usize> {
        if !self.get_has_started_writing() {
            self.end_reading_begin_writing()?;
        }
        self.tcp_connection_context.write(b)
    }

    fn write_str(&self, b: &str) -> std::io::Result<usize> {
        self.write(b.as_bytes())
    }

    fn write_line(&self, b: &String) -> std::io::Result<usize> {
        if !self.get_has_started_writing() {
            self.end_reading_begin_writing()?;
        }
        self.tcp_connection_context.write_line(b)
    }

    fn begin_reading(&self) -> std::io::Result<()> {
        Ok(())
    }

    fn end_reading(self: &Self) -> std::io::Result<()> {
        Ok(())
    }

    fn end_reading_begin_writing(&self) -> std::io::Result<()> {
        self.end_reading()?;
        self.begin_writing()?;
        Ok(())
    }

    fn begin_writing(&self) -> std::io::Result<()> {
        if !self.get_has_started_writing() {
            self.has_started_writing.replace(true);
            
            // write the http version, status code, and status message
            let status_code = self.get_pending_status_code();

            // source_stream.borrow_mut().
            self.tcp_connection_context.write_line(&format!("HTTP/1.1 {} {}", status_code.as_str(), self.get_pending_status_message()))?;

            // write the headers
            for header in self.get_pending_headers().iter() {
                // write the header name
                self.tcp_connection_context.write(header.0.as_str().as_bytes())?;
                self.tcp_connection_context.write(b": ")?;
                // write the header value
                self.tcp_connection_context.write(&header.1.as_bytes())?;
                // write the header new line
                self.tcp_connection_context.write(b"\r\n")?;
            }
            // marker for end of headers and start of body
            self.tcp_connection_context.write(b"\r\n")?;

            self.flush()?;
        }

        Ok(())
    }

    fn end_writing(self: &Self) -> std::io::Result<()> {
        Ok(())
    }

    fn get_pending_status_code(&self) -> StatusCode {
        self.pending_status_code.borrow().clone().expect("status code not set")
    }

    fn get_pending_status_message(&self) -> String {
        match *self.pending_http_version.borrow() {
            http::Version::HTTP_10 | http::Version::HTTP_11 => self.pending_status_code.borrow().unwrap().canonical_reason().unwrap_or("").to_string(),
            _ => "".into()
        }
    }

    fn get_pending_headers(&self) -> HeaderMap {
        self.pending_headers.borrow().clone()
    }

    fn get_has_started_writing(&self) -> bool {
        *self.has_started_writing.borrow()
    }

    fn read_bytes(&self) -> std::io::Result<Vec<u8>> {
        let mut b = vec![0; 2048];
        self.read(&mut b)?;
        Ok(b)
    }

    fn read(&self, b: &mut [u8]) -> std::io::Result<usize> {
        self.tcp_connection_context.read(b)
    }

    fn read_line(&self) -> std::io::Result<String> {
        Ok(self.tcp_connection_context.read_line()?)
    }

    fn set_pending_status_code(&self, status_code: StatusCode) {
        self.pending_status_code.replace(Some(status_code));
    }

    fn set_pending_status_message(&self, status_message: String) {
        self.pending_status_message.replace(Some(status_message));
    }

    fn add_header_string(&self, name: String, value: String) {
        self.pending_headers.borrow_mut().insert(HeaderName::from_bytes(name.as_bytes()).unwrap(), HeaderValue::from_bytes(value.as_bytes()).unwrap());
    }

    fn add_header_str(&self, name: &str, value: &str) {
        self.pending_headers.borrow_mut().insert(HeaderName::from_bytes(name.as_bytes()).unwrap(), HeaderValue::from_bytes(value.as_bytes()).unwrap());
    }

    fn get_connection_id(&self) -> u32 {
        self.tcp_connection_context.get_connection_id()
    }

    fn add_stream_decoders(&self, decoders: &[Rc<dyn IHttpBodyStreamFormat>], content_type: &ContentType) {
        self.tcp_connection_context.add_stream_decoders(decoders, content_type);
    }
}