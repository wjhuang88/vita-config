mod config_structs;
mod dataservice_spec;
mod datasource_spec;

use std::io::Read;
use crate::errors::Result;
pub(crate) use config_structs::Config;

pub fn load(reader: impl Read) -> Result<Config> {
    Config::load(reader)
}

#[cfg(test)]
mod tests {

    use std::{fs::File, path::PathBuf};

    use crate::config::load;

    #[test]
    fn test_load_config() {
        let conf_path = PathBuf::from("tests/config/test_dataservice.yaml");
        let config = load(File::open(conf_path).unwrap()).unwrap();
        println!("Deserialized map: {:?}", config);

        assert_eq!("dataservice", config.kind);
        assert_eq!("test-data-service", config.name);

        let service = config.service.unwrap();
        assert_eq!("/test", service.path);
        assert_eq!(1, service.version);
        assert_eq!("http", service.transport.protocol);
        assert_eq!("restful", service.transport.style);
        assert_eq!(false, service.transport.readonly);
    }
}