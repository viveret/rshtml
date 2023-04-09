use std::any::Any;
use std::cell::RefCell;
use std::collections::HashMap;
use std::error::Error;
use std::rc::Rc;

use crate::action_results::iaction_result::IActionResult;

use crate::contexts::request_context::RequestContext;

use crate::controllers::icontroller::IController;

use crate::routing::route_data::RouteData;


pub trait IConnectionContext {
    fn to_string(self: &Self) -> String;
    fn get_remote_addr(self: &Self) -> std::net::SocketAddr;
}

pub struct ConnectionContext {
    remote_addr: std::net::SocketAddr,
}

impl ConnectionContext {
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