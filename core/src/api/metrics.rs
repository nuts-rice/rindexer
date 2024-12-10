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

fn get_prometheus_exe() -> Result<PathBuf, ()> {
    unimplemented!()
}

pub struct MetricsServer {
    pid: u32,
}

#[derive(thiserror::Error, Debug)]
pub enum StartMetricServerError {
    #[error("Can not read metrics environment variable: {0}")]
    UnableToReadMetricUrl(#[from] env::VarError),

    #[error("Could not start up metrics server {0}")]
    MertricsServerStartupError(String),
}

pub async fn start_metrics_server() -> Result<MetricsServer, StartMetricServerError> {
    unimplemented!()
}
