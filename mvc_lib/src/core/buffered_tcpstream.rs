use std::io::{Read, Write};
use std::net::TcpStream;
use std::string::FromUtf8Error;

use super::itcp_stream_wrapper::ITcpStreamWrapper;


pub struct BufferedTcpStream {
    stream: TcpStream,
}

impl BufferedTcpStream {
    pub fn new(
        stream: TcpStream,
    ) -> Self {
        Self {
            stream: stream,
        }
    }

    pub fn new_from_tcp(stream: TcpStream) -> BufferedTcpStream {
        Self::new(stream)
    }
}

// pub struct TcpStreamRefCell {

// }

impl ITcpStreamWrapper for BufferedTcpStream {
    fn shutdown(&self, how: std::net::Shutdown) -> std::io::Result<()> {
        self.stream.shutdown(how)
    }

    // flush if writer is set, otherwise flush stream
    fn flush(&mut self) -> std::io::Result<()> {
        self.stream.flush()
    }

    fn read(&mut self, b: &mut [u8]) -> std::io::Result<usize> {
        self.stream.read(b)
    }

    fn read_line(&mut self) -> Result<String, FromUtf8Error> {
        // read until \r\n
        let mut s: Vec<char> = vec![];
        let mut last_char = '\0';
        loop {
            let mut buf = [0; 1];
            let read = self.stream.read(&mut buf).unwrap_or_default();
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

    fn write(&mut self, b: &[u8]) -> std::io::Result<usize> {
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
            let num_written = self.stream.write(&b[i..])?;
            if num_written == 0 {
                break;
            }
            i += num_written;
        }
        self.stream.flush()?;
        
        if i != b.len() {
            println!("write: {} != {}", i, b.len());
        }
        Ok(i)
    }

    fn write_line(&mut self, b: &String) -> std::io::Result<usize> {
        self.write(format!("{}\r\n", b).as_bytes())
    }

    fn remote_addr(&self) -> std::net::SocketAddr {
        self.stream.peer_addr().unwrap()
    }
}
