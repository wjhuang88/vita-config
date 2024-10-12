mod datasource;

use crate::datasource::def::DataSource;
use crate::errors::Result;

pub(super) fn connect<T: AsRef<std::path::Path>>(path: T) -> Result<Box<impl DataSource>> {
    datasource::SqliteDataSource::connect(path)
}

#[cfg(test)]
mod tests {

    use crate::datasource::{DataSource, Value};

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
}
