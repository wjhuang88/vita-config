use sqlite::{BindableWithIndex, Connection, State};
use std::collections::HashMap;
use std::str::FromStr;

use crate::datasource::{DataSource, FieldCondition, Value};
use crate::errors::{Error, Result};

impl From<&sqlite::Value> for Value {
    fn from(val: &sqlite::Value) -> Self {
        match val {
            sqlite::Value::Binary(vec) => Value::Binary(vec.clone()),
            sqlite::Value::Float(n) => Value::Float(*n),
            sqlite::Value::Integer(n) => Value::Integer(*n),
            sqlite::Value::String(s) => Value::String(s.clone()),
            sqlite::Value::Null => Value::Null,
        }
    }
}

impl From<Vec<u8>> for Value {
    fn from(val: Vec<u8>) -> Self {
        Value::Binary(val)
    }
}

impl From<f64> for Value {
    fn from(val: f64) -> Self {
        Value::Float(val)
    }
}

impl From<i64> for Value {
    fn from(val: i64) -> Self {
        Value::Integer(val)
    }
}

impl From<String> for Value {
    fn from(val: String) -> Self {
        Value::String(val)
    }
}

impl From<&str> for Value {
    fn from(val: &str) -> Self {
        Value::String(val.to_string())
    }
}

impl BindableWithIndex for Value {
    fn bind<T: sqlite::ParameterIndex>(
        self,
        statement: &mut sqlite::Statement,
        index: T,
    ) -> sqlite::Result<()> {
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
        Ok(Box::new(SqliteDataSource { connection }))
    }

    fn command<S>(&self, statement: S) -> Result<()>
    where
        S: AsRef<str>,
    {
        Ok(self.connection.execute(statement)?)
    }

    fn query<S, P>(&self, statement: S, params: P) -> Result<Vec<HashMap<String, Value>>>
    where
        S: AsRef<str>,
        P: AsRef<[Value]>,
    {
        let result = self
            .connection
            .prepare(statement)?
            .into_iter()
            .bind(params.as_ref())?;
        let column_names = result.column_names().to_vec();
        let vec: Vec<HashMap<String, Value>> = result
            .map(|row| {
                let mut map: HashMap<String, Value> = HashMap::new();
                if let Ok(row) = row {
                    for name in &column_names {
                        let value = &row[name.as_str()];
                        map.insert(name.clone(), value.into());
                    }
                }
                map
            })
            .collect();
        Ok(vec)
    }

    fn insert(&self, table: impl AsRef<str>, item: impl AsRef<[(String, Value)]>) -> Result<usize> {
        let mut names_part = String::new();
        let mut values_part = String::new();
        let mut values_list = Vec::new();
        let mut first = true;
        for (field, value) in item.as_ref() {
            values_list.push(value.clone());
            if !first {
                names_part.push(',');
                values_part.push(',');
            }
            names_part.push_str(field);
            values_part.push('?');
            first = false;
        }
        let sql = format!(
            "INSERT INTO {} ({}) VALUES ({})",
            table.as_ref(),
            names_part,
            values_part,
        );
        let mut st = self.connection.prepare(sql)?;
        st.reset()?;
        st.bind(values_list.as_slice())?;
        if let State::Done = st.next()? {
            return Ok(self.connection.change_count());
        }
        Err(Error::DB {
            cause: String::from_str("insert failed")?,
        })
    }

    fn update(
        &self,
        table: impl AsRef<str>,
        sets: impl AsRef<[(String, Value)]>,
        conditions: impl AsRef<[FieldCondition]>,
    ) -> Result<usize> {
        let mut set_part = String::new();
        let mut condition_part = String::new();
        let mut value_list = Vec::new();
        let mut first = true;
        for (field, value) in sets.as_ref() {
            value_list.push(value.clone());
            if !first {
                set_part.push(',');
            }
            set_part.push_str(field);
            set_part.push_str("=?");
            first = false;
        }

        let mut first = true;
        for condi in conditions.as_ref() {
            let (field, mut values) = convert_condition(condi.clone());
            value_list.append(&mut values);
            if !first {
                condition_part.push_str(" AND ");
            }
            condition_part.push_str(&field);
            first = false;
        }
        let sql = format!(
            "UPDATE {} SET {} WHERE {}",
            table.as_ref(),
            set_part,
            condition_part
        );
        let mut st = self.connection.prepare(sql)?;
        st.reset()?;
        st.bind(value_list.as_slice())?;
        if let State::Done = st.next()? {
            return Ok(self.connection.change_count());
        }
        Err(Error::DB {
            cause: String::from_str("update failed")?,
        })
    }
}

fn convert_condition(condition: FieldCondition) -> (String, Vec<Value>) {
    match condition {
        FieldCondition::Equal(field, value) => (format!("{} = ?", field), vec![value]),
        FieldCondition::NotEqual(field, value) => (format!("{} != ?", field), vec![value]),
        FieldCondition::GraterThan(field, value) => (format!("{} > ?", field), vec![value]),
        FieldCondition::LessThan(field, value) => (format!("{} < ?", field), vec![value]),
        FieldCondition::NotNull(field) => (format!("{} IS NOT NULL", field), vec![]),
        FieldCondition::IsNull(field) => (format!("{} IS NULL", field), vec![]),
        FieldCondition::Between(field, left, right) => {
            (format!("{} BETWEEN ? AND ?", field), vec![left, right])
        }
    }
}
