use std::cell::RefCell;
use std::rc::Rc;

use crate::core::buffered_tcpstream::BufferedTcpStream;
use crate::core::itcp_stream_wrapper::ITcpStreamWrapper;
use crate::http::http_body_content::ContentType;
use crate::http::ihttp_body_stream_format::IHttpBodyStreamFormat;

use super::itcpconnection_context::ITcpConnectionContext;


// this struct implements ITcpConnectionContext and represents a TCP connection.
pub struct TcpConnectionContext {
    // source_stream: Rc<RefCell<dyn ITcpStreamWrapper>>,
    stream: RefCell<Rc<RefCell<dyn ITcpStreamWrapper>>>,
    connection_id: u32,
}

impl TcpConnectionContext {
    // create a new instance from a remote address.
    // remote_addr: the remote address of the connection.
    // returns: a new instance.
    pub fn new(
        source_stream: Rc<RefCell<dyn ITcpStreamWrapper>>,
        connection_id: u32
    ) -> Self {
        Self {
            // source_stream: source_stream.clone(),
            connection_id: connection_id,
            stream: RefCell::new(source_stream),
        }
    }

    pub fn new_from_stream(stream: std::net::TcpStream, connection_id: u32) -> Self {
        Self::new(Rc::new(RefCell::new(BufferedTcpStream::new_from_tcp(stream))), connection_id)
    }
}

impl ITcpConnectionContext for TcpConnectionContext {
    fn to_string(self: &Self) -> String {
        format!("{:?}", self.get_remote_addr())
    }

    fn get_remote_addr(self: &Self) -> std::net::SocketAddr {
        self.stream.borrow().borrow().remote_addr().clone()
    }

    fn get_connection_id(&self) -> u32 {
        self.connection_id
    }

    fn shutdown(&self, how: std::net::Shutdown) -> std::io::Result<()> {
        self.stream.borrow().borrow_mut().shutdown(how)
    }

    fn flush(&self) -> std::io::Result<()> {
        self.stream.borrow().borrow_mut().flush()
    }

    fn write(&self, b: &[u8]) -> std::io::Result<usize> {
        self.stream.borrow().borrow_mut().write(b)
    }

    fn write_line(&self, b: &String) -> std::io::Result<usize> {
        self.stream.borrow().borrow_mut().write_line(b)
    }

    fn read(&self, b: &mut [u8]) -> std::io::Result<usize> {
        self.stream.borrow().borrow_mut().read(b)
    }

    fn read_line(&self) -> std::io::Result<String> {
        self.stream.borrow().borrow_mut().read_line()
    }

    fn add_stream_decoders(&self, decoders: &[Rc<dyn IHttpBodyStreamFormat>], content_type: &ContentType) {
        for decoder in decoders.iter() {
            let decoded_stream = decoder.decode(self.stream.borrow().clone(), content_type);
            self.stream.replace(decoded_stream);
        }
    }
}
