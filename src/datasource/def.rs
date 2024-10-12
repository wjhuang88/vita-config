use std::collections::HashMap;

use crate::errors::Result;

pub trait DataSource {
    fn connect<T: AsRef<std::path::Path>>(path: T) -> Result<Box<Self>>;

    fn command<S>(&self, statement: S) -> Result<()>
    where
        S: AsRef<str>;

    fn query<S>(&self, statement: S, params: &[Value]) -> Result<Vec<HashMap<String, Value>>>
    where
        S: AsRef<str>;

    fn raw_query<S>(&self, statement: S) -> Result<Vec<HashMap<String, Value>>>
    where
        S: AsRef<str>;
}

#[derive(Clone, Debug, Default, PartialEq)]
pub enum Value {
    /// Binary data.
    Binary(Vec<u8>),
    /// A floating-point number.
    Float(f64),
    /// An integer number.
    Integer(i64),
    /// A string.
    String(String),
    /// A null value.
    #[default]
    Null,
}
