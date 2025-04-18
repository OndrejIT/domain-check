use domain_check::args;
use domain_check::args::ARGS;
use domain_check::config;
use domain_check::config::CONFIG;
use domain_check::telegram;
use domain_check::whois;
use tokio;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    args::init().expect("Failed to load arguments");
    let args = ARGS();
    env_logger::Builder::new()
        .filter_level(args.log_level().into())
        .init();

    config::init().expect("Failed to load configuration");
    let cfg = CONFIG();

    for domain in &cfg.domain {
        let host = &domain.host;
        match whois::query(&host) {
            Ok(resp) => {
                if resp.parse_no_match() {
                    telegram::send_message(&format!("🔥🔥🔥 {} is free to register! 🔥🔥🔥", host))
                        .await?;
                } else {
                    match resp.parse_expiry_date() {
                        Ok(dt) => {
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
                telegram::send_message(&format!("WHOIS query error: {}", e)).await?;
            }
        }
    }
    Ok(())
}
