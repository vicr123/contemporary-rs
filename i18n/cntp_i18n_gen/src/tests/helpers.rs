use crate::TrMacroVisitor;
use icu::locale::Locale;
use icu::plurals::PluralRules;
use std::cell::RefCell;
use std::path::PathBuf;
use std::rc::Rc;
use syn::parse_file;
use syn::visit::Visit;

pub fn visit_string(string: &str, locale: &str) -> TrMacroVisitor {
    let Ok(syntax) = parse_file(string) else {
        panic!("failed to parse source file");
    };

    let mut visitor = TrMacroVisitor::new(
        PluralRules::try_new(
            Locale::try_from_str(locale).expect("Invalid Locale").into(),
            Default::default(),
        )
        .unwrap(),
        Rc::new(RefCell::new(PathBuf::default())),
    );

    visitor.visit_file(&syntax);
    visitor.finish();

    visitor
}
