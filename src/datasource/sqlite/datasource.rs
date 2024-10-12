use sqlite::{BindableWithIndex, Connection};
use std::collections::HashMap;

use crate::datasource::def::{DataSource, Value};
use crate::errors::Result;

impl Into<Value> for &sqlite::Value {
    fn into(self) -> Value {
        match self {
            sqlite::Value::Binary(vec) => Value::Binary(vec.clone()),
            sqlite::Value::Float(n) => Value::Float(*n),
            sqlite::Value::Integer(n) => Value::Integer(*n),
            sqlite::Value::String(s) => Value::String(s.clone()),
            sqlite::Value::Null => Value::Null,
        }
    }
}

impl Into<Value> for Vec<u8> {
    fn into(self) -> Value {
        Value::Binary(self)
    }
}

impl Into<Value> for f64 {
    fn into(self) -> Value {
        Value::Float(self)
    }
}

impl Into<Value> for i64 {
    fn into(self) -> Value {
        Value::Integer(self)
    }
}

impl Into<Value> for String {
    fn into(self) -> Value {
        Value::String(self)
    }
}

impl BindableWithIndex for Value {
    fn bind<T: sqlite::ParameterIndex>(self, statement: &mut sqlite::Statement, index: T) -> sqlite::Result<()> {
        let sqlite_value = match self {
            Value::Binary(vec) => sqlite::Value::Binary(vec),
            Value::Float(n) => sqlite::Value::Float(n),
            Value::Integer(n) => sqlite::Value::Integer(n),
            Value::String(s) => sqlite::Value::String(s),
            Value::Null => sqlite::Value::Null,
        };
        sqlite_value.bind(statement, index)
    }
}

pub(super) struct SqliteDataSource {
    connection: Connection,
}

impl DataSource for SqliteDataSource {
    fn connect<T: AsRef<std::path::Path>>(path: T) -> Result<Box<Self>> {
        let connection = sqlite::open(path)?;
        Ok(Box::new(SqliteDataSource {
            connection,
        }))
    }
    
    fn command<S>(&self, statement: S) -> Result<()>
    where
        S: AsRef<str>,
    {
        Ok(self.connection.execute(statement)?)
    }
    
    fn query<S>(&self, statement: S, params: &[Value]) -> Result<Vec<HashMap<String, Value>>>
    where
        S: AsRef<str>,
    {
        let result = self.connection.prepare(statement)?.into_iter().bind(params)?;
        let column_names = result.column_names().to_vec();
        let vec: Vec<HashMap<String, Value>> = result.map(|row| {
                let mut map: HashMap<String, Value> = HashMap::new();
                if let Ok(row) = row {
                    for name in &column_names {
                        let value = &row[name.as_str()];
                        map.insert(name.clone(), value.into());
                    }
                }
                map
            }).collect();
        Ok(vec)
    }
    
    fn raw_query<S>(&self, statement: S) -> Result<Vec<HashMap<String, Value>>>
    where
        S: AsRef<str>,
    {
        let result = self.connection.prepare(statement)?.into_iter();
        let column_names = result.column_names().to_vec();
        let vec: Vec<HashMap<String, Value>> = result.map(|row| {
                let mut map: HashMap<String, Value> = HashMap::new();
                if let Ok(row) = row {
                    for name in &column_names {
                        let value = &row[name.as_str()];
                        map.insert(name.clone(), value.into());
                    }
                }
                map
            }).collect();
        Ok(vec)
    }

}