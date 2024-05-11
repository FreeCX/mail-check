use clap::Parser;
use notify_rust::Notification;

use manager::Manager;

mod cli;
mod consts;
mod mail;
mod manager;

fn default(manager: Manager) {
    let mut message = String::new();
    for (login, count) in manager.check() {
        if count > 0 {
            message.push_str(&format!("{login}: {count}\n"));
        }
    }

    if !message.is_empty() {
        Notification::new()
            .summary("Mail Check")
            .body(&format!("You have unreaded messages!\n{message}"))
            .icon("mail-message-new")
            .appname(consts::APPNAME)
            .timeout(0)
            .show()
            .unwrap();
    }
}

fn main() {
    let args = cli::Cli::parse();
    let mut manager = manager::Manager::load(&args.config);

    match &args.command {
        Some(cli::Commands::Add { login, domain, port }) => {
            manager.add_account(login, domain, *port);
            manager.save(&args.config);
        }
        Some(cli::Commands::Remove { login }) => {
            manager.remove_account(login);
            manager.save(&args.config);
        }
        None => default(manager),
    }
}
