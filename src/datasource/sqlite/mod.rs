mod datasource;

use crate::datasource::def::DataSource;
use crate::errors::Result;

pub(super) fn connect<T: AsRef<std::path::Path>>(path: T) -> Result<Box<impl DataSource>> {
    datasource::SqliteDataSource::connect(path)
}

#[cfg(test)]
mod tests {

    use crate::datasource::def::DataSource;

    #[test]
    fn test_connect_query() {
        let query = include_str!("../../../tests/sqlite/init.sql");
        
        let db = super::connect(":memory:").unwrap();
        db.command(query).unwrap();

        let query = "SELECT * FROM t_test_01 WHERE F_COUNT > ?";

        let query_result = db.query(query, &[3000i64.into()]).unwrap();

        for row in &query_result {
            for (name, value) in row {
                println!("{} = {:?}", name, value);
            }
        }
    }
}