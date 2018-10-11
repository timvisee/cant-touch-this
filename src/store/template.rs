use std::{fs, io::Result, path::Path, sync::Mutex};

use rayon::prelude::*;
use toml;

use config::{sample::DISTANCE, template::FILE};
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
                    "Long straight line".into(),
                    Model::new(RotTrace::new(vec![RotPoint::new(0.0, DISTANCE); 40])),
                ),
                Template::new(
                    "Circle clockwise".into(),
                    Model::new(RotTrace::new(vec![RotPoint::new(-0.19, DISTANCE); 30])),
                ),
                Template::new(
                    "Circle counter-clockwise".into(),
                    Model::new(RotTrace::new(vec![RotPoint::new(0.19, DISTANCE); 30])),
                ),
                Template::new(
                    "Big circle clockwise".into(),
                    Model::new(RotTrace::new(vec![RotPoint::new(-0.07, DISTANCE); 60])),
                ),
                Template::new(
                    "Big circle counter-clockwise".into(),
                    Model::new(RotTrace::new(vec![RotPoint::new(0.07, DISTANCE); 60])),
                ),
                Template::new(
                    "Triangle clockwise".into(),
                    Model::new(RotTrace::new(vec![
                        RotPoint::from_degrees(0.0, DISTANCE),
                        RotPoint::from_degrees(0.0, DISTANCE),
                        RotPoint::from_degrees(0.0, DISTANCE),
                        RotPoint::from_degrees(0.0, DISTANCE),
                        RotPoint::from_degrees(0.0, DISTANCE),
                        RotPoint::from_degrees(0.0, DISTANCE),
                        RotPoint::from_degrees(0.0, DISTANCE),
                        RotPoint::from_degrees(0.0, DISTANCE),
                        RotPoint::from_degrees(0.0, DISTANCE),
                        RotPoint::from_degrees(0.0, DISTANCE),
                        RotPoint::from_degrees(0.0, DISTANCE),
                        RotPoint::from_degrees(0.0, DISTANCE),
                        RotPoint::from_degrees(0.0, DISTANCE),
                        RotPoint::from_degrees(0.0, DISTANCE),
                        RotPoint::from_degrees(0.0, DISTANCE),
                        RotPoint::from_degrees(-120.0, DISTANCE),
                        RotPoint::from_degrees(0.0, DISTANCE),
                        RotPoint::from_degrees(0.0, DISTANCE),
                        RotPoint::from_degrees(0.0, DISTANCE),
                        RotPoint::from_degrees(0.0, DISTANCE),
                        RotPoint::from_degrees(0.0, DISTANCE),
                        RotPoint::from_degrees(0.0, DISTANCE),
                        RotPoint::from_degrees(0.0, DISTANCE),
                        RotPoint::from_degrees(0.0, DISTANCE),
                        RotPoint::from_degrees(0.0, DISTANCE),
                        RotPoint::from_degrees(0.0, DISTANCE),
                        RotPoint::from_degrees(0.0, DISTANCE),
                        RotPoint::from_degrees(0.0, DISTANCE),
                        RotPoint::from_degrees(0.0, DISTANCE),
                        RotPoint::from_degrees(0.0, DISTANCE),
                        RotPoint::from_degrees(0.0, DISTANCE),
                        RotPoint::from_degrees(-120.0, DISTANCE),
                        RotPoint::from_degrees(0.0, DISTANCE),
                        RotPoint::from_degrees(0.0, DISTANCE),
                        RotPoint::from_degrees(0.0, DISTANCE),
                        RotPoint::from_degrees(0.0, DISTANCE),
                        RotPoint::from_degrees(0.0, DISTANCE),
                        RotPoint::from_degrees(0.0, DISTANCE),
                        RotPoint::from_degrees(0.0, DISTANCE),
                        RotPoint::from_degrees(0.0, DISTANCE),
                        RotPoint::from_degrees(0.0, DISTANCE),
                        RotPoint::from_degrees(0.0, DISTANCE),
                        RotPoint::from_degrees(0.0, DISTANCE),
                        RotPoint::from_degrees(0.0, DISTANCE),
                        RotPoint::from_degrees(0.0, DISTANCE),
                        RotPoint::from_degrees(0.0, DISTANCE),
                        RotPoint::from_degrees(0.0, DISTANCE),
                    ])),
                ),
                Template::new(
                    "Triangle counter-clockwise".into(),
                    Model::new(RotTrace::new(vec![
                        RotPoint::from_degrees(0.0, DISTANCE),
                        RotPoint::from_degrees(0.0, DISTANCE),
                        RotPoint::from_degrees(0.0, DISTANCE),
                        RotPoint::from_degrees(0.0, DISTANCE),
                        RotPoint::from_degrees(0.0, DISTANCE),
                        RotPoint::from_degrees(0.0, DISTANCE),
                        RotPoint::from_degrees(0.0, DISTANCE),
                        RotPoint::from_degrees(0.0, DISTANCE),
                        RotPoint::from_degrees(0.0, DISTANCE),
                        RotPoint::from_degrees(0.0, DISTANCE),
                        RotPoint::from_degrees(0.0, DISTANCE),
                        RotPoint::from_degrees(0.0, DISTANCE),
                        RotPoint::from_degrees(0.0, DISTANCE),
                        RotPoint::from_degrees(0.0, DISTANCE),
                        RotPoint::from_degrees(0.0, DISTANCE),
                        RotPoint::from_degrees(120.0, DISTANCE),
                        RotPoint::from_degrees(0.0, DISTANCE),
                        RotPoint::from_degrees(0.0, DISTANCE),
                        RotPoint::from_degrees(0.0, DISTANCE),
                        RotPoint::from_degrees(0.0, DISTANCE),
                        RotPoint::from_degrees(0.0, DISTANCE),
                        RotPoint::from_degrees(0.0, DISTANCE),
                        RotPoint::from_degrees(0.0, DISTANCE),
                        RotPoint::from_degrees(0.0, DISTANCE),
                        RotPoint::from_degrees(0.0, DISTANCE),
                        RotPoint::from_degrees(0.0, DISTANCE),
                        RotPoint::from_degrees(0.0, DISTANCE),
                        RotPoint::from_degrees(0.0, DISTANCE),
                        RotPoint::from_degrees(0.0, DISTANCE),
                        RotPoint::from_degrees(0.0, DISTANCE),
                        RotPoint::from_degrees(0.0, DISTANCE),
                        RotPoint::from_degrees(120.0, DISTANCE),
                        RotPoint::from_degrees(0.0, DISTANCE),
                        RotPoint::from_degrees(0.0, DISTANCE),
                        RotPoint::from_degrees(0.0, DISTANCE),
                        RotPoint::from_degrees(0.0, DISTANCE),
                        RotPoint::from_degrees(0.0, DISTANCE),
                        RotPoint::from_degrees(0.0, DISTANCE),
                        RotPoint::from_degrees(0.0, DISTANCE),
                        RotPoint::from_degrees(0.0, DISTANCE),
                        RotPoint::from_degrees(0.0, DISTANCE),
                        RotPoint::from_degrees(0.0, DISTANCE),
                        RotPoint::from_degrees(0.0, DISTANCE),
                        RotPoint::from_degrees(0.0, DISTANCE),
                        RotPoint::from_degrees(0.0, DISTANCE),
                        RotPoint::from_degrees(0.0, DISTANCE),
                        RotPoint::from_degrees(0.0, DISTANCE),
                    ])),
                ),
                Template::new(
                    "Mini square clockwise".into(),
                    Model::new(RotTrace::new(vec![
                        RotPoint::from_degrees(0.0, DISTANCE),
                        RotPoint::from_degrees(0.0, DISTANCE),
                        RotPoint::from_degrees(0.0, DISTANCE),
                        RotPoint::from_degrees(0.0, DISTANCE),
                        RotPoint::from_degrees(0.0, DISTANCE),
                        RotPoint::from_degrees(0.0, DISTANCE),
                        RotPoint::from_degrees(-90.0, DISTANCE),
                        RotPoint::from_degrees(0.0, DISTANCE),
                        RotPoint::from_degrees(0.0, DISTANCE),
                        RotPoint::from_degrees(0.0, DISTANCE),
                        RotPoint::from_degrees(0.0, DISTANCE),
                        RotPoint::from_degrees(0.0, DISTANCE),
                        RotPoint::from_degrees(0.0, DISTANCE),
                        RotPoint::from_degrees(-90.0, DISTANCE),
                        RotPoint::from_degrees(0.0, DISTANCE),
                        RotPoint::from_degrees(0.0, DISTANCE),
                        RotPoint::from_degrees(0.0, DISTANCE),
                        RotPoint::from_degrees(0.0, DISTANCE),
                        RotPoint::from_degrees(0.0, DISTANCE),
                        RotPoint::from_degrees(0.0, DISTANCE),
                        RotPoint::from_degrees(-90.0, DISTANCE),
                        RotPoint::from_degrees(0.0, DISTANCE),
                        RotPoint::from_degrees(0.0, DISTANCE),
                        RotPoint::from_degrees(0.0, DISTANCE),
                        RotPoint::from_degrees(0.0, DISTANCE),
                        RotPoint::from_degrees(0.0, DISTANCE),
                        RotPoint::from_degrees(0.0, DISTANCE),
                        // RotPoint::from_degrees(90.0, DISTANCE),
                    ])),
                ),
                Template::new(
                    "Square clockwise".into(),
                    Model::new(RotTrace::new(vec![
                        RotPoint::from_degrees(0.0, DISTANCE),
                        RotPoint::from_degrees(0.0, DISTANCE),
                        RotPoint::from_degrees(0.0, DISTANCE),
                        RotPoint::from_degrees(0.0, DISTANCE),
                        RotPoint::from_degrees(0.0, DISTANCE),
                        RotPoint::from_degrees(0.0, DISTANCE),
                        RotPoint::from_degrees(0.0, DISTANCE),
                        RotPoint::from_degrees(0.0, DISTANCE),
                        RotPoint::from_degrees(0.0, DISTANCE),
                        RotPoint::from_degrees(0.0, DISTANCE),
                        RotPoint::from_degrees(0.0, DISTANCE),
                        RotPoint::from_degrees(0.0, DISTANCE),
                        RotPoint::from_degrees(0.0, DISTANCE),
                        RotPoint::from_degrees(0.0, DISTANCE),
                        RotPoint::from_degrees(0.0, DISTANCE),
                        RotPoint::from_degrees(-90.0, DISTANCE),
                        RotPoint::from_degrees(0.0, DISTANCE),
                        RotPoint::from_degrees(0.0, DISTANCE),
                        RotPoint::from_degrees(0.0, DISTANCE),
                        RotPoint::from_degrees(0.0, DISTANCE),
                        RotPoint::from_degrees(0.0, DISTANCE),
                        RotPoint::from_degrees(0.0, DISTANCE),
                        RotPoint::from_degrees(0.0, DISTANCE),
                        RotPoint::from_degrees(0.0, DISTANCE),
                        RotPoint::from_degrees(0.0, DISTANCE),
                        RotPoint::from_degrees(0.0, DISTANCE),
                        RotPoint::from_degrees(0.0, DISTANCE),
                        RotPoint::from_degrees(0.0, DISTANCE),
                        RotPoint::from_degrees(0.0, DISTANCE),
                        RotPoint::from_degrees(0.0, DISTANCE),
                        RotPoint::from_degrees(0.0, DISTANCE),
                        RotPoint::from_degrees(-90.0, DISTANCE),
                        RotPoint::from_degrees(0.0, DISTANCE),
                        RotPoint::from_degrees(0.0, DISTANCE),
                        RotPoint::from_degrees(0.0, DISTANCE),
                        RotPoint::from_degrees(0.0, DISTANCE),
                        RotPoint::from_degrees(0.0, DISTANCE),
                        RotPoint::from_degrees(0.0, DISTANCE),
                        RotPoint::from_degrees(0.0, DISTANCE),
                        RotPoint::from_degrees(0.0, DISTANCE),
                        RotPoint::from_degrees(0.0, DISTANCE),
                        RotPoint::from_degrees(0.0, DISTANCE),
                        RotPoint::from_degrees(0.0, DISTANCE),
                        RotPoint::from_degrees(0.0, DISTANCE),
                        RotPoint::from_degrees(0.0, DISTANCE),
                        RotPoint::from_degrees(0.0, DISTANCE),
                        RotPoint::from_degrees(0.0, DISTANCE),
                        RotPoint::from_degrees(-90.0, DISTANCE),
                        RotPoint::from_degrees(0.0, DISTANCE),
                        RotPoint::from_degrees(0.0, DISTANCE),
                        RotPoint::from_degrees(0.0, DISTANCE),
                        RotPoint::from_degrees(0.0, DISTANCE),
                        RotPoint::from_degrees(0.0, DISTANCE),
                        RotPoint::from_degrees(0.0, DISTANCE),
                        RotPoint::from_degrees(0.0, DISTANCE),
                        RotPoint::from_degrees(0.0, DISTANCE),
                        RotPoint::from_degrees(0.0, DISTANCE),
                        RotPoint::from_degrees(0.0, DISTANCE),
                        RotPoint::from_degrees(0.0, DISTANCE),
                        RotPoint::from_degrees(0.0, DISTANCE),
                        RotPoint::from_degrees(0.0, DISTANCE),
                        RotPoint::from_degrees(0.0, DISTANCE),
                        RotPoint::from_degrees(0.0, DISTANCE),
                    ])),
                ),
                Template::new(
                    "Square counter-clockwise".into(),
                    Model::new(RotTrace::new(vec![
                        RotPoint::from_degrees(0.0, DISTANCE),
                        RotPoint::from_degrees(0.0, DISTANCE),
                        RotPoint::from_degrees(0.0, DISTANCE),
                        RotPoint::from_degrees(0.0, DISTANCE),
                        RotPoint::from_degrees(0.0, DISTANCE),
                        RotPoint::from_degrees(0.0, DISTANCE),
                        RotPoint::from_degrees(0.0, DISTANCE),
                        RotPoint::from_degrees(0.0, DISTANCE),
                        RotPoint::from_degrees(0.0, DISTANCE),
                        RotPoint::from_degrees(0.0, DISTANCE),
                        RotPoint::from_degrees(0.0, DISTANCE),
                        RotPoint::from_degrees(0.0, DISTANCE),
                        RotPoint::from_degrees(0.0, DISTANCE),
                        RotPoint::from_degrees(0.0, DISTANCE),
                        RotPoint::from_degrees(0.0, DISTANCE),
                        RotPoint::from_degrees(90.0, DISTANCE),
                        RotPoint::from_degrees(0.0, DISTANCE),
                        RotPoint::from_degrees(0.0, DISTANCE),
                        RotPoint::from_degrees(0.0, DISTANCE),
                        RotPoint::from_degrees(0.0, DISTANCE),
                        RotPoint::from_degrees(0.0, DISTANCE),
                        RotPoint::from_degrees(0.0, DISTANCE),
                        RotPoint::from_degrees(0.0, DISTANCE),
                        RotPoint::from_degrees(0.0, DISTANCE),
                        RotPoint::from_degrees(0.0, DISTANCE),
                        RotPoint::from_degrees(0.0, DISTANCE),
                        RotPoint::from_degrees(0.0, DISTANCE),
                        RotPoint::from_degrees(0.0, DISTANCE),
                        RotPoint::from_degrees(0.0, DISTANCE),
                        RotPoint::from_degrees(0.0, DISTANCE),
                        RotPoint::from_degrees(0.0, DISTANCE),
                        RotPoint::from_degrees(90.0, DISTANCE),
                        RotPoint::from_degrees(0.0, DISTANCE),
                        RotPoint::from_degrees(0.0, DISTANCE),
                        RotPoint::from_degrees(0.0, DISTANCE),
                        RotPoint::from_degrees(0.0, DISTANCE),
                        RotPoint::from_degrees(0.0, DISTANCE),
                        RotPoint::from_degrees(0.0, DISTANCE),
                        RotPoint::from_degrees(0.0, DISTANCE),
                        RotPoint::from_degrees(0.0, DISTANCE),
                        RotPoint::from_degrees(0.0, DISTANCE),
                        RotPoint::from_degrees(0.0, DISTANCE),
                        RotPoint::from_degrees(0.0, DISTANCE),
                        RotPoint::from_degrees(0.0, DISTANCE),
                        RotPoint::from_degrees(0.0, DISTANCE),
                        RotPoint::from_degrees(0.0, DISTANCE),
                        RotPoint::from_degrees(0.0, DISTANCE),
                        RotPoint::from_degrees(90.0, DISTANCE),
                        RotPoint::from_degrees(0.0, DISTANCE),
                        RotPoint::from_degrees(0.0, DISTANCE),
                        RotPoint::from_degrees(0.0, DISTANCE),
                        RotPoint::from_degrees(0.0, DISTANCE),
                        RotPoint::from_degrees(0.0, DISTANCE),
                        RotPoint::from_degrees(0.0, DISTANCE),
                        RotPoint::from_degrees(0.0, DISTANCE),
                        RotPoint::from_degrees(0.0, DISTANCE),
                        RotPoint::from_degrees(0.0, DISTANCE),
                        RotPoint::from_degrees(0.0, DISTANCE),
                        RotPoint::from_degrees(0.0, DISTANCE),
                        RotPoint::from_degrees(0.0, DISTANCE),
                        RotPoint::from_degrees(0.0, DISTANCE),
                        RotPoint::from_degrees(0.0, DISTANCE),
                        RotPoint::from_degrees(0.0, DISTANCE),
                    ])),
                ),
                Template::new(
                    "Z".into(),
                    Model::new(RotTrace::new(vec![
                        RotPoint::from_degrees(0.0, DISTANCE),
                        RotPoint::from_degrees(0.0, DISTANCE),
                        RotPoint::from_degrees(0.0, DISTANCE),
                        RotPoint::from_degrees(0.0, DISTANCE),
                        RotPoint::from_degrees(0.0, DISTANCE),
                        RotPoint::from_degrees(0.0, DISTANCE),
                        RotPoint::from_degrees(0.0, DISTANCE),
                        RotPoint::from_degrees(0.0, DISTANCE),
                        RotPoint::from_degrees(0.0, DISTANCE),
                        RotPoint::from_degrees(0.0, DISTANCE),
                        RotPoint::from_degrees(0.0, DISTANCE),
                        RotPoint::from_degrees(0.0, DISTANCE),
                        RotPoint::from_degrees(0.0, DISTANCE),
                        RotPoint::from_degrees(0.0, DISTANCE),
                        RotPoint::from_degrees(0.0, DISTANCE),
                        RotPoint::from_degrees(-120.0, DISTANCE),
                        RotPoint::from_degrees(0.0, DISTANCE),
                        RotPoint::from_degrees(0.0, DISTANCE),
                        RotPoint::from_degrees(0.0, DISTANCE),
                        RotPoint::from_degrees(0.0, DISTANCE),
                        RotPoint::from_degrees(0.0, DISTANCE),
                        RotPoint::from_degrees(0.0, DISTANCE),
                        RotPoint::from_degrees(0.0, DISTANCE),
                        RotPoint::from_degrees(0.0, DISTANCE),
                        RotPoint::from_degrees(0.0, DISTANCE),
                        RotPoint::from_degrees(0.0, DISTANCE),
                        RotPoint::from_degrees(0.0, DISTANCE),
                        RotPoint::from_degrees(0.0, DISTANCE),
                        RotPoint::from_degrees(0.0, DISTANCE),
                        RotPoint::from_degrees(0.0, DISTANCE),
                        RotPoint::from_degrees(0.0, DISTANCE),
                        RotPoint::from_degrees(120.0, DISTANCE),
                        RotPoint::from_degrees(0.0, DISTANCE),
                        RotPoint::from_degrees(0.0, DISTANCE),
                        RotPoint::from_degrees(0.0, DISTANCE),
                        RotPoint::from_degrees(0.0, DISTANCE),
                        RotPoint::from_degrees(0.0, DISTANCE),
                        RotPoint::from_degrees(0.0, DISTANCE),
                        RotPoint::from_degrees(0.0, DISTANCE),
                        RotPoint::from_degrees(0.0, DISTANCE),
                        RotPoint::from_degrees(0.0, DISTANCE),
                        RotPoint::from_degrees(0.0, DISTANCE),
                        RotPoint::from_degrees(0.0, DISTANCE),
                        RotPoint::from_degrees(0.0, DISTANCE),
                        RotPoint::from_degrees(0.0, DISTANCE),
                        RotPoint::from_degrees(0.0, DISTANCE),
                        RotPoint::from_degrees(0.0, DISTANCE),
                    ])),
                ),
            ]),
        }
    }

    /// Delete the template with the given `id`.
    /// Nothing happends if no template exists with the specified `id`.
    ///
    /// This method is expensive, as it clones the list of templates.
    pub fn delete(&self, id: u32) -> Result<()> {
        // Remove the template
        self.templates
            .lock()
            .expect("failed to lock templates list to remove item")
            .retain(|template| template.id() != id);

        // Save the results
        self.save()
    }


    /// Get a list of templates available in this store.
    ///
    /// This method is expensive, as it clones the list of templates.
    pub fn to_templates(&self) -> Vec<Template> {
        self.templates
            .lock()
            .expect("failed to lock templates list")
            .clone()
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
            let _ = fs::remove_file(FILE);
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
        // TODO: do not clone to improve performance
        let templates = self
            .templates
            .lock()
            .expect("failed to lock templates list for detecting gestures");

        // Match if there is any
        if !templates.is_empty() {
            templates
                .par_iter()
                .find_any(|template| template.model().matches(other.model()))
                .cloned()
        } else {
            None
        }
    }
}
