use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use crate::assets::global_manager::ASSET_MANAGER;
use crate::assets::icon_theme_asset_source::IconThemeAssetSource;
use crate::assets::manager::Manager;
use crate::assets::window_controls_asset_source::WindowControlsAssetSource;
use contemporary_config::LocalisedString;
use contemporary_i18n::tr;
use gpui::{Application, Global, SharedString};
use indexmap::IndexMap;

#[derive(Hash, Eq, Clone, PartialEq, Debug)]
pub enum ApplicationLink {
    FileBug,
    SourceCode,
    HelpContents,
    Other {
        icon: &'static str,
        text: SharedString,
    },
}

impl ApplicationLink {
    pub fn get_name(&self) -> SharedString {
        match self {
            ApplicationLink::FileBug => tr!("HELP_FILE_BUG", "File Bug").into(),
            ApplicationLink::SourceCode => tr!("HELP_SOURCE_CODE", "Source Code").into(),
            ApplicationLink::HelpContents => tr!("HELP").into(),
            ApplicationLink::Other { icon: _, text } => text.clone(),
        }
    }

    pub fn get_icon(&self) -> SharedString {
        match self {
            ApplicationLink::FileBug => "tools-report-bug".into(),
            ApplicationLink::SourceCode => "commit".into(),
            ApplicationLink::HelpContents => "help-contents".into(),
            ApplicationLink::Other { icon, text: _ } => (**icon).into(),
        }
    }
}

pub struct GeneratableDetails {
    pub application_name: LocalisedString,
    pub application_generic_name: LocalisedString,
    pub desktop_entry: &'static str,
}

pub struct Details {
    pub generatable: GeneratableDetails,
    pub application_version: &'static str,
    pub copyright_holder: &'static str,
    pub copyright_year: &'static str,
    pub license: License,
    pub links: IndexMap<ApplicationLink, &'static str>,
}

impl Global for Details {}

pub enum License {
    Gpl3,
    Gpl3OrLater,
    Gpl2,
    Gpl2OrLater,
    Lgpl3,
    Lgpl3OrLater,
    Lgpl2_1,
    Lgpl2_1OrLater,
    Mit,
    Other(SharedString),
}

pub struct Versions {
    pub contemporary_version: &'static str,
    pub versions: Arc<Mutex<HashMap<SharedString, SharedString>>>,
}

impl Global for Versions {}

pub fn new_contemporary_application() -> Application {
    ASSET_MANAGER.read().unwrap().add_sources(vec![
        Box::new(IconThemeAssetSource),
        Box::new(WindowControlsAssetSource),
    ]);

    Application::new().with_assets(Manager)
}
