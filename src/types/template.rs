use types::Model;

pub struct Template {
    name: String,
    model: Model,
}

impl Template {
    /// Get the template name.
    pub fn name(&self) -> &str {
        self.name
    }

    /// Get the template model.
    pub fn model(&self) -> &Model {
        self.model
    }
}
