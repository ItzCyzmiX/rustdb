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
    fn validate_row(row: &TableEntry, schema: &Schema) -> Result<(), DBError> {
        if !keys_match(schema, row) {
            return Err(DBError::InvalidRow);
        }

        for (column_name, expected_type) in schema {
            let value = match row.get(column_name) {
                Some(v) => v,
                None => return Err(DBError::MissingRow(column_name.to_owned())),
            };

            let valid = match (value, expected_type) {
                (Value::Int(_), ValueType::Int) => true,
                (Value::Float(_), ValueType::Float) => true,
                (Value::String(_), ValueType::String) => true,
                (Value::Bool(_), ValueType::Bool) => true,
                (Value::Vec(_), ValueType::Vec) => true,
                _ => false,
            };

            if !valid {
                return Err(DBError::InvalidValueType(*expected_type));
            }
        }

        Ok(())
    }

    pub fn add(&mut self, json: &TableEntry) -> Result<(), DBError> {
        Self::validate_row(json, &self.schema)?;
        self.rows.push(json.clone());
        Ok(())
    }

    pub fn update_if<F>(&mut self, remove_method: F, json: &TableEntry) -> Vec<TableEntry>
    where
        F: Fn(&TableEntry) -> bool,
    {
        if Self::validate_row(json, &self.schema).is_err() {
            return Vec::new();
        }

        let mut updated: Vec<TableEntry> = Vec::new();

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
        if Self::validate_row(new_json, &self.schema).is_err() {
            return Vec::new();
        }

        let mut updated: Vec<TableEntry> = Vec::new();

        for row in self.rows.iter_mut() {
            if maps_match(row, old_json) {
                let original = row.clone();
                updated.push(original.clone());

                for (key, value) in row.iter_mut() {
                    if let Some(new_value) = new_json.get(key) {
                        *value = new_value.clone();
                    }
                }

                for (key, value) in new_json {
                    if !row.contains_key(key) {
                        row.insert(key.clone(), value.clone());
                    }
                }
            }
        }

        updated
    }

    pub fn remove_exact(&mut self, json: &TableEntry) -> Vec<TableEntry> {
        let mut removed = Vec::new();
        self.rows.retain(|row| {
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
        self.rows.retain(|row| {
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

    pub fn all(&self) -> Vec<TableEntry> {
        self.rows.clone()
    }
}
