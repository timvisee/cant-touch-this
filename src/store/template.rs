use types::{Model, Template};

/// Used for storing templates
///
/// TODO: implement storage logic, make models comparable somewhere
pub struct TemplateStore {
    _templates: Vec<Template>,
}

impl TemplateStore {
    /// Construct a new template store
    pub fn new() -> Self {
        Self {
            _templates: Vec::new(),
        }
    }

    /// Compare a given model against the tempaltes,
    /// to see whether there is a gesture match.
    pub fn detect_gesture(&self, other: &Model) {
        // Loop through each template
        for template in self._templates {
            // Get the model to compare against
            let model = template.model();

            // TODO: do the model comparison

            // TODO: build a list of matching templates to return for actions to process
        }
    }
}

// TODO: TemplateStore::save()
// TODO: TemplateStore::load()
