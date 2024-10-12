use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use super::config_structs::Spec;

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct DataServiceSpec {
    pub(crate) datasource: String,
    pub(crate) table: String,
    pub(crate) field: HashMap<String, String>,
    pub(crate) computed: HashMap<String, String>,
}

impl Spec for DataServiceSpec {}

#[cfg(test)]
mod tests {
    use std::fs::File;
    use std::path::PathBuf;
    use super::DataServiceSpec;

    use crate::config::load;


    #[test]
    fn test_load_dataservice() {
        let conf_path = PathBuf::from("tests/config/test_dataservice.yaml");
        let config = load(File::open(conf_path).unwrap()).unwrap();

        let spec: DataServiceSpec = config.map_spec().unwrap();
        println!("Deserialized map: {:?}", spec);

        assert_eq!("sqlite_01", spec.datasource);
        assert_eq!("t_test_01", spec.table);
        assert_eq!("ID", spec.field["id"]);
        assert_eq!("F_NAME", spec.field["name"]);
        assert_eq!("F_UNIT_PRICE", spec.field["unitPrice"]);
        assert_eq!("F_COUNT", spec.field["count"]);
        assert_eq!("F_CREATE_TIME", spec.field["createTime"]);
        assert_eq!("unitPrice * count", spec.computed["price"]);
    }
}