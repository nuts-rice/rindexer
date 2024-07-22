use std::{
    sync::Arc,
    time::{Duration, Instant},
};

// use alloy::providers::{Provider, ProviderBuilder, WsConnect};
use ethers::{
    middleware::Middleware,
    prelude::Log,
    providers::{Http, Provider, ProviderError, RetryClient, RetryClientBuilder},
    types::{Block, BlockNumber, H256, U256, U64},
};
use ethers_providers::Ws;
use thiserror::Error;
use tokio::sync::Mutex;
use url::Url;

use crate::{event::RindexerEventFilter, manifest::core::Manifest};

// #[derive(Debug)]
// pub struct WsRpcProvider {
//     pub provider: Arc<Provider<RetryClient<Ws>>>,
//     cache: Mutex<Option<(Instant, Arc<Block<H256>>)>>,
//     pub max_block_range: Option<U64>,
// }

// impl JsonRpcCachedProvider for WsRpcProvider {}

#[derive(Debug)]
pub struct JsonRpcCachedProvider {
    // WSProvider: Arc<Provider<RetryClient<Ws>>>,
    provider: Arc<ethers::providers::Provider<RetryClient<Http>>>,
    cache: Mutex<Option<(Instant, Arc<Block<H256>>)>>,
    pub max_block_range: Option<U64>,
}

impl JsonRpcCachedProvider {
    pub fn new(
        provider: ethers::providers::Provider<RetryClient<Http>>,
        // ws_provider: Provider<RetryClient<Ws>>,
        max_block_range: Option<U64>,
    ) -> Self {
        JsonRpcCachedProvider {
            provider: Arc::new(provider),
            // WSProvider: Arc::new(ws_provider),
            cache: Mutex::new(None),
            max_block_range,
        }
    }

    pub async fn get_latest_block(&self) -> Result<Option<Arc<Block<H256>>>, ProviderError> {
        let mut cache_guard = self.cache.lock().await;

        if let Some((timestamp, block)) = &*cache_guard {
            if timestamp.elapsed() < Duration::from_millis(300) {
                return Ok(Some(Arc::clone(block)));
            }
        }

        let latest_block = self.provider.get_block(BlockNumber::Latest).await?;

        if let Some(block) = latest_block {
            let arc_block = Arc::new(block);
            *cache_guard = Some((Instant::now(), Arc::clone(&arc_block)));
            return Ok(Some(arc_block));
        } else {
            *cache_guard = None;
        }

        Ok(None)
    }

    pub async fn get_block_number(&self) -> Result<U64, ProviderError> {
        self.provider.get_block_number().await
    }

    pub async fn get_logs(&self, filter: &RindexerEventFilter) -> Result<Vec<Log>, ProviderError> {
        self.provider.get_logs(filter.raw_filter()).await
    }

    pub async fn get_chain_id(&self) -> Result<U256, ProviderError> {
        self.provider.get_chainid().await
    }

    // pub fn get_inner_ws_provider(&self) -> Arc<Provider<RetryClient<Ws>>> {
    //     Arc::clone(&self.WSProvider)
    // }

    pub fn get_inner_provider(&self) -> Arc<ethers::providers::Provider<RetryClient<Http>>> {
        Arc::clone(&self.provider)
    }
}
#[derive(Error, Debug)]
pub enum RetryClientError {
    #[error("http provider can't be created for {0}: {1}")]
    HttpProviderCantBeCreated(String, String),
}

pub async fn create_client(
    rpc_url: &str,
    ws_rpc_url: &str,
    compute_units_per_second: Option<u64>,
    max_block_range: Option<U64>,
    is_ws: bool,
) -> Result<Arc<JsonRpcCachedProvider>, RetryClientError> {
    let ws_url = Url::parse(ws_rpc_url).expect("Couldn't parse ws url");

    let url = Url::parse(rpc_url).map_err(|e| {
        RetryClientError::HttpProviderCantBeCreated(rpc_url.to_string(), e.to_string())
    })?;

    let provider = Http::new(url);
    let instance = ethers::providers::Provider::new(
        RetryClientBuilder::default()
            // assume minimum compute units per second if not provided as growth plan standard
            .compute_units_per_second(compute_units_per_second.unwrap_or(660))
            .rate_limit_retries(5000)
            .timeout_retries(1000)
            .initial_backoff(Duration::from_millis(500))
            .build(provider, Box::<ethers::providers::HttpRateLimitRetryPolicy>::default()),
    );
    if is_ws {

        // let ws_provider = Provider::<Ws>::connect(ws_url).await?;
        // let ws_inst
    }
    Ok(Arc::new(JsonRpcCachedProvider::new(instance, max_block_range)))
}

pub async fn get_chain_id(rpc_url: &str) -> Result<U256, ProviderError> {
    let url = Url::parse(rpc_url).map_err(|_| ProviderError::UnsupportedRPC)?;
    let provider = ethers::providers::Provider::new(Http::new(url));

    provider.get_chainid().await
}

#[derive(Debug)]
pub struct CreateNetworkProvider {
    pub network_name: String,
    pub client: Arc<JsonRpcCachedProvider>,
}

impl CreateNetworkProvider {
    pub async fn create(
        manifest: &Manifest,
    ) -> Result<Vec<CreateNetworkProvider>, RetryClientError> {
        let mut result: Vec<CreateNetworkProvider> = vec![];
        for network in &manifest.networks {
            let provider = create_client(
                &network.rpc,
                &network.ws_rpc,
                network.compute_units_per_second,
                network.max_block_range,
                network.is_ws,
            )
            .await?;
            result.push(CreateNetworkProvider {
                network_name: network.name.clone(),
                client: provider,
            });
        }

        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // fn test_create_ws_retry_client() {
    //     let rpc_url = "http://localhost:8545";
    //     let ws_rpc_url = "ws://localhost:8546";
    //     let result = create_client(rpc_url, ws_rpc_url, Some(660), None, true);
    //     assert!(result.is_ok());
    // }

    #[test]
    fn test_create_retry_client() {
        let rpc_url = "http://localhost:8545";
        let result = create_client(rpc_url, Some(660), None);
        assert!(result.is_ok());
    }

    #[test]
    fn test_create_retry_client_invalid_url() {
        let rpc_url = "invalid_url";
        let result = create_client(rpc_url, Some(660), None);
        assert!(result.is_err());
        if let Err(RetryClientError::HttpProviderCantBeCreated(url, _)) = result {
            assert_eq!(url, rpc_url);
        } else {
            panic!("Expected HttpProviderCantBeCreated error");
        }
    }
}
