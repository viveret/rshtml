use std::cell::RefCell;
use std::rc::Rc;
use std::vec;

use http::{HeaderName, HeaderValue, HeaderMap};
use http::status::StatusCode;

use crate::core::buffered_tcpstream::BufferedTcpStream;
use crate::core::itcp_stream_wrapper::ITcpStreamWrapper;
use crate::http::ihttp_body_stream_format::IHttpBodyStreamFormat;


// this trait represents a TCP connection.
pub trait IConnectionContext {
    // returns a string representation of the connection context.
    fn to_string(self: &Self) -> String;

    // get the remote address of the connection.
    fn get_remote_addr(self: &Self) -> std::net::SocketAddr;

    // // get the body stream of the connection.
    // fn mut_body_stream(self: &Self) -> &RefCell<Rc<dyn ITcpStreamWrapper>>;
}

// this struct implements IConnectionContext and represents a HTTP connection.
pub struct ConnectionContext {
    source_stream: RefCell<BufferedTcpStream>,
}

impl ConnectionContext {
    // create a new ConnectionContext struct from a remote address.
    // remote_addr: the remote address of the connection.
    // returns: a new ConnectionContext struct.
    pub fn new(
        source_stream: RefCell<BufferedTcpStream>,
    ) -> Self {
        Self {
            source_stream: source_stream,
        }
    }

    pub fn new_from_stream(stream: BufferedTcpStream) -> Self {
        let buf_reader = RefCell::new(stream);

        Self::new(buf_reader)
    }

    fn shutdown(&self, how: std::net::Shutdown) -> Result<(), std::io::Error> {
        self.source_stream.borrow().shutdown(how)
    }

    fn flush(&self) -> Result<(), std::io::Error> {
        self.source_stream.borrow_mut().flush()
    }

    fn write(&self, b: &[u8]) -> Result<usize, std::io::Error> {
        self.source_stream.borrow_mut().write(b)
    }

    fn write_line(&self, b: &String) -> Result<usize, std::io::Error> {
        self.source_stream.borrow_mut().write_line(b)
    }

    fn read(&self, b: &mut [u8]) -> Result<usize, std::io::Error> {
        self.source_stream.borrow_mut().read(b)
    }

    fn read_line(&self) -> Result<String, std::string::FromUtf8Error> {
        self.source_stream.borrow_mut().read_line()
    }
}

impl IConnectionContext for ConnectionContext {
    fn to_string(self: &Self) -> String {
        format!("{:?}", self.get_remote_addr())
    }

    fn get_remote_addr(self: &Self) -> std::net::SocketAddr {
        self.source_stream.borrow().remote_addr().clone()
    }
}


pub trait IHttpConnectionContext {
    fn get_tcp_context(&self) -> &dyn IConnectionContext;
    fn get_stream(&self) -> &RefCell<Rc<dyn ITcpStreamWrapper>>;

    fn get_pending_status_code(&self) -> StatusCode;
    fn get_pending_status_message(&self) -> String;
    fn get_pending_headers(&self) -> HeaderMap;
    fn get_has_started_writing(&self) -> bool;

    fn set_pending_status_code(&self, status_code: StatusCode);
    fn set_pending_status_message(&self, status_message: String);

    fn add_header_string(&self, name: String, value: String);
    fn add_header_str(&self, name: &str, value: &str);

    fn begin_reading(&self);
    fn end_reading(self: &Self);
    fn end_reading_begin_writing(&self);
    fn begin_writing(&self);
    fn end_writing(self: &Self);

    fn write(&self, b: &[u8]) -> std::io::Result<usize>;
    fn write_str(&self, b: &str) -> std::io::Result<usize>;
    fn write_line(&self, b: &String) -> std::io::Result<usize>;
    fn flush(&self) -> std::io::Result<()>;

    fn read(&self) -> std::io::Result<Vec<u8>>;
    fn read_line(&self) -> std::io::Result<String>;
}

pub struct HttpConnectionContext {
    pub tcp_connection_context: ConnectionContext,
    pub stream: RefCell<Rc<dyn ITcpStreamWrapper>>,
    pub has_started_writing: RefCell<bool>,

    pub pending_http_version: RefCell<http::Version>,
    pub pending_status_code: RefCell<Option<StatusCode>>,
    pub pending_status_message: RefCell<Option<String>>,
    pub pending_headers: RefCell<HeaderMap>,
}

impl HttpConnectionContext {
    pub fn new(connection_context: ConnectionContext) -> Self {
        let stream = RefCell::new(Rc::new(BufferedTcpStream::new_self(&connection_context.source_stream))); // connection_context.source_stream.borrow()
        Self {
            tcp_connection_context: connection_context,
            has_started_writing: RefCell::new(false),
            pending_http_version: RefCell::new(http::Version::HTTP_11),
            pending_status_code: RefCell::new(None),
            pending_status_message: RefCell::new(None),
            pending_headers: RefCell::new(HeaderMap::new()),
            stream: stream
        }
    }

    pub fn new_from_stream(stream: std::net::TcpStream) -> HttpConnectionContext {
        Self::new(ConnectionContext::new_from_stream(BufferedTcpStream::new_from_tcp(stream)))
    }

    pub fn shutdown(&self, how: std::net::Shutdown) -> std::io::Result<()> {
        self.tcp_connection_context.shutdown(how)
    }
}

impl IHttpConnectionContext for HttpConnectionContext {
    fn get_tcp_context(&self) -> &dyn IConnectionContext {
        &self.tcp_connection_context
    }

    fn get_stream(&self) -> &RefCell<Rc<dyn ITcpStreamWrapper>> {
        &self.stream
    }

    fn flush(&self) -> std::io::Result<()> {
        self.tcp_connection_context.flush()
    }

    fn write(&self, b: &[u8]) -> std::io::Result<usize> {
        if !self.get_has_started_writing() {
            self.begin_writing();
        }
        self.tcp_connection_context.write(b)
    }

    fn write_str(&self, b: &str) -> std::io::Result<usize> {
        self.write(b.as_bytes())
    }

    fn write_line(&self, b: &String) -> std::io::Result<usize> {
        if !self.get_has_started_writing() {
            self.begin_writing();
        }
        self.tcp_connection_context.write_line(b)
    }

    fn begin_reading(&self) {
        // todo!()
    }

    fn end_reading(self: &Self) {
        // todo!()
    }

    fn end_reading_begin_writing(&self) {
        self.end_reading();
        self.begin_writing();
    }

    fn begin_writing(&self) {
        self.has_started_writing.replace(true);
        
        // write the http version, status code, and status message
        let status_code = self.get_pending_status_code();

        self.tcp_connection_context.source_stream.borrow_mut().write_line(&format!("HTTP/1.1 {} {}", status_code.as_str(), self.get_pending_status_message())).unwrap();

        // write the headers
        for header in self.get_pending_headers().iter() {
            // write the header name
            self.tcp_connection_context.source_stream.borrow_mut().write(header.0.as_str().as_bytes()).unwrap();
            self.tcp_connection_context.source_stream.borrow_mut().write(b": ").unwrap();
            // write the header value
            self.tcp_connection_context.source_stream.borrow_mut().write(&header.1.as_bytes()).unwrap();
            // write the header new line
            self.tcp_connection_context.source_stream.borrow_mut().write(b"\r\n").unwrap();
        }
        // marker for end of headers and start of body
        self.tcp_connection_context.source_stream.borrow_mut().write(b"\r\n").unwrap();
    }

    fn end_writing(self: &Self) {
        // todo!()
    }

    fn get_pending_status_code(&self) -> StatusCode {
        self.pending_status_code.borrow().clone().expect("status code not set")
    }

    fn get_pending_status_message(&self) -> String {
        match *self.pending_http_version.borrow() {
            http::Version::HTTP_10 | http::Version::HTTP_11 => self.pending_status_code.borrow().unwrap().canonical_reason().unwrap_or("").to_string(),
            _ => "".into()
        }
        // self.pending_status_message.borrow().clone().expect("status message not set")
    }

    fn get_pending_headers(&self) -> HeaderMap {
        self.pending_headers.borrow().clone()
    }

    fn get_has_started_writing(&self) -> bool {
        *self.has_started_writing.borrow()
    }

    fn read(&self) -> std::io::Result<Vec<u8>> {
        let mut b = vec![0; 2048];
        let num_read = self.tcp_connection_context.read(&mut b)?;
        Ok(b[..num_read].to_vec())
    }

    fn read_line(&self) -> std::io::Result<String> {
        Ok(self.tcp_connection_context.read_line().unwrap())
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
}