use std::{fs, io::Result, path::Path, sync::Mutex};

use rayon::prelude::*;
use toml;

use config::template::FILE;
use fragment::Fragment;
use types::{Model, RotPoint, RotTrace, Template};

/// Used for storing templates.
#[derive(Debug)]
pub struct TemplateStore {
    templates: Mutex<Vec<Template>>,
}

impl TemplateStore {
    /// Construct a new empty template store
    pub fn new() -> Self {
        Self {
            // TODO: after debugging, load an emtpy list of templates instead
            templates: Mutex::new(vec![
                Template::new(
                    "Straight line".into(),
                    Model::new(RotTrace::new(vec![RotPoint::new(0.0, 15.0); 25])),
                ),
                Template::new(
                    "Circle clockwise".into(),
                    Model::new(RotTrace::new(vec![RotPoint::new(-0.25, 15.0); 25])),
                ),
                Template::new(
                    "Circle counter-clockwise".into(),
                    Model::new(RotTrace::new(vec![RotPoint::new(0.25, 15.0); 25])),
                ),
                Template::new(
                    "Big circle clockwise".into(),
                    Model::new(RotTrace::new(vec![RotPoint::new(-0.10, 15.0); 25])),
                ),
                Template::new(
                    "Big circle counter-clockwise".into(),
                    Model::new(RotTrace::new(vec![RotPoint::new(0.10, 15.0); 25])),
                ),
            ]),
        }
    }

    /// Get a list of all template names.
    pub fn names(&self) -> Vec<String> {
        self.templates
            .lock()
            .expect("failed to lock templates list")
            .iter()
            .map(|t| t.name().into())
            .collect()
    }

    /// Load a list of templates from a file.
    /// On success, the current list of templates is replaced with the list of tempaltes from the
    /// file.
    ///
    /// If the file doesn't exist, nothing is loaded and `Ok` is returned.
    pub fn load(&self) -> Result<()> {
        // Build the file path
        let path = Path::new(FILE);

        // Ensure a file exists
        if !path.is_file() {
            eprintln!("Not loading templates, no file exists");
            return Ok(());
        }

        // Obtain a templates list lock
        let mut templates = self
            .templates
            .lock()
            .expect("failed to lock templates list for loading");

        // Load, deserialize and set the list of templates
        *templates = toml::from_str(&fs::read_to_string(path)?)
            .expect("failed to deserialize templates from loaded file");

        println!("Loaded {} template(s)", templates.len());

        Ok(())
    }

    /// Save the current list of templates to a file.
    ///
    /// TODO: handle toml errors properly, return an error on failure instead of panicing.
    pub fn save(&self) -> Result<()> {
        // Obtain a templates list lock
        let templates = self
            .templates
            .lock()
            .expect("failed to lock templates list for saving");

        // Remove template files if there are not tempaltes to save
        if templates.is_empty() {
            fs::remove_file(FILE);
            return Ok(());
        }

        println!("Saving {} template(s) to file...", templates.len());
        fs::write(
            FILE,
            toml::to_string(&*templates)
                .expect("failed to serialize template data, unable to save"),
        )
    }

    /// Find a matching template in this template store, for the given `other` fragment.
    /// This may be used for gesture detection based on templates.
    ///
    /// This attempts to find a matching template in parallel. Only one match may be returned, and
    /// searching for a match is stalled when a first match is found.
    ///
    /// If no template is matching, `None` is returned.
    #[inline]
    pub fn find_matching(&self, other: &mut Fragment) -> Option<Template> {
        // Obtain a templates list lock, and attempt to find a matching template
        self.templates
            .lock()
            .expect("failed to lock templates list for detecting gestures")
            .par_iter()
            .find_any(|template| template.model().matches(other.model()))
            .cloned()
    }
}
