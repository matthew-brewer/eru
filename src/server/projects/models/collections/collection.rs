use crate::projects::models::tables::Table;

/// Represents a collection of `Tables` belonging to a `Source`.
pub struct Collection {
    pub name: String,
    pub location: String,
    pub tables: Vec<Table>
}

impl Collection {
    pub fn new(name: String, location: String) -> Self {
        Self {
            name,
            location,
            tables: vec![]
        }
    }

    pub fn add_table(mut self, table: Table) {
        self.tables.push(table);
    }
}