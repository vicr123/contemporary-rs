use crate::linux::appimage::deploy_appimage;
use crate::linux::rootdir::deploy_rootdir;
use crate::tool_setup::ToolSetup;
use std::process::exit;
use tracing::error;

pub fn deploy_linux(setup_data: &ToolSetup, platform_subtype: &Option<String>, output_file: &str) {
    let subtype = platform_subtype.clone().unwrap_or("appimage".into());
    match subtype.as_str() {
        "appimage" => {
            deploy_appimage(setup_data, output_file);
        }
        "rootdir" => {
            deploy_rootdir(setup_data, output_file);
        }
        _ => {
            error!("Unsupported platform subtype: {}", subtype);
            error!("Supported platform subtypes: appimage, rootdir");
            exit(1);
        }
    }
}
