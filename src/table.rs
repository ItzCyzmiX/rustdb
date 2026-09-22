use std::collections::HashMap;

use crate::{
    enums::{DBError, Operator, Value, ValueType},
    utils::{keys_match, maps_match},
};

#[derive(Debug, PartialEq)]
pub struct Table {
    pub rows: Vec<HashMap<String, Value>>,
    pub schema: HashMap<String, ValueType>,
    pub name: String,
}

impl Table {
    pub fn add(&mut self, json: HashMap<String, Value>) -> Result<(), DBError> {
        if !keys_match(&self.schema, &json) {
            return Err(DBError::InvalidRow);
        }

        for column in &self.schema {
            let key = match json.get(column.0) {
                Some(v) => v,
                None => return Err(DBError::MissingRow(column.0.to_owned())),
            };

            if !match (key, column.1) {
                (Value::Int(_), ValueType::Int) => true,
                (Value::Float(_), ValueType::Float) => true,
                (Value::String(_), ValueType::String) => true,
                (Value::Bool(_), ValueType::Bool) => true,
                _ => false,
            } {
                return Err(DBError::InvalidValueType(*column.1));
            }
        }

        self.rows.push(HashMap::from_iter(json));

        Ok(())
    }

    pub fn remove(&mut self, json: HashMap<String, Value>) -> Vec<HashMap<String, Value>> {
        let mut removed = Vec::new();
        loop {
            match self.rows.pop_if(|f| maps_match(f, &json)) {
                Some(value) => removed.push(value),
                None => break,
            };
        }
        removed
    }

    pub fn get(&mut self, value: Vec<(String, Value)>) -> Option<&mut HashMap<String, Value>> {
        let target = HashMap::from_iter(value);

        self.rows.iter_mut().find(|row| **row == target)
    }

    pub fn get_if<F>(&self, filter_method: F) -> Vec<&HashMap<String, Value>>
    where
        F: Fn(&HashMap<String, Value>) -> bool,
    {
        self.rows.iter().filter(|f| filter_method(*f)).collect()
    }

    pub fn get_with_op(
        &self,
        op: Operator,
        feild: &str,
        value: Value,
    ) -> Vec<&HashMap<String, Value>> {
        self.rows
            .iter()
            .filter(|f| op.compare(f.get(feild).unwrap(), &value))
            .collect::<Vec<&HashMap<String, Value>>>()
    }
}
