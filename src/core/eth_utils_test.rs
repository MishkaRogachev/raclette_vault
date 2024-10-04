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

    #[test_case("0x0000000000000000000000000000000000000084", Ok(web3::types::Address::from_low_u64_be(132)))]
    #[test_case("12345678901234567890123456789012345678900", Err(eth_utils::ERR_INVALID_ADDRESS_PREFIX))]
    #[test_case("0x123456789012345678901234567890123456789", Err(eth_utils::ERR_INVALID_ADDRESS_LENGTH))]
    #[test_case("0x1234567890123456789012345678901234567890x", Err(eth_utils::ERR_INVALID_ADDRESS_LENGTH))]
    #[test_case("0x123456789012345678901234567890123456789z", Err(eth_utils::ERR_INVALID_ADDRESS))]
    fn test_str_to_eth_address(address: &str, expected: Result<web3::types::Address, &str>) {
        match eth_utils::str_to_eth_address(address) {
            Ok(valid_address) => assert_eq!(valid_address, expected.unwrap()),
            Err(err) => assert_eq!(err.to_string(), expected.unwrap_err()),
        }
    }
}
