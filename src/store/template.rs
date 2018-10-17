use std::{fs, io::Result, path::PathBuf, sync::Mutex};

use directories::ProjectDirs;
use rayon::prelude::*;
use serde_json;

use config::{sample::DISTANCE, template::TEMPLATES_FILE};
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
            templates: Mutex::new(Vec::new()),
        }
    }

    /// Add the given template.
    pub fn add(&self, template: Template) -> Result<()> {
        // Add the template
        self.templates
            .lock()
            .expect("failed to lock templates list to add item")
            .push(template);

        // Save the results
        self.save()
    }

    /// Add a list of templates.
    pub fn add_list(&self, mut templates: Vec<Template>) -> Result<()> {
        // Add the template
        self.templates
            .lock()
            .expect("failed to lock templates list to add items")
            .append(&mut templates);

        // Save the results
        self.save()
    }

    /// Add the built-in list of templates to the store.
    pub fn add_builtin(&self) -> Result<()> {
        self.add_list(builtin_templates())
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

    /// Delete all templates.
    pub fn delete_all(&self) -> Result<()> {
        // Remove the template
        self.templates
            .lock()
            .expect("failed to lock templates list to delete all")
            .clear();

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
        // Get the file path
        let file = Self::file();

        // Ensure a file exists
        if !file.is_file() {
            eprintln!("Not loading templates, no file exists");
            return Ok(());
        }

        // Obtain a templates list lock
        let mut templates = self
            .templates
            .lock()
            .expect("failed to lock templates list for loading");

        println!("Loading templates from {}...", file.to_str().unwrap_or("?"));

        // Load, deserialize and set the list of templates
        *templates = serde_json::from_str(&fs::read_to_string(file)?)
            .expect("failed to deserialize templates from loaded file");

        println!("Loaded {} template(s)", templates.len());

        Ok(())
    }

    /// Save the current list of templates to a file.
    ///
    /// TODO: handle errors properly, return an error on failure instead of panicing.
    pub fn save(&self) -> Result<()> {
        // Obtain a templates list lock
        let templates = self
            .templates
            .lock()
            .expect("failed to lock templates list for saving");

        // Determine where to save
        let file = Self::file();

        // Remove template files if there are not tempaltes to save
        if templates.is_empty() {
            let _ = fs::remove_file(file);
            return Ok(());
        }

        println!(
            "Saving {} template(s) to {}...",
            templates.len(),
            file.to_str().unwrap_or("?"),
        );

        // Create all parent directories
        fs::create_dir_all(
            file.parent()
                .expect("failed to determine parent directory of template save file location"),
        )
        .expect("failed to create directory to store templates file in");

        // Write the file
        fs::write(
            file,
            serde_json::to_string_pretty(&*templates)
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

    /// Get the file the templates are saved to.
    fn file<'a>() -> PathBuf {
        ProjectDirs::from("", "", crate_name!())
            .unwrap()
            .cache_dir()
            .join(TEMPLATES_FILE)
    }
}

/// Generate a list of built-in templates.
fn builtin_templates() -> Vec<Template> {
    vec![
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
    ]
}
