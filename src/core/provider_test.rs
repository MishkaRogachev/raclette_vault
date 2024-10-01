#[cfg(test)]
mod tests {
    use test_case::test_case;
    use crate::core::token::Token;
    use super::super::chain::Chain;
    use super::super::provider::Provider;

    #[test_case(Chain::EthereumMainnet)]
    #[test_case(Chain::EthereumSepolia)]
    #[tokio::test]
    async fn test_access_web3_provider(chain: Chain) -> anyhow::Result<()> {
        let infura_token = match std::env::var("INFURA_TOKEN") {
            Ok(token) => token,
            Err(_) => {
                eprintln!("Skipping test: INFURA_TOKEN environment variable not set");
                return Ok(()); // Skip the test if the token is not set
            }
        };
        let endpoint_url = format!("infura.io/v3/{}", infura_token);

        let tokens = vec![
            Token::new("Ethereum", "ETH", "0xEeeeeEeeeEeEeeEeEeEeeEEEeeeeEeeeeeeeEEeE", 18),
        ];

        let account = web3::types::Address::from_low_u64_be(0);
        let provider = Provider::new(&endpoint_url, chain)?;

        let balances = provider.get_balances(account, &tokens).await?;
        assert_eq!(balances.len(), tokens.len());
        assert_eq!(balances[0].currency, "ETH");
        assert_ne!(balances[0].value, 0.0);

        Ok(())
    }

    #[test_case("0x6B175474E89094C44Da98b954EedeAC495271d0F", "DAI", "Dai Stablecoin", 18)]
    #[test_case("0xA0b86991c6218b36c1d19D4a2e9Eb0cE3606EB48", "USDC", "USD Coin", 6)]
    #[test_case("0xdAC17F958D2ee523a2206206994597C13D831ec7", "USDT", "Tether USD", 6)]
    #[tokio::test]
    async fn test_get_token_metadata(contract_address: &str, symbol: &str, name: &str, decimals: u16) -> anyhow::Result<()> {
        let infura_token = match std::env::var("INFURA_TOKEN") {
            Ok(token) => token,
            Err(_) => {
                eprintln!("Skipping test: INFURA_TOKEN environment variable not set");
                return Ok(()); // Skip the test if the token is not set
            }
        };
        let endpoint_url = format!("infura.io/v3/{}", infura_token);

        let contract_address: web3::types::Address = contract_address.parse()?;
        let provider = Provider::new(&endpoint_url, Chain::EthereumMainnet)?;

        let token = provider.get_token_metadata(contract_address).await?;
        assert_eq!(token.symbol, symbol);
        assert_eq!(token.name, name);
        assert_eq!(token.decimals, decimals);

        Ok(())
    }
}
