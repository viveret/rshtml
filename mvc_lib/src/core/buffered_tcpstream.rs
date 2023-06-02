use std::cell::RefCell;
use std::io::{Read, Write};
use std::net::TcpStream;
use std::string::FromUtf8Error;

use super::itcp_stream_wrapper::ITcpStreamWrapper;


pub struct BufferedTcpStream {
    stream: RefCell<TcpStream>,
}

impl BufferedTcpStream {
    pub fn new(
        stream: TcpStream,
    ) -> Self {
        Self {
            stream: RefCell::new(stream),
        }
    }

    pub fn new_from_tcp(stream: TcpStream) -> BufferedTcpStream {
        Self::new(stream)
    }

    pub fn new_self(source_stream: &RefCell<BufferedTcpStream>) -> BufferedTcpStream {
        Self::new(source_stream.borrow().stream.borrow().try_clone().unwrap())
    }
}

// pub struct TcpStreamRefCell {

// }

impl ITcpStreamWrapper for BufferedTcpStream {
    fn shutdown(&self, how: std::net::Shutdown) -> std::io::Result<()> {
        self.stream.borrow_mut().shutdown(how)
    }

    // flush if writer is set, otherwise flush stream
    fn flush(&self) -> std::io::Result<()> {
        self.stream.borrow_mut().flush()
    }

    fn read(&self, b: &mut [u8]) -> std::io::Result<usize> {
        self.stream.borrow_mut().read(b)
    }

    fn read_line(&self) -> Result<String, FromUtf8Error> {
        // read until \r\n
        let mut s: Vec<char> = vec![];
        let mut last_char = '\0';
        loop {
            let mut buf = [0; 1];
            let read = self.stream.borrow_mut().read(&mut buf).unwrap_or_default();
            let c = buf[0] as char;
            if read == 0 {
                break;
            } else if c == '\n' && last_char == '\r' {
                s.pop();
                break;
            } else {
                s.push(c);
                last_char = c;
            }
        }
        
        let debug = false;
        if debug {
            let s = s.into_iter().collect();
            println!("read_line: {:?}", s);
            Ok(s)
        } else {
            Ok(s.into_iter().collect())
        }
    }

    fn write(&self, b: &[u8]) -> std::io::Result<usize> {
        // // preview of output
        // if b.len() > 20 {
        //     println!("writing {} bytes: {} ... {}", b.len(),
        //                 String::from_utf8(b[..10].to_vec()).unwrap(),
        //                 String::from_utf8(b[b.len()-10..].to_vec()).unwrap());
        // } else {
        //     println!("writing {} bytes: {}", b.len(), String::from_utf8(b.to_vec()).unwrap());
        // }
        
        let mut i = 0;
        while i < b.len() {
            let num_written = self.stream.borrow_mut().write(&b[i..])?;
            if num_written == 0 {
                break;
            }
            i += num_written;
        }
        self.flush()?;
        
        if i != b.len() {
            println!("write: {} != {}", i, b.len());
        }
        Ok(i)
    }

    fn write_line(&self, b: &String) -> std::io::Result<usize> {
        self.write(format!("{}\r\n", b).as_bytes())
    }

    fn remote_addr(&self) -> std::net::SocketAddr {
        self.stream.borrow().peer_addr().unwrap()
    }
}
