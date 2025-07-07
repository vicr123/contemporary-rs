use cntp_i18n::{Locale, LocaleFormattable, i18n_manager, tr};
use contemporary::components::constrainer::constrainer;
use contemporary::components::grandstand::grandstand;
use contemporary::components::layer::layer;
use contemporary::components::subtitle::subtitle;
use contemporary::components::text_field::TextField;
use gpui::{
    App, AppContext, Context, Entity, IntoElement, ParentElement, Render, Styled, Window, div, px,
};

pub struct I18n {
    i18n_language: Entity<TextField>,
    quote_strings_text_field: Entity<TextField>,
}

impl I18n {
    pub fn new(cx: &mut App) -> Entity<Self> {
        cx.new(|cx| I18n {
            i18n_language: TextField::new(
                cx,
                "i18n-preferred-language",
                i18n_manager!().locale.messages.first().unwrap().into(),
                tr!("I18N_LANGUAGE_CODE", "Enter a language code?").into(),
            ),
            quote_strings_text_field: TextField::new(
                cx,
                "quote-strings-text",
                "".into(),
                tr!("QUOTE_STRINGS_PLACEHOLDER", "What's your favourite song?").into(),
            ),
        })
    }
}

impl Render for I18n {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let requested_locale = self.i18n_language.read(cx).current_text(cx);
        let quote_strings_text = self.quote_strings_text_field.read(cx).current_text(cx);

        let locale = Locale::new_from_locale_identifier(requested_locale);

        div()
            .w_full()
            .h_full()
            .flex()
            .flex_col()
            .child(
                grandstand("i18n-grandstand")
                    .text(tr!("I18N_TITLE", "Locales"))
                    .pt(px(36.)),
            )
            .child(
                constrainer("i18n")
                    .flex()
                    .flex_col()
                    .w_full()
                    .p(px(8.))
                    .gap(px(8.))
                    .child(
                        layer("i18n-setup")
                            .flex()
                            .flex_col()
                            .p(px(8.))
                            .w_full()
                            .child(subtitle(tr!("I18N_SETUP", "Locales")))
                            .child(
                                div()
                                    .flex()
                                    .flex_col()
                                    .gap(px(8.))
                                    .child(tr!(
                                        "I18N_SETUP_DESCRIPTION",
                                        "Enter a locale code to see information about that locale."
                                    ))
                                    .child(self.i18n_language.clone())
                                    .child(tr!(
                                        "I18N_SELECTED_LANGUAGE",
                                        "Language: {{language}}",
                                        language = locale.human_readable_locale_name()
                                    )),
                            ),
                    )
                    .child(
                        layer("i18n-quote-string")
                            .flex()
                            .flex_col()
                            .p(px(8.))
                            .w_full()
                            .child(subtitle(tr!("I18N_QUOTE_STRINGS", "Quote Strings")))
                            .child(
                                div()
                                    .flex()
                                    .flex_col()
                                    .gap(px(8.))
                                    .child(tr!(
                                        "I18N_QUOTE_STRING_DESCRIPTION",
                                        "Type some text below to see it quoted."
                                    ))
                                    .child(self.quote_strings_text_field.clone())
                                    .child(tr!(
                                        "I18N_QUOTE_STRING_QUOTED",
                                        "Your favourite song is {{favourite_song}}",
                                        favourite_song =
                                            locale.quote_string(quote_strings_text.clone())
                                    ))
                                    .child(tr!(
                                        "I18N_QUOTE_STRING_QUOTED",
                                        "Your favourite song is {{favourite_song}}",
                                        favourite_song =
                                            locale.quote_string_alternate(quote_strings_text)
                                    )),
                            ),
                    )
                    .child(
                        layer("i18n-numeric-formats")
                            .flex()
                            .flex_col()
                            .p(px(8.))
                            .w_full()
                            .child(subtitle(tr!("I18N_NUMERIC_FORMATS", "Numeric Formats")))
                            .child(
                                div()
                                    .flex()
                                    .flex_col()
                                    .gap(px(8.))
                                    .child(tr!(
                                        "I18N_NUMERIC_FORMAT_DESCRIPTION",
                                        "Numbers are formatted in the following manner in this locale:"
                                    ))
                                    .child(tr!(
                                        "I18N_NUMERIC_FORMAT_PI",
                                        "Pi: {{pi}}",
                                        pi =
                                            std::f64::consts::PI.to_locale_string(&locale)
                                    ))
                                    .child(tr!(
                                        "I18N_NUMERIC_FORMAT_NEGATIVE_E",
                                        "-e: {{e_neg}}",
                                        e_neg =
                                            (-std::f64::consts::E).to_locale_string(&locale)
                                    ))
                            ),
                    ),
            )
    }
}
