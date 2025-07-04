use crate::assets::global_manager::ManagerSource;
#[cfg(target_os = "linux")]
use freedesktop_icons::lookup;
use gpui::SharedString;
use rust_embed::RustEmbed;
use std::borrow::Cow;
use url::Url;

pub struct IconThemeAssetSource;

#[cfg(not(target_os = "linux"))]
#[derive(RustEmbed)]
#[folder = "../submodules/contemporary-icons"]
struct BundledContemporaryIcons;

#[cfg(not(target_os = "linux"))]
struct BundledContemporaryIconDescriptor {
    name: String,
    category: String,
    size: u8,
}

#[cfg(not(target_os = "linux"))]
impl BundledContemporaryIcons {
    pub fn load_icon(
        descriptor: &BundledContemporaryIconDescriptor,
    ) -> gpui::Result<Option<Cow<'static, [u8]>>> {
        let name = &descriptor.name;
        let category = &descriptor.category;
        let size = descriptor.size;
        Ok(Self::get(format!("{category}/{size}/{name}.svg").as_str())
            .map(|f| Some(f.data))
            .unwrap_or(None))
    }

    pub fn sizes(icon_name: &str) -> Vec<BundledContemporaryIconDescriptor> {
        let icon_file_name = format!("{icon_name}.svg");
        Self::iter()
            .filter_map(|p| {
                if p.ends_with(icon_file_name.as_str()) {
                    if let Some(descriptor) = Self::path_to_descriptor(&p) {
                        return Some(descriptor);
                    }
                }
                None
            })
            .collect()
    }

    fn path_to_descriptor(path: &str) -> Option<BundledContemporaryIconDescriptor> {
        let parts: Vec<&str> = path.split('/').collect();

        if parts.len() < 3 {
            return None;
        }

        let category = parts[parts.len() - 3].to_string();
        let size: u8 = parts[parts.len() - 2].parse().ok()?;
        let name = parts[parts.len() - 1].strip_suffix(".svg")?.to_string();

        Some(BundledContemporaryIconDescriptor {
            category,
            size,
            name,
        })
    }
}

impl ManagerSource for IconThemeAssetSource {
    fn scheme(&self) -> &'static str {
        "icon"
    }

    #[allow(unused_variables)]
    #[allow(unreachable_code)]
    fn load(&self, url: &Url) -> gpui::Result<Option<Cow<'static, [u8]>>> {
        if url.scheme() != "icon" {
            return Ok(None);
        }

        let size = url
            .query_pairs()
            .find(|(k, _)| k == "size")
            .and_then(|(_, value)| value.parse::<f32>().ok())
            .unwrap_or(16.);

        let icon_name = &url.path()[1..];

        #[cfg(target_os = "linux")]
        {
            let Some(file) = lookup(icon_name)
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

        #[cfg(not(target_os = "linux"))]
        {
            let icons = BundledContemporaryIcons::sizes(icon_name);
            if icons.is_empty() {
                return Ok(None);
            }

            let preferred_icon = icons
                .iter()
                .filter(|icon| icon.size as f32 <= size)
                .max_by_key(|icon| icon.size)
                .unwrap_or_else(|| icons.iter().min_by_key(|icon| icon.size).unwrap());

            BundledContemporaryIcons::load_icon(preferred_icon)
        }
    }

    fn list(&self, _path: &str) -> gpui::Result<Vec<SharedString>> {
        Ok(Vec::new())
    }
}
