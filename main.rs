use std::io::{self, Write};
use whois_rust::{WhoIs, WhoIsLookupOptions};
use reqwest::Client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Prompt for the website URL
    let mut input = String::new();
    print!("Enter website URL: ");
    io::stdout().flush()?;
    io::stdin().read_line(&mut input)?;
    let url = input.trim();

    // Add "https://www." prefix for ping request
    let ping_url = format!("https://www.{}", url);

    // Create a new reqwest client
    let client = Client::new();

    // Send a GET request to the website
    let response = client.get(&ping_url).send().await?;

    if response.status().is_success() {
        println!("Ping successful! Status: {}", response.status());
    } else {
        println!("Ping failed! Status: {}", response.status());
    }

    // Perform WHOIS lookup
    let whois = WhoIs::from_path("./servers.json")?;
    let whois_options = WhoIsLookupOptions::from_string(url)?;
    
    let whois_result = tokio::task::block_in_place(|| {
        whois.lookup(whois_options).map_err(|e| e.to_string())
    })?;

    println!("WHOIS lookup result:\n{}", whois_result);

    println!("Finished! Scroll up and read above.");

    Ok(())
}
