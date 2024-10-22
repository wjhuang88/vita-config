use serde::{de::DeserializeOwned, Deserialize, Serialize};
use serde_yml::Value;

use crate::errors::Result;

pub(crate) trait Spec {}

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct Config {
    pub(crate) kind: ConfigKind,
    pub(crate) name: String,
    pub(crate) service: Option<Service>,
    pub(crate) spec: Value,
}

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct Service {
    pub(crate) path: String,
    pub(crate) version: u16,
    pub(crate) transport: Transport,
}

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct Transport {
    pub(crate) protocol: Protocol,
    pub(crate) style: ServiceStyle,
    pub(crate) readonly: bool,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub(crate) enum ConfigKind {
    #[serde(rename = "dataservice")]
    DataService,
    #[serde(rename = "datasource")]
    DataSource,
}

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct QueryParam {
    pub(crate) name: String,
    #[serde(rename = "type")]
    pub(crate) p_type: ParamType,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub(crate) enum ParamType {
    #[serde(rename = "float")]
    Float,
    #[serde(rename = "int")]
    Integer,
    #[serde(rename = "string")]
    String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub(crate) enum ServiceStyle {
    #[serde(rename = "restful")]
    Restful,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub(crate) enum Protocol {
    #[serde(rename = "http")]
    Http,
}

impl Config {
    pub fn load<T: std::io::Read>(reader: T) -> Result<Config> {
        Ok(serde_yml::from_reader(reader)?)
    }

    pub fn map_spec<S: Spec + DeserializeOwned>(self) -> Result<S> {
        Ok(serde_yml::from_value(self.spec)?)
    }
}

#[cfg(test)]
mod tests {

    use std::{fs::File, path::PathBuf};

    use crate::config::config_structs::{ConfigKind, Protocol, ServiceStyle};

    #[test]
    fn test_load_config() {
        let conf_path = PathBuf::from("tests/config/test_dataservice.yaml");
        let config = super::Config::load(File::open(conf_path).unwrap()).unwrap();
        println!("Deserialized map: {:?}", config);

        assert_eq!(ConfigKind::DataService, config.kind);
        assert_eq!("test-data-service", config.name);

        let service = config.service.unwrap();
        assert_eq!("/test", service.path);
        assert_eq!(1, service.version);
        assert_eq!(Protocol::Http, service.transport.protocol);
        assert_eq!(ServiceStyle::Restful, service.transport.style);
        assert!(!service.transport.readonly);
    }
}
