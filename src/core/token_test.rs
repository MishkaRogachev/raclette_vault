#[cfg(test)]
mod tests {
    use web3::types::Address;

    use crate::core::{eth_chain::EthChain, token::{Token, TokenList}};

    #[test]
    fn test_flattern_token_list() {
        let tokens = vec![
            Token::new("Ethereum", "ETH"),
            Token::new("Dai Stablecoin", "DAI"),
            Token::new("USD Coin", "USDC"),
            Token::new("Tether USD", "USDT"),
            Token::new("Wrapped BTC", "WBTC"),
        ];

        let dummy_address = Address::from_low_u64_be(0);

        let mainnet_decimals = 13;
        let mainnet_tokens = tokens.iter().map(|token| token.clone().with_chain_data(
            EthChain::EthereumMainnet, dummy_address, mainnet_decimals)).collect::<TokenList>();

        let sepolia_decimals = 14;
        let sepolia_tokens = tokens.iter().map(|token| token.clone().with_chain_data(
            EthChain::EthereumSepolia, dummy_address, sepolia_decimals)).collect::<TokenList>();

        let arbitrum_decimals = 15;
        let arbitrum_tokens = tokens.iter().map(|token| token.clone().with_chain_data(
            EthChain::ArbitrumMainnet, dummy_address, arbitrum_decimals)).collect::<TokenList>();

        let final_list = Token::flatten_token_lists(vec![mainnet_tokens, sepolia_tokens, arbitrum_tokens]);

        assert_eq!(final_list.len(), tokens.len());
        for token in final_list {
            assert_eq!(token.chain_data.len(), 3);
            assert_eq!(token.chain_data.get(&EthChain::EthereumMainnet).unwrap().decimals, mainnet_decimals);
            assert_eq!(token.chain_data.get(&EthChain::EthereumSepolia).unwrap().decimals, sepolia_decimals);
            assert_eq!(token.chain_data.get(&EthChain::ArbitrumMainnet).unwrap().decimals, arbitrum_decimals);
        }
    }
}
