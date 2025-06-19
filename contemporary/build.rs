use std::{env, path::PathBuf};

fn main() {
    let path: PathBuf = env::var("CARGO_MANIFEST_DIR")
        .expect("CARGO_MANIFEST_DIR is not set")
        .into();

    contemporary_i18n_gen::generate(&path);
}
