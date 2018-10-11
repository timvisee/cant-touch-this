use rand::{RngCore, thread_rng};

use types::Model;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Template {
    id: u32,
    name: String,
    model: Model,
}

impl Template {
    /// Construct a new template with the given `name` and `model`.
    ///
    /// A random ID will be picked for this template.
    pub fn new(name: String, model: Model) -> Self {
        // Pick a random ID
        let id = thread_rng().next_u32();

        Template { id, name, model }
    }

    /// Get the template id.
    pub fn id(&self) -> u32 {
        self.id
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
