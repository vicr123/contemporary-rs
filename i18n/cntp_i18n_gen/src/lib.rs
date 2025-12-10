#[cfg(test)]
mod tests;

use std::{
    cell::RefCell,
    collections::HashMap,
    ffi::OsStr,
    fs::{self, OpenOptions},
    io::Write,
    path::{Path, PathBuf},
    process::exit,
    rc::Rc,
};

use cntp_i18n_build_core::config::get_i18n_config;
use cntp_i18n_parse::{tr::TrMacroInput, trn::TrnMacroInput};
use icu::{
    locale::Locale,
    plurals::{PluralCategory, PluralRules},
};
use itertools::Itertools;
use proc_macro2::Span;
use serde_json::json;
use syn::{Expr, Lit, Macro, Token, parse_file, spanned::Spanned, visit::Visit};
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

struct TrInfo {
    string: TrString,
    file: PathBuf,
    plural: bool,
    description: Option<String>,
    line_no: usize,
}

struct ExpectedString {
    id: String,
    file: PathBuf,
    span: Span,
}

struct TrMacroVisitor {
    pub strings: HashMap<String, TrInfo>,
    pub expected_strings: Vec<ExpectedString>,
    pub plural_rules: PluralRules,
    pub current_path: Rc<RefCell<PathBuf>>,
    pub errors: Vec<VisitorError>,
}

impl TrMacroVisitor {
    pub fn new(plural_rules: PluralRules, path: Rc<RefCell<PathBuf>>) -> Self {
        Self {
            strings: HashMap::new(),
            expected_strings: Default::default(),
            plural_rules,
            current_path: path,
            errors: Default::default(),
        }
    }

    pub fn finish(&mut self) {
        // Check that all expected strings have been found
        for expected in self.expected_strings.iter() {
            if !self.strings.contains_key(expected.id.as_str()) {
                self.errors.push(VisitorError {
                    span: expected.span,
                    file: expected.file.clone(),
                    error_type: VisitorErrorType::MissingDefinition {
                        id: expected.id.clone(),
                    },
                });
            }
        }
    }

    pub fn print_errors(&self, root_directory: &Path) {
        for error in self.errors.iter() {
            error.print_error(root_directory);
        }
    }
}

impl VisitorError {
    pub fn print_error(&self, root_directory: &Path) {
        error!("{}", self.error_string(root_directory))
    }

    pub fn error_string(&self, root_directory: &Path) -> String {
        match &self.error_type {
            VisitorErrorType::BadPluralArgumentCount {
                id,
                expected_count,
                actual_count,
            } => format!(
                "expected category count {expected_count} but received actual string count {actual_count} for {id}",
            ),
            VisitorErrorType::DuplicateDefinition {
                id,
                last_seen_file,
                last_seen_line,
            } => {
                format!(
                    "Duplicate definition for {id}. Last seen in {}:{last_seen_line}",
                    last_seen_file
                        .strip_prefix(root_directory)
                        .unwrap()
                        .to_str()
                        .unwrap(),
                )
            }
            VisitorErrorType::MissingDefinition { id } => format!(
                "Missing definition for {id}. Referenced at {}:{}",
                self.file
                    .strip_prefix(root_directory)
                    .unwrap()
                    .to_str()
                    .unwrap(),
                self.span.start().line,
            ),
        }
    }
}

#[derive(Debug, Clone)]
pub enum VisitorErrorType {
    BadPluralArgumentCount {
        id: String,
        expected_count: usize,
        actual_count: usize,
    },
    DuplicateDefinition {
        id: String,
        last_seen_file: PathBuf,
        last_seen_line: usize,
    },
    MissingDefinition {
        id: String,
    },
}

#[derive(Debug, Clone)]
pub struct VisitorError {
    pub span: Span,
    pub file: PathBuf,
    pub error_type: VisitorErrorType,
}

impl<'ast> Visit<'ast> for TrMacroVisitor {
    fn visit_macro(&mut self, mac: &'ast Macro) {
        match mac.path.segments.last().unwrap().ident.to_string().as_str() {
            "tr" | "tr_noop" => {
                if let Ok(contents) = syn::parse2::<TrMacroInput>(mac.tokens.clone()) {
                    if let Some(default_string) = contents.default_string {
                        let replaced = self.strings.insert(
                            contents.translation_id.value(),
                            TrInfo {
                                string: TrString::Single(default_string.value()),
                                file: self.current_path.borrow().clone(),
                                plural: false,
                                description: contents
                                    .context
                                    .iter()
                                    .find(|v| v.name == "description")
                                    .and_then(|v| match &v.value {
                                        Expr::Lit(lit) => match &lit.lit {
                                            Lit::Str(str) => Some(str.value()),
                                            _ => None,
                                        },
                                        _ => None,
                                    }),
                                line_no: mac.tokens.span().start().line,
                            },
                        );

                        if let Some(replaced) = replaced {
                            self.errors.push(VisitorError {
                                span: mac.tokens.span(),
                                file: self.current_path.borrow().clone(),
                                error_type: VisitorErrorType::DuplicateDefinition {
                                    id: contents.translation_id.value(),
                                    last_seen_file: replaced.file,
                                    last_seen_line: replaced.line_no,
                                },
                            });
                        }
                    } else {
                        self.expected_strings.push(ExpectedString {
                            id: contents.translation_id.value(),
                            file: self.current_path.borrow().clone(),
                            span: mac.tokens.span(),
                        })
                    }

                    for variable in contents.variables {
                        self.visit_expr(&variable.value);
                    }
                }
            }
            "trn" | "trn_noop" => {
                if let Ok(contents) = syn::parse2::<TrnMacroInput>(mac.tokens.clone()) {
                    let category_count = self.plural_rules.categories().count();
                    let string_count = contents.default_strings.len();
                    let id = contents.translation_id.value();

                    // If we're reusing an existing translation, don't process this instance
                    if string_count == 0 {
                        self.expected_strings.push(ExpectedString {
                            id,
                            file: self.current_path.borrow().clone(),
                            span: mac.tokens.span(),
                        })
                    } else if category_count != string_count {
                        self.errors.push(VisitorError {
                            span: mac.tokens.span(),
                            file: self.current_path.borrow().clone(),
                            error_type: VisitorErrorType::BadPluralArgumentCount {
                                id: id.clone(),
                                expected_count: category_count,
                                actual_count: string_count,
                            },
                        });
                    } else {
                        let forms = self
                            .plural_rules
                            .categories()
                            .zip(contents.default_strings.iter())
                            .map(|(category, lit_str)| (category, lit_str.value()))
                            .collect();

                        let replaced = self.strings.insert(
                            id,
                            TrInfo {
                                string: TrString::Plural(forms),
                                file: self.current_path.borrow().clone(),
                                plural: true,
                                description: contents
                                    .context
                                    .iter()
                                    .find(|v| v.name == "description")
                                    .and_then(|v| match &v.value {
                                        Expr::Lit(lit) => match &lit.lit {
                                            Lit::Str(str) => Some(str.value()),
                                            _ => None,
                                        },
                                        _ => None,
                                    }),
                                line_no: mac.tokens.span().start().line,
                            },
                        );

                        if let Some(replaced) = replaced {
                            self.errors.push(VisitorError {
                                span: mac.tokens.span(),
                                file: self.current_path.borrow().clone(),
                                error_type: VisitorErrorType::DuplicateDefinition {
                                    id: contents.translation_id.value(),
                                    last_seen_file: replaced.file,
                                    last_seen_line: replaced.line_no,
                                },
                            });
                        }
                    }

                    for variable in contents.variables {
                        self.visit_expr(&variable.value);
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

#[derive(Debug, Clone)]
pub struct GenerationErrorHandler {
    pub errors: Vec<GenerationError>,
}

impl GenerationErrorHandler {
    pub fn push_string(&mut self, string: String) {
        error!("{}", string);
        self.errors.push(GenerationError::String(string));
    }

    pub fn push_visitor_errors(&mut self, errors: Vec<VisitorError>) {
        self.errors
            .extend(errors.into_iter().map(GenerationError::VisitorError));
    }
}

#[derive(Debug, Clone)]
pub enum GenerationResult {
    Successful,
    ErrorsEncountered(GenerationErrorHandler),
}

#[derive(Debug, Clone)]
pub enum GenerationError {
    String(String),
    VisitorError(VisitorError),
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

    let current_file = Rc::new(RefCell::new(PathBuf::new()));

    let mut visitor = TrMacroVisitor::new(plural_rules, current_file.clone());

    let mut errors_encountered = GenerationErrorHandler {
        errors: Default::default(),
    };

    for entry in WalkDir::new(manifest_directory.join("src"))
        .follow_links(true)
        .into_iter()
        .filter_map(|result| result.ok())
        .filter(|inner| inner.path().extension() == Some(OsStr::new("rs")))
    {
        debug!("reading {:?}", entry.path());

        let Ok(contents) = fs::read_to_string(entry.path()) else {
            errors_encountered
                .push_string(format!("failed to read source file {:?}", entry.path()));
            continue;
        };

        *current_file.borrow_mut() = entry.path().to_path_buf();

        let Ok(syntax) = parse_file(&contents) else {
            errors_encountered
                .push_string(format!("failed to parse source file {:?}", entry.path()));
            continue;
        };

        visitor.visit_file(&syntax);
    }

    visitor.finish();
    visitor.print_errors(manifest_directory);
    errors_encountered.push_visitor_errors(visitor.errors);

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
    let catalog = visitor.strings.iter().sorted_by_key(|x| x.0).fold(
        json!({}),
        |mut catalog, (key, value)| {
            match &value.string {
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
        },
    );

    let catalog_path = config.i18n.translation_catalog_file(manifest_directory);

    let Ok(mut file) = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(&catalog_path)
    else {
        error!("failed to open catalog file, exiting");
        exit(1);
    };

    let Ok(serialized) = serde_json::to_string_pretty(&catalog) else {
        error!("failed to serialize catalog file, exiting");
        exit(1);
    };

    let write_result = file.write_all(serialized.replace("\\r\\n", "\\n").as_bytes());

    if let Err(err) = write_result {
        error!("failed to write catalog file: {}, exiting", err);
        exit(1);
    }

    info!(
        "successfully wrote {} key/string pairs to {:#?}",
        visitor.strings.len(),
        catalog_path
    );

    let meta =
        visitor
            .strings
            .iter()
            .sorted_by_key(|x| x.0)
            .fold(json!({}), |mut meta, (key, value)| {
                meta[key] = json!({
                    "context": value.file.file_name().and_then(|v| v.to_str()),
                    "definedIn": value.file
                        .strip_prefix(manifest_directory)
                        .ok()
                        .map(|v| format!(
                            "{}:{}",
                            v.to_path_buf()
                                .iter()
                                .flat_map(OsStr::to_str)
                                .join("/")
                            , value.line_no
                        )),
                    "plural": value.plural,
                    "description": value.description,
                });
                meta
            });

    let meta_path = catalog_path.with_file_name("meta.json");

    let Ok(meta_file) = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(&meta_path)
    else {
        error!("failed to open meta file, exiting");
        exit(1);
    };

    let write_result = serde_json::to_writer_pretty(meta_file, &meta);

    if let Err(error) = write_result {
        error!("failed to write meta file: {:?}, exiting", error);
        exit(1);
    }

    if !errors_encountered.errors.is_empty() {
        error!(
            "{} error(s) encountered, generated translation catalog will be incomplete",
            errors_encountered.errors.len()
        )
    }

    if !errors_encountered.errors.is_empty() {
        GenerationResult::ErrorsEncountered(errors_encountered)
    } else {
        GenerationResult::Successful
    }
}

pub fn generate_default(manifest_directory: &Path) {
    println!("cargo::rerun-if-changed=src");
    if let GenerationResult::ErrorsEncountered(errors) = generate(&manifest_directory) {
        println!(
            "cargo::warning={} error(s) generated while building translation file.",
            errors.errors.len()
        );
        for error in errors.errors {
            println!(
                "cargo::warning={}",
                match error {
                    GenerationError::String(string) => string,
                    GenerationError::VisitorError(error) => error.error_string(manifest_directory),
                }
            );
        }
    };
}
