use crate::cldr::ListPatterns;
use crate::{ListFunction, ListWidth};

pub struct ListBuilder<'patterns, 'strings> {
    patterns: &'patterns ListPatterns,
    parts: Vec<&'strings String>,

    list_function: ListFunction,
    list_width: ListWidth,
}

impl<'patterns, 'strings> ListBuilder<'patterns, 'strings> {
    pub fn new(parts: Vec<&'strings String>, patterns: &'patterns ListPatterns) -> Self {
        ListBuilder {
            patterns,
            parts,
            list_function: Default::default(),
            list_width: Default::default(),
        }
    }

    pub fn with_list_function(mut self, list_function: ListFunction) -> Self {
        self.list_function = list_function;
        self
    }

    pub fn with_list_width(mut self, list_width: ListWidth) -> Self {
        self.list_width = list_width;
        self
    }

    pub fn build(&self) -> String {
        let pattern = match self.list_function {
            ListFunction::Standard => match self.list_width {
                ListWidth::Wide => &self.patterns.standard,
                ListWidth::Short => &self.patterns.standard_short,
                ListWidth::Narrow => &self.patterns.standard_narrow,
            },
            ListFunction::Or => match self.list_width {
                ListWidth::Wide => &self.patterns.or,
                ListWidth::Short => &self.patterns.or_short,
                ListWidth::Narrow => &self.patterns.or_narrow,
            },
            ListFunction::Unit => match self.list_width {
                ListWidth::Wide => &self.patterns.unit,
                ListWidth::Short => &self.patterns.unit_short,
                ListWidth::Narrow => &self.patterns.unit_narrow,
            },
        };

        match self.parts.len() {
            0 => String::new(),
            1 => self.parts[0].clone(),
            2 => pattern
                .two
                .replace("{0}", self.parts[0])
                .replace("{1}", self.parts[1]),
            _ => {
                let len = self.parts.len();

                self.parts
                    .iter()
                    .enumerate()
                    .fold(String::new(), |acc: String, (i, part)| {
                        if i == 0 {
                            pattern.start.replace("{0}", part)
                        } else if i == len - 2 {
                            acc.replace("{1}", &pattern.end.replace("{0}", part))
                        } else if i == len - 1 {
                            acc.replace("{1}", part)
                        } else {
                            acc.replace("{1}", &pattern.middle.replace("{0}", part))
                        }
                    })
            }
        }
    }
}

impl From<ListBuilder<'_, '_>> for String {
    fn from(builder: ListBuilder) -> Self {
        builder.build()
    }
}
