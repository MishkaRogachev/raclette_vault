#[cfg(test)]
mod tests {
    use test_case::test_case;
    use crate::core::utils;

    #[test_case(1_000_000_000_000, 0.000001)]
    #[test_case(1_000_000_000_000_000_000, 1.0)]
    #[test_case(1_000_000_000_000_000_000_000, 1000.0)]
    fn test_wei_to_eth(wei: u128, eth: f64) {
        let result = utils::wei_to_eth(wei.into());
        assert_eq!(result, eth);
    }

    #[test_case(0.000001, 1_000_000_000_000)]
    #[test_case(1.0, 1_000_000_000_000_000_000)]
    #[test_case(1000.0, 1_000_000_000_000_000_000_000)]
    fn test_eth_to_wei(eth: f64, wei: u128) {
        let result = utils::eth_to_wei(eth);
        assert_eq!(result.as_u128(), wei);
    }
}
