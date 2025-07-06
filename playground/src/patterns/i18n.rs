use contemporary::components::constrainer::constrainer;
use contemporary::components::grandstand::grandstand;
use contemporary::components::layer::layer;
use contemporary::components::subtitle::subtitle;
use contemporary::components::text_field::TextField;
use contemporary_i18n::{i18n_manager, tr};
use gpui::{
    App, AppContext, Context, Entity, IntoElement, ParentElement, Render, Styled, Window, div, px,
};

pub struct I18n {
    quote_strings_text_field: Entity<TextField>,
}

impl I18n {
    pub fn new(cx: &mut App) -> Entity<Self> {
        cx.new(|cx| I18n {
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
        let quote_strings_text = self.quote_strings_text_field.read(cx).current_text(cx);

        let locale = &i18n_manager!().locale;

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
                    ),
            )
    }
}
