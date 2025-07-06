use crate::cldr::delimiters::Delimiters;

mod delimiters;

pub struct CldrData {
    pub delimiters: Delimiters,
}

impl CldrData {
    pub fn new(language: &str) -> Self {
        Self {
            delimiters: Delimiters::new(language),
        }
    }
}
