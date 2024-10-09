#[cfg(test)]
mod tests {
    use crate::core::{eth_chain, transaction};
    use super::super::db::Db;

    fn create_test_db() -> anyhow::Result<Db> {
        let db_name = format!("test_raclette_trasnsaction_db_{}", uuid::Uuid::new_v4().to_string());
        let mut path = std::env::temp_dir();
        path.push(db_name);

        let config = sled::Config::new().temporary(true).path(path);
        let db = config.open()?;

        Db::open(db, "12345678")
    }

    #[test]
    fn test_transactions_db() -> anyhow::Result<()> {
        let db = create_test_db()?;

        let account = web3::types::Address::from_low_u64_be(12);
        let other = web3::types::Address::from_low_u64_be(13);

        let first = transaction::TransactionResult {
            tx_hash: "0x9f3be51fb7b3f83bc7d4a37d3b5f4bb5d4c82b898e8b5c35c6a7ec5e93371c2d".parse()?,
            from: Some(account),
            to: Some(other),
            amount: 1.0,
            fee: 0.01,
            chain: eth_chain::EthChain::EthereumMainnet,
            block_number: Some(18000000.into()),
            status: transaction::TransactionStatus::Successed,
        };
        db.save_transaction(account, &first)?;

        let mut second = transaction::TransactionResult {
            tx_hash: "0xb3c4a8ec44b5d8b9925b4cb1fc65666c66d29c07ac1faac5740b227fdbb6f5ed".parse()?,
            from: Some(other),
            to: Some(account),
            amount: 2.0,
            fee: 0.02,
            chain: eth_chain::EthChain::OptimismMainnet,
            block_number: Some(17500000.into()),
            status: transaction::TransactionStatus::Pending,
        };
        db.save_transaction(account, &second)?;

        // Update the transaction to check synthetic key
        second.amount = 3.0;
        db.save_transaction(account, &second)?;

        let transactions = db.get_transactions(account, 0, 1)?;
        assert_eq!(transactions.len(), 1);
        assert_eq!(transactions[0], first);

        let transactions = db.get_transactions(account, 1, 1)?;
        assert_eq!(transactions.len(), 1);
        assert_eq!(transactions[0], second);

        let transactions = db.get_transactions(account, 2, 3)?;
        assert_eq!(transactions.len(), 0);

        Ok(())
    }
}
