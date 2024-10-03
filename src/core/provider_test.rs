#[cfg(test)]
mod tests {
    use test_case::test_case;
    use crate::core::token::Token;
    use super::super::chain::Chain;
    use super::super::provider::Provider;

    fn get_endpoint_url() -> anyhow::Result<String> {
        let infura_token = match std::env::var("INFURA_TOKEN") {
            Ok(token) => token,
            Err(_) => anyhow::bail!("Skipping test: INFURA_TOKEN environment variable not set")
        };
        Ok(format!("infura.io/v3/{}", infura_token))
    }

    #[test_case("0x6B175474E89094C44Da98b954EedeAC495271d0F", "DAI", "Dai Stablecoin", 18)]
    #[test_case("0xA0b86991c6218b36c1d19D4a2e9Eb0cE3606EB48", "USDC", "USD Coin", 6)]
    #[test_case("0xdAC17F958D2ee523a2206206994597C13D831ec7", "USDT", "Tether USD", 6)]
    #[tokio::test]
    async fn test_get_token_metadata(contract_address: &str, symbol: &str, name: &str, decimals: u16) -> anyhow::Result<()> {
        let endpoint_url = get_endpoint_url()?;

        let contract_address: web3::types::Address = contract_address.parse()?;
        let provider = Provider::new(&endpoint_url, Chain::EthereumMainnet)?;

        let token = provider.get_token_metadata(contract_address).await?;
        assert_eq!(token.symbol, symbol);
        assert_eq!(token.name, name);

        let token_chain_data = token.chain_data.get(&Chain::EthereumMainnet);
        assert!(token_chain_data.is_some());
        let token_chain_data = token_chain_data.unwrap();
        assert_eq!(token_chain_data.contract_address, contract_address);
        assert_eq!(token_chain_data.decimals, decimals);

        Ok(())
    }

    #[test_case(Chain::EthereumMainnet)]
    #[test_case(Chain::EthereumSepolia)]
    #[test_case(Chain::OptimismMainnet)]
    #[tokio::test]
    async fn test_get_eth_balance(chain: Chain) -> anyhow::Result<()> {
        let endpoint_url = get_endpoint_url()?;

        let account = web3::types::Address::from_low_u64_be(0);
        let provider = Provider::new(&endpoint_url, chain)?;

        let balance = provider.get_eth_balance(account).await?;
        assert_eq!(balance.currency, "ETH");
        assert_eq!(balance.chain_values.len(), 1);
        assert_eq!(balance.chain_values.contains_key(&chain), true);

        Ok(())
    }

    #[test_case("0x6B175474E89094C44Da98b954EedeAC495271d0F", "DAI", "Dai Stablecoin", 18)]
    #[test_case("0xA0b86991c6218b36c1d19D4a2e9Eb0cE3606EB48", "USDC", "USD Coin", 6)]
    #[test_case("0xdAC17F958D2ee523a2206206994597C13D831ec7", "USDT", "Tether USD", 6)]
    #[tokio::test]
    async fn test_get_token_balances(contract_address: &str, symbol: &str, name: &str, decimals: u16) -> anyhow::Result<()> {
        let endpoint_url = get_endpoint_url()?;

        let token = Token::new(name, symbol).with_chain_data(Chain::EthereumMainnet, contract_address.parse()?, decimals);

        let account = web3::types::Address::from_low_u64_be(0);
        let provider = Provider::new(&endpoint_url, Chain::EthereumMainnet)?;

        let balances = provider.get_token_balances(account, &vec![token]).await?;
        assert_eq!(balances.len(), 1);
        assert_eq!(balances[0].currency, symbol);

        Ok(())
    }
}
