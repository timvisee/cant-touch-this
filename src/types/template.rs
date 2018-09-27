use types::Model;

#[derive(Debug, Serialize, Deserialize)]
pub struct Template {
    name: String,
    model: Model,
}

impl Template {
    /// Construct a new template with the given `name` and `model`.
    pub fn new(name: String, model: Model) -> Self {
        Template { name, model }
    }

    /// Get the template name.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Get the template model.
    pub fn model(&self) -> &Model {
        &self.model
    }
}
