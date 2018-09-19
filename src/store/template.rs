use types::Template;

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
}

// TODO: TemplateStore::save()
// TODO: TemplateStore::load()
