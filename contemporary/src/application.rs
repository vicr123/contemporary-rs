use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use gpui::{Global, SharedString};

pub struct Details {
    pub application_name: &'static str,
    pub application_generic_name: &'static str,
    pub application_version: &'static str,
    pub copyright_holder: &'static str,
    pub copyright_year: &'static str,
    pub desktop_entry: &'static str,
    pub license: License,
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
