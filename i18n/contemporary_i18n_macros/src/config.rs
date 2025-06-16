use std::{env, path::PathBuf};

use contemporary_i18n_core::config::{Config, get_i18n_config};
use once_cell::sync::Lazy;

// Cache the i18n config using Lazy
pub static I18N_CONFIG: Lazy<Config> = Lazy::new(|| {
    let path: PathBuf = env::var("CARGO_MANIFEST_DIR")
        .expect("CARGO_MANIFEST_DIR is not set")
        .into();
    get_i18n_config(&path)
});
