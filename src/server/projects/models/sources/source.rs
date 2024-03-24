use crate::projects::models::collections::Collection;

/// Represents a data source from which `Collections` and `Tables` can be loaded and/or stored.
pub struct Source {
    pub name: String,
    pub location: String,
    pub collections: Vec<Collection>
}

impl Source {
    pub fn new(name: String, location: String) -> Self {
        Self {
            name,
            location,
            collections: vec![]
        }
    }

    pub fn add_collection(mut self, collection: Collection) {
        self.collections.push(collection);
    }
}