use serde::Deserialize;
use std::fs;
use std::sync::OnceLock;

#[derive(Deserialize, Clone)]
pub struct Config {
    pub telegram: Telegram,
    pub domain: Vec<Domain>,
    #[serde(default = "default_whois")]
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

fn default_whois() -> Vec<Whois> {
    vec![Whois {
        server: "whois.verisign-grs.com".to_string(),
        tld: "com".to_string(),
        port: default_whois_port(),
    }]
}

#[derive(Deserialize, Clone)]
pub struct Telegram {
    pub token: String,
    pub chat_id: String,
}

pub fn get_whois_server(domain: &str) -> Result<String, Box<dyn std::error::Error>> {
    let tld = domain.split('.').last().unwrap_or("");
    let config = get_config();

    let whois = config.whois.iter().find(|w| w.tld == tld).cloned();

    match whois {
        Some(w) => Ok(format!("{}:{}", w.server, w.port)),
        None if tld == "com" => Ok("whois.verisign-grs.com:43".to_string()),
        None => Err(format!("No WHOIS server found for domain {}", domain).into()),
    }
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
