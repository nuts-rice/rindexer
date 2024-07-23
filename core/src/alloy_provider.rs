use std::{
    sync::Arc,
    time::{Duration, Instant},
};

use alloy::providers::{Provider, ProviderBuilder};

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
