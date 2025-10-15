use cntp_i18n::tr;
use contemporary::components::constrainer::constrainer;
use contemporary::components::grandstand::grandstand;
use contemporary::components::layer::layer;
use contemporary::components::subtitle::subtitle;
use contemporary::components::text_field::TextField;
use contemporary::styling::theme::Theme;
use gpui::{
    App, AppContext, Context, Entity, IntoElement, ParentElement, Render, Styled, Window, div, px,
};

pub struct TextInput {
    text_field: Entity<TextField>,
    password_text_field: Entity<TextField>,
    borderless_text_field: Entity<TextField>,
    disabled_text_field: Entity<TextField>,
}

impl TextInput {
    pub fn new(cx: &mut App) -> Entity<Self> {
        cx.new(|cx| {
            let text_input = TextInput {
                text_field: TextField::new(
                    cx,
                    "text-field",
                    "".into(),
                    tr!("TEXT_FIELD_PLACEHOLDER", "Text Field").into(),
                ),
                password_text_field: TextField::new(
                    cx,
                    "password-text-field",
                    "".into(),
                    tr!("PASSWORD_TEXT_FIELD_PLACEHOLDER", "Password Text Field").into(),
                ),
                borderless_text_field: TextField::new(
                    cx,
                    "borderless-text-field",
                    "".into(),
                    tr!("BORDERLESS_TEXT_FIELD_PLACEHOLDER", "Borderless Text Field").into(),
                ),
                disabled_text_field: TextField::new(
                    cx,
                    "disabled-text-field",
                    "".into(),
                    tr!("TEXT_FIELD_DISABLED_PLACEHOLDER", "Disabled Text Field").into(),
                ),
            };
            text_input.password_text_field.update(cx, |this, cx| {
                this.password_field(cx, true);
                cx.notify();
            });
            text_input.borderless_text_field.update(cx, |this, cx| {
                this.borderless(true);
                cx.notify();
            });
            text_input.disabled_text_field.update(cx, |this, cx| {
                this.disabled(cx, true);
                cx.notify();
            });
            text_input
        })
    }
}

impl Render for TextInput {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = cx.global::<Theme>();
        div()
            .bg(theme.background)
            .w_full()
            .h_full()
            .flex()
            .flex_col()
            .child(
                grandstand("text-input-grandstand")
                    .text(tr!("TEXT_INPUT_TITLE", "Text Input"))
                    .pt(px(36.)),
            )
            .child(
                constrainer("text-input")
                    .flex()
                    .flex_col()
                    .w_full()
                    .p(px(8.))
                    .gap(px(8.))
                    .child(
                        layer()
                            .flex()
                            .flex_col()
                            .p(px(8.))
                            .w_full()
                            .child(subtitle(tr!("TEXT_FIELDS_TITLE", "Text Fields")))
                            .child(
                                div()
                                    .flex()
                                    .flex_col()
                                    .gap(px(8.))
                                    .child(self.text_field.clone().into_any_element())
                                    .child(self.password_text_field.clone().into_any_element())
                                    .child(self.borderless_text_field.clone().into_any_element())
                                    .child(self.disabled_text_field.clone().into_any_element()),
                            ),
                    ),
            )
    }
}
