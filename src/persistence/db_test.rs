#[cfg(test)]
mod tests {
    use super::super::db::Db;
    use serde::{Serialize, Deserialize};

    #[derive(Serialize, Deserialize, Debug, PartialEq)]
    struct TestData {
        field1: String,
        field2: u32,
    }

    fn create_test_db() -> anyhow::Result<Db> {
        let db_name = format!("test_raclette_db_{}", uuid::Uuid::new_v4().to_string());
        let mut path = std::env::temp_dir();
        path.push(db_name);

        let config = sled::Config::new().temporary(true).path(path);
        let db = config.open()?;

        Db::open(db, "12345678")
    }

    #[test]
    fn test_insert_and_get() -> anyhow::Result<()> {
        let db = create_test_db()?;
        let test_data = TestData {
            field1: "Test".to_string(),
            field2: 42,
        };

        db.insert(b"test_key", &test_data)?;

        let retrieved_data: Option<TestData> = db.get(b"test_key")?;
        assert_eq!(retrieved_data, Some(test_data));
        Ok(())
    }

    #[test]
    fn test_insert_and_remove() -> anyhow::Result<()> {
        let db = create_test_db()?;
        let test_data = TestData {
            field1: "Test remove".to_string(),
            field2: 99,
        };

        db.insert(b"remove_key", &test_data)?;
        db.remove(b"remove_key")?;

        let retrieved_data = db.get::<TestData>(b"remove_key")?;
        assert_eq!(retrieved_data, None);
        Ok(())
    }

    #[test]
    fn test_get_nonexistent_key() -> anyhow::Result<()> {
        let db = create_test_db()?;
        let retrieved_data = db.get::<TestData>(b"nonexistent_key")?;
        assert_eq!(retrieved_data, None);
        Ok(())
    }
}
