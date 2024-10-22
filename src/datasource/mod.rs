mod def;
mod sqlite;

use crate::config::{DataSourceDriver, DataSourceSpec};
use crate::errors::Result;

pub use def::{DataSource, FieldCondition, Value};

pub fn connect(db_spec: &DataSourceSpec) -> Result<Box<impl DataSource>> {
    let path = db_spec.path.clone();
    let result = match db_spec.driver {
        DataSourceDriver::Sqlite => sqlite::connect(path)?,
    };
    if let Some(init_sql) = &db_spec.init_script {
        result.command(init_sql)?;
    }
    Ok(result)
}
