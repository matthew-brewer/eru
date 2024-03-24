use project::{Project, DataSource, Schema, Table};

extern crate common;

mod project;

fn main() {
    let table_one = Table::new("table_one".to_string());
    let table_two = Table::new("table_two".to_string());

    let mut schema = Schema::new("schema_one".to_string());
    schema.add_table(table_one);
    schema.add_table(table_two);

    let mut data_source = DataSource::new("data_source_one".to_string());
    data_source.add_schema(schema);

    let mut project = Project::new("new_project".to_string());
    project.model.add_data_source(data_source);

    println!("Project: {:#?}", project);
}