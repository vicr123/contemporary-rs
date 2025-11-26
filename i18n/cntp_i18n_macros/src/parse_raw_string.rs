use cntp_i18n_core::string::I18nString;
use quote::quote;

fn match_line_endings(string: &str) -> proc_macro2::TokenStream {
    let windows_string = string.replace("\n", "\r\n");
    let unix_string = string.replace("\r\n", "\n");

    return quote! {
        {
            #[cfg(target_os = "windows")]
            {#windows_string}

            #[cfg(not(target_os = "windows"))]
            {#unix_string}
        }
    };

    quote! { #string }
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


pub fn parse_raw_string(string: &str) -> proc_macro2::TokenStream {
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
                    let normalised = match_line_endings(string.as_str());
                    parts.push(quote! {
                        cntp_i18n::I18nStringPart::Static(cntp_i18n::I18nString::Borrowed(#normalised))
                    });
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
                        parts.push(quote! {
                            cntp_i18n::I18nStringPart::Count
                        });
                    } else {
                        let normalised = match_line_endings(string.as_str());
                        parts.push(quote! {
                        cntp_i18n::I18nStringPart::Variable(cntp_i18n::I18nString::Borrowed(#normalised))
                        });
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
                let normalised = match_line_endings(string.as_str());
                parts.push(quote! {
                    cntp_i18n::I18nStringPart::Static(cntp_i18n::I18nString::Borrowed(#normalised))
                });
            }
        }
        StateMachine::OneBrace(string) => {
            let normalised = match_line_endings(format!("{string}{{").as_str());
            parts.push(quote! {
                cntp_i18n::I18nStringPart::Static(cntp_i18n::I18nString::Borrowed(#normalised))
            });
        }
        StateMachine::Variable(string) => {
            let normalised = match_line_endings(format!("{{{{{string}").as_str());
            parts.push(quote! {
                cntp_i18n::I18nStringPart::Static(cntp_i18n::I18nString::Borrowed(#normalised))
            });
        }
        StateMachine::VariableClosing(string) => {
            let normalised = match_line_endings(format!("{{{{{string}}}").as_str());
            parts.push(quote! {
                cntp_i18n::I18nStringPart::Static(cntp_i18n::I18nString::Borrowed(#normalised))
            });
        }
    }

    quote! {
        {
            &[#( #parts, )*]
        }
    }
}

pub fn parse_raw_string_2(string: &str) -> Vec<I18nFullStringPart> {
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
