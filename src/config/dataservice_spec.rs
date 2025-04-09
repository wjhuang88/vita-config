use super::config_structs::{QueryParam, Spec};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct DataServiceSpec {
    pub(crate) datasource: String,
    pub(crate) table: String,
    pub(crate) field: HashMap<String, QueryParam>,
    pub(crate) computed: HashMap<String, String>,
}

impl Spec for DataServiceSpec {}

#[cfg(test)]
mod tests {
    use super::DataServiceSpec;
    use std::fs::File;
    use std::path::PathBuf;

    use crate::config::{load, ParamType};

    #[test]
    fn test_load_dataservice() {
        let conf_path = PathBuf::from("tests/config/test_dataservice.yaml");
        let config = load(File::open(conf_path).unwrap()).unwrap();

        let spec: DataServiceSpec = config.map_spec().unwrap();
        println!("Deserialized map: {:?}", spec);

        assert_eq!("sqlite_01", spec.datasource);
        assert_eq!("t_test_01", spec.table);

        assert_eq!("ID", spec.field["id"].name);
        assert_eq!(ParamType::Integer, spec.field["id"].p_type);

        assert_eq!("F_NAME", spec.field["name"].name);
        assert_eq!(ParamType::String, spec.field["name"].p_type);

        assert_eq!("F_UNIT_PRICE", spec.field["unitPrice"].name);
        assert_eq!(ParamType::Float, spec.field["unitPrice"].p_type);

        assert_eq!("F_COUNT", spec.field["count"].name);
        assert_eq!(ParamType::Integer, spec.field["count"].p_type);

        assert_eq!("F_CREATE_TIME", spec.field["createTime"].name);
        assert_eq!(ParamType::String, spec.field["createTime"].p_type);

        assert_eq!("unitPrice * count", spec.computed["price"]);
    }
}
