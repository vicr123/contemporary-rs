use crate::linux::flatpak::deploy_flatpak;
use crate::tool_setup::ToolSetup;
use std::process::exit;
use tracing::error;

pub fn deploy_linux(setup_data: &ToolSetup, platform_subtype: &Option<String>, output_file: &str) {
    let subtype = platform_subtype.clone().unwrap_or("flatpak".into());
    match subtype.as_str() {
        "flatpak" => {
            deploy_flatpak(setup_data, output_file);
        }
        _ => {
            error!("Unsupported platform subtype: {}", subtype);
            error!("Supported platform subtypes: flatpak");
            exit(1);
        }
    }
}
