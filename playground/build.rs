use std::{env, path::PathBuf};

use cntp_i18n_gen::{GenerationError, GenerationResult};

fn main() {
    let path: PathBuf = env::var("CARGO_MANIFEST_DIR")
        .expect("CARGO_MANIFEST_DIR is not set")
        .into();

    if let GenerationResult::ErrorsEncountered(errors) = cntp_i18n_gen::generate(&path) {
        println!(
            "cargo::warning={} errors generated while building translation file, \
            run cntp-i18n generate manually to see them",
            errors.errors.len()
        );
        for error in errors.errors {
            println!(
                "cargo::warning={}",
                match error {
                    GenerationError::String(string) => string,
                    GenerationError::VisitorError(error) => error.error_type.error_string(),
                }
            );
        }
    };
}
