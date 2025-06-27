use serde::Deserialize;

#[derive(Deserialize)]
pub struct ContemporaryConfigApplication {
    pub theme_colors: Vec<String>,
}
