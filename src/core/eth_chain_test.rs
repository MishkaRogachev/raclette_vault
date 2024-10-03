#[cfg(test)]
mod tests {
    use test_case::test_case;
    use super::super::eth_chain::EthChain;

    #[test_case(EthChain::EthereumMainnet, false)]
    #[test_case(EthChain::EthereumSepolia, true)]
    #[test_case(EthChain::OptimismMainnet, false)]
    #[test_case(EthChain::OptimismSepolia, true)]
    #[test_case(EthChain::ArbitrumMainnet, false)]
    #[test_case(EthChain::ArbitrumSepolia, true)]
    fn test_chain_utility(chain: EthChain, is_test_net: bool) -> anyhow::Result<()> {
        let endpoint_url = "test";

        assert!(chain.finalize_endpoint_url(&endpoint_url).starts_with("https://"));
        assert!(!chain.get_display_name().is_empty());
        assert_eq!(chain.is_test_network(), is_test_net);
        assert!(chain.get_chainlink_contract_address().to_string().starts_with("0x"));

        Ok(())
    }
}
