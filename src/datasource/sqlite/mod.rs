mod datasource;

use crate::datasource::def::DataSource;
use crate::errors::Result;

pub(super) fn connect<T: AsRef<std::path::Path>>(path: T) -> Result<Box<impl DataSource>> {
    datasource::SqliteDataSource::connect(path)
}

#[cfg(test)]
mod tests {

    use crate::datasource::{DataSource, FieldCondition, Value};

    #[test]
    fn test_connect_query() {
        let query = include_str!("../../../tests/sqlite/init.sql");

        let db = super::connect(":memory:").unwrap();
        db.command(query).unwrap();

        let query = "SELECT * FROM t_test_01 WHERE F_COUNT > ?";

        let query_result = db.query(query, &[3000i64.into()]).unwrap();

        assert_eq!(1, query_result.len());
        println!("query result: {:?}", query_result);

        assert_eq!(Value::Integer(3), query_result[0]["ID"]);
        assert_eq!(
            Value::String("Goods_03".to_string()),
            query_result[0]["F_NAME"]
        );
        assert_eq!(Value::Float(1.5f64), query_result[0]["F_UNIT_PRICE"]);
        assert_eq!(Value::Integer(6000), query_result[0]["F_COUNT"]);
    }

    #[test]
    fn test_insert() {
        let query = include_str!("../../../tests/sqlite/init.sql");

        let db = super::connect(":memory:").unwrap();
        db.command(query).unwrap();

        // (ID, F_NAME, F_UNIT_PRICE, F_COUNT, F_CREATE_TIME): (1, 'Goods_01', 3.2, 3000, CURRENT_TIMESTAMP)
        let insert = db
            .insert(
                "t_test_01",
                &[
                    ("ID".to_string(), 5i64.into()),
                    ("F_NAME".to_string(), "Goods_05".into()),
                    ("F_UNIT_PRICE".to_string(), 3.3f64.into()),
                    ("F_COUNT".to_string(), 1000i64.into()),
                ],
            )
            .unwrap();

        assert_eq!(1, insert);

        let query = "SELECT * FROM t_test_01 WHERE F_COUNT = ?";
        let query_result = db.query(query, &[1000i64.into()]).unwrap();

        assert_eq!(1, query_result.len());
        println!("query result: {:?}", query_result);

        assert_eq!(Value::Integer(5), query_result[0]["ID"]);
        assert_eq!(
            Value::String("Goods_05".to_string()),
            query_result[0]["F_NAME"]
        );
        assert_eq!(Value::Float(3.3f64), query_result[0]["F_UNIT_PRICE"]);
        assert_eq!(Value::Integer(1000), query_result[0]["F_COUNT"]);
    }

    #[test]
    fn test_update() {
        let query = include_str!("../../../tests/sqlite/init.sql");

        let db = super::connect(":memory:").unwrap();
        db.command(query).unwrap();

        let query = "SELECT * FROM t_test_01 WHERE F_COUNT = ?";
        let query_result = db.query(query, &[2000i64.into()]).unwrap();
        assert_eq!(2, query_result.len());
        println!("before result: {:?}", query_result);
        assert_eq!(Value::Float(4.5f64), query_result[0]["F_UNIT_PRICE"]);
        assert_eq!(Value::Float(5.7f64), query_result[1]["F_UNIT_PRICE"]);

        let update = db
            .update(
                "t_test_01",
                &[("F_UNIT_PRICE".to_string(), 10.0f64.into())],
                &[FieldCondition::Equal(
                    "F_COUNT".to_string(),
                    Value::Integer(2000),
                )],
            )
            .unwrap();
        assert_eq!(2, update);

        let query = "SELECT * FROM t_test_01 WHERE F_COUNT = ?";
        let query_result = db.query(query, &[2000i64.into()]).unwrap();

        assert_eq!(2, query_result.len());
        println!("after result: {:?}", query_result);
        assert_eq!(Value::Float(10f64), query_result[0]["F_UNIT_PRICE"]);
        assert_eq!(Value::Float(10f64), query_result[1]["F_UNIT_PRICE"]);
    }
}
