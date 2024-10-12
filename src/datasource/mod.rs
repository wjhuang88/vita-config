mod def;
mod sqlite;

use serde::{Deserialize, Serialize};

pub use def::DataSource;
use crate::errors::Result;

pub fn connect_sqlite<T: AsRef<std::path::Path>>(path: T) -> Result<Box<impl DataSource>> {
    sqlite::connect(path)
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub enum DataSourceDriver {
    #[serde(rename = "sqlite")]
    Sqlite,
}