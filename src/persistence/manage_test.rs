#[cfg(test)]
mod tests {
    use super::super::manage;

    #[test]
    fn test_create_and_remove_db() -> anyhow::Result<()> {
        let address = web3::types::Address::from_low_u64_be(13);
        _ = manage::open_database(std::env::temp_dir(), address, "12345678")?;

        manage::remove_database(std::env::temp_dir(), address)?;

        Ok(())
    }
}