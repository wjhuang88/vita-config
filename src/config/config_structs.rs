use serde::{de::DeserializeOwned, Deserialize, Serialize};
use serde_yml::Value;

use crate::errors::Result;

pub(crate) trait Spec {}

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct Config {
    pub(crate) kind: String,
    pub(crate) name: String,
    pub(crate) service: Option<Service>,
    pub(crate) spec: Value
}

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct Service {
    pub(crate) path: String,
    pub(crate) version: u16,
    pub(crate) transport: Transport,
}

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct Transport {
    pub(crate) protocol: String,
    pub(crate) style: String,
    pub(crate) readonly: bool,
}

impl Config {
    pub fn load<T: std::io::Read>(reader: T) -> Result<Config> {
        Ok(serde_yml::from_reader(reader)?)
    }

    pub fn map_spec<S: Spec + DeserializeOwned>(&self) -> Result<S> {
        Ok(serde_yml::from_value(self.spec.clone())?)
    }
}