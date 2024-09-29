#[cfg(test)]
mod tests {
    use test_case::test_case;
    use super::super::chain::Chain;

    #[test_case(Chain::EthereumMainnet, false)]
    #[test_case(Chain::EthereumSepolia, true)]
    #[test_case(Chain::OptimismMainnet, false)]
    #[test_case(Chain::OptimismSepolia, true)]
    #[test_case(Chain::ArbitrumMainnet, false)]
    #[test_case(Chain::ArbitrumSepolia, true)]
    fn test_chain_utility(chain: Chain, is_test_net: bool) -> anyhow::Result<()> {
        let endpoint_url = "test";

        assert!(chain.finalize_endpoint_url(&endpoint_url).starts_with("https://"));
        assert!(!chain.get_display_name().is_empty());
        assert_eq!(chain.is_test_network(), is_test_net);
        assert!(chain.get_chainlink_contract_address().to_string().starts_with("0x"));

        Ok(())
    }
}
