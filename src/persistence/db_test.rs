#[cfg(test)]
mod tests {
    use crate::persistence::db::Db;
    use serde::{Serialize, Deserialize};

    #[derive(Serialize, Deserialize, Debug, PartialEq)]
    struct TestData {
        field1: String,
        field2: u32,
    }

    fn create_test_db() -> Db {
        let mut path = std::env::temp_dir();
        path.push("test_raclette_db");
        Db::open(path.to_str().unwrap(), "12345678")
            .expect("Can't open a database")
    }

    #[test]
    fn test_insert_and_get() {
        let db = create_test_db();
        let test_data = TestData {
            field1: "Test".to_string(),
            field2: 42,
        };

        db.insert(b"test_key", &test_data).expect("Insert failed");

        let retrieved_data: Option<TestData> = db.get(b"test_key").expect("Get failed");
        assert_eq!(retrieved_data, Some(test_data));
    }

    #[test]
    fn test_insert_and_remove() {
        let db = create_test_db();
        let test_data = TestData {
            field1: "Test remove".to_string(),
            field2: 99,
        };

        db.insert(b"remove_key", &test_data).expect("Insert failed");
        db.remove(b"remove_key").expect("Remove failed");

        let retrieved_data = db.get::<TestData>(b"remove_key")
            .expect("Get failed");
        assert_eq!(retrieved_data, None);
    }

    #[test]
    fn test_get_nonexistent_key() {
        let db = create_test_db();
        let retrieved_data = db.get::<TestData>(b"nonexistent_key")
            .expect("Get failed");
        assert_eq!(retrieved_data, None);
    }
}
