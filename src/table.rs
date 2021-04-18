extern crate config;

use config::Config;
use std::error::Error;
use postgres::Row;
use std::collections::HashMap;

pub struct Table {
    columns: String,
    table_name: String,
    pub pk: String
}

impl Table {
    pub fn new(configs: Config) -> Result<Self, Box<dyn Error>> {
        Ok(Table {
            columns: configs.get::<String>("columns")?,
            table_name: configs.get::<String>("table_name")?,
            pk: configs.get::<String>("pk")?
        })
    }

    pub fn select(&self) -> String {
        format!("SELECT {} FROM {}", self.columns, self.table_name)
    }

    pub fn update(&self, id: String) -> String {
        let mut c = 0;
        let columns = self.columns
            .split(", ")
            .collect::<Vec<_>>()
            .drain_filter(|item| { item.to_string() != self.pk })
            .map(|item| { c += 1; if item != self.pk { format!("{} = ${}", item, c) } else { "".to_string() } } )
            .collect::<Vec<_>>()
            .join(", ");
        format!("UPDATE {} SET {} WHERE {} = {}", self.table_name, columns, self.pk, id)
    }

    pub fn update_from_hash(&self, id: String, hash: HashMap<String, i32>) -> String {
        let columns = self.columns
            .split(", ")
            .collect::<Vec<_>>()
            .drain_filter(|item| { item.to_string() != self.pk })
            .map(|item| { if item != self.pk { format!("{} = {}", item, hash.get(item).unwrap()) } else { "".to_string() } } )
            .collect::<Vec<_>>()
            .join(", ");
        format!("UPDATE {} SET {} WHERE {} = {}", self.table_name, columns, self.pk, id)
    }

    pub fn parse_row_to_hash(&self, row: &Row) -> HashMap<String, i32> {
        let columns = self.columns
            .split(", ")
            .collect::<Vec<_>>()
            .drain_filter(|item| { item.to_string() != self.pk })
            .collect::<Vec<_>>();
        let mut hash_map: HashMap<String, i32> = HashMap::new();

        for i in 0..columns.len() {
            hash_map.insert(columns[i].to_string(), row.get(i));
        }
        hash_map
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_table() -> Table {
        Table {
            columns: "a, b, id".to_string(),
            table_name: "teste".to_string(),
            pk: "id".to_string()
        }
    }

    #[test]
    fn test_select_builder() {
        let table = create_test_table();
        assert_eq!("SELECT a, b, id FROM teste", table.select());
    }

    #[test]
    fn test_update_builder() {
        let table = create_test_table();
        assert_eq!("UPDATE teste SET a = $1, b = $2 WHERE id = 2", table.update("2".to_string()));
    }
}