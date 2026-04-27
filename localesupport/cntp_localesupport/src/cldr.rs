use crate::cldr::delimiters::Delimiters;
use crate::cldr::list_patterns::ListPatterns;

mod delimiters;
mod list_patterns;

pub struct CldrData {
    pub delimiters: Delimiters,
    pub list_patterns: ListPatterns,
}

impl CldrData {
    pub fn new(language: &str) -> Self {
        Self {
            delimiters: Delimiters::new(language),
            list_patterns: ListPatterns::new(language),
        }
    }
}
