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

use crate::{indexer::Indexer, meterics::MetricsDetails , helpers::kill_process_on_port };

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
) -> Result<Response, StartMetricServerError> {
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
    let port = metrics_details.port.unwrap();

    let metrics_endpoint = format!("https://localhost:{}", port);
    tracing::info!("Metrics endpoint: {}", metrics_endpoint);
    kill_process_on_port(port).map_err(|e| {
        tracing::error!("Failed to kill process on port: {}", e);
        StartMetricServerError::MetricServerStartupError(e)
    })?;
    // tracing::info!("Metrics value: {:?}", value);
    let client = reqwest::Client::new();
    let result = client.post(&metrics_endpoint).body(response).send().await.map_err(|e| {
        tracing::error!("Failed to send metrics: {}", e);
        StartMetricServerError::MetricServerStartupError(e.to_string())
    })?;
    // match client.post(&metrics_endpoint).body(response).send().await {

    //     Ok(response) if response.status().is_success() => {
    //         let response_json = response.json::<serde_json::Value>().await;
    //         match response_json {
    //             Ok(response_json) => {
    //                 if response_json.get("errors").is_none() {
    //         tracing::info!("Metrics sent successfully");
    //         tracing::info!("Metrics endpoint ready at {} ", metrics_endpoint);
    //                 }
    //             }
    //     Err(_) => {
    //         tracing::info!("error reaching metrics endpoint");
    //     }
    //         }
    //     }
    //     _ => {
    //         tracing::info!("error reaching metrics endpoint");
    //     }
            
    // }

    Ok(result)
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
