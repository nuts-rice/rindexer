use std::{
    env,
    sync::Arc,
    time::{self, Duration},
};

use bb8::{Pool, RunError};
use ethers::{
    middleware::Middleware,
    providers::{Provider, RetryClient, RetryClientBuilder},
};
use ethers_providers::Ws;
pub use serde_json::Value;
use tokio::{
    sync::{
        mpsc::{UnboundedReceiver, UnboundedSender},
        Mutex,
    },
    task,
    time::timeout,
};
pub struct WSConfig;

#[derive(Debug)]
pub struct WsRpcProvider {
    pub provider: Arc<Provider<RetryClient<Ws>>>,
    cache: Mutex<Option<(Instant, Arc<Block<H256>>)>>,
    pub max_block_range: Option<U64>,
}
pub struct WSMsg {
    message: Value,
}
pub async fn ws_conn_manager(
    ws_tx: Arc<Mutex<UnboundedSender<WSMsg>>>,
    mut ws_rx: UnboundedReceiver<WSMsg>,
) {
    let start = time::Instant::now();
    todo!()
}
