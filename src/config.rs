use serde::Deserialize;
use std::fs;
use std::sync::OnceLock;

#[derive(Deserialize, Clone)]
pub struct Config {
    pub telegram: Telegram,
    pub domain: Vec<Domain>,
    pub whois: Vec<Whois>,
}

#[derive(Deserialize, Clone)]
pub struct Domain {
    pub host: String,
}

#[derive(Deserialize, Clone)]
pub struct Whois {
    pub server: String,
    pub tld: String,
    #[serde(default = "default_whois_port")]
    pub port: u16,
}

fn default_whois_port() -> u16 {
    43
}

#[derive(Deserialize, Clone)]
pub struct Telegram {
    pub token: String,
    pub chat_id: String,
}

pub fn get_whois_server(domain: &str) -> Option<Whois> {
    let tld = domain.split('.').last().unwrap_or("");
    get_config()
        .whois
        .iter()
        .find(|w| w.tld == tld)
        .cloned()
        .or_else(|| {
            Some(Whois {
                server: "whois.verisign-grs.com".to_string(),
                tld: "com".to_string(),
                port: default_whois_port(),
            })
        })
}

static STATIC_CONFIG: OnceLock<Config> = OnceLock::new();

pub fn get_config() -> &'static Config {
    STATIC_CONFIG
        .get()
        .expect("Configuration is not initialized")
}

pub fn init() -> Result<(), Box<dyn std::error::Error>> {
    let config_file = fs::read_to_string("./config.toml")?;
    let config: Config = toml::from_str(&config_file)?;
    let _ = STATIC_CONFIG.set(config);

    Ok(())
}

pub use get_config as CONFIG;
