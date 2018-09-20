use types::Model;

#[derive(Debug, Serialize, Deserialize)]
pub struct Template {
    name: String,
    model: Model,
}
