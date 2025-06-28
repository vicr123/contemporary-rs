use std::{env, path::PathBuf};

use contemporary_i18n_gen::GenerationResult;

fn main() {
    let path: PathBuf = env::var("CARGO_MANIFEST_DIR")
        .expect("CARGO_MANIFEST_DIR is not set")
        .into();

    if let GenerationResult::ErrorsEncountered(count) = contemporary_i18n_gen::generate(&path) {
        println!(
            "cargo::warning={count} errors generated while building translation file, \
            run cntp-i18n generate manually to see them",
        );
    };
}
