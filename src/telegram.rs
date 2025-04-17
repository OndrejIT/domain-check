use crate::config;
use reqwest::Error;
use std::collections::HashMap;

pub async fn send_message(text: &str) -> Result<(), Error> {
    let config = config::get_config();
    let url = format!(
        "https://api.telegram.org/bot{}/sendMessage",
        &config.telegram.token
    );

    let mut map = HashMap::new();
    map.insert("chat_id", config.telegram.chat_id.clone());
    map.insert("text", text.to_string());

    let client = reqwest::Client::new();
    client.post(url).json(&map).send().await?;

    Ok(())
}
