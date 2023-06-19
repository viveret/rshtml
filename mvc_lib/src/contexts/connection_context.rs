use std::cell::RefCell;
use std::rc::Rc;

use crate::core::buffered_tcpstream::BufferedTcpStream;
use crate::core::itcp_stream_wrapper::ITcpStreamWrapper;
use crate::http::http_body_content::ContentType;
use crate::http::ihttp_body_stream_format::IHttpBodyStreamFormat;

use super::iconnection_context::IConnectionContext;


// this struct implements IConnectionContext and represents a TCP connection.
pub struct ConnectionContext {
    source_stream: BufferedTcpStream,
    stream: RefCell<Rc<RefCell<dyn ITcpStreamWrapper>>>,
    connection_id: u32,
}

impl ConnectionContext {
    // create a new ConnectionContext struct from a remote address.
    // remote_addr: the remote address of the connection.
    // returns: a new ConnectionContext struct.
    pub fn new(
        source_stream: BufferedTcpStream,
        connection_id: u32
    ) -> Self {
        Self {
            source_stream: source_stream.clone(),
            connection_id: connection_id,
            stream: RefCell::new(Rc::new(RefCell::new(source_stream))),
        }
    }

    pub fn new_from_stream(stream: BufferedTcpStream, connection_id: u32) -> Self {
        Self::new(stream, connection_id)
    }
}

impl IConnectionContext for ConnectionContext {
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
