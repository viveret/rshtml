

// this trait represents a HTTP connection.
pub trait IConnectionContext {
    // returns a string representation of the connection context.
    fn to_string(self: &Self) -> String;
    // get the remote address of the connection.
    fn get_remote_addr(self: &Self) -> std::net::SocketAddr;
}

// this struct implements IConnectionContext.
pub struct ConnectionContext {
    // the remote address of the connection.
    remote_addr: std::net::SocketAddr,
}

impl ConnectionContext {
    // create a new ConnectionContext struct from a remote address.
    // remote_addr: the remote address of the connection.
    // returns: a new ConnectionContext struct.
    pub fn new(
        remote_addr: std::net::SocketAddr,
    ) -> Self {
        Self {
            remote_addr: remote_addr,
        }
    }
}

impl IConnectionContext for ConnectionContext {
    fn to_string(self: &Self) -> String {
        format!("{:?}", self.remote_addr)
    }

    fn get_remote_addr(self: &Self) -> std::net::SocketAddr {
        self.remote_addr.clone()
    }
}