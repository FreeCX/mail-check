use std::path::PathBuf;

use clap::{Parser, Subcommand};

#[derive(Parser)]
pub struct Cli {
    /// Configuration file
    #[arg(short, long, default_value = "config.toml")]
    pub config: PathBuf,

    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Add credentials
    Add {
        #[arg(short, long)]
        login: String,
        #[arg(short, long)]
        domain: String,
        #[arg(short, long)]
        port: u16,
    },

    /// Update password
    Update {
        #[arg(short, long)]
        login: String,
    },

    /// Remove credentials
    Remove {
        #[arg(short, long)]
        login: String,
    },

    /// Show all accounts
    Show,
}
