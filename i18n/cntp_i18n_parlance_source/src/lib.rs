mod parse_raw_string;

use crate::parse_raw_string::parse_raw_string;
use cntp_i18n::{I18nEntry, I18nPluralStringEntry, I18nSource, I18nString, I18nStringPart, Locale};
use serde::Deserialize;
use std::collections::HashMap;
use std::error::Error;
use std::fmt::{Display, Formatter};
use url::ParseError;
use zed_reqwest::{Client, Url};

pub struct CntpI18nParlanceSource<'a> {
    base_url: Url,
    project: String,
    subproject: String,

    crate_name: String,

    entries: HashMap<String, HashMap<String, I18nEntry<'a>>>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct ParlanceEntry {
    key: String,
    context: String,
    source: String,
    translation: Vec<ParlanceEntryTranslation>,
    requires_pluralisation: bool,
    comment: Option<String>,
    old_source_string: Option<String>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct ParlanceEntryTranslation {
    plural_type: String,
    translation_content: String,
}

#[derive(Debug)]
pub enum ParlanceSourceError {
    RequestError(zed_reqwest::Error),
    UrlParseError,
}

impl Display for ParlanceSourceError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::RequestError(e) => write!(f, "Request error: {}", e),
            Self::UrlParseError => write!(f, "URL parse error"),
        }
    }
}

impl Error for ParlanceSourceError {}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct SubprojectResponse {
    available_languages: Vec<SubprojectAvailableLanguage>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct SubprojectAvailableLanguage {
    language: String,
}

impl From<ParseError> for ParlanceSourceError {
    fn from(_: ParseError) -> Self {
        ParlanceSourceError::UrlParseError
    }
}

impl From<zed_reqwest::Error> for ParlanceSourceError {
    fn from(value: zed_reqwest::Error) -> Self {
        ParlanceSourceError::RequestError(value)
    }
}

impl<'a> CntpI18nParlanceSource<'a> {
    pub async fn new(
        base_url: Url,
        project: String,
        subproject: String,
        crate_name: String,
    ) -> Result<Self, ParlanceSourceError> {
        let client = Client::builder().build()?;
        let mut entries = HashMap::new();

        // Find all languages supported by the project
        let response = client
            .get(base_url.join(&format!("api/projects/{}/{}", project, subproject))?)
            .send()
            .await?;

        let response = response.json::<SubprojectResponse>().await?;
        for language in response.available_languages {
            // Find entries for this language
            let response = client
                .get(base_url.join(&format!(
                    "api/projects/{}/{}/{}/entries",
                    project, subproject, &language.language
                ))?)
                .send()
                .await?;

            let parlance_entries = response.json::<Vec<ParlanceEntry>>().await?;

            let mut i18n_entries = HashMap::new();
            for entry in parlance_entries {
                if let Some(i18n_entry) = entry.to_i18n_entry(language.language.clone().into()) {
                    i18n_entries.insert(entry.key.clone(), i18n_entry);
                }
            }
            entries.insert(language.language, i18n_entries);
        }

        Ok(Self {
            base_url,
            project,
            subproject,
            crate_name,
            entries,
        })
    }
}

impl<'a> ParlanceEntry {
    pub fn to_i18n_entry(&self, locale: I18nString) -> Option<I18nEntry<'a>> {
        if self.requires_pluralisation {
            let parts_for_plural = |plural_type: &str| -> Option<&'static [I18nStringPart]> {
                Some(
                    parse_raw_string(
                        &self
                            .translation
                            .iter()
                            .find(|t| t.plural_type == plural_type)?
                            .translation_content,
                    )
                    .iter()
                    .map(|part| part.calculate_string_part(self.key.as_str()))
                    .collect::<Vec<_>>()
                    .leak(),
                )
            };

            let plural_entry = I18nPluralStringEntry {
                locale,
                zero: parts_for_plural("zero"),
                one: parts_for_plural("one"),
                two: parts_for_plural("two"),
                few: parts_for_plural("few"),
                many: parts_for_plural("many"),
                other: parts_for_plural("other")?,
            };

            Some(I18nEntry::PluralEntry(plural_entry))
        } else {
            let parts = parse_raw_string(&self.translation.first()?.translation_content)
                .iter()
                .map(|part| part.calculate_string_part(self.key.as_str()))
                .collect::<Vec<_>>();
            Some(I18nEntry::Entry(parts.leak()))
        }
    }
}

impl I18nSource for CntpI18nParlanceSource<'_> {
    fn lookup(
        &'_ self,
        locale: &Locale,
        id: &str,
        lookup_crate: &str,
    ) -> Option<&'_ I18nEntry<'_>> {
        if self.crate_name != lookup_crate {
            return None;
        }

        for locale in &locale.messages {
            let Some(entries) = self.entries.get(locale) else {
                continue;
            };

            for (key, entry) in entries {
                if key == id {
                    return Some(entry);
                }
            }
        }
        None
    }
}
