use clap::Parser;
use clap_verbosity_flag::InfoLevel;

use contemporary_bundle_lib::tool_setup::{DeploymentType, setup_tool};
use std::path::Path;
use tracing::info;

#[cfg(target_os = "linux")]
use contemporary_bundle_lib::linux::deploy_linux;

#[cfg(target_os = "macos")]
use contemporary_bundle_lib::macos::deploy::deploy_macos;

#[derive(Parser, Debug)]
#[command(name = "cargo cntp-deploy")] // all of this is necessary so things work as expected wrt. cargo
#[command(bin_name = "cargo")]
#[command(version, about, long_about = None)]
struct Args {
    /// The profile to build
    #[arg(short, long)]
    profile: Option<String>,

    /// The targets to build for
    #[arg(short, long)]
    target: Vec<String>,

    /// How loud should we be?
    #[clap(flatten)]
    verbosity: clap_verbosity_flag::Verbosity<InfoLevel>,

    /// Don't open the output directory after deployment is complete
    #[clap(long, default_value_t = false)]
    no_open: bool,

    #[arg(short, long)]
    output_file: String,
}

fn main() {
    let args = Args::parse();

    tracing_subscriber::fmt()
        .with_target(false)
        .without_time()
        .with_max_level(args.verbosity)
        .init();

    let setup_data = setup_tool(args.profile, args.target, "bundle");

    info!(
        "Deploying {}",
        setup_data.cargo_metadata.root_package().unwrap().name
    );
    info!("Profile: {}", setup_data.profile);
    info!("Target:  {}", setup_data.targets.join(";"));
    info!("Output:  {}", args.output_file);

    match setup_data.deployment_type {
        DeploymentType::Linux => {
            #[cfg(target_os = "linux")]
            deploy_linux(&setup_data, &args.output_file);


            #[cfg(not(target_os = "linux"))]
            panic!("Tried to compile for Linux when not on Linux");
        },
        DeploymentType::MacOS => {
            #[cfg(target_os = "macos")]
            deploy_macos(&setup_data, &args.output_file);

            #[cfg(not(target_os = "macos"))]
            panic!("Tried to compile for macOS when not on macOS");
        },
        DeploymentType::Windows => {
        }
    }

    if !args.no_open {
        let _ = open::that(Path::new(&args.output_file).parent().unwrap());
    }
}
