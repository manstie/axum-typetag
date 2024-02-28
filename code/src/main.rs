mod errors;

use axum::{
    extract::{Path, State},
    routing::get,
    Json, Router,
};
use deadpool_diesel::postgres::Pool as DbPool;
use dotenvy::dotenv;
use errors::AppError;
use log::LevelFilter;
use serde::{Deserialize, Serialize};
use std::{env, net::SocketAddr};

#[typetag::serde(tag = "type")]
pub trait Queueable {
    fn test(&self);
}

#[derive(Clone, Serialize, Deserialize)]
pub struct ExampleJob {
    pub payload: u64,
}

#[typetag::serde]
impl Queueable for ExampleJob {
    fn test(&self) {
        log::info!("I did something!");
    }
}

#[derive(Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum NotTypeTag {
    ExampleJob(ExampleJob),
}

#[tokio::main]
async fn main() {
    dotenv().ok();

    // Set up logger before any other operations
    fern::Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "[{date}][{target}][{level}] {message}",
                date = chrono::Local::now().format("%Y-%m-%d %H:%M:%S"),
                target = record.target(),
                level = record.level(),
                message = message
            ))
        })
        .level(LevelFilter::Debug)
        .chain(std::io::stdout())
        .apply()
        .unwrap();

    // Establish connection to database
    let database_url: String = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    // set up connection pools
    let db_manager =
        deadpool_diesel::postgres::Manager::new(database_url, deadpool_diesel::Runtime::Tokio1);
    let db_pool = deadpool_diesel::postgres::Pool::builder(db_manager)
        .build()
        .unwrap();

    let app = Router::new()
        .route("/test/:data", get(test))
        .with_state(db_pool);

    let port = env::var("PORT").unwrap_or(String::from("8000"));
    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{port}"))
        .await
        .expect("Failed to bind Tokio listener");
    log::info!("listening on {}", listener.local_addr().unwrap());
    axum::serve(
        listener,
        app.into_make_service_with_connect_info::<SocketAddr>(),
    )
    .await
    .unwrap();
}

async fn test(
    State(pool): State<DbPool>,
    Path(data): Path<u64>,
) -> Result<Json<serde_json::Value>, AppError> {
    let example = ExampleJob { payload: data };

    // Using typetag, `_conn` can not be present, else the route has an error
    let queueable: Box<dyn Queueable> = Box::new(example.clone());
    let _conn = pool.get().await?;

    // Works with or without `_conn`
    // let queueable = NotTypeTag::ExampleJob(example);

    // serialize
    let payload = serde_json::to_value(queueable)?;
    // add to jobs table
    // ... database insert code (doesn't matter)

    log::info!("Queued a job to be worked");
    Ok(Json(payload))
}
