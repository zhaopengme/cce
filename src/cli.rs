use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(
    name = "cce",
    about = "Claude Config Environment - A tool for switching Claude environment variables",
    version = "0.2.0"
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// List all service providers
    List,

    /// Add a service provider
    Add {
        /// Provider name
        name: String,
        /// API URL
        api_url: String,
        /// API Token
        token: String,
    },

    /// Delete the specified service provider
    Delete {
        /// Name of provider to delete
        name: String,
    },

    /// Use the specified service provider
    Use {
        /// Name of provider to use
        name: String,
        /// Only output environment variable commands, no other info
        #[arg(long)]
        eval: bool,
    },

    /// Check current environment variable status
    Check,

    /// Output shell integration function
    Shellenv,

    /// Clear environment variables to use official Claude client
    Clear {
        /// Only output unset commands, no other info
        #[arg(long)]
        eval: bool,
    },

    /// Install shell integration for immediate environment variable effects
    Install {
        /// Force reinstall even if already installed
        #[arg(long)]
        force: bool,
    },
}

impl Cli {
    pub fn parse_args() -> Self {
        Self::parse()
    }
}
