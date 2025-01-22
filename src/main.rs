use std::env;

mod utils;
use crate::utils::fetch::{fetch_mortgage_rates, fetch_unemployment_rates};
use crate::utils::webhook::send_to_discord;

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
