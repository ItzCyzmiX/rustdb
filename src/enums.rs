#[derive(Debug)]
pub enum DBError {
    TableAlreadyExists(String),
    TableNotFound(String),
    InvalidValueType(ValueType),
    InvalidRow,
    MissingRow(String),
    EntryNotFound,
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
}

#[derive(PartialEq, PartialOrd, Debug, Clone)]
pub enum Value {
    Int(i64),
    Float(f64),
    String(String),
    Bool(bool),
    Vec(Vec<Value>),
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

impl From<f64> for Value {
    fn from(n: f64) -> Self {
        Value::Float(n)
    }
}

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
