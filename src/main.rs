use dotenvy::dotenv;
use serde::Deserialize;
use std::env;

#[derive(Debug, Deserialize)]
struct ApiResponse {
    timestamp: String,
    query: Query,
    #[serde(rename = "stop_groups")]
    stop_groups: Vec<StopGroup>,
}

#[derive(Debug, Deserialize)]
struct Query {
    #[serde(rename = "queryTime")]
    query_time: String,
    query: String,
}

#[derive(Debug, Deserialize)]
struct StopGroup {
    id: String,
    name: String,
    #[serde(rename = "area_type")]
    area_type: String,
    #[serde(rename = "average_daily_stop_times")]
    average_daily_stop_times: f64,
    #[serde(rename = "transport_modes")]
    transport_modes: Vec<String>,
    stops: Vec<Stop>,
}

#[derive(Debug, Deserialize)]
struct Stop {
    id: String,
    name: String,
    lat: f64,
    lon: f64,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    let api_key = env::var("TRAFIKLAB_API_KEY").expect("ERR: TRAFIKLAB_API_KEY not found in .env");

    let url = format!(
        "https://realtime-api.trafiklab.se/v1/stops/name/sto/?key={}",
        api_key
    );

    let client = reqwest::Client::new();

    let data: ApiResponse = client.get(url).send().await?.json().await?;

    println!("{:#?}", data);

    Ok(())
}
