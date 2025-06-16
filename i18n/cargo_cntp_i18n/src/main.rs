mod generate;

use cargo_metadata::camino::Utf8PathBuf;
use clap::{CommandFactory, Parser, Subcommand};
use clap_cargo::style::CLAP_STYLING;
use generate::generate;

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
    println!("{:?}", args);

    let path = get_manifest_path(&args);

    if let Err(reason) = path {
        let mut cmd = Command::command();

        cmd.error(
            clap::error::ErrorKind::Io,
            format!(
                "failed to discover cargo manifest path, \
                is the working directory a cargo project? {:?}",
                reason
            ),
        )
        .exit()
    }

    match args.command {
        CntpI18nSubCommand::Generate => generate(path.unwrap()),
    }
}
