use serde::Deserialize;
use std::fs;
use std::sync::OnceLock;

#[derive(Deserialize, Clone)]
pub struct Config {
    pub telegram: Telegram,
    pub domain: Vec<Domain>,
}

#[derive(Deserialize, Clone)]
pub struct Domain {
    pub host: String,
}

#[derive(Deserialize, Clone)]
pub struct Telegram {
    pub token: String,
    pub chat_id: String,
}

static CONFIG: OnceLock<Config> = OnceLock::new();

pub fn get_config() -> &'static Config {
    CONFIG.get().expect("Configuration is not initialized")
}

pub fn init() -> Result<(), Box<dyn std::error::Error>> {
    let config_file = fs::read_to_string("./config.toml")?;
    let config: Config = toml::from_str(&config_file)?;
    let _ = CONFIG.set(config);

    Ok(())
}
