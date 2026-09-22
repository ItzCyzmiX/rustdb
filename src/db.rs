use std::collections::HashMap;

use crate::enums::{DBError, ValueType};
use crate::table::Table;

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
        schema: Vec<(String, ValueType)>,
    ) -> Result<(), DBError> {
        let t = Table {
            rows: Vec::new(),
            schema: HashMap::from_iter(schema),
            name: name.to_string(),
        };
        if self.tables.contains_key(name) {
            return Err(DBError::TableAlreadyExists(name.to_string()));
        };
        self.tables.insert(String::from(name), t);
        Ok(())
    }

    pub fn from(&mut self, name: &str) -> Result<&mut Table, DBError> {
        match self.tables.get_mut(name) {
            Some(table) => Ok(table),
            None => Err(DBError::TableNotFound(name.to_string())),
        }
    }
}
