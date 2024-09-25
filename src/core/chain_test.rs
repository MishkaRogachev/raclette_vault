#[cfg(test)]
mod tests {
    use test_case::test_case;
    use super::super::chain::Chain;

    #[test_case(Chain::EthereumMainnet, false)]
    #[test_case(Chain::Optimism, false)]
    #[test_case(Chain::Arbitrum, false)]
    #[test_case(Chain::Goerli, true)]
    #[test_case(Chain::Kovan, true)]
    #[test_case(Chain::Rinkeby, true)]
    fn test_chain_utility(chain: Chain, is_test_net: bool) -> anyhow::Result<()> {
        let infura_token = "test";

        assert!(chain.get_infura_url(&infura_token).starts_with("https://"));
        assert!(!chain.get_display_name().is_empty());
        assert_eq!(chain.is_test_network(), is_test_net);

        Ok(())
    }
}
