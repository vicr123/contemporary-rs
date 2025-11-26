use std::{env, path::Path, sync::LazyLock};

use cntp_i18n_core::load::{self, TranslationEntry};
use rustc_hash::FxHashMap;

use crate::config::I18N_CONFIG;

pub static TRANSLATION_FILE_CACHE: LazyLock<
    FxHashMap<String, FxHashMap<String, TranslationEntry>>,
> = LazyLock::new(|| {
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").expect("Failed to get CARGO_MANIFEST_DIR");

    let config = &*I18N_CONFIG;
    let catalog_files = config.i18n.catalog_files(Path::new(&manifest_dir));

    catalog_files
        .iter()
        .map(|file| {
            let language = file.file_stem().unwrap().to_str().unwrap().to_string();
            let decoded_file = load::translation(&file).unwrap();

            (language, decoded_file.into_iter().collect())
        })
        .collect()
});
