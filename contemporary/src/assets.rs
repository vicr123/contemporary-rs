use gpui::{AssetSource, SharedString};
use std::borrow::Cow;

#[cfg(target_os = "linux")]
use freedesktop_icons::lookup;

pub struct IconAssetSource;

impl AssetSource for IconAssetSource {
    #[allow(unused_variables)]
    #[allow(unreachable_code)]
    fn load(&self, path: &str) -> gpui::Result<Option<Cow<'static, [u8]>>> {
        #[cfg(target_os = "linux")]
        {
            use url::Url;

            let Ok(url) = Url::parse(path) else {
                return Ok(None);
            };

            if url.scheme() != "icon" {
                return Ok(None);
            }

            let size = url
                .query_pairs()
                .find(|(k, _)| k == "size")
                .and_then(|(_, value)| value.parse::<f32>().ok())
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

            return Ok(Some(Cow::Owned(contents)));
        }

        Ok(None)
    }

    fn list(&self, _path: &str) -> gpui::Result<Vec<SharedString>> {
        Ok(Vec::new())
    }
}
