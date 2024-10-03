#[cfg(test)]
mod tests {
    use super::super::{balance::Balance, eth_chain::EthChain};

    #[test]
    fn test_extend_balances() {
        let balances = vec![
            Balance::new("ETH", EthChain::ArbitrumMainnet, 0.5, 1300.0),
            Balance::new("USDC", EthChain::ArbitrumMainnet, 50.0, 50.0),
        ];
        let new_balances = vec![
            Balance::new("ETH", EthChain::OptimismMainnet, 0.25, 0.0),
            Balance::new("DAI", EthChain::OptimismMainnet, 100.0, 100.0),
        ];
        let extended_balances = Balance::extend_balances(balances, &new_balances);
        assert_eq!(extended_balances.len(), 3);

        assert_eq!(extended_balances[0].currency, "ETH");
        assert_eq!(extended_balances[0].chain_values.len(), 2);
        let summary = extended_balances[0].summary();
        assert_eq!(summary.value, 0.75);
        assert_eq!(summary.usd_value, 1300.0);

        assert_eq!(extended_balances[1].currency, "USDC");
        assert_eq!(extended_balances[1].chain_values.len(), 1);
        let summary = extended_balances[1].summary();
        assert_eq!(summary.value, 50.0);
        assert_eq!(summary.usd_value, 50.0);

        assert_eq!(extended_balances[2].currency, "DAI");
        assert_eq!(extended_balances[2].chain_values.len(), 1);
        let summary = extended_balances[2].summary();
        assert_eq!(summary.value, 100.0);
        assert_eq!(summary.usd_value, 100.0);
    }
}
