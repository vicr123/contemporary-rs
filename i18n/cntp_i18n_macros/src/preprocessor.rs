use std::{
    collections::{HashMap, HashSet},
    path::PathBuf,
};

use cntp_i18n_core::load::{TranslationEntry, translation};
use proc_macro2::TokenTree;
use regex::Regex;
use rustc_hash::{FxHashMap, FxHashSet};

enum PartedEntry {
    PartedString(Vec<StringParts>, FxHashSet<String>),
    PluralPartedString(HashMap<String, (Vec<StringParts>, FxHashSet<String>)>),
}

enum StringParts {
    String(String),
    Variable(String),
}

fn break_string(input_string: &str) -> (Vec<StringParts>, FxHashSet<String>) {
    let bracket_regex = Regex::new(r"\{\{([^}]+)\}\}").unwrap();
    let mut variables = FxHashSet::default();
    let mut parts = Vec::new();
    let mut last_end = 0;

    for capture in bracket_regex.captures_iter(input_string) {
        let matched = capture.get(0).unwrap();
        let variable_name = capture.get(1).unwrap();

        if matched.start() > last_end {
            let pre_text = &input_string[last_end..matched.start()];
            if !pre_text.is_empty() {
                parts.push(StringParts::String(pre_text.to_string()));
            }
        }

        parts.push(StringParts::Variable(variable_name.as_str().to_string()));
        variables.insert(variable_name.as_str().to_string());
        last_end = matched.end();
    }

    let remaining_text = &input_string[last_end..];
    if !remaining_text.is_empty() {
        parts.push(StringParts::String(remaining_text.to_string()));
    }

    (parts, variables)
}

/// Builds a map of maps of translation strings, accessed by Key -> Language -> String. This is the
/// reverse of the actual resulting map (accessed by Language -> Key -> String) because in order to
/// break the string into parts the final order & count of variables must be known which involves
/// collating all the data from every language's translations.
fn build_map(files: Vec<PathBuf>) -> FxHashMap<String, FxHashMap<String, PartedEntry>> {
    let mut map = FxHashMap::default();

    for file in files {
        let decoded = translation(&file).unwrap();
        let language = file.file_stem().unwrap().to_str().unwrap();

        for (key, value) in decoded {
            let inner_map = match map.get_mut(&key) {
                Some(inner_map) => inner_map,
                _ => {
                    map.insert(key.clone(), FxHashMap::default());
                    map.get_mut(&key).unwrap()
                }
            };

            match value {
                TranslationEntry::Entry(v) => {
                    let (parts, variables) = break_string(&v);
                    inner_map.insert(
                        language.to_string(),
                        PartedEntry::PartedString(parts, variables),
                    );
                }
                TranslationEntry::PluralEntry(hash_map) => {
                    let mut plural_map = HashMap::new();
                    for (plural_key, plural_value) in hash_map {
                        let out = break_string(&plural_value);
                        plural_map.insert(plural_key.clone(), out);
                    }
                    inner_map.insert(
                        language.to_string(),
                        PartedEntry::PluralPartedString(plural_map),
                    );
                }
            }
        }
    }

    map
}

/// Collects variables on a per-key basis into a single hashmap, then moves each value into a Vec
/// and sorts them alphabetically. This is done so that all strings have a consistent index for
/// variable lookup.
fn collect_variables(
    map: &mut FxHashMap<String, FxHashMap<String, PartedEntry>>,
) -> FxHashMap<String, Vec<String>> {
    let mut set_map: FxHashMap<String, FxHashSet<String>> = FxHashMap::default();

    for (key, entry_map) in map {
        let variable_set = match set_map.get_mut(key) {
            Some(set) => set,
            _ => {
                set_map.insert(key.clone(), FxHashSet::default());
                set_map.get_mut(key).unwrap()
            }
        };

        for (_language, var_set) in entry_map {
            match var_set {
                PartedEntry::PartedString(_, variables) => {
                    for variable in &*variables {
                        variable_set.insert(variable.clone());
                    }
                }
                PartedEntry::PluralPartedString(plural_map) => {
                    for (_form, variables) in plural_map {
                        for variable in &variables.1 {
                            variable_set.insert(variable.clone());
                        }
                    }
                }
            }
        }
    }

    set_map
        .into_iter()
        .map(|(key, set)| {
            let mut vec = set.into_iter().collect::<Vec<_>>();
            vec.sort_unstable();
            (key, vec)
        })
        .collect()
}

fn build_final_parts(
    parts_map: &mut FxHashMap<String, FxHashMap<String, PartedEntry>>,
    variable_map: FxHashMap<String, Vec<String>>,
) -> &mut FxHashMap<String, FxHashMap<String, FxHashMap<String, TokenTree>>> {
    todo!()
}
