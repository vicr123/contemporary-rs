use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use gpui::{Application, Global, SharedString};
use indexmap::IndexMap;
use crate::assets::icon_theme_asset_source::IconThemeAssetSource;
use crate::assets::manager::Manager;

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
            ApplicationLink::FileBug => "File Bug".into(),
            ApplicationLink::SourceCode => "Source Code".into(),
            ApplicationLink::HelpContents => "Help".into(),
            ApplicationLink::Other { icon: _, text } => text.clone(),
        }
    }
}

pub struct Details {
    pub application_name: &'static str,
    pub application_generic_name: &'static str,
    pub application_version: &'static str,
    pub copyright_holder: &'static str,
    pub copyright_year: &'static str,
    pub desktop_entry: &'static str,
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
    let mut asset_manager = Manager::default();
    asset_manager.add_source(Box::new(IconThemeAssetSource));
    
    Application::new()
        .with_assets(asset_manager)
}