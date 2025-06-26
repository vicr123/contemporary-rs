use gpui::{AssetSource, SharedString};
use once_cell::sync::Lazy;
use std::borrow::Cow;
use std::sync::{Arc, RwLock};
use url::Url;

pub static ASSET_MANAGER: Lazy<RwLock<GlobalManager>> =
    Lazy::new(|| RwLock::new(GlobalManager::default()));

pub trait ManagerSource {
    fn scheme(&self) -> &'static str;
    fn load(&self, url: &Url) -> gpui::Result<Option<Cow<'static, [u8]>>>;
    fn list(&self, path: &str) -> gpui::Result<Vec<SharedString>>;
}

pub struct GlobalManager {
    sources: Arc<RwLock<Vec<Box<dyn ManagerSource + Send + Sync>>>>,
}

impl Default for GlobalManager {
    fn default() -> Self {
        GlobalManager {
            sources: Arc::new(RwLock::new(Vec::new())),
        }
    }
}

impl GlobalManager {
    pub fn add_source(&self, source: Box<dyn ManagerSource + Send + Sync>) {
        if let Ok(mut sources) = self.sources.write() {
            sources.push(source);
        }
    }

    pub fn add_sources(&self, mut sources: Vec<Box<dyn ManagerSource + Send + Sync>>) {
        if let Ok(mut global_sources) = self.sources.write() {
            global_sources.append(&mut sources);
        }
    }
}

impl AssetSource for GlobalManager {
    #[allow(unused_variables)]
    #[allow(unreachable_code)]
    fn load(&self, path: &str) -> gpui::Result<Option<Cow<'static, [u8]>>> {
        let Ok(url) = Url::parse(path) else {
            return Ok(None);
        };

        for source in self.sources.read().unwrap().iter() {
            if url.scheme() != source.scheme() {
                continue;
            }

            let data = source.load(&url);
            if let Ok(Some(data)) = data {
                return Ok(Some(data));
            }
        }

        Ok(None)
    }

    fn list(&self, _path: &str) -> gpui::Result<Vec<SharedString>> {
        Ok(Vec::new())
    }
}
