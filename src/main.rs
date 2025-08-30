mod cli;
mod config;
mod provider;

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
        } => {
            ProviderManager::add_provider(&mut config, name, api_url, token)?;
        }

        Commands::Delete { name } => {
            ProviderManager::remove_provider(&mut config, &name)?;
        }

        Commands::Use { name, eval } => {
            if eval {
                ProviderManager::use_provider_eval(&mut config, &name)?;
            } else {
                ProviderManager::use_provider(&mut config, &name)?;
            }
        }

        Commands::Check => {
            ProviderManager::check_environment(&config)?;
        }

        Commands::Shellenv => {
            ProviderManager::output_shellenv()?;
        }

        Commands::Clear { eval } => {
            if eval {
                ProviderManager::clear_provider_eval(&mut config)?;
            } else {
                ProviderManager::clear_provider(&mut config)?;
            }
        }

        Commands::Install { force } => {
            ProviderManager::install_shell_integration(force)?;
        }
    }

    Ok(())
}
