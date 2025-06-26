use crate::assets::global_manager::ManagerSource;
use gpui::SharedString;
use std::borrow::Cow;
use url::Url;

pub struct WindowControlsAssetSource;

impl ManagerSource for WindowControlsAssetSource {
    fn scheme(&self) -> &'static str {
        "window-controls"
    }

    #[allow(unused_variables)]
    #[allow(unreachable_code)]
    fn load(&self, url: &Url) -> gpui::Result<Option<Cow<'static, [u8]>>> {
        if url.scheme() != "window-controls" {
            return Ok(None);
        }

        Ok(match url.path() {
            "/close" => Some(Cow::Borrowed(include_bytes!("../../assets/close.svg"))),
            "/min" => Some(Cow::Borrowed(include_bytes!("../../assets/min.svg"))),
            "/max" => Some(Cow::Borrowed(include_bytes!("../../assets/max.svg"))),
            "/res" => Some(Cow::Borrowed(include_bytes!("../../assets/res.svg"))),
            _ => None,
        })
    }

    fn list(&self, path: &str) -> gpui::Result<Vec<SharedString>> {
        Ok(Vec::new())
    }
}
