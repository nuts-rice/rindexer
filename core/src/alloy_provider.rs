use std::{
    sync::Arc,
    time::{Duration, Instant},
};

use alloy::{
    eips::{BlockId, BlockNumberOrTag},
    primitives::*,
    providers::{network::AnyNetwork, Provider, ProviderBuilder, RootProvider},
    rpc::types::{Block, BlockTransactionsKind},
    transports::http::Http,
};
use thiserror::Error;
use tokio::sync::Mutex;
#[derive(Error, Debug)]
pub enum AlloyProviderError {
    #[error("Provider error for {0}: {1} ")]
    ProviderCreatorError(String, String),
}
// #[derive(Debug)]
struct AlloyProvider {
    provider: Arc<RootProvider<Http<reqwest::Client>>>,
    cache: Mutex<Option<(Instant, Arc<Block>)>>,
    pub max_block_range: Option<U64>,
}

impl AlloyProvider {
    pub fn new(
        provider: Arc<RootProvider<Http<reqwest::Client>>>,
        max_block_range: Option<U64>,
    ) -> Self {
        AlloyProvider { provider, cache: Mutex::new(None), max_block_range }
    }

    pub async fn get_latest_block(&self) -> Result<Option<Arc<Block>>, AlloyProviderError> {
        let mut cache_guard = self.cache.lock().await;

        if let Some((timestamp, block)) = &*cache_guard {
            if timestamp.elapsed() < Duration::from_millis(300) {
                return Ok(Some(Arc::clone(block)));
                // return Ok(Some(Arc::clone(block)));
            }
        }

        let latest_block =
            self.provider.get_block(BlockId::latest(), BlockTransactionsKind::Full).await.unwrap();
        if let Some(block) = latest_block {
            let arc_block = Arc::new(block);
            *cache_guard = Some((Instant::now(), Arc::clone(&arc_block)));
            return Ok(Some(arc_block))
        } else {
            *cache_guard = None;
        }
        Ok(None)
    }
}

pub fn create_client(
    rpc_url: &str,
    max_block_range: Option<U64>,
) -> Result<AlloyProvider, AlloyProviderError> {
    let url = rpc_url.parse().unwrap();
    let http_provider = ProviderBuilder::new().on_http(url);
    Ok(AlloyProvider::new(Arc::new(http_provider), max_block_range))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_alloy_provider() {
        use alloy::{
            primitives::*,
            providers::{Provider, ProviderBuilder},
        };
        let rpc_url = "https://eth.merkle.io".parse().unwrap();
        let http_provider = ProviderBuilder::new().on_http(rpc_url);
        let block_num = http_provider.get_block_number().await;
        println!("block_num: {:?}", block_num);
    }
}
