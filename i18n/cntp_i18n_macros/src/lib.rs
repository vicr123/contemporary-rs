//! # `cntp_i18n_macros`
//!
//! This crate provides the procedural macros for the `cntp_i18n` system.
//! These macros are re-exported by `cntp_i18n`, so you typically don't need to
//! depend on this crate directly.
//!
//! ## Macros
//!
//! - [`tr!`] - Translate a simple string
//! - [`trn!`] - Translate a plural string
//! - [`tr_load!`] - Load translations into an [`I18nSource`](cntp_i18n_core::I18nSource)
//! - [`tr_noop!`] / [`trn_noop!`] - Mark strings for extraction without runtime lookup
//!
//! ## Usage
//!
//! ```rust,ignore
//! use cntp_i18n::{tr, trn, tr_load, I18N_MANAGER};
//!
//! fn main() {
//!     // Load translations at startup
//!     I18N_MANAGER.write().unwrap().load_source(tr_load!());
//!
//!     // Simple translation
//!     let greeting = tr!("HELLO", "Hello!");
//!
//!     // Translation with variables
//!     let welcome = tr!("WELCOME", "Welcome, {{name}}!", name = user_name);
//!
//!     // Plural translation
//!     let items = trn!(
//!         "ITEMS",
//!         "{{count}} item",
//!         "{{count}} items",
//!         count = item_count
//!     );
//! }
//! ```

#![warn(missing_docs)]

use proc_macro::TokenStream;

mod config;
mod parse_raw_string;
mod tr;
mod tr_load;
mod translation_file_cache;

/// Returns a translated string for the given key.
///
/// This macro looks up a translation by key and returns the localized string.
/// If the translation is not found, the default text (second argument) is returned.
///
/// # Syntax
///
/// ```rust,ignore
/// tr!("KEY", "Default text");
/// tr!("KEY", "Hello, {{name}}!", name = value);
/// tr!("KEY", "Value is {{val}}.", val:modifier = value);
/// tr!("KEY", "Value is {{val}}.", val:modifier(arg) = value);
/// ```
///
/// # Arguments
///
/// - `key` - A unique string identifier for this translation (e.g., `"BUTTON_SIGN_IN"`)
/// - `default` - The default text in your source language, used when no translation exists
/// - `var = value` - Variable substitutions (replace `{{var}}` in the string)
/// - `var:modifier = value` - Apply a modifier to the variable before substitution
///
/// ## Meta variables
///
/// Meta variables are special variables used for purposes other than variable substitution.
/// They are prefixed with an octothorpe (`#`) and can be used to control the behavior of
/// the translation.
///
/// - `#locale` - A reference to a string containing a locale identifier.
///               Overrides the default locale for this call only.
/// - `#description` - A reference to a string containing a description of the translation.
///                    Used by `cntp_i18n_gen` to generate `meta.json`.
///
/// # Variable substitution
///
/// Variables are specified using `{{variable_name}}` syntax in the string:
///
/// ```rust,ignore
/// let name = "Alice";
/// let city = "Paris";
/// tr!("INTRO", "My name is {{name}} and I live in {{city}}.", name = name, city = city);
/// // Output: "My name is Alice and I live in Paris."
/// ```
///
/// # Modifiers
///
/// Modifiers transform variables before insertion. Built-in modifiers include:
///
/// - `quote` - Wrap in locale-appropriate quotation marks
/// - `date` - Format as a date/time
///
/// ```rust,ignore
/// // Quote a string
/// tr!("SAID", "They said {{phrase}}.", phrase:quote = some_text);
/// // English output: They said "hello".
/// // French output: They said « hello ».
///
/// // Format a date
/// tr!("EVENT", "Event on {{date}}.", date:date("YMD") = timestamp);
/// // Output varies by locale: "Event on Jan 15, 2024." or "Event on 15/01/2024."
/// ```
///
/// Modifiers can be chained together by separating them with a colon, with the first modifier
/// being applied first. For example:
///
/// ```rust,ignore
/// tr!("SAID", "Event on {{date}}.", phrase:date("YMD"):quote = some_text);
/// // English output: Event on "Jan 15, 2024."
/// // French output: Event on « 15/01/2024 ».
/// ```
///
/// The built-in modifiers are special cased and can be used by simply typing their names.
/// If you have built your own modifiers, you can use them by specifying a Rust path:
///
/// ```
/// // Custom modifier
/// tr!("SAID", "Text using my modifier: {{phrase}}.", phrase:crate::modifiers::MyModifier = some_text);
///
/// // Relative paths are supported
/// use crate::modifiers;
/// tr!("SAID", "Text using my modifier: {{phrase}}.", phrase:modifiers::MyModifier = some_text);
/// ```
///
/// # Examples
///
/// ```rust,ignore
/// // Basic usage
/// tr!("BUTTON_SIGN_IN", "Sign In");
///
/// // With variable
/// tr!("GREETING", "Hello, {{name}}!", name = user.name);
///
/// // Multiple variables
/// tr!(
///     "USER_INFO",
///     "{{name}} ({{email}})",
///     name = user.name,
///     email = user.email
/// );
///
/// // With Quote modifier
/// tr!("FILE_NAME", "Opening {{file}}...", file:quote = filename);
/// ```
///
/// # Return Type
///
/// Returns an [`I18nString`](cntp_i18n_core::string::I18nString) which implements
/// `Display`, `Deref<Target=str>`, and can be converted to `String` or `Arc<str>`.
#[proc_macro]
pub fn tr(body: TokenStream) -> TokenStream {
    tr::tr(body)
}

/// Returns a translated, plural-matched string for the given key.
///
/// This macro handles pluralization according to the locale's plural rules.
/// Different languages have different plural forms - English has 2 (singular/plural),
/// while Arabic has 6, and Japanese has 1.
///
/// This is a plural version of the [`tr`] macro - for documentation not specific to
/// pluralization, see the documentation for that macro.
///
/// # Syntax
///
/// ```rust,ignore
/// trn!("KEY", "singular form", "plural form", count = value);
/// trn!("KEY", "{{count}} item", "{{count}} items", count = item_count);
/// trn!("KEY", "one form", "other form", count = n, other_var = value);
/// ```
///
/// # Arguments
///
/// The arguments are the same as those for the [`tr!`] macro, with the addition of:
/// - `count = value` - **Required.** The count value (usize) determining which form to use
///
/// # The `{{count}}` Placeholder
///
/// The `{{count}}` placeholder is automatically replaced with the locale-formatted
/// count value. You can use it in both singular and plural forms:
///
/// ```rust,ignore
/// trn!(
///     "FILES_SELECTED",
///     "{{count}} file selected",
///     "{{count}} files selected",
///     count = selected_count
/// );
/// // English: "1 file selected" or "5 files selected"
/// // German: "1 Datei ausgewählt" or "5 Dateien ausgewählt"
/// ```
///
/// # Plural Categories
///
/// The generated translation files support all ICU plural categories:
///
/// - `zero` - For languages that have a special zero form
/// - `one` - Typically singular (1 in English)
/// - `two` - For languages with dual forms (e.g., Arabic)
/// - `few` - For languages with a "few" category (e.g., Russian 2-4)
/// - `many` - For languages with a "many" category (e.g., Russian 5-20)
/// - `other` - The general plural form (required, used as fallback)
///
/// The required plural categories are determined by the default language's plural rules.
/// The default language can be changed in the library's configuration - see the documentation
/// for `cntp_i18n_build_core::config`.
///
/// # Examples
///
/// ```rust,ignore
/// // Basic plural
/// trn!(
///     "UNREAD_EMAILS",
///     "You have {{count}} unread email.",
///     "You have {{count}} unread emails.",
///     count = emails.len()
/// );
///
/// // With additional variables
/// trn!(
///     "USERS_ONLINE",
///     "{{count}} user online: {{users}}",
///     "{{count}} users online: {{users}}",
///     users = online_users.join(", "),
///     count = online_users.len()
/// );
/// ```
///
/// # Translation File Format
///
/// The generated translation file will contain:
///
/// ```json
/// {
///     "UNREAD_EMAILS": {
///         "one": "You have {{count}} unread email.",
///         "other": "You have {{count}} unread emails."
///     }
/// }
/// ```
///
/// Translators can add additional forms as needed for their language.
#[proc_macro]
pub fn trn(body: TokenStream) -> TokenStream {
    tr::trn(body)
}

/// Marks a string for translation extraction without performing a lookup.
///
/// This macro has the same syntax as [`tr!`] but does not perform any translation
/// lookup at runtime. It simply returns nothing (unit type). Its purpose is to mark
/// strings for extraction by the translation generator without affecting runtime behavior.
///
/// # Example
///
/// ```rust,ignore
/// // Mark strings for extraction
/// tr_noop!("MENU_FILE", "File");
/// tr_noop!("MENU_EDIT", "Edit");
/// tr_noop!("MENU_VIEW", "View");
/// ```
#[proc_macro]
pub fn tr_noop(_: TokenStream) -> TokenStream {
    TokenStream::default()
}

/// Marks a plural string for translation extraction without performing a lookup.
///
/// This is the plural equivalent of [`tr_noop!`]. It has the same syntax as [`trn!`]
/// but does not perform any translation lookup at runtime.
///
/// # Example
///
/// ```rust,ignore
/// // Mark plural strings for extraction
/// trn_noop!("ITEMS_COUNT", "{{count}} item", "{{count}} items");
/// ```
#[proc_macro]
pub fn trn_noop(_: TokenStream) -> TokenStream {
    TokenStream::default()
}

/// Generates an [`I18nSource`](cntp_i18n_core::I18nSource) from the translation files.
///
/// This macro reads all translation files from the configured translation directory
/// at compile time and generates a static data structure containing all translations.
/// The result implements [`I18nSource`](cntp_i18n_core::I18nSource) and can be loaded
/// into the [`I18nManager`](cntp_i18n::I18nManager).
///
/// # Line Ending Handling
///
/// By default, line endings in translation strings are normalized to match the
/// target platform (`\n` on Unix, `\r\n` on Windows). This can be disabled in
/// the `i18n.toml` configuration:
///
/// ```toml
/// [i18n]
/// match_line_endings = false
/// ```
///
/// # Example
///
/// ```rust,ignore
/// use cntp_i18n::{I18N_MANAGER, tr_load};
///
/// fn main() {
///     // Load all translations at startup
///     I18N_MANAGER.write().unwrap().load_source(tr_load!());
///
///     // Translations are now available via tr! and trn!
/// }
/// ```
///
/// # Multiple translation sources
///
/// You can load multiple translation sources. They are searched in order:
///
/// ```rust,ignore
/// // Load base translations
/// I18N_MANAGER.write().unwrap().load_source(tr_load!());
///
/// // Load override translations (searched first)
/// I18N_MANAGER.write().unwrap().load_source(custom_translations);
/// ```
///
/// # Compile-time behavior
///
/// This macro embeds all translation data directly into the binary at compile time.
/// This means:
///
/// - No runtime file I/O is needed to load translations
/// - Translation files must exist at compile time
/// - Changes to translation files require recompilation
///
/// If you'd like to load translations at runtime, you can write your own implementation
/// of [`I18nSource`](cntp_i18n_core::I18nSource).
#[proc_macro]
pub fn tr_load(body: TokenStream) -> TokenStream {
    tr_load::tr_load(body)
}
