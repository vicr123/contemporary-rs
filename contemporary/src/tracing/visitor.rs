use std::collections::HashMap;
use std::fmt::Debug;
use tracing::field::{Field, Visit};

pub struct ContemporaryVisitor {
    fields: HashMap<String, String>,
}

impl ContemporaryVisitor {
    pub fn new() -> Self {
        Self {
            fields: HashMap::new(),
        }
    }
}

impl ContemporaryVisitor {
    pub fn message(&self) -> String {
        self.fields.get("message").cloned().unwrap_or_default()
    }
}

impl Visit for ContemporaryVisitor {
    fn record_str(&mut self, field: &Field, value: &str) {
        self.fields
            .insert(field.name().to_string(), value.to_string());
    }

    fn record_debug(&mut self, field: &Field, value: &dyn Debug) {
        self.fields
            .insert(field.name().to_string(), format!("{value:?}"));
    }
}
