use clap::Parser;
use clap_verbosity_flag::InfoLevel;
use tracing::info;
use contemporary_bundle_lib::tool_setup::setup_tool;

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
}

fn main() {
    let args = Args::parse();

    let setup_data = setup_tool(args.profile, args.target, "deploy");
    
    info!(
        "Deploying {}",
        setup_data.cargo_metadata.root_package().unwrap().name
    );
    info!("Profile: {}", setup_data.profile);
    info!("Target:  {}", setup_data.targets.join(";"));
    info!("Output:  {}", setup_data.output_directory.display());
}
