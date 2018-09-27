use std::{cmp::min, fs, io::Result, path::Path, sync::Mutex};

use toml;

use types::{Model, RotPoint, RotTrace, Template};

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
            // TODO: after debugging, load an emtpy list of templates instead
            templates: Mutex::new(vec![Template::new(
                "dummy".into(),
                Model::new(RotTrace::new(vec![RotPoint::new(0.0); 64])),
            )]),
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
        let path = Path::new(TEMPLATES_FILE_PATH);

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
            fs::remove_file(TEMPLATES_FILE_PATH);
            return Ok(());
        }

        println!("Saving {} template(s) to file...", templates.len());
        fs::write(
            TEMPLATES_FILE_PATH,
            toml::to_string(&*templates)
                .expect("failed to serialize template data, unable to save"),
        )
    }

    /// Compare a given model against the tempaltes,
    /// to see whether there is a gesture match.
    pub fn detect_gesture(&self, other: &Model) {
        // Obtain a templates list lock
        let templates = self
            .templates
            .lock()
            .expect("failed to lock templates list for detecting gestures");

        // Loop through each template
        for template in &*templates {
            // Get the model to compare against
            let model = template.model();

            // Get the model and other model points
            let model_points = model.trace().points();
            let other_points = other.trace().points();
            let model_count = model_points.len();
            let other_count = other_points.len();

            // Determine how many points to process, minimum length wins
            let count = min(model_points.len(), other_points.len());

            // Select the last points based on the determined count to use
            let model_points = &model.trace().points()[model_count - count..model_count];
            let other_points = &other.trace().points()[other_count - count..other_count];

            // Caluclate the difference for each point
            let diff: Vec<f64> = model_points
                .iter()
                .rev()
                .zip(other_points.iter().rev())
                .map(|(a, b)| b.radians() - a.radians())
                .collect();

            // Report the points
            if !diff.is_empty() {
                let avg = diff.iter().sum::<f64>() / diff.len() as f64;
                println!("Diff: {}", avg);
                // println!("Diff: {:?}", diff);
            }
        }
    }
}
