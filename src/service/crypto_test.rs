#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use crate::{core::eth_chain::EthChain, persistence::db::Db};
    use super::super::crypto::Crypto;

    fn create_test_db() -> anyhow::Result<Db> {
        let db_name = format!("test_raclette_crypto_{}", uuid::Uuid::new_v4().to_string());
        let mut path = std::env::temp_dir();
        path.push(db_name);

        let config = sled::Config::new().temporary(true).path(path);
        let db = config.open()?;

        Db::open(db, "12345678")
    }

    #[tokio::test]
    async fn test_crypto_chains() -> anyhow::Result<()> {
        let db = Arc::new(create_test_db()?);
        let mut crypto = Crypto::new(db, "http://localhost:8545");
        assert_eq!(crypto.get_active_networks().len(), 0);

        crypto.load_active_networks()?;
        assert_eq!(crypto.get_active_networks().len(), 0);

        crypto.save_active_networks(vec![EthChain::EthereumMainnet, EthChain::ArbitrumMainnet]).await?;
        assert_eq!(crypto.get_active_networks().len(), 2);

        assert!(crypto.get_active_networks().contains(&EthChain::EthereumMainnet));
        assert!(crypto.get_active_networks().contains(&EthChain::ArbitrumMainnet));
        assert!(!crypto.get_active_networks().contains(&EthChain::OptimismSepolia));

        Ok(())
    }
}
