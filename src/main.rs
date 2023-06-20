use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions};
use sqlx::ConnectOptions;
use std::error::Error;
use std::str::FromStr;
use std::time::Duration;

mod logging;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    logging::setup(log::LevelFilter::Error);

    let database_url = "sqlite://data.db?mode=rwc";

    let mut connection_options =
        SqliteConnectOptions::from_str(database_url).expect("Unable to create connection options.");

    connection_options
        .log_statements(log::LevelFilter::Error)
        .log_slow_statements(log::LevelFilter::Warn, Duration::from_secs(1));

    let db = SqlitePoolOptions::new()
        .connect_with(connection_options)
        .await
        .expect("Unable to create database pool.");

    sqlx::migrate!("migrations/sqlite3")
        .run(&db)
        .await
        .expect("Could not run database migrations.");

    Ok(())
}
