use super::config_structs::{QueryParam, Spec};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct DataSourceSpec {
    pub(crate) driver: DataSourceDriver,
    pub(crate) path: String,
    #[serde(rename = "init-script")]
    pub(crate) init_script: Option<String>,
    pub(crate) params: Vec<QueryParam>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub(crate) enum DataSourceDriver {
    #[serde(rename = "sqlite")]
    Sqlite,
}

impl Spec for DataSourceSpec {}

#[cfg(test)]
mod tests {
    use super::DataSourceSpec;
    use std::fs::File;
    use std::path::PathBuf;

    use super::DataSourceDriver;
    use crate::config::{load, ParamType};

    #[test]
    fn test_load_datasource() {
        let conf_path = PathBuf::from("tests/config/test_datasource.yaml");
        let config = load(File::open(conf_path).unwrap()).unwrap();

        let spec: DataSourceSpec = config.map_spec().unwrap();
        println!("Deserialized map: {:?}", spec);
        assert_eq!(DataSourceDriver::Sqlite, spec.driver);
        assert_eq!(":memory:", spec.path);

        let param_list = spec.params;
        let param_instant_1 = &param_list[0];
        assert_eq!("ID", param_instant_1.name);
        assert_eq!(ParamType::Integer, param_instant_1.p_type);

        let param_instant_2 = &param_list[1];
        assert_eq!("F_NAME", param_instant_2.name);
        assert_eq!(ParamType::String, param_instant_2.p_type);
    }
}
