#[cfg(test)]
mod tests {
    use super::super::manage;

    #[test]
    fn test_create_list_and_remove_databases() -> anyhow::Result<()> {
        let tmp = std::env::temp_dir().join("manage"); // Add `manage` to not interfere with other tests

        manage::remove_all_databases(&tmp)?;

        let addresses: Vec<web3::types::Address> = (104..113)
            .map(|i| web3::types::Address::from_low_u64_be(i))
            .collect();

        for &address in &addresses {
            _ = manage::open_database(&tmp, address, "12345678")?;
        }

        let listed_addresses = manage::list_databases(&tmp)?;
        assert_eq!(listed_addresses.len(), addresses.len());

        for address in &addresses {
            assert!(listed_addresses.contains(address), "Address {:?} not found", address);
        }

        for address in &addresses {
            manage::remove_database(&tmp, *address)?;
        }

        Ok(())
    }
}