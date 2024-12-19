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
use tokio::net::TcpListener;
use warp::Filter;

use crate::{helpers::kill_process_on_port, indexer::Indexer, meterics::MetricsDetails};

fn get_prometheus_exe() -> Result<PathBuf, ()> {
    let prometheus_filename = match env::consts::OS {
        "windows" => "rindexer-prometheus-win.exe",
        "macos" => "rindexer-prometheus-macos",
        "linux" => "rindexer-prometheus-linux",
        _ => {
            panic!("Unsupported OS: {}", env::consts::OS);
        }
    };
    let mut paths = vec![];

    // Assume `resources` directory is in the same directory as the executable (installed)
    if let Ok(executable_path) = env::current_exe() {
        let mut path = executable_path.to_path_buf();
        path.pop(); // Remove the executable name
        path.push("resources");
        path.push(prometheus_filename);
        paths.push(path);

        // Also consider when running from within the `rindexer` directory
        let mut path = executable_path;
        path.pop(); // Remove the executable name
        path.pop(); // Remove the 'release' or 'debug' directory
        path.push("resources");
        path.push(prometheus_filename);
        paths.push(path);
    }

    // Check additional common paths
    if let Ok(home_dir) = env::var("HOME") {
        let mut path = PathBuf::from(home_dir);
        path.push(".rindexer");
        path.push("resources");
        path.push(prometheus_filename);
        paths.push(path);
    }

    // Return the first valid path
    for path in &paths {
        if path.exists() {
            return Ok(path.to_path_buf());
        }
    }

    // If none of the paths exist, return the first one with useful error message
    let extra_looking =
        paths.into_iter().next().expect("Failed to determine rindexer prometheus path");

    if !extra_looking.exists() {
        return Err(());
    }

    Ok(extra_looking)
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
pub async fn start_metrics_server(port: u16) {
    let metrics_route = warp::path("metrics").and_then(metrics_handler);

    crate::meterics::RINDEXER_HTTP_REQUESTS.with_label_values(&["GET", "/metrics"]).inc();

    warp::serve(metrics_route).run(([127, 0, 0, 1], port)).await;
}
// tracing::info!("Starting metrics server");
//         // let connection_string = indexer.connection_string();
// crate::meterics::RINDEXER_HTTP_REQUESTS.with_label_values(&["GET", "/metrics"]).inc();

// let encoder = TextEncoder::new();
// let mut buffer = vec![];
// encoder.encode(&prometheus::gather(), &mut buffer).expect("Failed to encode metrics");

// let response = String::from_utf8(buffer.clone()).expect("Failed to convert bytes to string");
// tracing::info!("Metrics response: {}", response);

// // let json: serde_json::Value = serde_json::from_str(&response).expect("Failed to convert to
// // json");
// buffer.clear();
// let port = metrics_details.port.unwrap();

// let metrics_endpoint = format!("https://localhost:{}/metrics", port);
// tracing::info!("Metrics endpoint: {}", metrics_endpoint);
// kill_process_on_port(port).map_err(|e| {
//     tracing::error!("Failed to kill process on port: {}", e);
//     StartMetricServerError::MetricServerStartupError(e)
// })?;
// let rindexer_prometheus_exe = get_prometheus_exe().map_err(|_| {
//     StartMetricServerError::MetricServerStartupError(
//         "rindexer-prometheus executable not found".to_string(),
//     )
// })?;
// Ok(response)
// let connection_string = indexer.connection_string();

// tracing::info!("Metrics value: {:?}", value);
// let result = reqwest::ClientBuilder::new()
// let client = reqwest::Client::new();
// let result = client.post(&metrics_endpoint).body(response).send().await.map_err(|e| {
//     tracing::error!("Failed to send metrics: {}", e);
//     StartMetricServerError::MetricServerStartupError(e.to_string())
// })?;
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

pub async fn spawn_start_metrics() {
    unimplemented!()
}

pub async fn start_server(
    rindexer_prometheus_exe: &Path,
    connection_string: &str,
    port: u16,
) -> Result<Child, String> {
    Command::new(rindexer_prometheus_exe)
        .arg(connection_string)
        .arg(port.to_string())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
        .map_err(|e| e.to_string())
}

pub async fn metrics_handler() -> Result<impl warp::Reply, warp::Rejection> {
    let encoder = TextEncoder::new();
    let mut buffer = vec![];
    encoder.encode(&prometheus::gather(), &mut buffer).expect("Failed to encode metrics");
    let response = String::from_utf8(buffer.clone()).expect("Failed to convert bytes to string");
    Ok(warp::reply::with_header(response, "content-type", encoder.format_type()))
}
