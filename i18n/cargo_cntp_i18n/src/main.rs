//! # cargo-cntp-i18n
//!
//! A Cargo subcommand for managing Contemporary i18n translation files.
//!
//! ## Installation
//!
//! ```bash
//! cargo install --git https://github.com/vicr123/contemporary-rs cargo-cntp-i18n
//! ```
//!
//! ## Usage
//!
//! Currently, there is only one subcommand:
//!
//! ```bash
//! cargo cntp-i18n generate
//! ```
//!
//! This command scans your `src` directory for `tr!` and `trn!` macro invocations
//! and generates the translation catalog files in your `translations` directory.
//!
//! ## When to Use
//!
//! You can use this command as an alternative to integrating `cntp_i18n_gen` into
//! your `build.rs`. This is useful when:
//!
//! - You want to manually control when translation files are regenerated
//! - You're working on translations and want to see changes immediately
//! - Your build system doesn't support build scripts well
//!
//! For most projects, using `cntp_i18n_gen::generate_default()` in `build.rs` is
//! recommended as it ensures translations are always up to date before the
//! application is built.
//!
//! ## Options
//!
//! - `--manifest-path <PATH>` - Path to Cargo.toml (defaults to current directory)
//! - `-v`, `--verbose` - Increase verbosity (can be repeated for more detail)
//! - `-q`, `--quiet` - Decrease verbosity
//!
//! ## Configuration
//!
//! The command reads configuration from `i18n.toml` in your project root. The
//! defaults are as follows:
//!
//! ```toml
//! [i18n]
//! default_language = "en"
//! translation_directory = "translations"
//! ```

use std::process::exit;

use cargo_metadata::camino::Utf8PathBuf;
use clap::{Parser, Subcommand};
use clap_cargo::style::CLAP_STYLING;
use clap_verbosity_flag::InfoLevel;
use cntp_i18n_gen::generate;
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
