use serde::{Deserialize, Serialize};
use super::config_structs::Spec;

use crate::datasource::DataSourceDriver;

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct DataSourceSpec {
    pub(crate) driver: DataSourceDriver,
    pub(crate) path: String,
    #[serde(rename = "init-sql")]
    pub(crate) init_sql: Option<String>,
    pub(crate) query: Option<Vec<QueryInstant>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct QueryInstant {
    pub(crate) name: String,
    pub(crate) sql: String,
    pub(crate) params: Option<Vec<QueryParams>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct QueryParams {
    pub(crate) name: String,
    #[serde(rename = "type")]
    pub(crate) p_type: String,
}

impl Spec for DataSourceSpec {}

#[cfg(test)]
mod tests {
    use std::fs::File;
    use std::path::PathBuf;
    use super::DataSourceSpec;

    use crate::config::load;
    use crate::datasource::DataSourceDriver;


    #[test]
    fn test_load_dataservice() {
        let conf_path = PathBuf::from("tests/config/test_datasource.yaml");
        let config = load(File::open(conf_path).unwrap()).unwrap();

        let spec: DataSourceSpec = config.map_spec().unwrap();
        println!("Deserialized map: {:?}", spec);
        assert_eq!(DataSourceDriver::Sqlite, spec.driver);
        assert_eq!(":memory:", spec.path);

        let query_list = spec.query.unwrap();

        let query_instant = &query_list[0];
        assert_eq!("get_all", query_instant.name);
        assert_eq!("SELECT * FROM t_test_01\n", query_instant.sql);
        assert!(query_instant.params.is_none());

        let query_instant = &query_list[1];
        assert_eq!("get_part", query_instant.name);
        assert_eq!("SELECT * FROM t_test_01 WHERE F_COUNT > ?\n", query_instant.sql);
        let param_instant = &query_instant.params.as_ref().unwrap()[0];
        assert_eq!("count", param_instant.name);
        assert_eq!("int", param_instant.p_type);
    }
}