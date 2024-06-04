use std::os::unix::process::CommandExt;
use std::process::Command;

use notify_rust::{Notification, NotificationHandle};

use crate::consts;

fn notify_base(msg: &str) -> Notification {
    Notification::new()
        .summary(consts::APP_TITLE)
        .body(msg)
        .icon(consts::APP_ICON)
        .appname(consts::APP_NAME)
        .timeout(0)
        .finalize()
}

pub fn message_with_action(msg: &str, app: &str, action_name: &str) -> anyhow::Result<()> {
    notify_base(msg).action(consts::ACTION_NAME, action_name).show()?.wait_for_action(|action| {
        if action == consts::ACTION_NAME {
            let _ = Command::new(app).exec();
        }
    });
    Ok(())
}

pub fn message(msg: &str) -> anyhow::Result<NotificationHandle> {
    Ok(notify_base(msg).show()?)
}
