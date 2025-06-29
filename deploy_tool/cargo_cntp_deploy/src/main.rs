use clap::Parser;
use clap_verbosity_flag::InfoLevel;
use contemporary_bundle_lib::linux::{bundle_linux, deploy_linux};
use contemporary_bundle_lib::macos::deploy::deploy_macos;
use contemporary_bundle_lib::tool_setup::{setup_tool, DeploymentType};
use std::path::Path;
use tracing::info;

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

    let setup_data = setup_tool(args.profile, args.target, "deploy");

    info!(
        "Deploying {}",
        setup_data.cargo_metadata.root_package().unwrap().name
    );
    info!("Profile: {}", setup_data.profile);
    info!("Target:  {}", setup_data.targets.join(";"));
    info!("Output:  {}", args.output_file);

    match setup_data.deployment_type {
        DeploymentType::Linux => deploy_linux(&setup_data, &args.output_file),
        DeploymentType::MacOS => deploy_macos(&setup_data, &args.output_file),
    }

    if !args.no_open {
        let _ = open::that(Path::new(&args.output_file).parent().unwrap());
    }
}
