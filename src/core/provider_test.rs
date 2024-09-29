#[cfg(test)]
mod tests {
    use test_case::test_case;
    use super::super::chain::Chain;
    use super::super::provider::Provider;

    #[tokio::test]
    #[test_case(Chain::EthereumMainnet)]
    #[test_case(Chain::EthereumSepolia)]
    async fn test_access_web3_provider(chain: Chain) -> anyhow::Result<()> {
        let infura_token = match std::env::var("INFURA_TOKEN") {
            Ok(token) => token,
            Err(_) => {
                eprintln!("Skipping test: INFURA_TOKEN environment variable not set");
                return Ok(()); // Skip the test if the token is not set
            }
        };
        let endpoint_url = format!("infura.io/v3/{}", infura_token);

        let account = web3::types::Address::from_low_u64_be(0);
        let provider = Provider::new(&endpoint_url, chain)?;

        let balance = provider.get_eth_balance(account).await?;
        assert_eq!(balance.currency, "ETH");
        assert_ne!(balance.value, 0.0);

        Ok(())
    }
}
