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
    let to_string = |item: Vec<u8>| String::from_utf8(item).unwrap_or("<not valid utf8>".to_string());

    notify_base(msg).action(consts::ACTION_NAME, action_name).show()?.wait_for_action(|action| {
        if action == consts::ACTION_NAME {
            println!("run action app: {app}");
            match Command::new(app).output() {
                Ok(output) => {
                    println!(
                        "status:\n---\n{}\n---\nstdout:\n---\n{}\n---\nstderr:\n---\n{}\n---",
                        output.status,
                        to_string(output.stdout),
                        to_string(output.stderr)
                    );
                }
                Err(error) => println!("failed:\n---\n{error}\n---"),
            }
        }
    });
    Ok(())
}

pub fn message(msg: &str) -> anyhow::Result<NotificationHandle> {
    Ok(notify_base(msg).show()?)
}
