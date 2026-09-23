use std::collections::HashSet;

#[derive(Debug)]
pub enum DBError {
    TableAlreadyExists(String),
    TableNotFound(String),
    InvalidValueType(ValueType),
    InvalidRow,
    MissingRow(String),
    EntryNotFound,
    ConstraintNotMet,
}

#[derive(Debug)]
pub enum DBResult {
    RowFound,
    RowNotFound,
}

#[derive(Debug, PartialEq, PartialOrd, Eq, Ord, Clone, Copy)]
pub enum ValueType {
    Int,
    Float,
    String,
    Bool,
    Vec,
    Null,
}

#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub enum Constraints {
    Min(i64),
    Max(i64),
    // MinF(f64),
    // MaxF(f64),
    Contains(String),
    NotContains(String),
    Between(i64, i64),
    StartsWith(String),
    NotStartsWith(String),
    Unique,
}

impl Constraints {
    pub fn check(&self, value: &Value, uniques: Option<&HashSet<Value>>) -> bool {
        match self {
            Constraints::Min(x) => value > &Value::Int(*x),
            Constraints::Max(x) => value < &Value::Int(*x),
            // Constraints::MinF(x) => value > &Value::Float(*x),
            // Constraints::MaxF(x) => value < &Value::Float(*x),
            Constraints::Between(x, y) => &Value::Int(*x) < value && value < &Value::Int(*y),
            Constraints::Contains(str) => {
                Into::<String>::into(value.clone()).contains(str.as_str())
            }
            Constraints::NotContains(str) => {
                !Into::<String>::into(value.clone()).contains(str.as_str())
            }
            Constraints::StartsWith(str) => Into::<String>::into(value.clone()).starts_with(str),
            Constraints::NotStartsWith(str) => {
                !Into::<String>::into(value.clone()).starts_with(str)
            }
            Constraints::Unique => {
                return match uniques {
                    Some(set) => !set.contains(value),
                    None => false,
                };
            }
        }
    }
}

#[derive(PartialEq, Hash, Eq, PartialOrd, Debug, Clone)]
pub enum Value {
    Int(i64),
    // Float(f64), not hashable
    String(String),
    Bool(bool),
    Vec(Vec<Value>),
    Null,
}

impl Into<String> for Value {
    fn into(self) -> String {
        match self {
            Value::String(str) => str.clone(),
            _ => return String::new(),
        }
    }
}

impl From<&str> for Value {
    fn from(s: &str) -> Self {
        Value::String(s.to_string())
    }
}

impl From<String> for Value {
    fn from(s: String) -> Self {
        Value::String(s)
    }
}

impl From<i64> for Value {
    fn from(n: i64) -> Self {
        Value::Int(n)
    }
}

// impl From<f64> for Value {
//     fn from(n: f64) -> Self {
//         Value::Float(n)
//     }
// }

impl From<bool> for Value {
    fn from(n: bool) -> Self {
        Value::Bool(n)
    }
}

impl From<Vec<Value>> for Value {
    fn from(value: Vec<Value>) -> Self {
        Value::Vec(value)
    }
}
