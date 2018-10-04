use std::{cmp::min, f64::consts::PI, fs, io::Result, path::Path, sync::Mutex};

use toml;

use config::{
    recognition::{GROUP_DIFF_MAX, GROUP_SIZE, POINT_DIFF_MAX, TOTAL_DIFF_MAX},
    template::FILE,
};
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

    /// Compare a given model against the tempaltes,
    /// to see whether there is a gesture match.
    pub fn detect_gesture(&self, other: &mut Fragment) {
        // Obtain a templates list lock
        let templates = self
            .templates
            .lock()
            .expect("failed to lock templates list for detecting gestures");

        // Loop through each template
        for template in &*templates {
            // Get the model to compare against
            let model = template.model();

            {
                // Get the model and other model points
                let model_points = model.trace().points();
                let other_points = other.model().trace().points();
                let model_count = model_points.len();
                let other_count = other_points.len();

                // Skip if the template has more points than our current trace
                if other_count < model_count {
                    continue;
                }

                // Determine how many points to process, minimum length wins
                let count = min(model_points.len(), other_points.len());

                // Select the last points based on the determined count to use
                let model_points = &model.trace().points()[model_count - count..model_count];
                let other_points =
                    &other.model().trace().points()[other_count - count..other_count];

                // Caluclate the difference for each point
                let diff = model_points
                    .iter()
                    .rev()
                    .zip(other_points.iter().rev())
                    .map(|(a, b)| rad_diff(b.radians(), a.radians()));

                // Calculate the cummulative difference on each point
                let cum_diff: Vec<f64> = diff
                    .scan(0.0, |acc, p| {
                        *acc += p;
                        Some(*acc)
                    })
                    .collect();

                // Skip if the total difference is too big
                if cum_diff.last().unwrap().abs() > TOTAL_DIFF_MAX {
                    continue;
                }

                // Skip if any of the points has a difference of more than 2
                if cum_diff.iter().any(|p| p.abs() > POINT_DIFF_MAX) {
                    continue;
                }

                // Skip if each window of 5 points has an average difference bigger than 1
                if GROUP_SIZE > 0 && cum_diff
                    .windows(GROUP_SIZE)
                    .any(|p| (p.iter().sum::<f64>().abs() / GROUP_SIZE as f64) > GROUP_DIFF_MAX)
                {
                    continue;
                }

                println!("### HIT: {}", template.name());
            }

            // Clear the fragment history
            other.clear();

            break;
        }
    }
}

/// Calculate the difference between two circular radian angles.
///
/// The returned value will always be `(-PI, PI]`.
fn rad_diff(a: f64, b: f64) -> f64 {
    (a - b + PI).mod_euc(2.0 * PI) - PI
}
