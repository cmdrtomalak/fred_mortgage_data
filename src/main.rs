use chrono::{Duration, Local};
use fred_rs::client::FredClient;
use fred_rs::series::observation::{Builder, Frequency, Response, Units};
use prettytable::{Cell, Row, Table};
use reqwest::Client;
use serde_json::json;
use std::env;

async fn fetch_mortgage_rates(
    series_id: &str,
    label: &str,
) -> Result<Table, Box<dyn std::error::Error>> {
    // Initialize FRED client
    let mut client = FredClient::new()?;

    let fred_key = env::var("FRED_API_KEY").expect("Need a key");
    client.with_key(&fred_key);

    let one_month_ago = (Local::now() - Duration::days(30))
        .format("%Y-%m-%d")
        .to_string();

    let mut builder = Builder::new();
    builder
        .units(Units::LIN)
        .frequency(Frequency::W)
        .observation_start(&one_month_ago);

    let resp: Response = client.series_observation(series_id, Some(builder))?;

    let mut table = Table::new();
    table.add_row(Row::new(vec![
        Cell::new("Date"),
        Cell::new(&label),
        Cell::new("Unit"),
        Cell::new("Frequency"),
    ]));

    for obs in resp.observations {
        table.add_row(Row::new(vec![
            Cell::new(&obs.date),
            Cell::new(&obs.value),
            Cell::new("Percent"),
            Cell::new("Weekly"),
        ]));
    }

    Ok(table)
}

async fn fetch_unemployment_rates() -> Result<Table, Box<dyn std::error::Error>> {
    // Initialize FRED client
    let mut client = FredClient::new()?;

    let fred_key = env::var("FRED_API_KEY").expect("Need a key");
    client.with_key(&fred_key);

    let one_month_ago = (Local::now() - Duration::days(30))
        .format("%Y-%m-%d")
        .to_string();

    let mut builder = Builder::new();
    builder.units(Units::LIN).observation_start(&one_month_ago);

    let resp: Response = client.series_observation("UNRATE", Some(builder))?;

    let mut table = Table::new();
    table.add_row(Row::new(vec![
        Cell::new("Date"),
        Cell::new("Unemployment Rate"),
        Cell::new("Unit"),
    ]));

    for obs in resp.observations {
        table.add_row(Row::new(vec![
            Cell::new(&obs.date),
            Cell::new(&obs.value),
            Cell::new("Percent"),
        ]));
    }

    Ok(table)
}
async fn send_to_discord(
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

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let series_id = "MORTGAGE30US";
    let label = "30-Year Fixed Rate";
    let table = fetch_mortgage_rates(&series_id, &label).await?;

    let webhook_url = env::var("DISCORD_WEBHOOK_URL").expect("Webhook url not found");
    send_to_discord(&webhook_url, table).await?;

    let series_id = "MORTGAGE15US";
    let label = "15-Year Fixed Rate";
    let table = fetch_mortgage_rates(&series_id, &label).await?;

    send_to_discord(&webhook_url, table).await?;

    let table = fetch_unemployment_rates().await?;

    send_to_discord(&webhook_url, table).await?;

    Ok(())
}
