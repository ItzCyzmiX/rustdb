use crate::enums::{Constraints, Value, ValueType};
use std::collections::HashMap;
pub type TableEntry = HashMap<String, Value>;
pub type Schema = HashMap<String, (ValueType, Vec<Constraints>)>;
