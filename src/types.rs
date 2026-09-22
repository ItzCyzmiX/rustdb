use crate::enums::{Value, ValueType};
use std::collections::HashMap;
pub type TableEntry = HashMap<String, Value>;
pub type Schema = HashMap<String, ValueType>;
