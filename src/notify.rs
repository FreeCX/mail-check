use notify_rust::{Notification, NotificationHandle};

use crate::consts;

pub fn message(msg: &str) -> anyhow::Result<NotificationHandle> {
    Ok(Notification::new()
        .summary(consts::APP_TITLE)
        .body(msg)
        .icon(consts::APP_ICON)
        .appname(consts::APP_NAME)
        .timeout(0)
        .show()?)
}
