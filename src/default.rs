use std::{
    net::{TcpStream, ToSocketAddrs},
    thread,
    time::{self, Duration},
};

use notify_rust::Notification;

use crate::consts;
use crate::manager::Manager;

fn is_online(timeout: Duration) -> bool {
    let addr = match "detectportal.firefox.com:80".to_socket_addrs() {
        Ok(value) => value.into_iter().next().unwrap(),
        Err(_) => return false,
    };
    match TcpStream::connect_timeout(&addr, timeout) {
        Ok(_) => true,
        Err(_) => false,
    }
}

fn notify_message(msg: &str) {
    Notification::new()
        .summary("Mail Check")
        .body(msg)
        .icon("mail-message-new")
        .appname(consts::APPNAME)
        .timeout(0)
        .show()
        .unwrap();
}

pub fn default(manager: Manager) {
    let tcp_timeout = time::Duration::from_secs(manager.config.tcp_timeout_secs);
    let online_wait = time::Duration::from_secs(manager.config.online_wait_secs);
    let mut no_connection = true;

    println!("check internet connection");
    for _ in 0..manager.config.retry_count {
        if is_online(tcp_timeout) {
            println!("online");
            no_connection = false;
            break;
        }
        println!("wait and try again...");
        thread::sleep(online_wait);
    }

    if no_connection {
        println!("no connection");
        notify_message("No internet :(");
        return;
    }

    let total_unread = manager.check();
    if total_unread > 0 {
        notify_message(&format!("You have {total_unread} unreaded messages!"));
    }
}
