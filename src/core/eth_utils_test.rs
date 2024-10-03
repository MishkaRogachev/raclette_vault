#[cfg(test)]
mod tests {
    use web3::types::U256;
    use test_case::test_case;
    use crate::core::eth_utils;

    #[test_case(1_000_000_000_000, 0.000001)]
    #[test_case(1_000_000_000_000_000_000, 1.0)]
    #[test_case(1_000_000_000_000_000_000_000, 1000.0)]
    fn test_wei_to_eth_and_back(wei: u128, eth: f64) {
        let wei = U256::from(wei);

        let eth_back = eth_utils::wei_to_eth(wei);
        assert_eq!(eth_back, eth);

        let wei_back = eth_utils::eth_to_wei(eth_back);
        assert_eq!(wei_back, wei);
    }
}
