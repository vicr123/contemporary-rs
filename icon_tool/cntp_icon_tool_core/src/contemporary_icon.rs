use regex::Regex;
use std::fs::OpenOptions;
use std::io::Read;
use std::path::PathBuf;

pub struct ContemporaryIcon {
    source: String,
    is_mac_icon: bool,
    generate_blueprint_icon: bool,
}

impl ContemporaryIcon {
    pub fn new(path: PathBuf, is_mac_icon: bool, generate_blueprint_icon: bool) -> Self {
        let mut file = OpenOptions::new()
            .read(true)
            .open(path)
            .expect("Failed to open file");
        let mut contents = String::new();
        file.read_to_string(&mut contents)
            .expect("Failed to read file");
        Self {
            source: contents,
            is_mac_icon,
            generate_blueprint_icon,
        }
    }

    pub fn generate(self, theme_color_1: &str, theme_color_2: &str) -> String {
        let mut base_resource = {
            if self.generate_blueprint_icon {
                if self.is_mac_icon {
                    include_str!("../assets/base-app-icon-blueprint-mac.svg")
                } else {
                    include_str!("../assets/base-app-icon-blueprint.svg")
                }
            } else if self.is_mac_icon {
                include_str!("../assets/base-app-icon-mac.svg")
            } else {
                include_str!("../assets/base-app-icon.svg")
            }
        }
        .to_string();

        if !self.generate_blueprint_icon {
            base_resource = base_resource
                .replace("%1", theme_color_1)
                .replace("%2", theme_color_2);
        }

        // Set regex to match newlines with the "s" flag (equivalent to DotMatchesEverythingOption)
        let layer_group_regex =
            Regex::new("(?s)</g>.+(<g.+id=\"iconlayer\".+</g>)").expect("Failed to create regex");

        // Match against overlayIcon (which would be self.source in this context)
        let captured = match layer_group_regex.captures(&self.source) {
            Some(captures) => captures.get(1).map(|m| m.as_str()).unwrap_or(""),
            None => return String::new(),
        };

        base_resource = base_resource.replace("%3", captured);

        base_resource
    }
}
