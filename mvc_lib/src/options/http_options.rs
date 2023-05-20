use std::any::Any;
use std::borrow::Cow;
use std::rc::Rc;

// this trait is used to get the http serving options.
pub trait IHttpOptions {
    // get the ip address to serve on.
    fn get_ip(self: &Self) -> Cow<'static, str>;
    // get the port to serve on.
    fn get_port(self: &Self) -> u16;
    // get the port to serve https on.
    fn get_port_https(self: &Self) -> u16;
    // get the ip address and port to serve on.
    fn get_ip_and_port(self: &Self) -> String;
}

// this struct implements IHttpOptions.
#[derive(Clone)]
pub struct HttpOptions {
    pub ip: Cow<'static, str>,
    pub port: u16,
    pub port_https: u16,
}

impl HttpOptions {
    // create a new HttpOptions struct from an ip address, port, and port for https.
    // if any of the parameters are None, the default value will be used.
    // the default values are:
    // ip: "127.0.0.1"
    // port: 80
    // port_https: 443
    // ip: the ip address to serve on.
    // port: the port to serve on.
    // port_https: the port to serve https on.
    // returns: a new HttpOptions struct.
    pub fn new(ip: Option<Cow<'static, str>>, port: Option<u16>, port_https: Option<u16>) -> Self {
        Self {
            ip: ip.unwrap_or(Cow::Borrowed("127.0.0.1")),
            port: port.unwrap_or(80),
            port_https: port_https.unwrap_or(443)
        }
    }

    // create a new HttpOptions struct with default values.
    pub fn new_default() -> Self {
        Self::new(None, None, None)
    }

    // create a new HttpOptions struct as a service from an ip address, port, and port for https.
    // if any of the parameters are None, the default value will be used.
    // the default values are:
    // ip: "127.0.0.1"
    // port: 80
    // port_https: 443
    // ip: the ip address to serve on.
    // port: the port to serve on.
    // port_https: the port to serve https on.
    // returns: a new HttpOptions struct as a service.
    pub fn new_service(ip: Option<Cow<'static, str>>, port: Option<u16>, port_https: Option<u16>) -> Box<dyn Any> {
        Box::new(Rc::new(Self::new(ip, port, port_https)) as Rc<dyn IHttpOptions>)
    }

    // create a new HttpOptions struct as a service with default values.
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