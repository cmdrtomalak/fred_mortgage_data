use chrono::{Duration, Local};
use fred_rs::client::FredClient;
use fred_rs::series::observation::{Builder, Frequency, Response, Units};
use prettytable::{Cell, Row, Table};
use std::env;

pub async fn fetch_mortgage_rates(
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

pub async fn fetch_unemployment_rates() -> Result<Table, Box<dyn std::error::Error>> {
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
