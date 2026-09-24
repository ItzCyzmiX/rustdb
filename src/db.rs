use crate::enums::{Constraints, DBError, ValueType};
use crate::table::Table;
use std::collections::HashMap;

#[derive(Debug)]
pub struct DB {
    tables: HashMap<String, Table>,
}

impl DB {
    pub fn new() -> Self {
        DB {
            tables: HashMap::new(),
        }
    }

    pub fn new_table(
        &mut self,
        name: &str,
        schema: Vec<(String, ValueType, Vec<Constraints>)>,
    ) -> Result<(), DBError> {
        if self.tables.contains_key(name) {
            return Err(DBError::TableAlreadyExists(name.to_string()));
        };

        let t = Table::new(
            name,
            &HashMap::from_iter(
                schema
                    .into_iter()
                    .map(|(name, value_type, constraints)| (name, (value_type, constraints))),
            ),
        );
        self.tables.insert(String::from(name), t);

        Ok(())
    }

    pub fn from(&mut self, name: &str) -> Result<&mut Table, DBError> {
        match self.tables.get_mut(name) {
            Some(table) => Ok(table),
            None => Err(DBError::TableNotFound(name.to_string())),
        }
    }

    pub fn delete(&mut self, name: &str) -> Result<bool, DBError> {
        return match self.tables.remove(name) {
            Some(_) => Ok(true),
            None => Err(DBError::TableNotFound(name.to_string())),
        };
    }
}
