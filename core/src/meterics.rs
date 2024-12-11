use std::{
    env,
    net::SocketAddr,
    sync::{Arc, RwLock},
};

use lazy_static::lazy_static;
use metrics_exporter_prometheus::PrometheusHandle;
use prometheus::{
    opts, register_histogram_vec, register_int_counter_vec, register_int_gauge, Encoder,
    HistogramVec, IntCounterVec, IntGauge, TextEncoder,
};
use prometheus_metric_storage::StorageRegistry;
use reqwest::{Client, Error, Response};
use serde::{Deserialize, Serialize};

use crate::indexer::Indexer;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetricsDetails {
    pub enabled: bool,
    pub port: Option<u16>,
}

// const METRICS_REGISTRY: prometheus::Registry = prometheus::Registry::new();

const HTTP_DURATION_BUCKETS: [f64; 5] = [0.1, 0.5, 1.0, 2.0, 5.0];
const WS_DURATION_BUCKETS: [f64; 5] = [0.1, 0.5, 1.0, 2.0, 5.0];
lazy_static! {
    pub static ref RINDEXER_HTTP_REQUESTS: IntCounterVec = register_int_counter_vec!(
        opts!("rindexer_http_requests", "rindexer HTTP requests").into(),
        &["method", "path"]
    )
    .expect("Failed to register rindexer_http_requests");
    pub static ref RINDEXER_WS_REQUESTS: IntCounterVec = register_int_counter_vec!(
        opts!("rindexer_WS_requests", "rindexer WebSocket requests").into(),
        &["method", "path"]
    )
    .expect("Failed to register rindexer_WS_requests");
    pub static ref RINDEXER_WS_REQUESTS_DURATION: HistogramVec = register_histogram_vec!(
        "rindexer_WS_requests_duration",
        "rindexer WS requests duration",
        &["method", "path"],
        WS_DURATION_BUCKETS.to_vec()
    )
    .expect("Failed to register rindexer_WS_requests_duration");
    pub static ref RINDEXER_HTTP_REQUESTS_DURATION: HistogramVec = register_histogram_vec!(
        "rindexer_http_requests_duration",
        "rindexer http requests duration",
        &["method", "path"],
        HTTP_DURATION_BUCKETS.to_vec()
    )
    .expect("Failed to register rindexer_rpc_requests_duration");
}

// async fn metrics_server(&self, listen_addr: SocketAddr, handle: PrometheusHandle) {
//     let client = reqwest::Client::new();
//     // let (tx, rx) = oneshot::channel();
//     let metrics_endpoint = format!("http://localhost:{}/metrics", listen_addr.port());
//     let encoder = prometheus::TextEncoder::new();
//     let mut buffer = vec![];
//     // tokio::spawn(async move {
//     loop {
//         tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
//         let metric_families = prometheus::gather();
//         encoder.encode(&metric_families, &mut buffer).unwrap();
//         let response =
//             String::from_utf8(buffer.clone()).expect("Failed to convert metrics to string");
//         buffer.clear();
//     }
// }

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
    let client = reqwest::Client::new();
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

pub async fn metrics_handler() -> Result<(), StartMetricServerError> {
    tracing::debug!("Starting metrics handler");
    let client = Client::new();
    unimplemented!()
}
