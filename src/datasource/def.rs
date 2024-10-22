use serde::Serialize;
use std::collections::HashMap;

use crate::errors::Result;

pub trait DataSource {
    fn connect<T: AsRef<std::path::Path>>(path: T) -> Result<Box<Self>>;

    fn command<S>(&self, statement: S) -> Result<()>
    where
        S: AsRef<str>;

    fn query<S, P>(&self, statement: S, params: P) -> Result<Vec<HashMap<String, Value>>>
    where
        S: AsRef<str>,
        P: AsRef<[Value]>;

    fn insert(&self, table: impl AsRef<str>, item: impl AsRef<[(String, Value)]>) -> Result<usize>;

    fn update(
        &self,
        table: impl AsRef<str>,
        sets: impl AsRef<[(String, Value)]>,
        conditions: impl AsRef<[FieldCondition]>,
    ) -> Result<usize>;
}

#[derive(Clone, Debug)]
pub enum FieldCondition {
    Equal(String, Value),
    NotEqual(String, Value),
    GraterThan(String, Value),
    LessThan(String, Value),
    NotNull(String),
    IsNull(String),
    Between(String, Value, Value),
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
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
