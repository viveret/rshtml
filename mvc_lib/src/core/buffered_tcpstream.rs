use std::cell::RefCell;
use std::io::{Read, Write};
use std::net::TcpStream;

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
        match self.stream.borrow_mut().take_error() {
            Ok(Some(e)) => {
                println!("TcpStream::take_error: {:?}", e);
            },
            Ok(None) => {
                // println!("TcpStream::take_error: None");
            },
            Err(e) => {
                println!("TcpStream::take_error: {:?}", e);
            },
        }
        match self.stream.borrow_mut().shutdown(how) {
            Ok(()) => Ok(()),
            Err(e) => {
                if e.kind() == std::io::ErrorKind::NotConnected ||
                    e.kind() == std::io::ErrorKind::BrokenPipe || 
                    e.raw_os_error().unwrap_or_default() == 104 || 
                    e.raw_os_error().unwrap_or_default() == 32 {
                    return Ok(());
                }
                Err(std::io::Error::new(e.kind(), format!("TcpStream::shutdown: {:?}", e)))
            },
        }
    }

    // flush if writer is set, otherwise flush stream
    fn flush(&self) -> std::io::Result<()> {
        match self.stream.borrow_mut().flush() {
            Ok(()) => Ok(()),
            Err(e) => {
                Err(std::io::Error::new(e.kind(), format!("TcpStream::flush: {:?}", e)))
            },
        }
    }

    fn read(&self, b: &mut [u8]) -> std::io::Result<usize> {
        match self.stream.borrow_mut().read(b) {
            Ok(n) => Ok(n),
            Err(e) => {
                Err(std::io::Error::new(e.kind(), format!("TcpStream::read: {:?}", e)))
            },
        }
    }

    fn read_line(&self) -> std::io::Result<String> {
        // read until \r\n
        let mut s: Vec<char> = vec![];
        let mut last_char = '\0';
        loop {
            let mut buf = [0; 1];
            let read = match self.stream.borrow_mut().read(&mut buf) {
                Ok(n) => n,
                Err(e) => {
                    return Err(std::io::Error::new(e.kind(), format!("TcpStream::read_line: {:?}", e)));
                },
            };
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
            let num_written = match self.stream.borrow_mut().write(&b[i..]) {
                Ok(n) => n,
                Err(e) => {
                    return Err(std::io::Error::new(e.kind(), format!("TcpStream::write: {:?}", e)));
                },
            };
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
        match self.write(format!("{}\r\n", b).as_bytes()) {
            Ok(n) => Ok(n),
            Err(e) => {
                Err(std::io::Error::new(e.kind(), format!("TcpStream::write_line: {:?}", e)))
            },
        }
    }

    fn remote_addr(&self) -> std::net::SocketAddr {
        self.stream.borrow().peer_addr().unwrap()
    }
}
