use std::collections::HashMap;

use crate::{
    enums::{DBError, Value, ValueType},
    types::{Schema, TableEntry},
    utils::{keys_match, maps_match},
};

#[derive(Debug, PartialEq)]
pub struct Table {
    current_idx: i64,
    pub rows: HashMap<i64, TableEntry>,
    pub schema: Schema,
    pub name: String,
}

impl Table {
    pub fn new(name: &str, schema: &Schema) -> Self {
        return Table {
            current_idx: 0,
            rows: HashMap::new(),
            schema: schema.clone(),
            name: name.to_string(),
        };
    }

    fn validate_type(value: &Value, expected_type: &ValueType) -> bool {
        match (value, expected_type) {
            (Value::Int(_), ValueType::Int) => true,
            (Value::Float(_), ValueType::Float) => true,
            (Value::String(_), ValueType::String) => true,
            (Value::Bool(_), ValueType::Bool) => true,
            (Value::Vec(_), ValueType::Vec) => true,
            _ => false,
        }
    }

    fn validate_row(row: &TableEntry, schema: &Schema) -> Result<(), DBError> {
        if !keys_match::<String, ValueType, Value>(schema, row) {
            return Err(DBError::InvalidRow);
        }

        for (column_name, expected_type) in schema {
            let value = match row.get(column_name) {
                Some(v) => v,
                None => return Err(DBError::MissingRow(column_name.to_owned())),
            };

            if !Self::validate_type(value, expected_type) {
                return Err(DBError::InvalidValueType(*expected_type));
            };
        }

        Ok(())
    }

    pub fn add(&mut self, json: &TableEntry) -> Result<i64, DBError> {
        Self::validate_row(json, &self.schema)?;

        self.current_idx += 1;

        let mut keyed_json = json.clone();

        keyed_json.insert("ID".to_string(), Value::Int(self.current_idx));

        self.rows.insert(self.current_idx, keyed_json);
        Ok(self.current_idx.clone())
    }

    pub fn update_if<F>(
        &mut self,
        remove_method: F,
        json: &TableEntry,
    ) -> Result<Vec<TableEntry>, DBError>
    where
        F: Fn(&TableEntry) -> bool,
    {
        for (key, value) in json {
            if key == "ID" {
                continue;
            }

            let expected_type = match self.schema.get(key) {
                Some(type_) => type_,
                None => return Err(DBError::InvalidRow),
            };

            if !Self::validate_type(value, expected_type) {
                return Err(DBError::InvalidValueType(*expected_type));
            }
        }

        let mut updated: Vec<TableEntry> = Vec::new();

        for (_, row) in self.rows.iter_mut() {
            if remove_method(row) {
                updated.push(row.clone());

                for (key, value) in json {
                    if key == "ID" {
                        continue;
                    }
                    row.insert(key.clone(), value.clone());
                }
            }
        }

        Ok(updated)
    }

    pub fn update_exact(
        &mut self,
        old_json: &TableEntry,
        new_json: &TableEntry,
    ) -> Result<Vec<TableEntry>, DBError> {
        for (key, value) in new_json {
            if key == "ID" {
                continue;
            }

            let expected_type = match self.schema.get(key) {
                Some(type_) => type_,
                None => return Err(DBError::InvalidRow),
            };

            if !Self::validate_type(value, expected_type) {
                return Err(DBError::InvalidValueType(*expected_type));
            }
        }

        let mut updated: Vec<TableEntry> = Vec::new();

        for (_, row) in self.rows.iter_mut() {
            if maps_match(row, old_json) {
                let original = row.clone();
                updated.push(original.clone());

                for (key, value) in new_json {
                    if key == "ID" {
                        continue;
                    }
                    row.insert(key.clone(), value.clone());
                }
            }
        }

        Ok(updated)
    }

    pub fn remove_exact(&mut self, json: &TableEntry) -> Vec<TableEntry> {
        let mut removed = Vec::new();
        self.rows.retain(|_, row| {
            if maps_match(row, json) {
                removed.push(row.clone());
                false
            } else {
                true
            }
        });
        removed
    }

    pub fn remove_if<F>(&mut self, remove_method: F) -> Vec<TableEntry>
    where
        F: Fn(&TableEntry) -> bool,
    {
        let mut removed = Vec::new();
        self.rows.retain(|_, row| {
            if remove_method(row) {
                removed.push(row.clone());
                false
            } else {
                true
            }
        });
        removed
    }

    pub fn get_exact(&mut self, value: &TableEntry) -> Option<&mut TableEntry> {
        self.rows
            .iter_mut()
            .find(|row| *row.1 == *value)
            .map(|row| row.1)
    }

    pub fn get_if<F>(&self, filter_method: F) -> Vec<&TableEntry>
    where
        F: Fn(&TableEntry) -> bool,
    {
        self.rows
            .iter()
            .filter(|f| filter_method(f.1))
            .map(|row| row.1)
            .collect()
    }

    pub fn get_id(&self, id: i64) -> Option<&TableEntry> {
        self.rows.get(&id)
    }

    pub fn remove_id(&mut self, id: i64) -> Option<TableEntry> {
        self.rows.remove(&id)
    }

    pub fn update_id(&mut self, id: i64, json: &TableEntry) -> Result<Option<TableEntry>, DBError> {
        let new = match self.rows.get_mut(&id) {
            Some(val) => val,
            None => return Ok(None),
        };
        let old = new.clone();

        for (key, value) in json {
            if key == &String::from("ID") {
                continue;
            }

            if !old.contains_key(key) {
                return Err(DBError::InvalidRow);
            };

            let expected_type = match self.schema.get(key) {
                Some(expected_type) => expected_type,
                None => return Err(DBError::InvalidRow),
            };

            if !Self::validate_type(value, expected_type) {
                return Err(DBError::InvalidValueType(expected_type.clone()));
            };

            new.insert(key.clone(), value.clone());
        }
        return Ok(Some(old));
    }

    pub fn clear(&mut self) {
        self.rows.clear();
    }

    pub fn all(&self) -> Vec<TableEntry> {
        self.rows.iter().map(|f| f.1.clone()).collect()
    }
}
