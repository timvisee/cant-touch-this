use std::{fs, io::Result, path::Path};

use toml;

use types::{Model, Template};

/// The default file path to save the templates in.
const TEMPLATES_FILE_PATH: &str = "~/.cant-touch-this-templates";

/// Used for storing templates.
pub struct TemplateStore {
    templates: Vec<Template>,
}

impl TemplateStore {
    /// Construct a new empty template store
    pub fn new() -> Self {
        Self {
            templates: Vec::new(),
        }
    }

    /// Load a list of templates from a file.
    /// On success, the current list of templates is replaced with the list of tempaltes from the
    /// file.
    ///
    /// If the file doesn't exist, nothing is loaded and `Ok` is returned.
    pub fn load(&mut self) -> Result<()> {
        // Build the file path
        let path = Path::new(TEMPLATES_FILE_PATH);

        // Ensure a file exists
        if !path.is_file() {
            eprintln!("Could not load templates, file does not exist");
            return Ok(());
        }

        // Load, deserialize and set the list of templates
        self.templates = toml::from_str(&fs::read_to_string(path)?)
            .expect("failed to deserialize templates from loaded file");

        Ok(())
    }

    /// Save the current list of templates to a file.
    ///
    /// TODO: handle toml errors properly, return an error on failure instead of panicing.
    /// TODO: make file configurable, or use a different file anyway
    pub fn save(&self) -> Result<()> {
        fs::write(
            TEMPLATES_FILE_PATH,
            toml::to_string(&self.templates)
                .expect("failed to serialize template data, unable to save"),
        )
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
