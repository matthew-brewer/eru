use projects::{models::{collections::Collection, sources::Source, tables::Table}, project::Project};

extern crate common;

mod projects;

fn main() {

    let project = Project::new(
        "New Project".to_string(),
         "/home/matthew/eru/projects/new_project".to_string()
    );

    let source = Source::new(
        "New Source".to_string(),
        "/home/matthew/eru/projects/new_project/new_source".to_string()  
    );

    let collection = Collection::new(
        "New Collection".to_string(),
        "/home/matthew/eru/projects/new_project/new_source/new_collection".to_string()
    );

    let table = Table::new(
        "New Table".to_string(),
        "/home/matthew/eru/projects/new_project/new_source/new_collection/new_table.csv".to_string()
    );

    source.add_collection(collection);
    collection.add_table(table);
    

}