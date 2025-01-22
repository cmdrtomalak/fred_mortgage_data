use std::env;

mod utils;
use crate::utils::fetch::{fetch_mortgage_rates, fetch_treasury_rates, fetch_unemployment_rates};
use crate::utils::webhook::send_to_discord;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let fred_key = env::var("FRED_API_KEY").expect("Need a key");
    let webhook_url = env::var("DISCORD_WEBHOOK_URL").expect("Webhook url not found");

    let table = fetch_mortgage_rates("MORTGAGE30US", "30-Year Fixed Rate", &fred_key).await?;
    send_to_discord(&webhook_url, table).await?;

    let table = fetch_mortgage_rates("MORTGAGE15US", "15-Year Fixed Rate", &fred_key).await?;
    send_to_discord(&webhook_url, table).await?;

    let table = fetch_unemployment_rates(&fred_key).await?;
    send_to_discord(&webhook_url, table).await?;

    let table = fetch_treasury_rates("DGS30", "30-Year Treasury Rate", &fred_key).await?;
    send_to_discord(&webhook_url, table).await?;

    let table = fetch_treasury_rates("DGS20", "20-Year Treasury Rate", &fred_key).await?;
    send_to_discord(&webhook_url, table).await?;

    let table = fetch_treasury_rates("DGS10", "10-Year Treasury Rate", &fred_key).await?;
    send_to_discord(&webhook_url, table).await?;

    let table = fetch_treasury_rates("DGS2", "2-Year Treasury Rate", &fred_key).await?;
    send_to_discord(&webhook_url, table).await?;

    let table = fetch_treasury_rates("DGS6MO", "6-Month Treasury Rate", &fred_key).await?;
    send_to_discord(&webhook_url, table).await?;
    Ok(())
}
