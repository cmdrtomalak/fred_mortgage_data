use prettytable::Table;
use reqwest::Client;
use serde_json::json;

pub async fn send_to_discord(
    webhook_url: &str,
    table: Table,
) -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();

    let table_string = table.to_string();

    let payload = json!({
        "content": format!("```\n{}```", table_string)
    });

    let response = client.post(webhook_url).json(&payload).send().await?;

    if response.status().is_success() {
        println!("Table sent to Discord successfully");
    } else {
        eprintln!("Failed to send table to Discord: {}", response.status());
    }

    Ok(())
}
