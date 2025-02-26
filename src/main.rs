use std::process::ExitCode;

use clap::Parser;

mod cli;
mod consts;
mod default;
mod mail;
mod manager;
mod notify;
mod online;

fn app(args: cli::Cli) -> anyhow::Result<()> {
    let mut manager = manager::Manager::load(&args.config)?;
    match &args.command {
        Some(cli::Commands::Add { login, domain, port }) => {
            manager.add_account(login, domain, *port)?;
            manager.save(&args.config)
        }
        Some(cli::Commands::Show) => manager.print_accounts(),
        Some(cli::Commands::Update { login }) => {
            manager.update_password(login)?;
            manager.save(&args.config)
        }
        Some(cli::Commands::Remove { login }) => {
            manager.remove_account(login)?;
            manager.save(&args.config)
        }
        // checking the mail by default
        Some(cli::Commands::Check) | None => default::default(manager),
    }
}

fn main() -> ExitCode {
    match app(cli::Cli::parse()) {
        Err(error) => {
            eprintln!("error: {}, reason: {}", error, error.root_cause());
            let _ = notify::message(&error.to_string());
            ExitCode::FAILURE
        }
        Ok(_) => ExitCode::SUCCESS,
    }
}
