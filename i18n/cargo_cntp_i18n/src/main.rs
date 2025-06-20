use std::process::exit;

use cargo_metadata::camino::Utf8PathBuf;
use clap::{Parser, Subcommand};
use clap_cargo::style::CLAP_STYLING;
use clap_verbosity_flag::InfoLevel;
use contemporary_i18n_gen::generate;
use tracing::error;

#[derive(Parser, Debug)]
#[command(name = "cargo")] // all of this is necessary so things work as expected wrt. cargo
#[command(bin_name = "cargo")]
#[command(styles = CLAP_STYLING)]
enum Command {
    CntpI18n(CntpI18nArgs),
}

#[derive(Parser, Debug)]
struct CntpI18nArgs {
    #[clap(flatten)]
    manifest: clap_cargo::Manifest,

    #[clap(flatten)]
    verbosity: clap_verbosity_flag::Verbosity<InfoLevel>,

    #[clap(subcommand)]
    command: CntpI18nSubCommand,
}

#[derive(Subcommand, Debug)]
enum CntpI18nSubCommand {
    Generate,
}

fn get_manifest_path(args: &CntpI18nArgs) -> anyhow::Result<Utf8PathBuf> {
    Ok(args
        .manifest
        .metadata()
        .exec()?
        .root_package()
        .ok_or(anyhow::anyhow!("no root package"))?
        .manifest_path
        .parent()
        .ok_or(anyhow::anyhow!("couldn't find parent of Cargo.toml"))?
        .to_path_buf())
}

fn main() {
    let Command::CntpI18n(args) = Command::parse();

    tracing_subscriber::fmt()
        .with_target(false)
        .without_time()
        .with_max_level(args.verbosity)
        .init();

    let path = get_manifest_path(&args);

    if let Err(reason) = path {
        error!(
            "failed to discover cargo manifest path: {:?}, \
            hint: is the working directory a cargo project?",
            reason
        );

        exit(1);
    }

    match args.command {
        CntpI18nSubCommand::Generate => {
            generate(path.unwrap().as_std_path());
        }
    }
}
