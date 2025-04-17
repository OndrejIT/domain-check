use domain_check::config;
use domain_check::telegram;
use domain_check::whois;
use tokio;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    config::init().expect("Failed to load configuration");
    let config = config::get_config();

    for domain in &config.domain {
        let host = &domain.host;
        match whois::query(&host) {
            Ok(resp) => {
                if resp.parse_no_match() {
                    println!("No match for: {}", host);
                    telegram::send_message(&format!("No match for: {}", host)).await?;
                } else {
                    match resp.parse_expiry_date() {
                        Ok(dt) => {
                            println!("Expiry Date: {} -> {}", host, dt);
                            telegram::send_message(&format!("Expiry Date: {} -> {}", host, dt))
                                .await?;
                        }
                        Err(e) => {
                            telegram::send_message(&format!("Parsing error: {}", e)).await?;
                        }
                    }
                }
            }
            Err(e) => {
                eprintln!("WHOIS query error: {}", e);
                telegram::send_message(&format!("WHOIS query error: {}", e)).await?;
            }
        }
    }
    Ok(())
}
