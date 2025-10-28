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
    #[command(alias = "ls")]
    /// List all service providers (alias)
    Ls,

    /// Add a service provider
    Add {
        /// Provider name
        name: String,
        /// API URL
        api_url: String,
        /// API Token
        token: String,
        /// Model name (optional)
        #[arg(short, long)]
        model: Option<String>,
    },

    /// Delete the specified service provider
    Delete {
        /// Name of provider to delete
        name: String,
    },
    #[command(alias = "del")]
    /// Delete the specified service provider (alias)
    Del {
        /// Name of provider to delete
        name: String,
    },

    /// Use the specified service provider
    Use {
        /// Name of provider to use
        name: String,
    },

    /// Check current environment variable status
    Check,

    /// Output shell integration function
    Shellenv,

    /// Clear environment variables to use official Claude client
    Clear,

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
