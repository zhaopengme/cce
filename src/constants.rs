/// Environment variable names used by CCE
pub const ENV_AUTH_TOKEN: &str = "ANTHROPIC_AUTH_TOKEN";
pub const ENV_BASE_URL: &str = "ANTHROPIC_BASE_URL";
pub const ENV_MODEL: &str = "ANTHROPIC_MODEL";
pub const ENV_DEFAULT_OPUS_MODEL: &str = "ANTHROPIC_DEFAULT_OPUS_MODEL";
pub const ENV_DEFAULT_SONNET_MODEL: &str = "ANTHROPIC_DEFAULT_SONNET_MODEL";
pub const ENV_DEFAULT_HAIKU_MODEL: &str = "ANTHROPIC_DEFAULT_HAIKU_MODEL";

/// Control variable for shell integration
pub const ENV_SHELL_INTEGRATION: &str = "CCE_SHELL_INTEGRATION";

/// Helper functions for environment variable management
use crate::config::Provider;

/// Set all environment variables for a provider
pub fn set_provider_env_vars(provider: &Provider) {
    std::env::set_var(ENV_AUTH_TOKEN, &provider.token);
    std::env::set_var(ENV_BASE_URL, &provider.api_url);

    if let Some(ref model) = provider.model {
        std::env::set_var(ENV_MODEL, model);
        std::env::set_var(ENV_DEFAULT_OPUS_MODEL, model);
        std::env::set_var(ENV_DEFAULT_SONNET_MODEL, model);
        std::env::set_var(ENV_DEFAULT_HAIKU_MODEL, model);
    }
}

/// Clear all environment variables managed by CCE
pub fn clear_all_env_vars() {
    std::env::remove_var(ENV_AUTH_TOKEN);
    std::env::remove_var(ENV_BASE_URL);
    std::env::remove_var(ENV_MODEL);
    std::env::remove_var(ENV_DEFAULT_OPUS_MODEL);
    std::env::remove_var(ENV_DEFAULT_SONNET_MODEL);
    std::env::remove_var(ENV_DEFAULT_HAIKU_MODEL);
}

/// Generate export commands for shell integration
pub fn generate_export_commands(provider: &Provider) -> String {
    let mut commands = Vec::new();
    commands.push(format!("export {}=\"{}\"", ENV_AUTH_TOKEN, provider.token));
    commands.push(format!("export {}=\"{}\"", ENV_BASE_URL, provider.api_url));

    if let Some(ref model) = provider.model {
        commands.push(format!("export {}=\"{}\"", ENV_MODEL, model));
        commands.push(format!("export {}=\"{}\"", ENV_DEFAULT_OPUS_MODEL, model));
        commands.push(format!("export {}=\"{}\"", ENV_DEFAULT_SONNET_MODEL, model));
        commands.push(format!("export {}=\"{}\"", ENV_DEFAULT_HAIKU_MODEL, model));
    }

    commands.join("\n")
}

/// Generate unset commands for shell integration
pub fn generate_unset_commands() -> String {
    let env_vars = [
        ENV_AUTH_TOKEN,
        ENV_BASE_URL,
        ENV_MODEL,
        ENV_DEFAULT_OPUS_MODEL,
        ENV_DEFAULT_SONNET_MODEL,
        ENV_DEFAULT_HAIKU_MODEL,
    ];

    env_vars
        .iter()
        .map(|var| format!("unset {}", var))
        .collect::<Vec<_>>()
        .join("\n")
}
