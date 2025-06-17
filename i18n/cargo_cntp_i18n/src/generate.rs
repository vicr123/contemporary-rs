use std::{
    collections::HashMap,
    ffi::OsStr,
    fs::{self, OpenOptions},
    process::exit,
};

use cargo_metadata::camino::Utf8PathBuf;
use contemporary_i18n_core::config::get_i18n_config;
use contemporary_i18n_parse::tr::TrMacroInput;
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

struct TrMacroVisitor {
    pub strings: HashMap<String, String>,
}

impl<'ast> Visit<'ast> for TrMacroVisitor {
    fn visit_macro(&mut self, mac: &'ast Macro) {
        if mac.path.is_ident("tr") {
            if let Ok(contents) = syn::parse2::<TrMacroInput>(mac.tokens.clone()) {
                if let Some(default_string) = contents.default_string {
                    self.strings
                        .insert(contents.translation_id.value(), default_string.value());
                }
            }
        } else {
            trace!("non-tr macro, attempting to enter");

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
            // // Heuristically try to parse macro tokens as common Rust constructs
            // // Try block (e.g., macro_rules! foo { { ... } })
            // if let Ok(block) = syn::parse2::<syn::Block>(mac.tokens.clone()) {
            //     self.visit_block(&block);
            // }
            // // Try array (e.g., vec![...])
            // else if let Ok(expr_array) = syn::parse2::<syn::ExprArray>(mac.tokens.clone()) {
            //     trace!(
            //         "found expr_array, trying to enter {}",
            //         mac.path.segments.last().unwrap().ident
            //     );
            //     for expr in expr_array.elems.iter() {
            //         self.visit_expr(expr);
            //     }
            // }
            // // Try tuple (e.g., macro!(a, b, c))
            // else if let Ok(expr_tuple) = syn::parse2::<syn::ExprTuple>(mac.tokens.clone()) {
            //     trace!(
            //         "found expr_tuple, trying to enter {}",
            //         mac.path.segments.last().unwrap().ident
            //     );
            //     for expr in expr_tuple.elems.iter() {
            //         self.visit_expr(expr);
            //     }
            // }
            // // Try parsing as a group of statements (e.g., macro! { ... })
            // else if let Ok(file) = syn::parse2::<syn::File>(mac.tokens.clone()) {
            //     self.visit_file(&file);
            // }
            // You can add more heuristics here if needed
        }

        syn::visit::visit_macro(self, mac);
    }
}

pub fn generate(manifest_directory: Utf8PathBuf) {
    let config = get_i18n_config(manifest_directory.as_std_path());

    let mut visitor = TrMacroVisitor {
        strings: HashMap::new(),
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
            catalog[key] = json!(value.as_str());
            catalog
        });

    let catalog_path = config
        .i18n
        .translation_catalog_file(manifest_directory.as_std_path());

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
}
