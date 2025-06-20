use std::{
    collections::HashMap,
    ffi::OsStr,
    fs::{self, OpenOptions},
    path::Path,
    process::exit,
};

use contemporary_i18n_core::config::get_i18n_config;
use contemporary_i18n_parse::{tr::TrMacroInput, trn::TrnMacroInput};
use icu::{
    locale::Locale,
    plurals::{PluralCategory, PluralRules},
};
use serde_json::json;
use syn::{Expr, Macro, Token, parse_file, visit::Visit};
use syn::{parse::Parse, punctuated::Punctuated};
use tracing::{debug, error, info, trace};
use walkdir::WalkDir;

struct CommaSeperatedExpr {
    exprs: Vec<Expr>,
}

impl Parse for CommaSeperatedExpr {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let exprs = Punctuated::<Expr, Token![,]>::parse_terminated(input)?
            .into_iter()
            .collect::<Vec<_>>();

        Ok(CommaSeperatedExpr { exprs })
    }
}

enum TrString {
    Single(String),
    Plural(Vec<(PluralCategory, String)>),
}

struct TrMacroVisitor {
    pub strings: HashMap<String, TrString>,
    pub plural_rules: PluralRules,
}

impl<'ast> Visit<'ast> for TrMacroVisitor {
    fn visit_macro(&mut self, mac: &'ast Macro) {
        match mac.path.segments.last().unwrap().ident.to_string().as_str() {
            "tr" => {
                if let Ok(contents) = syn::parse2::<TrMacroInput>(mac.tokens.clone()) {
                    if let Some(default_string) = contents.default_string {
                        self.strings.insert(
                            contents.translation_id.value(),
                            TrString::Single(default_string.value()),
                        );
                    }
                }
            }
            "trn" => {
                if let Ok(contents) = syn::parse2::<TrnMacroInput>(mac.tokens.clone()) {
                    let category_count = self.plural_rules.categories().count();
                    let string_count = contents.default_strings.len();
                    let id = contents.translation_id.value();

                    if category_count != string_count {
                        error!(
                            "expected category count {} but recieved actual string count {} for {}",
                            category_count, string_count, id,
                        )
                    } else {
                        let forms = self
                            .plural_rules
                            .categories()
                            .zip(contents.default_strings.iter())
                            .map(|(category, lit_str)| (category, lit_str.value()))
                            .collect();

                        self.strings.insert(id, TrString::Plural(forms));
                    }
                }
            }
            _ => {
                trace!("non-tr(n) macro, attempting to enter");

                if let Ok(CommaSeperatedExpr { exprs }) =
                    syn::parse2::<CommaSeperatedExpr>(mac.tokens.clone())
                {
                    trace!(
                        "found expr list, trying to enter {}",
                        mac.path.segments.last().unwrap().ident
                    );
                    for expr in exprs.iter() {
                        self.visit_expr(expr);
                    }
                }
            }
        }

        syn::visit::visit_macro(self, mac);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GenerationResult {
    Successful,
    ErrorsEncountered(usize),
}

pub fn generate(manifest_directory: &Path) -> GenerationResult {
    let config = get_i18n_config(manifest_directory);

    let Ok(locale) = Locale::try_from_str(&config.i18n.default_language) else {
        error!(
            "invalid locale {} in configuration, exiting",
            config.i18n.default_language
        );
        exit(1);
    };

    let plural_rules = PluralRules::try_new(locale.into(), Default::default())
        .expect("could not create plural_rules");

    let mut visitor = TrMacroVisitor {
        strings: HashMap::new(),
        plural_rules: plural_rules,
    };

    let mut errors_encountered: usize = 0;

    for entry in WalkDir::new(manifest_directory.join("src"))
        .follow_links(true)
        .into_iter()
        .filter_map(|result| result.ok())
        .filter(|inner| inner.path().extension() == Some(OsStr::new("rs")))
    {
        debug!("reading {:?}", entry.path());

        let Ok(contents) = fs::read_to_string(entry.path()) else {
            error!("failed to read source file {:?}", entry.path());
            errors_encountered += 1;
            continue;
        };

        let Ok(syntax) = parse_file(&contents) else {
            error!("failed to parse source file {:?}", entry.path());
            errors_encountered += 1;
            continue;
        };

        visitor.visit_file(&syntax);
    }

    if errors_encountered > 0 {
        error!(
            "{} errors encountered, generated translation catalog will be incomplete",
            errors_encountered
        )
    }

    info!(
        "scan complete, found {} unique string(s)",
        visitor.strings.len()
    );

    let keys = visitor
        .strings
        .keys()
        .map(|string| &**string)
        .collect::<Vec<_>>()
        .join(", ");

    debug!("located string key(s): {}", keys);

    // TODO: add option to only modify the existing file instead of erasing and regenerating every time
    let catalog = visitor
        .strings
        .iter()
        .fold(json!({}), |mut catalog, (key, value)| {
            match value {
                TrString::Single(string) => {
                    catalog[key] = json!(string.as_str());
                }
                TrString::Plural(strings) => {
                    catalog[key] = strings
                        .iter()
                        .fold(json!({}), |mut key, (category, string)| {
                            let category_id = match category {
                                PluralCategory::Zero => "zero",
                                PluralCategory::One => "one",
                                PluralCategory::Two => "two",
                                PluralCategory::Few => "few",
                                PluralCategory::Many => "many",
                                PluralCategory::Other => "other",
                            };

                            key[category_id] = json!(string.as_str());
                            key
                        })
                }
            }
            catalog
        });

    let catalog_path = config.i18n.translation_catalog_file(manifest_directory);

    let Ok(file) = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(&catalog_path)
    else {
        error!("failed to open catalog file, exiting");
        exit(1);
    };

    let write_result = serde_json::to_writer_pretty(file, &catalog);

    if let Err(error) = write_result {
        error!("failed to write catalog file: {:?}, exiting", error);
        exit(1);
    }

    info!(
        "successfully wrote {} key/string pairs to {:#?}",
        visitor.strings.len(),
        catalog_path
    );

    if errors_encountered > 0 {
        GenerationResult::ErrorsEncountered(errors_encountered)
    } else {
        GenerationResult::Successful
    }
}
