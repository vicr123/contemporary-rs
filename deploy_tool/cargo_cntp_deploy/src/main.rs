use clap::Parser;
use clap_cargo::style::CLAP_STYLING;
use clap_verbosity_flag::InfoLevel;

use cntp_bundle_lib::tool_setup::{DeploymentType, setup_tool};
use std::path::Path;
use std::process::exit;
use tracing::{Level, error, info};

#[cfg(target_os = "linux")]
use cntp_bundle_lib::linux::deploy_linux;

#[cfg(target_os = "macos")]
use cntp_bundle_lib::macos::deploy::deploy_macos;

#[cfg(target_os = "windows")]
use cntp_bundle_lib::windows::deploy::deploy_windows;

#[derive(Parser, Debug)]
#[command(name = "cargo cntp-deploy")] // all of this is necessary so things work as expected wrt. cargo
#[command(bin_name = "cargo")]
#[command(styles = CLAP_STYLING)]
enum Command {
    CntpDeploy(Args),
}

#[derive(Parser, Debug)]
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
    let Command::CntpDeploy(args) = Command::parse();

    tracing_subscriber::fmt()
        .with_target(false)
        .without_time()
        .with_max_level(args.verbosity.tracing_level().or_else(|| {
            if std::env::var("RUNNER_DEBUG").is_ok_and(|runner_debug| runner_debug == "1") {
                Some(Level::DEBUG)
            } else {
                None
            }
        }))
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
            {
                error!("Unable to deploy for Linux on non-Linux platform");
                exit(1);
            }
        }
        DeploymentType::MacOS => {
            #[cfg(target_os = "macos")]
            deploy_macos(&setup_data, &args.output_file);

            #[cfg(not(target_os = "macos"))]
            {
                error!("Unable to deploy for macOS on non-Macintosh platform");
                exit(1);
            }
        }
        DeploymentType::Windows => {
            #[cfg(target_os = "windows")]
            deploy_windows(&setup_data, &args.output_file);

            #[cfg(not(target_os = "windows"))]
            {
                error!("Unable to deploy for Windows on non-Windows platform");
                exit(1);
            }
        }
    }

    if !args.no_open {
        let _ = open::that(Path::new(&args.output_file).parent().unwrap());
    }
}
