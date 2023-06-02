
use std::string::FromUtf8Error;

pub trait ITcpStreamWrapper {
    fn shutdown(&self, how: std::net::Shutdown) -> std::io::Result<()>;
    fn flush(&self) -> std::io::Result<()>;
    fn read(&self, b: &mut [u8]) -> std::io::Result<usize>;
    fn read_line(&self) -> Result<String, FromUtf8Error>;
    fn write(&self, b: &[u8]) -> std::io::Result<usize>;
    fn write_line(&self, b: &String) -> std::io::Result<usize>;
    fn remote_addr(&self) -> std::net::SocketAddr;


    // fn shutdown(&self, how: std::net::Shutdown) -> std::io::Result<()>;
    // fn flush(&self) -> std::io::Result<()>;
    // fn read(&self, b: &mut [u8]) -> std::io::Result<usize>;
    // fn read_line(&self) -> Result<String, FromUtf8Error>;
    // fn write(&self, b: &[u8]) -> std::io::Result<usize>;
    // fn write_line(&self, b: &String) -> std::io::Result<usize>;
    // fn remote_addr(&self) -> std::net::SocketAddr;
}