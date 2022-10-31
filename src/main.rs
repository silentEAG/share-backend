use axum::Extension;
use config::ConfigBuilder;
use dotenv::dotenv;
use migration::{Migrator, MigratorTrait};
use model::s3::SharkS3Client;
use once_cell::sync::Lazy;
use rusoto_s3::{S3Client, S3};
use sea_orm::{ConnectOptions, Database};
use std::{net::SocketAddr, time::Duration};
use tokio::{join, time::Instant};
use tower::ServiceBuilder;
use tracing::log;
use tracing_appender::{non_blocking, rolling};
use tracing_subscriber::{prelude::__tracing_subscriber_SubscriberExt, util::SubscriberInitExt};

use crate::router::router as app;

mod common;
mod config;
mod error;
mod model;
mod router;
mod util;

pub type Result<T> = std::result::Result<T, crate::error::ServerError>;

static CONFIG: Lazy<config::ConfigItems> = Lazy::new(|| {
    let builder = ConfigBuilder::default().add_env();
    builder.build()
});

static S3: Lazy<SharkS3Client> = Lazy::new(|| {
    let access_key = CONFIG.s3_access_key();
    let secret_key = CONFIG.s3_secret_key();
    let cred = rusoto_credential::StaticProvider::new_minimal(access_key, secret_key);
    let client = rusoto_core::request::HttpClient::new().unwrap();
    let region = rusoto_signature::Region::Custom {
        name: "eu-east-1".to_string(),
        endpoint: CONFIG.s3_endpoint(),
    };
    SharkS3Client::new(
        CONFIG.s3_bucket_name(),
        S3Client::new_with(client, cred, region),
    )
});

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();
    let start_time = Instant::now();

    // Logger subscribe
    // TODO: Make a LogWriter for more features such as filtering ansi
    // Generate none blocking logger in file
    let file_appender = rolling::daily(CONFIG.log_file_dir(), CONFIG.log_file_name());
    let (none_blocking_file_appender, _guard) = non_blocking(file_appender);
    let (none_blocking_std_appender, _guard) = non_blocking(std::io::stdout());

    // Tracing subscriber
    tracing_subscriber::registry()
        // Set the Log level
        .with(tracing_subscriber::EnvFilter::new(CONFIG.log_level()))
        // Set the file logger
        .with(
            tracing_subscriber::fmt::layer()
                .with_ansi(false)
                .with_writer(none_blocking_file_appender),
        )
        // Set the console logger
        .with(tracing_subscriber::fmt::layer().with_writer(none_blocking_std_appender))
        .init();

    // Make sure config loading is right
    tracing::info!("\nShark Share Config Info:{}", *CONFIG);

    let opt = ConnectOptions::new(CONFIG.database_url())
        .max_connections(CONFIG.database_max_connections())
        .min_connections(5)
        .sqlx_logging(CONFIG.app_mode() == *"dev")
        .sqlx_logging_level(log::LevelFilter::Debug)
        .to_owned();

    let conn = Database::connect(opt).await?;
    Migrator::up(&conn, None).await.unwrap();

    // Set router
    let app = app()
        .await
        .layer(ServiceBuilder::new().layer(Extension(conn)));

    // Prepare to start
    let addr = SocketAddr::from(([127, 0, 0, 1], CONFIG.app_port()));
    let server = axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .with_graceful_shutdown(shutdown_signel());

    tracing::info!(
        "Started Server after {}ms",
        start_time.elapsed().as_millis()
    );
    let (res,) = join!(server);
    if res.is_err() {
        tracing::error!("Server Error: {}", res.err().unwrap());
    }
    Ok(())
}

/// Receive shutdown signel
async fn shutdown_signel() {
    let recv_ctrl_c = async {
        tokio::signal::ctrl_c()
            .await
            .expect("Failed to install Ctrl-C handler");
    };

    #[cfg(unix)]
    let recv_terminate = async {
        tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate())
            .expect("Failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let recv_terminate = std::future::pending::<()>();

    tokio::select! {
        _ = recv_ctrl_c => {work_before_shutdown()},
        _ = recv_terminate => {work_before_shutdown()},
    }
}

// TODO: Can do something there before shutdown : )
fn work_before_shutdown() {
    tracing::info!("Going to shutdown...");
}

#[test]
fn test() {
    use jsonwebtoken::{Algorithm, Validation};
}
