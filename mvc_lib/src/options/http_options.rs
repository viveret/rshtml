use std::any::Any;
use std::borrow::Cow;
use std::rc::Rc;

pub trait IHttpOptions {
    fn get_ip(self: &Self) -> Cow<'static, str>;
    fn get_port(self: &Self) -> u16;
    fn get_port_https(self: &Self) -> u16;

    fn get_ip_and_port(self: &Self) -> String;
}

#[derive(Clone)]
pub struct HttpOptions {
    pub ip: Cow<'static, str>,
    pub port: u16,
    pub port_https: u16,
}

impl HttpOptions {
    pub fn new(ip: Option<Cow<'static, str>>, port: Option<u16>, port_https: Option<u16>) -> Self {
        Self {
            ip: ip.unwrap_or(Cow::Borrowed("127.0.0.1")),
            port: port.unwrap_or(80),
            port_https: port_https.unwrap_or(443)
        }
    }

    pub fn new_default() -> Self {
        Self::new(None, None, None)
    }

    pub fn new_service(ip: Option<Cow<'static, str>>, port: Option<u16>, port_https: Option<u16>) -> Box<dyn Any> {
        Box::new(Rc::new(Self::new(ip, port, port_https)) as Rc<dyn IHttpOptions>)
    }

    pub fn new_service_default() -> Box<dyn Any> {
        Self::new_service(None, None, None)
    }
}

impl IHttpOptions for HttpOptions {
    fn get_ip(self: &Self) -> Cow<'static, str> {
        self.ip.clone()
    }

    fn get_port(self: &Self) -> u16 {
        self.port
    }

    fn get_port_https(self: &Self) -> u16 {
        self.port_https
    }

    fn get_ip_and_port(self: &Self) -> String {
        format!("{}:{}", self.ip, self.port)
    }
}