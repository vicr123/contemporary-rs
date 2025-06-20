use freedesktop_icons::lookup;
use gpui::{AssetSource, SharedString};
use std::borrow::Cow;
use url::Url;

pub struct IconAssetSource;

impl AssetSource for IconAssetSource {
    fn load(&self, path: &str) -> gpui::Result<Option<Cow<'static, [u8]>>> {
        let Ok(url) = Url::parse(path) else {
            return Ok(None);
        };

        if url.scheme() != "icon" {
            return Ok(None);
        }

        let size = url
            .query_pairs()
            .find(|(k, _)| k == "size")
            .map(|(_, value)| value.parse::<f32>().ok())
            .flatten()
            .unwrap_or(16.);

        let Some(file) = lookup(&url.path()[1..])
            .with_cache()
            .with_theme(url.host_str().unwrap())
            .with_size(size as u16)
            .find()
        else {
            return Ok(None);
        };

        let Ok(contents) = std::fs::read(file) else {
            return Ok(None);
        };

        Ok(Some(Cow::Owned(contents)))
    }

    fn list(&self, path: &str) -> gpui::Result<Vec<SharedString>> {
        Ok(Vec::new())
    }
}
