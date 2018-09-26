use std::{
    fs,
    io::Result,
    path::Path,
    sync::Mutex,
};

use toml;

use types::{Model, Template};

/// The default file path to save the templates in.
const TEMPLATES_FILE_PATH: &str = "~/.config/cant-touch-this/templates.toml";

/// Used for storing templates.
#[derive(Debug)]
pub struct TemplateStore {
    templates: Mutex<Vec<Template>>,
}

impl TemplateStore {
    /// Construct a new empty template store
    pub fn new() -> Self {
        Self {
            templates: Mutex::new(Vec::new()),
        }
    }

    /// Load a list of templates from a file.
    /// On success, the current list of templates is replaced with the list of tempaltes from the
    /// file.
    ///
    /// If the file doesn't exist, nothing is loaded and `Ok` is returned.
    pub fn load(&self) -> Result<()> {
        // Build the file path
        let path = Path::new(TEMPLATES_FILE_PATH);

        // Ensure a file exists
        if !path.is_file() {
            eprintln!("Not loading templates, no file exists");
            return Ok(());
        }

        // Obtain a templates list lock
        let mut templates = self.templates.lock().expect("failed to lock templates list for loading");

        // Load, deserialize and set the list of templates
        *templates = toml::from_str(&fs::read_to_string(path)?)
            .expect("failed to deserialize templates from loaded file");

        Ok(())
    }

    /// Save the current list of templates to a file.
    ///
    /// TODO: handle toml errors properly, return an error on failure instead of panicing.
    pub fn save(&self) -> Result<()> {
        // Obtain a templates list lock
        let templates = self.templates.lock().expect("failed to lock templates list for saving");

        // Remove template files if there are not tempaltes to save
        if templates.is_empty() {
            fs::remove_file(TEMPLATES_FILE_PATH);
            return Ok(());
        }

        println!("Saving templates to file...");
        fs::write(
            TEMPLATES_FILE_PATH,
            toml::to_string(&*templates)
                .expect("failed to serialize template data, unable to save"),
        )
    }

    /// Compare a given model against the tempaltes,
    /// to see whether there is a gesture match.
    pub fn detect_gesture(&self, other: &Model) {
        println!("DEBUG: processing model:");

        // Obtain a templates list lock
        let templates = self.templates.lock().expect("failed to lock templates list for detecting gestures");

        // Loop through each template
        for template in &*templates {
            // Get the model to compare against
            let model = template.model();

            // TODO: do the model comparison

            // TODO: build a list of matching templates to return for actions to process

            println!("DEBUG: testing template");
        }
    }
}
