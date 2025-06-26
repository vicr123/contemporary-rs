use gpui::{AssetSource, SharedString};
use std::borrow::Cow;
use url::Url;

pub trait ManagerSource {
    fn scheme(&self) -> &'static str;
    fn load(&self, path: &str) -> gpui::Result<Option<Cow<'static, [u8]>>>;
    fn list(&self, path: &str) -> gpui::Result<Vec<SharedString>>;
}

#[derive(Default)]
pub struct Manager {
    sources: Vec<Box<dyn ManagerSource + Send + Sync>>,
}

impl Manager {
    pub fn add_source(&mut self, source: Box<dyn ManagerSource + Send + Sync>) {
        self.sources.push(source);   
    }
}

impl AssetSource for Manager {
    #[allow(unused_variables)]
    #[allow(unreachable_code)]
    fn load(&self, path: &str) -> gpui::Result<Option<Cow<'static, [u8]>>> {
        let Ok(url) = Url::parse(path) else {
            return Ok(None);
        };

        for source in &self.sources {
            if url.scheme() != source.scheme() {
                continue;
            }

            let data = source.load(url.path());
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
