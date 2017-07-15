use config::{Config, File};
use regex::Regex;
use slog::Logger;

const CONFIG_FILE: &'static str = "config";

#[derive(Debug, Default, Deserialize)]
pub struct Settings {
    database: DatabaseSettings,
    api_key: String,
}

#[derive(Debug, Deserialize)]
pub struct DatabaseSettings {
    url: String,
    pool_size: u32,
}

impl Default for DatabaseSettings {
    fn default() -> Self {
        Self {
            url: String::default(),
            pool_size: 3,
        }
    }
}

pub fn load(logger: &Logger) -> Settings {
    // TODO: This should load individual defaults first
    let settings = Config::new()
        .merge(File::with_name(CONFIG_FILE))
        .to_result()
        .and_then(|config| config.deserialize())
        .unwrap_or(Settings::default());

    info!(logger, "API key: {}", settings.api_key);
    info!(logger, "Database URL: {}", settings.database.url);
    info!(
        logger,
        "Database pool size: {}",
        settings.database.pool_size
    );

    validate(&settings, logger);
    settings
}

fn validate(settings: &Settings, logger: &Logger) {
    if settings.api_key.is_empty() {
        crit!(logger, "Provided API key is empty");
    }

    let matcher = Regex::new(r"^postgres://\w+").unwrap();
    if !matcher.is_match(&settings.database.url) {
        crit!(logger, "Database URL is not a recognised format");
    }
}

// TODO: Tests should be added for defaulting scenario
