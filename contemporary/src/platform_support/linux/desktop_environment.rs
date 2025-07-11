use std::env;

pub enum DesktopEnvironment {
    GNOME,
    KDE
}

impl DesktopEnvironment {
    pub fn current() -> Option<DesktopEnvironment> {
        match env::var("XDG_CURRENT_DESKTOP").unwrap_or_default().as_str() {
            "KDE" => Some(DesktopEnvironment::KDE),
            _ => None
        }
    }
}