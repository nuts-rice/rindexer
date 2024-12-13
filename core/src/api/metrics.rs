use std::{
    env,
    path::{Path, PathBuf},
    process::{Child, Command, Stdio},
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc, Mutex,
    },
    time::Duration,
};

use prometheus::{
    opts, register_histogram_vec, register_int_counter_vec, register_int_gauge, Encoder,
    HistogramVec, IntCounterVec, IntGauge, TextEncoder,
};
use reqwest::{Client, Error, Response};

use crate::{indexer::Indexer, meterics::MetricsDetails};

fn get_prometheus_exe() -> Result<PathBuf, ()> {
    unimplemented!()
}

#[derive(thiserror::Error, Debug)]
pub enum StartMetricServerError {
    #[error("Can not read database environment variable: {0}")]
    UnableToReadMetricUrl(#[from] env::VarError),

    #[error("Could not start up prometheusd server {0}")]
    MetricServerStartupError(String),
}

pub struct MetricsServer {
    pid: u32,
}

//start up prometheus exe here
pub async fn start_metrics_server(
    indexer: &Indexer,
    metrics_details: MetricsDetails,
) -> Result<(), StartMetricServerError> {
    tracing::info!("Starting metrics server");
    crate::meterics::RINDEXER_HTTP_REQUESTS.with_label_values(&["GET", "/metrics"]).inc();

    let encoder = TextEncoder::new();
    let mut buffer = vec![];
    encoder.encode(&prometheus::gather(), &mut buffer).expect("Failed to encode metrics");

    let response = String::from_utf8(buffer.clone()).expect("Failed to convert bytes to string");
    tracing::info!("Metrics response: {}", response);

    // let json: serde_json::Value = serde_json::from_str(&response).expect("Failed to convert to
    // json");
    buffer.clear();

    let metrics_endpoint = format!("https://localhost:{}/metrics", metrics_details.port.unwrap());
    tracing::info!("Metrics endpoint: {}", metrics_endpoint);
    // let value: serde_json::Value = serde_json::from_str(&response).unwrap();
    // tracing::info!("Metrics value: {:?}", value);
    let client = reqwest::Client::new();
    client.post(&metrics_endpoint).body(response).send().await.unwrap();

    // client.post(&metrics_endpoint).body(response).send().await.unwrap();
    // let res = client.post(&metrics_endpoint).body(response).send().await.unwrap();

    Ok(())
}
// match res {
//     Ok(_) => {
//         let pid = std::process::id();
//         Ok(MetricsServer { pid })
//     }
//     Err(e) => Err(StartMetricServerError::MetricServerStartupError(e.to_string())),
// }

pub async fn spawn_start_metrics() {
    unimplemented!()
}

pub async fn start_server(rindexer_prometheus_exe: &Path, connection_string: &str, port: u16) {
    unimplemented!()
}

pub async fn metrics_handler() -> Result<(), StartMetricServerError> {
    tracing::debug!("Starting metrics handler");
    let client = Client::new();
    unimplemented!()
}
