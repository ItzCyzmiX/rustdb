#[derive(Debug)]
pub enum DBError {
    TableAlreadyExists(String),
    TableNotFound(String),
    InvalidValueType(ValueType),
    InvalidRow,
    MissingRow(String),
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
}

#[derive(PartialEq, PartialOrd, Debug, Clone)]
pub enum Value {
    Int(i64),
    Float(f64),
    String(String),
    Bool(bool),
}

#[derive(Debug, Clone, Copy)]
pub enum Operator {
    Bigger,
    Smaller,
    Eq,
    BiggerOrEq,
    SmallerOrEq,
    NotEq,
}

impl Operator {
    pub fn compare(&self, a: &Value, b: &Value) -> bool {
        match self {
            Operator::Bigger => a > b,
            Operator::Smaller => a < b,
            Operator::BiggerOrEq => a >= b,
            Operator::SmallerOrEq => a <= b,
            Operator::Eq => a == b,
            Operator::NotEq => a != b,
        }
    }
}
