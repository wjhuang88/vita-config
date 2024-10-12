mod config_structs;
mod dataservice_spec;
mod datasource_spec;

use crate::errors::Result;
pub(crate) use config_structs::Config;
pub(crate) use datasource_spec::{DataSourceDriver, DataSourceSpec, ParamType};
use std::io::Read;

pub fn load(reader: impl Read) -> Result<Config> {
    Config::load(reader)
}
