use super::sources::Source;

/// Represents the data model for a `Project`
pub struct Model {
    pub sources: Vec<Source>
}

impl Model {
    pub fn new() -> Self {
        Model {
            sources: vec![]
        }
    }

    pub fn add_source(mut self, source: Source) {
        self.sources.push(source);
    }
}
