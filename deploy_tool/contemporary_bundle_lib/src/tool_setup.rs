use cargo_metadata::{Metadata, MetadataCommand};
use contemporary_config::ContemporaryConfig;
use current_platform::CURRENT_PLATFORM;
use std::env;
use std::path::PathBuf;
use std::process::exit;
use tracing::error;
use crate::VersionTuple;

pub struct ToolSetup {
    pub cargo_metadata: Metadata,
    pub base_path: PathBuf,
    pub contemporary_config: ContemporaryConfig,
    pub deployment_type: DeploymentType,
    pub targets: Vec<String>,
    pub output_directory: PathBuf,
    pub profile: String,
    pub version: VersionTuple
}

pub enum DeploymentType {
    Linux,
    MacOS,
}

pub fn setup_tool(
    profile: Option<String>,
    targets: Vec<String>,
    output_directory_subfolder: &str,
) -> ToolSetup {
    let profile = profile.unwrap_or("release".into());

    let targets = if targets.is_empty() {
        vec![CURRENT_PLATFORM.to_string()]
    } else {
        targets
            .iter()
            .flat_map(|s| s.split(';'))
            .map(String::from)
            .collect()
    };

    let current_dir = env::current_dir().unwrap();

    let cargo_toml_path = current_dir.join("Cargo.toml");
    if !cargo_toml_path.exists() {
        error!("Unable to find Cargo.toml in current directory.");
        exit(1)
    };

    let Ok(cargo_metadata) = MetadataCommand::new().manifest_path("./Cargo.toml").exec() else {
        error!("Unable to read Cargo.toml in current directory.");
        exit(1);
    };

    let Some(root_package) = cargo_metadata.root_package() else {
        error!("The current project is not a binary project.");
        error!("Please rerun this command in the root of a binary project.");
        exit(1);
    };

    let Some(bin_target) = root_package.targets.iter().find(|target| target.is_bin()) else {
        error!("The current project is not a binary project.");
        error!("Please rerun this command in the root of a binary project.");
        exit(1);
    };

    let Some(config) = ContemporaryConfig::new_from_path(current_dir.join("Contemporary.toml"))
    else {
        error!("Unable to find Contemporary.toml in current directory.");
        exit(1);
    };

    let deployment_type = {
        if targets.iter().all(|target| {
            matches!(
                target.as_str(),
                "x86_64-unknown-linux-gnu"
                    | "aarch64-unknown-linux-gnu"
                    | "x86_64-unknown-linux-musl"
                    | "aarch64-unknown-linux-musl"
            )
        }) && targets.len() == 1
        {
            DeploymentType::Linux
        } else if targets.iter().all(|target| {
            matches!(
                target.as_str(),
                "aarch64-apple-darwin" | "x86_64-apple-darwin"
            )
        }) {
            DeploymentType::MacOS
        } else {
            error!("Unsupported target configuration: {}", targets.join(";"));
            exit(1);
        }
    };

    let output_directory = cargo_metadata
        .target_directory
        .join(output_directory_subfolder)
        .join(targets.join("-"))
        .join(&profile)
        .into();

    let version = &cargo_metadata.root_package().unwrap().version;
    let version = (version.major, version.minor, version.patch);

    ToolSetup {
        cargo_metadata,
        base_path: current_dir,
        contemporary_config: config,
        deployment_type,
        targets,
        output_directory,
        profile,
        version
    }
}
