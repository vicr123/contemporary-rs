use crate::LocalisedString;
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Deserialize)]
pub struct ContemporaryConfigApplicationDef {
    pub theme_colors: Vec<String>,
}

#[derive(Deserialize, Clone)]
pub struct ContemporaryConfigConfigDef {
    pub i18n_dir: Option<String>,
}

#[derive(Deserialize, Clone, Default)]
pub struct ContemporaryConfigDeploymentDef {
    pub application_name: Option<String>,
    pub application_generic_name: Option<String>,
    pub desktop_entry: Option<String>,
    pub icon: Option<String>,
    pub contemporary_base_icon: Option<String>,

    // macOS only
    pub apple_localisation_dir: Option<String>,
    pub extra_info_plist_attributes: Option<HashMap<String, String>>,
    pub minimum_system_version: Option<String>,
    pub supports_automatic_graphics_switching: Option<bool>,

    #[serde(flatten)]
    pub children: HashMap<String, ContemporaryConfigDeploymentDef>,
}

pub struct ContemporaryConfigDeployment {
    pub application_name: Option<LocalisedString>,
    pub application_generic_name: Option<LocalisedString>,
    pub desktop_entry: Option<LocalisedString>,
    pub icon: Option<String>,
    pub contemporary_base_icon: Option<String>,

    // macOS only
    pub apple_localisation_dir: Option<LocalisedString>,
    pub extra_info_plist_attributes: HashMap<String, LocalisedString>,
    pub minimum_system_version: String,
    pub supports_automatic_graphics_switching: bool,
}
