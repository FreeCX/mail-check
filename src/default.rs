use std::thread::sleep;
use std::time::Duration;

use crate::manager::Manager;
use crate::notify;
use crate::online;

pub fn default(manager: Manager) -> anyhow::Result<()> {
    let tcp_timeout = Duration::from_secs(manager.config.tcp_timeout_secs);
    let online_wait = Duration::from_secs(manager.config.online_wait_secs);
    let mut no_connection = true;

    println!("check internet connection");
    for _ in 0..manager.config.retry_count {
        if online::is_online(tcp_timeout) {
            println!("online");
            no_connection = false;
            break;
        }
        println!("wait and try again...");
        sleep(online_wait);
    }

    if no_connection {
        println!("no connection");
        if manager.config.show_no_internet_msg {
            notify::message("No internet :(")?;
        }
        return Ok(());
    }

    let total_unread = manager.check()?;
    if total_unread > 0 {
        let message = format!("You have {total_unread} unreaded messages!");
        if let Some(app) = manager.config.action_run {
            notify::message_with_action(&message, &app, &manager.config.action_name)?;
        } else {
            notify::message(&message)?;
        }
    }

    Ok(())
}
