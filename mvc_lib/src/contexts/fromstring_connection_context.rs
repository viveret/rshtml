use std::cell::RefCell;
use std::rc::Rc;

use crate::http::http_body_content::ContentType;
use crate::http::ihttp_body_stream_format::IHttpBodyStreamFormat;

use super::itcpconnection_context::ITcpConnectionContext;


// this is a mock implementation of ITcpConnectionContext that reads from a string and writes to an in-memory buffer.
pub struct FromStringConnectionContext {
    data: String,
    connection_id: u32,
    is_shutdown: RefCell<Option<std::net::Shutdown>>,
    input_position: RefCell<usize>,
    output_buffer: RefCell<Vec<u8>>,
}

impl FromStringConnectionContext {
    pub fn new(data: String, connection_id: u32) -> Self {
        Self {
            data: data,
            connection_id: connection_id,
            is_shutdown: RefCell::new(None),
            input_position: RefCell::new(0),
            output_buffer: RefCell::new(Vec::new()),
        }
    }
}

impl ITcpConnectionContext for FromStringConnectionContext {
    fn to_string(self: &Self) -> String {
        format!("FromStringConnectionContext ({}): {}", self.connection_id, self.data)
    }

    fn get_remote_addr(self: &Self) -> std::net::SocketAddr {
        unimplemented!()
    }

    fn get_connection_id(&self) -> u32 {
        self.connection_id
    }

    fn shutdown(&self, how: std::net::Shutdown) -> std::io::Result<()> {
        if self.is_shutdown.borrow().is_some() {
            Err(std::io::Error::new(std::io::ErrorKind::NotConnected, "Already shutdown"))
        } else {
            self.is_shutdown.replace(Some(how));
            Ok(())
        }
    }

    fn flush(&self) -> std::io::Result<()> {
        // nothing to do
        Ok(())
    }

    fn write(&self,b: &[u8]) -> std::io::Result<usize> {
        self.output_buffer.borrow_mut().extend_from_slice(b);
        Ok(b.len())
    }

    fn write_line(&self,b: &String) -> std::io::Result<usize> {
        self.output_buffer.borrow_mut().extend_from_slice(b.as_bytes());
        Ok(b.len())
    }

    fn read(&self,b: &mut[u8]) -> std::io::Result<usize> {
        b.copy_from_slice(self.data.as_bytes());
        Ok(self.data.len())
    }

    fn read_line(&self) -> std::io::Result<String> {
        let pos = *self.input_position.borrow();
        if pos < self.data.len() {
            let line: String = self.data.as_str().chars().skip(pos).take_while(|c| *c != '\r' && *c != '\n').collect();
            let mut num_read = line.len();
            
            if self.data.chars().nth(pos + num_read) == Some('\r') {
                num_read += 1;
            }
            if self.data.chars().nth(pos + num_read) == Some('\n') {
                num_read += 1;
            }

            self.input_position.replace(pos + num_read);
            Ok(line)
        } else {
            Err(std::io::Error::new(std::io::ErrorKind::UnexpectedEof, "End of stream"))
        }
    }

    fn add_stream_decoders(&self, _decoders: &[Rc<dyn IHttpBodyStreamFormat>], _content_type: &ContentType) {
        todo!()
    }
}