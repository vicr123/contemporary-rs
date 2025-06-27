use std::collections::HashMap;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct ContemporaryConfigApplication {
    pub theme_colors: Vec<String>,
}

#[derive(Deserialize, Clone)]
pub struct ContemporaryConfigDeployment {
    pub application_name: Option<String>,
    pub apple_localisation_dir: Option<String>,
    pub desktop_entry: Option<String>,
    pub icon: Option<String>,
    pub contemporary_base_icon: Option<String>,

    #[serde(flatten)]
    pub children: HashMap<String, ContemporaryConfigDeployment>
}

impl ContemporaryConfigDeployment {
    pub fn new() -> Self {
        Self {
            application_name: None,
            apple_localisation_dir: None,
            desktop_entry: None,
            icon: None,
            contemporary_base_icon: None,
            children: HashMap::new()
        }
    }
}
