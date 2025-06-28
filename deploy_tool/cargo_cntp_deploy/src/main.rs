use cargo_metadata::MetadataCommand;
use clap::Parser;
use clap_verbosity_flag::InfoLevel;
use contemporary_config::ContemporaryConfig;
use contemporary_deploy_lib::linux::deploy_linux;
use contemporary_deploy_lib::macos::deploy_macos;
use current_platform::CURRENT_PLATFORM;
use std::env;
use std::env::consts::EXE_EXTENSION;
use std::process::exit;
use tracing::{error, info};

#[derive(Parser, Debug)]
#[command(name = "cargo cntp-deploy")] // all of this is necessary so things work as expected wrt. cargo
#[command(bin_name = "cargo")]
#[command(version, about, long_about = None)]
struct Args {
    /// The profile to build
    #[arg(short, long, default_value_t = String::from("release"))]
    profile: String,

    /// The architecture to build for
    #[arg(short, long, default_value_t = String::from(CURRENT_PLATFORM))]
    arch: String,

    /// How loud should we be?
    #[clap(flatten)]
    verbosity: clap_verbosity_flag::Verbosity<InfoLevel>,

    /// Don't open the output directory after deployment is complete
    #[clap(long, default_value_t = false)]
    no_open: bool,
}

fn main() {
    let args = Args::parse();

    tracing_subscriber::fmt()
        .with_target(false)
        .without_time()
        .with_max_level(args.verbosity)
        .init();

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

    let output_directory = cargo_metadata
        .target_directory
        .join("deploy")
        .join(args.arch.clone())
        .join(args.profile.clone());

    info!("Deploying {}", root_package.name);
    info!("Profile: {}", args.profile);
    info!("Target:  {}", args.arch);
    info!("Output:  {}", output_directory);

    // Find the executable to publish
    let mut target_directory = cargo_metadata.target_directory.clone();
    let triple_directory = target_directory.join(args.arch.clone());
    if triple_directory.exists() {
        target_directory = triple_directory;
    }
    target_directory = target_directory.join(args.profile);
    let bin_target = target_directory
        .join(root_package.name.as_str())
        .with_extension(EXE_EXTENSION);

    // TODO: Run cargo build
    if !bin_target.exists() {
        error!("Unable to find executable at {}", bin_target)
    }

    let version = &root_package.version;
    let version_tuple = (version.major, version.minor, version.patch);

    match args.arch.as_str() {
        "x86_64-unknown-linux-gnu" => deploy_linux(
            args.arch,
            current_dir,
            bin_target.into(),
            output_directory.clone().into(),
            config,
        ),
        "aarch64-unknown-linux-gnu" => deploy_linux(
            args.arch,
            current_dir,
            bin_target.into(),
            output_directory.clone().into(),
            config,
        ),
        "x86_64-unknown-linux-musl" => deploy_linux(
            args.arch,
            current_dir,
            bin_target.into(),
            output_directory.clone().into(),
            config,
        ),
        "aarch64-unknown-linux-musl" => deploy_linux(
            args.arch,
            current_dir,
            bin_target.into(),
            output_directory.clone().into(),
            config,
        ),
        "aarch64-apple-darwin" => deploy_macos(
            args.arch,
            version_tuple,
            current_dir,
            bin_target.into(),
            output_directory.clone().into(),
            config,
        ),
        "x86-64-apple-darwin" => deploy_macos(
            args.arch,
            version_tuple,
            current_dir,
            bin_target.into(),
            output_directory.clone().into(),
            config,
        ),
        _ => {
            error!("Unsupported target triple: {}", args.arch);
            exit(1);
        }
    }

    if !args.no_open {
        let _ = open::that(output_directory);
    }
}
