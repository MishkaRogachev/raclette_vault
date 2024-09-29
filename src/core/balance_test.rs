#[cfg(test)]
mod tests {
    use super::super::balance::Balance;

    #[test]
    fn test_extend_balances() {
        let balances = vec![
            Balance::new(0.5, 1300.0, "ETH", false),
            Balance::new(50.0, 50.0, "USDC", false),
        ];
        let new_balances = vec![
            Balance::new(0.25, 0.0, "ETH", false),
            Balance::new(100.0, 100.0, "DAI", true),
        ];
        let extended_balances = Balance::extend_balances(&balances, &new_balances);
        assert_eq!(extended_balances.len(), 3);
        assert_eq!(extended_balances[0].value, 0.75);
    }
}
