use std::{
    net::{TcpStream, ToSocketAddrs},
    time::Duration,
};

use crate::consts;

pub fn is_online(timeout: Duration) -> bool {
    let addr = match consts::DETECT_PORTAL.to_socket_addrs() {
        Ok(value) => value.into_iter().next().unwrap(),
        Err(_) => return false,
    };
    match TcpStream::connect_timeout(&addr, timeout) {
        Ok(_) => true,
        Err(_) => false,
    }
}
