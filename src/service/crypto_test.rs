#[cfg(test)]
mod tests {
    use crate::core::chain::Chain;
    use super::super::crypto::Crypto;

    #[test]
    fn test_crypto_chains() -> anyhow::Result<()> {
        let mut crypto = Crypto::new("http://localhost:8545");
        assert_eq!(crypto.get_active_chains().len(), 0);

        crypto.add_chain(Chain::EthereumMainnet)?;
        assert_eq!(crypto.get_active_chains().len(), 1);

        crypto.set_chains(vec![Chain::EthereumMainnet, Chain::ArbitrumMainnet])?;
        assert_eq!(crypto.get_active_chains().len(), 2);

        assert!(crypto.get_active_chains().contains(&Chain::EthereumMainnet));
        assert!(crypto.get_active_chains().contains(&Chain::ArbitrumMainnet));
        assert!(!crypto.get_active_chains().contains(&Chain::OptimismSepolia));

        Ok(())
    }
}
