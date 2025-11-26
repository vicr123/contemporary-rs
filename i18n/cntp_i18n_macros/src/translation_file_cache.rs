use std::{env, path::Path, sync::LazyLock};

use cntp_i18n_core::load::{self, TranslationEntry};
use rustc_hash::{FxHashMap, FxHashSet};

use crate::{
    config::I18N_CONFIG,
    parse_raw_string::{I18nFullStringPart, parse_raw_string_2},
};

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

pub static VARIABLE_LIST: LazyLock<FxHashMap<String, Vec<String>>> = LazyLock::new(|| {
    let sets: FxHashMap<String, FxHashSet<String>> = TRANSLATION_FILE_CACHE.iter().fold(
        FxHashMap::<String, FxHashSet<String>>::default(),
        |mut foldit, (_language, strings)| {
            for (key, entry) in strings.iter() {
                let variables = match entry {
                    TranslationEntry::Entry(string) => parse_raw_string_2(string),
                    TranslationEntry::PluralEntry(hash_map) => hash_map
                        .iter()
                        .flat_map(|(_, string)| parse_raw_string_2(string))
                        .collect::<Vec<_>>(),
                }
                .iter()
                .filter_map(|thing| match thing {
                    I18nFullStringPart::Static(_) => None,
                    I18nFullStringPart::Variable(name) => Some(name.to_string()),
                    I18nFullStringPart::Count => Some("count".to_string()),
                })
                .collect::<FxHashSet<_>>();

                let z = foldit.entry(key.clone()).or_default(); //.union(&variables);

                for variable in variables {
                    z.insert(variable);
                }
            }

            foldit
        },
    );

    sets.into_iter()
        .map(|(key, variables)| (key.clone(), variables.iter().cloned().collect::<Vec<_>>()))
        .collect()
});
