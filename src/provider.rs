use crate::config::{Config, Provider};
use crate::constants::*;
use anyhow::Result;
use colored::*;

pub struct ProviderManager;

impl ProviderManager {
    pub fn list_providers(config: &Config) -> Result<()> {
        if config.providers.is_empty() {
            println!("{}", "No service providers configured".yellow());
            return Ok(());
        }

        println!("{}", "Configured service providers:".blue().bold());
        println!();

        for (name, provider) in &config.providers {
            let is_current = config.current_provider.as_ref() == Some(name);

            let marker = if is_current {
                "●".green()
            } else {
                "○".white()
            };
            let name_color = if is_current {
                name.green().bold()
            } else {
                name.white()
            };

            println!("  {} {}", marker, name_color);
            println!("    API URL: {}", provider.api_url.cyan());
            println!(
                "    Token: {}****",
                &provider.token[..provider.token.len().min(8)].dimmed()
            );
            if let Some(ref model) = provider.model {
                println!("    Model: {}", model.cyan());
            }

            if is_current {
                println!("    {}", "(currently active)".green().italic());
            }
            println!();
        }

        Ok(())
    }

    pub fn add_provider(
        config: &mut Config,
        name: String,
        api_url: String,
        token: String,
        model: Option<String>,
    ) -> Result<()> {
        if config.providers.contains_key(&name) {
            println!(
                "{} Service provider '{}' already exists, overwriting",
                "⚠️".yellow(),
                name.yellow()
            );
        }

        config.add_provider(name.clone(), api_url, token, model);
        config.save()?;

        println!(
            "{} Successfully added service provider '{}'",
            "✅".green(),
            name.green().bold()
        );
        Ok(())
    }

    pub fn remove_provider(config: &mut Config, name: &str) -> Result<()> {
        if !config.providers.contains_key(name) {
            println!(
                "{} Service provider '{}' does not exist",
                "❌".red(),
                name.red()
            );
            return Ok(());
        }

        config.remove_provider(name);
        config.save()?;

        println!(
            "{} Successfully removed service provider '{}'",
            "🗑️".green(),
            name.green().bold()
        );
        Ok(())
    }

    pub fn use_provider(config: &mut Config, name: &str) -> Result<()> {
        if !config.providers.contains_key(name) {
            println!(
                "{} Service provider '{}' does not exist",
                "❌".red(),
                name.red()
            );
            return Ok(());
        }

        let shell_mode = Self::shell_integration_active();

        if let Some(current) = &config.current_provider {
            if current == name && !shell_mode {
                println!(
                    "{} Already using service provider '{}'",
                    "ℹ️".blue(),
                    name.blue().bold()
                );
                return Ok(());
            }
        }

        let provider = config
            .providers
            .get(name)
            .expect("Provider should exist after check")
            .clone();

        // Set environment variables
        config.set_current_provider(name);
        config.save()?;

        set_provider_env_vars(&provider);

        if shell_mode {
            Self::emit_export_commands(&provider);
        } else {
            println!(
                "{} Switched to service provider '{}'",
                "🔄".green(),
                name.green().bold()
            );
            println!("  API URL: {}", provider.api_url.cyan());
        }

        Ok(())
    }

    pub fn check_environment(config: &Config) -> Result<()> {
        println!(
            "{}",
            "🔍 Checking environment variable status".blue().bold()
        );
        println!();

        // Check current environment variables
        let current_api_key = std::env::var(ENV_AUTH_TOKEN);
        let current_api_url = std::env::var(ENV_BASE_URL);

        println!("{}", "Current environment variables:".cyan().bold());
        match &current_api_key {
            Ok(key) => {
                let masked_key = if key.len() > 8 {
                    format!("{}****", &key[..8])
                } else {
                    "****".to_string()
                };
                println!("  {}: {}", ENV_AUTH_TOKEN, masked_key.green());
            }
            Err(_) => {
                println!("  {}: {}", ENV_AUTH_TOKEN, "Not set".red());
            }
        }

        match &current_api_url {
            Ok(url) => {
                println!("  {}: {}", ENV_BASE_URL, url.green());
            }
            Err(_) => {
                println!("  {}: {}", ENV_BASE_URL, "Not set".red());
            }
        }

        println!();

        // Check configuration status
        if let Some(current_provider) = &config.current_provider {
            if let Some(provider) = config.providers.get(current_provider) {
                println!("{}", "CCE configuration status:".cyan().bold());
                println!("  Current provider: {}", current_provider.green().bold());
                println!("  Configured URL: {}", provider.api_url.cyan());

                // Verify if environment variables match configuration
                let env_matches = match (&current_api_key, &current_api_url) {
                    (Ok(env_key), Ok(env_url)) => {
                        env_key == &provider.token && env_url == &provider.api_url
                    }
                    _ => false,
                };

                if env_matches {
                    println!(
                        "  Status: {}",
                        "✅ Environment variables match configuration".green()
                    );
                } else {
                    println!(
                        "  Status: {}",
                        "⚠️ Environment variables do not match configuration".yellow()
                    );
                    println!(
                        "  Suggestion: Run 'cce use {}' to reset",
                        current_provider.cyan()
                    );
                }
            } else {
                println!(
                    "{}",
                    "❌ Configuration error: Current provider does not exist".red()
                );
            }
        } else {
            println!("{}", "CCE configuration status:".cyan().bold());
            println!("  Current provider: {}", "None selected".yellow());
            if !config.providers.is_empty() {
                println!("  Suggestion: Use 'cce use <provider-name>' to select a provider");
            } else {
                println!("  Suggestion: Use 'cce add' to add a service provider");
            }
        }

        Ok(())
    }

    pub fn clear_provider(config: &mut Config) -> Result<()> {
        // Check if there's a current provider to clear
        if config.current_provider.is_none() {
            println!("{} No service provider is currently active", "ℹ️".blue());
            return Ok(());
        }

        let previous_provider = config.current_provider.clone();

        // Clear current provider in config
        config.clear_current_provider();
        config.save()?;

        let shell_mode = Self::shell_integration_active();

        if !shell_mode {
            if let Some(provider_name) = previous_provider {
                println!("{} Cleared service provider configuration", "🧹".green());
                println!(
                    "{} Removed '{}' as the active provider",
                    "✓".green(),
                    provider_name.yellow()
                );
            }
        }

        clear_all_env_vars();

        if !shell_mode {
            println!(
                "{}",
                "Environment variables cleared from current session".green()
            );
        }

        if shell_mode {
            Self::emit_unset_commands();
        }

        Ok(())
    }

    fn emit_export_commands(provider: &Provider) {
        println!("{}", generate_export_commands(provider));
    }

    fn emit_unset_commands() {
        // Output unset commands for shell
        println!("{}", generate_unset_commands());
    }

    fn shell_integration_active() -> bool {
        std::env::var(ENV_SHELL_INTEGRATION)
            .map(|v| v == "1")
            .unwrap_or(false)
    }

    pub fn install_shell_integration(force: bool) -> Result<()> {
        use std::fs::{File, OpenOptions};
        use std::io::{BufRead, BufReader, Write};

        // Detect shell type
        let shell = std::env::var("SHELL").unwrap_or_else(|_| "/bin/bash".to_string());
        let shell_name = shell.split('/').last().unwrap_or("bash");

        let (config_file, comment_prefix) = match shell_name {
            "zsh" => ("~/.zshrc", "#"),
            "bash" => ("~/.bashrc", "#"),
            "fish" => ("~/.config/fish/config.fish", "#"),
            _ => ("~/.bashrc", "#"),
        };

        // Expand tilde
        let config_path = if config_file.starts_with("~/") {
            let home = dirs::home_dir()
                .ok_or_else(|| anyhow::anyhow!("Could not determine home directory"))?;
            home.join(&config_file[2..])
        } else {
            std::path::PathBuf::from(config_file)
        };

        // Check if already installed
        let integration_line = r#"eval "$(cce shellenv)""#;
        let mut already_installed = false;

        if config_path.exists() {
            let file = File::open(&config_path)?;
            let reader = BufReader::new(file);

            for line in reader.lines() {
                let line = line?;
                let trimmed = line.trim();
                // Skip commented lines
                if !trimmed.starts_with('#') && trimmed == integration_line {
                    already_installed = true;
                    break;
                }
            }
        }

        if already_installed && !force {
            println!(
                "{} Shell integration is already installed in {}",
                "ℹ️".blue(),
                config_file.cyan()
            );
            println!(
                "{} Use {} to reinstall",
                "💡".blue(),
                "cce install --force".yellow()
            );
            return Ok(());
        }

        // Create config directory if it doesn't exist (for fish)
        if let Some(parent) = config_path.parent() {
            std::fs::create_dir_all(parent)?;
        }

        // Add shell integration
        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&config_path)?;

        let integration_block = format!(
            r#"
{} CCE Shell Integration
{}"#,
            comment_prefix, integration_line
        );

        writeln!(file, "{}", integration_block)?;

        println!("{} Shell integration installed successfully!", "✅".green());
        println!("📄 Added to: {}", config_path.display().to_string().cyan());
        println!();
        println!("{} To activate in current terminal:", "🔄".blue().bold());
        println!("   {}", format!("source {}", config_file).yellow());
        println!();
        println!(
            "{} Or restart your terminal for changes to take effect.",
            "🆕".blue().bold()
        );

        Ok(())
    }

    pub fn output_shellenv() -> Result<()> {
        // Get current executable path
        let current_exe =
            std::env::current_exe().unwrap_or_else(|_| std::path::PathBuf::from("cce"));
        let cce_path = current_exe.display();

        // Output complete shell function definition
        println!(
            r#"cce() {{
    local cce_binary="{}"
    
    if [[ "$1" == "use" && -n "$2" ]]; then
        local env_output
        env_output=$(CCE_SHELL_INTEGRATION=1 "$cce_binary" use "$2" 2>/dev/null)
        if [[ $? -eq 0 && -n "$env_output" ]]; then
            eval "$env_output"
            echo "⚡ Switched to service provider '$2'"
            echo "✅ Environment variables are now active in current terminal"
        else
            "$cce_binary" "$@"
        fi
    elif [[ "$1" == "clear" ]]; then
        local env_output
        env_output=$(CCE_SHELL_INTEGRATION=1 "$cce_binary" clear 2>/dev/null)
        if [[ $? -eq 0 && -n "$env_output" ]]; then
            eval "$env_output"
            echo "🧹 Cleared service provider configuration"
            echo "✅ Environment variables are now unset in current terminal"
        else
            "$cce_binary" "$@"
        fi
    else
        "$cce_binary" "$@"
    fi
}}"#,
            cce_path
        );

        Ok(())
    }
}
