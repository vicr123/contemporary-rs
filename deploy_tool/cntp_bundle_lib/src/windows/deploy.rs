use crate::tool_setup::ToolSetup;
use std::env::consts::EXE_EXTENSION;
use std::fs::copy;
use std::process::exit;
use tracing::error;

pub fn deploy_windows(setup_data: &ToolSetup, output_file: &str) {
    let deployment = setup_data
        .contemporary_config
        .deployment(setup_data.targets.first().unwrap());

    let Some(application_name) = deployment.application_name() else {
        error!("No application name specified in config");
        exit(1);
    };

    let bundled_executable = setup_data
        .output_directory
        .join(application_name.default_value())
        .with_extension(EXE_EXTENSION);
    if !bundled_executable.exists() {
        error!("Bundled application does not exist. Please bundle first.");
        exit(1);
    };

    if copy(bundled_executable, &output_file).is_err() {
        error!("Failed to copy executable");
        exit(1);
    };
}
