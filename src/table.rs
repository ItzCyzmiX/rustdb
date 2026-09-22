use std::collections::HashMap;

use crate::{
    enums::{DBError, Value, ValueType},
    types::{Schema, TableEntry},
    utils::{keys_match, maps_match},
};

#[derive(Debug, PartialEq)]
pub struct Table {
    pub rows: Vec<TableEntry>,
    pub schema: Schema,
    pub name: String,
}

impl Table {
    pub fn add(&mut self, json: &TableEntry) -> Result<(), DBError> {
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
                (Value::Vec(_), ValueType::Vec) => true,
                _ => false,
            } {
                return Err(DBError::InvalidValueType(*column.1));
            }
        }

        self.rows.push(json.clone());

        Ok(())
    }

    pub fn update_if<F>(&mut self, remove_method: F, json: &TableEntry) -> Vec<TableEntry>
    where
        F: Fn(&TableEntry) -> bool,
    {
        let mut updated = Vec::new();

        for row in self.rows.iter_mut() {
            if remove_method(row) {
                updated.push(row.clone());

                for (key, value) in json {
                    row.insert(key.clone(), value.clone());
                }
            }
        }

        updated
    }

    pub fn update_exact(
        &mut self,
        old_json: &TableEntry,
        new_json: &TableEntry,
    ) -> Vec<TableEntry> {
        let mut updated = Vec::new();

        for row in self.rows.iter_mut() {
            if maps_match(row, old_json) {
                updated.push(row.clone());

                for (key, value) in row.iter_mut() {
                    if let Some(new_value) = new_json.get(key) {
                        *value = new_value.clone();
                    }
                }
            }
        }

        updated
    }

    pub fn remove_exact(&mut self, json: &TableEntry) -> Vec<TableEntry> {
        let mut removed = Vec::new();
        loop {
            match self.rows.pop_if(|f| maps_match(f, json)) {
                Some(value) => removed.push(value),
                None => break,
            };
        }
        removed
    }

    pub fn remove_if<F>(&mut self, remove_method: F) -> Vec<TableEntry>
    where
        F: Fn(&TableEntry) -> bool,
    {
        let mut removed = Vec::new();
        loop {
            match self.rows.pop_if(|f| remove_method(f)) {
                Some(row) => removed.push(row),
                None => break,
            };
        }

        removed
    }

    pub fn get_exact(&mut self, value: &TableEntry) -> Option<&mut HashMap<String, Value>> {
        self.rows.iter_mut().find(|row| **row == *value)
    }

    pub fn get_if<F>(&self, filter_method: F) -> Vec<&TableEntry>
    where
        F: Fn(&TableEntry) -> bool,
    {
        self.rows.iter().filter(|f| filter_method(*f)).collect()
    }

    pub fn clear(&mut self) {
        self.rows.clear();
    }
}
