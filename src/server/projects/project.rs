use super::models::Model;

pub struct Project {
    pub name: String,
    pub location: String,
    pub model: Model
}

impl Project {
    pub fn new(name: String, location: String) -> Self {
        let model = Model::new();
        
        Project {
            name,
            location,
            model
        }
    }
}