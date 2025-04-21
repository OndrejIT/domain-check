use crate::config::get_whois_server;
use chrono::{DateTime, Utc};
use std::io::{self, Read, Write};
use std::net::TcpStream;

pub struct WhoisResponse {
    pub data: String,
}

impl WhoisResponse {
    pub fn parse_expiry_date(&self) -> Result<DateTime<Utc>, String> {
        let prefix = "Registry Expiry Date:";
        if let Some(line) = self
            .data
            .lines()
            .find(|line| line.trim_start().starts_with(prefix))
        {
            let date_string = line
                .trim_start()
                .trim_end()
                .trim_start_matches(prefix)
                .trim();
            chrono::DateTime::parse_from_rfc3339(date_string)
                .map(|dt| dt.with_timezone(&Utc))
                .map_err(|e| format!("Data parsing error: {}", e))
        } else {
            Err(format!(
                "Error: prefix '{}' not found in WHOIS data",
                prefix
            ))
        }
    }

    pub fn parse_no_match(&self) -> bool {
        let prefix = "No match for";
        self.data
            .lines()
            .any(|line| line.trim_start().starts_with(prefix))
    }
}

pub fn query(domain: &str) -> io::Result<WhoisResponse> {
    match get_whois_server(domain) {
        Ok(whois_addr) => {
            let mut stream = TcpStream::connect(&whois_addr)?;
            stream.write_all(format!("{}\r\n", domain).as_bytes())?;

            let mut response = String::new();
            stream.read_to_string(&mut response)?;

            Ok(WhoisResponse { data: response })
        }
        Err(e) => Err(io::Error::new(io::ErrorKind::Other, e.to_string())),
    }
}
