use crate::enums::{DBError, ValueType};
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
        schema: Vec<(String, ValueType)>,
    ) -> Result<(), DBError> {
        if self.tables.contains_key(name) {
            return Err(DBError::TableAlreadyExists(name.to_string()));
        };

        let t = Table::new(name, &HashMap::from_iter(schema));
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
