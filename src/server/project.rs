#[derive(Debug)]
pub struct Project {
    pub name: String,
    pub model: DataModel
}

impl Project {
    pub fn new(name: String) -> Self {
        Self {
            name,
            model: DataModel::new()
        }
    }
}

/// Represents the entire data model for a `Project`.
#[derive(Debug)]
pub struct DataModel {
    data_sources: Vec<DataSource>
}

impl DataModel {
    pub fn new() -> Self {
        Self {
            data_sources: vec![]
        }
    }

    pub fn add_data_source(&mut self, data_source: DataSource) {
        self.data_sources.push(data_source);
    }

    pub fn get_data_sources(&self) -> Vec<&DataSource> {
        self.data_sources.iter().collect()
    }
}

/// Represents a data source - e.g., a database server.
#[derive(Debug)]
pub struct DataSource {
    pub name: String,
    schemas: Vec<Schema>
}

impl DataSource {
    pub fn new(name: String) -> Self {
        Self {
            name,
            schemas: vec![]
        }
    }

    pub fn add_schema(&mut self, schema: Schema) {
        self.schemas.push(schema);
    }

    pub fn get_schemas(&self, schema: Schema) -> Vec<&Schema> {
        self.schemas.iter().collect()
    }
}

/// Represents a collection of tables, or `Schema`, within a `DataSource`.
#[derive(Debug)]
pub struct Schema {
    pub name: String,
    tables: Vec<Table>
}

impl Schema {
    pub fn new(name: String) -> Self {
        Self {
            name,
            tables: vec![]
        }
    }

    pub fn add_table(&mut self, table: Table) {
        self.tables.push(table);
    }

    pub fn get_tables(&self) -> Vec<&Table> {
        self.tables.iter().collect()
    }
}

/// Represents an individual `Table`, belonging to a `Schema`.
#[derive(Debug)]
pub struct Table {
    pub name: String
}

impl Table {
    pub fn new(name: String) -> Self {
        Self {
            name
        }
    }
}