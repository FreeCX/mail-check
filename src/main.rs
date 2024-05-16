use clap::Parser;

mod cli;
mod consts;
mod default;
mod mail;
mod manager;

fn main() -> anyhow::Result<()> {
    let args = cli::Cli::parse();
    let mut manager = manager::Manager::load(&args.config)?;

    match &args.command {
        Some(cli::Commands::Add { login, domain, port }) => {
            manager.add_account(login, domain, *port)?;
            manager.save(&args.config)
        }
        Some(cli::Commands::Remove { login }) => {
            manager.remove_account(login)?;
            manager.save(&args.config)
        }
        None => default::default(manager),
    }
}
