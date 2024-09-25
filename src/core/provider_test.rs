#[cfg(test)]
mod tests {
    use super::super::chain::Chain;
    use super::super::provider::Provider;

    #[tokio::test]
    async fn test_access_web3_provider() -> anyhow::Result<()> {
        let infura_token = match std::env::var("INFURA_TOKEN") {
            Ok(token) => token,
            Err(_) => {
                eprintln!("Skipping test: INFURA_TOKEN environment variable not set");
                return Ok(()); // Skip the test if the token is not set
            }
        };

        let account = web3::types::Address::from_low_u64_be(0);
        let provider = Provider::new(&infura_token, Chain::EthereumMainnet)?;

        let balance = provider.get_eth_balance(account).await?;
        assert_ne!(balance, 0.into());

        Ok(())
    }
}
