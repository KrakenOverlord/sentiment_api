mod database;

use database::Rollup;
use lambda_http::{run, service_fn, Body, Error, Request, RequestExt, Response};
use anyhow::{Result, bail, Context};
use dotenv::dotenv;
use serde::Serialize;
use sqlx::types::chrono::Utc;
use tracing::info;
use chrono::Duration;

use crate::database::Database;

async fn function_handler(event: Request) -> Result<Response<Body>, Error> {
    info!("{:?}", event);

    // Get data from database and format it
    let rollups = get_rollups().await?;
    let dates: Vec<String> = rollups.iter().map(|r| r.date.to_string()).collect();
    let sentiments: Vec<f32> = rollups.iter().map(|r| r.sentiment).collect();
    let prices: Vec<i32> = rollups.iter().map(|r| r.price).collect();

    #[derive(Debug, Serialize)]
    pub struct Data {
        dates: Vec<String>,
        sentiments: Vec<f32>,
        prices: Vec<i32>,
    }

    let data = Data {
        dates,
        sentiments,
        prices,
    };

    // Serialize it to a JSON string.
    let body = serde_json::to_string(&data)?;

    // Return something that implements IntoResponse.
    // It will be serialized to the right response event automatically by the runtime
    let resp = Response::builder()
        .status(200)
        .header("content-type", "text/json")
        .body(body.into())
        .map_err(Box::new)?;
    Ok(resp)
}

async fn get_rollups() -> Result<Vec<Rollup>> {
    let date = (Utc::now() - Duration::days(29)).date_naive();
    let mut database = Database::new().await?;
    let rollups = database.get_rollups(date).await?;

    Ok(rollups)
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    dotenv().ok();

    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        // disable printing the name of the module in every log line.
        .with_target(false)
        // disabling time is handy because CloudWatch will add the ingestion time.
        .without_time()
        .init();

    run(service_fn(function_handler)).await
}
