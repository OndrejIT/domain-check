use crate::config::CONFIG;
use log;
use reqwest::Error;
use std::collections::HashMap;

pub async fn send_message(text: &str) -> Result<(), Error> {
    log::info!("{}", text);
    let cfg = CONFIG();
    let url = format!(
        "https://api.telegram.org/bot{}/sendMessage",
        &cfg.telegram.token
    );

    let mut map = HashMap::new();
    map.insert("chat_id", cfg.telegram.chat_id.clone());
    map.insert("text", text.to_string());

    let client = reqwest::Client::new();
    client.post(url).json(&map).send().await?;

    Ok(())
}
