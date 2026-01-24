use cntp_i18n_core::{I18nStringPart, string::I18nString};
use quote::quote;

use crate::translation_file_cache::VARIABLE_LIST;

fn match_line_endings(string: &str) -> proc_macro2::TokenStream {
    let windows_string = string.replace("\n", "\r\n");
    let unix_string = string.replace("\r\n", "\n");

    quote! {
        {
            #[cfg(target_os = "windows")]
            {#windows_string}

            #[cfg(not(target_os = "windows"))]
            {#unix_string}
        }
    }
}

enum StateMachine {
    Idle(String),
    OneBrace(String),
    Variable(String),
    VariableClosing(String),
}

#[derive(Clone)]
pub enum I18nFullStringPart {
    Static(I18nString),
    Variable(I18nString),
    Count,
}

impl I18nFullStringPart {
    pub fn calculate_string_part(&self, this_key: &str) -> I18nStringPart {
        match self {
            I18nFullStringPart::Static(i18n_string) => I18nStringPart::Static(i18n_string.clone()),
            I18nFullStringPart::Variable(i18n_string) => {
                let variables = &VARIABLE_LIST[this_key];
                I18nStringPart::Variable(
                    i18n_string.clone(),
                    variables
                        .iter()
                        .position(|string_to_check| **string_to_check == **i18n_string)
                        .expect("Expected variable list to contain all present variables"),
                )
            }
            I18nFullStringPart::Count => {
                let variables = &VARIABLE_LIST[this_key];
                I18nStringPart::Count(
                    variables
                        .iter()
                        .position(|variable| variable == "count")
                        .expect("Expected variable list to contain a count variable"),
                )
            }
        }
    }
}

pub fn parse_raw_string(string: &str) -> Vec<I18nFullStringPart> {
    let mut parts = Vec::new();
    let mut state_machine = StateMachine::Idle(String::default());
    for char in string.chars() {
        match state_machine {
            StateMachine::Idle(string) => {
                if char == '{' {
                    state_machine = StateMachine::OneBrace(string);
                } else {
                    state_machine = StateMachine::Idle(format!("{string}{char}"));
                }
            }
            StateMachine::OneBrace(string) => {
                if char == '{' {
                    parts.push(I18nFullStringPart::Static(string.into()));
                    state_machine = StateMachine::Variable(String::default());
                } else {
                    state_machine = StateMachine::Idle(format!("{string}{{{char}"));
                }
            }
            StateMachine::Variable(string) => {
                if char == '}' {
                    state_machine = StateMachine::VariableClosing(string);
                } else {
                    state_machine = StateMachine::Variable(format!("{string}{char}"));
                }
            }
            StateMachine::VariableClosing(string) => {
                if char == '}' {
                    if string == "count" {
                        parts.push(I18nFullStringPart::Count);
                    } else {
                        parts.push(I18nFullStringPart::Variable(string.into()));
                    }
                    state_machine = StateMachine::Idle(String::default());
                } else {
                    state_machine = StateMachine::Idle(format!("{string}}}{char}"));
                }
            }
        }
    }

    match state_machine {
        StateMachine::Idle(string) => {
            if !string.is_empty() {
                parts.push(I18nFullStringPart::Static(string.into()));
            }
        }
        StateMachine::OneBrace(string) => {
            parts.push(I18nFullStringPart::Static(format!("{string}{{").into()));
        }
        StateMachine::Variable(string) => {
            parts.push(I18nFullStringPart::Static(format!("{{{{{string}").into()));
        }
        StateMachine::VariableClosing(string) => {
            parts.push(I18nFullStringPart::Static(format!("{{{{{string}}}").into()));
        }
    }

    parts
}

pub trait I18nStringPartExtensions {
    fn to_tokens(&self) -> proc_macro2::TokenStream;
}

impl I18nStringPartExtensions for I18nStringPart {
    fn to_tokens(&self) -> proc_macro2::TokenStream {
        match self {
            I18nStringPart::Static(i18n_string) => {
                let normalised = match_line_endings(i18n_string);
                quote! {
                    cntp_i18n::I18nStringPart::Static(cntp_i18n::I18nString::Borrowed(#normalised))
                }
            }
            I18nStringPart::Variable(i18n_string, idx) => {
                let normalised = match_line_endings(i18n_string);
                quote! {
                    cntp_i18n::I18nStringPart::Variable(cntp_i18n::I18nString::Borrowed(#normalised), #idx)
                }
            }
            I18nStringPart::Count(idx) => {
                quote! {
                    cntp_i18n::I18nStringPart::Count(#idx)
                }
            }
        }
    }
}
