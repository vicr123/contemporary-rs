use crate::assets::global_manager::ASSET_MANAGER;
use gpui::{AssetSource, SharedString};
use std::borrow::Cow;

pub struct Manager;

impl AssetSource for Manager {
    fn load(&self, path: &str) -> gpui::Result<Option<Cow<'static, [u8]>>> {
        ASSET_MANAGER.read().unwrap().load(path)
    }

    fn list(&self, path: &str) -> gpui::Result<Vec<SharedString>> {
        ASSET_MANAGER.read().unwrap().list(path)
    }
}
