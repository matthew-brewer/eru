/// Represents an individual table within the data model.
pub struct Table {
    pub name: String,
    pub location: String
}

impl Table {
    pub fn new(name: String, location: String) -> Self {
        Self {
            name,
            location
        }
    }
}