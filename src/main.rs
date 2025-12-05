mod cli;
mod config;
mod constants;
mod provider;
mod tui;

use anyhow::Result;
use cli::{Cli, Commands};
use config::Config;
use provider::ProviderManager;

fn main() -> Result<()> {
    let cli = Cli::parse_args();
    let mut config = Config::load()?;

    match cli.command {
        Commands::List => {
            ProviderManager::list_providers(&config)?;
        }

        Commands::Add {
            name,
            api_url,
            token,
            model,
        } => {
            ProviderManager::add_provider(&mut config, name, api_url, token, model)?;
        }

        Commands::Delete { name } => {
            ProviderManager::remove_provider(&mut config, &name)?;
        }

        Commands::Use { name } => {
            ProviderManager::use_provider(&mut config, &name)?;
        }

        Commands::Check => {
            ProviderManager::check_environment(&config)?;
        }

        Commands::Shellenv => {
            ProviderManager::output_shellenv()?;
        }

        Commands::Clear => {
            ProviderManager::clear_provider(&mut config)?;
        }

        Commands::Install { force } => {
            ProviderManager::install_shell_integration(force)?;
        }

        Commands::Tui => {
            tui::run_tui(config)?;
        }
    }

    Ok(())
}
