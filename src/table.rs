use std::collections::{HashMap, HashSet};

use crate::{
    enums::{Constraints, DBError, Value, ValueType},
    types::{Schema, TableEntry},
    utils::{keys_match, maps_match},
};

#[derive(Debug, PartialEq)]
pub struct Table {
    current_id: i64,
    uniques: HashMap<String, HashSet<Value>>,
    pub rows: HashMap<i64, TableEntry>,
    pub schema: Schema,
    pub name: String,
}

impl Table {
    pub fn new(name: &str, schema: &Schema) -> Self {
        let uniques = schema
            .iter()
            .filter(|(_, (_, constraints))| constraints.contains(&Constraints::Unique))
            .map(|(key, _)| (key.clone(), HashSet::new()))
            .collect();

        return Table {
            uniques,
            current_id: 0,
            rows: HashMap::new(),
            schema: schema.clone(),
            name: name.to_string(),
        };
    }

    fn validate_type(value: &Value, expected_type: &ValueType) -> bool {
        match (value, expected_type) {
            (Value::Int(_), ValueType::Int) => true,
            // (Value::Float(_), ValueType::Float) => true,
            (Value::String(_), ValueType::String) => true,
            (Value::Bool(_), ValueType::Bool) => true,
            (Value::Vec(_), ValueType::Vec) => true,
            (Value::Int(_) | Value::Null, ValueType::OptionalInt) => true,
            (Value::Bool(_) | Value::Null, ValueType::OptionalBool) => true,
            (Value::Vec(_) | Value::Null, ValueType::OptionalVec) => true,
            (Value::String(_) | Value::Null, ValueType::OptionalString) => true,
            _ => false,
        }
    }

    fn validate_constraints(&self, json: &TableEntry) -> Result<(), DBError> {
        for (key, (_, constraints)) in &self.schema {
            let value = json.get(key).ok_or(DBError::InvalidRow)?;

            for constraint in constraints {
                if !constraint.check(value, self.uniques.get(key)) {
                    return Err(match constraint {
                        Constraints::Unique => DBError::ConstraintNotMet,
                        _ => DBError::InvalidRow,
                    });
                }
            }
        }

        Ok(())
    }

    fn rebuild_uniques(&mut self) {
        for values in self.uniques.values_mut() {
            values.clear();
        }

        for row in self.rows.values() {
            for (key, values) in &mut self.uniques {
                if let Some(value) = row.get(key) {
                    values.insert(value.clone());
                }
            }
        }
    }

    fn validate_unique_updates<F>(&self, matches: F, json: &TableEntry) -> bool
    where
        F: Fn(&TableEntry) -> bool,
    {
        let mut values = self.uniques.clone();

        for row in self.rows.values().filter(|row| matches(row)) {
            for (key, unique_values) in &mut values {
                if let Some(value) = row.get(key) {
                    unique_values.remove(value);
                }
            }
        }

        for row in self.rows.values().filter(|row| matches(row)) {
            for (key, unique_values) in &mut values {
                let value = json.get(key).or_else(|| row.get(key));
                if let Some(value) = value {
                    if !unique_values.insert(value.clone()) {
                        return false;
                    }
                }
            }
        }

        true
    }

    fn validate_row(row: &TableEntry, schema: &Schema) -> Result<(), DBError> {
        if !keys_match(schema, row) {
            return Err(DBError::InvalidRow);
        }

        for (column_name, (expected_type, _)) in schema {
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

        self.validate_constraints(json)?;

        self.current_id += 1;

        let mut keyed_json = json.clone();

        keyed_json.insert("ID".to_string(), Value::Int(self.current_id));

        self.rows.insert(self.current_id, keyed_json);
        self.rebuild_uniques();
        Ok(self.current_id)
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

            let (expected_type, constraints) = match self.schema.get(key) {
                Some(schema_entry) => schema_entry,
                None => return Err(DBError::InvalidRow),
            };

            if !Self::validate_type(value, expected_type) {
                return Err(DBError::InvalidValueType(*expected_type));
            }

            if !constraints
                .iter()
                .filter(|constraint| !matches!(constraint, Constraints::Unique))
                .all(|constraint| constraint.check(value, self.uniques.get(key)))
            {
                return Err(DBError::InvalidRow);
            }
        }

        if !self.validate_unique_updates(|row| remove_method(row), json) {
            return Err(DBError::ConstraintNotMet);
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

        self.rebuild_uniques();

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

            let (expected_type, constraints) = match self.schema.get(key) {
                Some(schema_entry) => schema_entry,
                None => return Err(DBError::InvalidRow),
            };

            if !Self::validate_type(value, expected_type) {
                return Err(DBError::InvalidValueType(*expected_type));
            }

            if !constraints
                .iter()
                .filter(|constraint| !matches!(constraint, Constraints::Unique))
                .all(|constraint| constraint.check(value, self.uniques.get(key)))
            {
                return Err(DBError::InvalidRow);
            }
        }

        if !self.validate_unique_updates(|row| maps_match(row, old_json), new_json) {
            return Err(DBError::ConstraintNotMet);
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

        self.rebuild_uniques();

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
        self.rebuild_uniques();
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
        self.rebuild_uniques();
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
        let removed = self.rows.remove(&id);
        if removed.is_some() {
            self.rebuild_uniques();
        }
        removed
    }

    pub fn update_id(&mut self, id: i64, json: &TableEntry) -> Result<Option<TableEntry>, DBError> {
        let old = match self.rows.get(&id) {
            Some(val) => val.clone(),
            None => return Ok(None),
        };

        if !self.validate_unique_updates(|row| row.get("ID") == Some(&Value::Int(id)), json) {
            return Err(DBError::ConstraintNotMet);
        }

        for (key, value) in json {
            if key == &String::from("ID") {
                continue;
            }

            if !old.contains_key(key) {
                return Err(DBError::InvalidRow);
            };

            let (expected_type, constraints) = match self.schema.get(key) {
                Some(schema_entry) => schema_entry,
                None => return Err(DBError::InvalidRow),
            };

            if !Self::validate_type(value, expected_type) {
                return Err(DBError::InvalidValueType(*expected_type));
            };

            if !constraints
                .iter()
                .filter(|constraint| !matches!(constraint, Constraints::Unique))
                .all(|constraint| constraint.check(value, self.uniques.get(key)))
            {
                return Err(DBError::InvalidRow);
            };
        }

        let new = self.rows.get_mut(&id).unwrap();
        for (key, value) in json {
            if key != "ID" {
                new.insert(key.clone(), value.clone());
            }
        }
        self.rebuild_uniques();
        return Ok(Some(old));
    }

    pub fn clear(&mut self) {
        self.rows.clear();
        self.rebuild_uniques();
    }

    pub fn all(&self) -> Vec<TableEntry> {
        self.rows.iter().map(|f| f.1.clone()).collect()
    }
}
