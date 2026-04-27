use crate::ListFunction;
use crate::cldr::ListPatterns;

pub struct ListBuilder<'patterns, 'strings> {
    patterns: &'patterns ListPatterns,
    parts: Vec<&'strings String>,
    list_function: ListFunction,
}

impl<'patterns, 'strings> ListBuilder<'patterns, 'strings> {
    pub fn new(parts: Vec<&'strings String>, patterns: &'patterns ListPatterns) -> Self {
        ListBuilder {
            patterns,
            parts,
            list_function: Default::default(),
        }
    }

    pub fn with_list_function(mut self, list_function: ListFunction) -> Self {
        self.list_function = list_function;
        self
    }

    pub fn build(&self) -> String {
        let pattern = match self.list_function {
            ListFunction::Standard => &self.patterns.standard,
            ListFunction::Or => &self.patterns.or,
            ListFunction::Unit => &self.patterns.unit,
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
