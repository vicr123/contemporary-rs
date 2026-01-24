use cntp_i18n::tr;
use contemporary::components::button::button;
use contemporary::components::constrainer::constrainer;
use contemporary::components::grandstand::grandstand;
use contemporary::components::layer::layer;
use contemporary::components::subtitle::subtitle;
use contemporary::components::text_field::{MaskMode, TextField};
use contemporary::styling::theme::{Theme, ThemeStorage};
use gpui::{
    App, AppContext, Context, Entity, IntoElement, ParentElement, Render, Styled, Window, div, px,
};

pub struct TextInput {
    text_field: Entity<TextField>,
    password_text_field: Entity<TextField>,
    borderless_text_field: Entity<TextField>,
    disabled_text_field: Entity<TextField>,
    big_text_field: Entity<TextField>,
}

impl TextInput {
    pub fn new(cx: &mut App) -> Entity<Self> {
        cx.new(|cx| {
            
            TextInput {
                text_field: cx.new(|cx| {
                    let mut text_field = TextField::new("text-field", cx);
                    text_field.set_placeholder(
                        tr!("TEXT_FIELD_PLACEHOLDER", "Text Field")
                            .to_string()
                            .as_str(),
                    );
                    text_field
                }),
                password_text_field: cx.new(|cx| {
                    let mut text_field = TextField::new("password-text-field", cx);
                    text_field.set_mask_mode(MaskMode::password_mask());
                    text_field.set_placeholder(
                        tr!("PASSWORD_TEXT_FIELD_PLACEHOLDER", "Password Text Field")
                            .to_string()
                            .as_str(),
                    );
                    text_field
                }),
                borderless_text_field: cx.new(|cx| {
                    let mut text_field = TextField::new("borderless-text-field", cx);
                    text_field.set_has_border(false);
                    text_field.set_placeholder(
                        tr!("BORDERLESS_TEXT_FIELD_PLACEHOLDER", "Borderless Text Field")
                            .to_string()
                            .as_str(),
                    );
                    text_field
                }),
                disabled_text_field: cx.new(|cx| {
                    let mut text_field = TextField::new("disabled-text-field", cx);
                    // TODO: Set as disabled
                    text_field.set_placeholder(
                        tr!("TEXT_FIELD_DISABLED_PLACEHOLDER", "Disabled Text Field")
                            .to_string()
                            .as_str(),
                    );
                    text_field
                }),
                big_text_field: cx.new(|cx| {
                    let mut text_field = TextField::new("text-field", cx);
                    cx.observe_global::<Theme>(|text_field: &mut TextField, cx| {
                        let theme = cx.theme();
                        text_field.text_style().font_size = Some(theme.heading_font_size.into());
                        cx.notify()
                    })
                    .detach();
                    text_field.set_placeholder(
                        tr!("BIG_TEXT_FIELD_PLACEHOLDER", "Big Text Field")
                            .to_string()
                            .as_str(),
                    );
                    text_field
                }),
            }
        })
    }
}

impl Render for TextInput {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = cx.theme();
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
                                    .child(self.disabled_text_field.clone().into_any_element())
                                    .child(self.big_text_field.clone().into_any_element())
                                    .child(
                                        button("flash-error-button")
                                            .child(tr!("BUTTON_FLASH_ERROR", "Flash Error"))
                                            .on_click(cx.listener(|this, _, window, cx| {
                                                this.text_field.update(cx, |text_field, cx| {
                                                    text_field.flash_error(window, cx)
                                                });
                                                this.password_text_field.update(
                                                    cx,
                                                    |text_field, cx| {
                                                        text_field.flash_error(window, cx)
                                                    },
                                                );
                                                this.borderless_text_field.update(
                                                    cx,
                                                    |text_field, cx| {
                                                        text_field.flash_error(window, cx)
                                                    },
                                                );
                                                this.disabled_text_field.update(
                                                    cx,
                                                    |text_field, cx| {
                                                        text_field.flash_error(window, cx)
                                                    },
                                                );
                                                this.big_text_field.update(cx, |text_field, cx| {
                                                    text_field.flash_error(window, cx)
                                                });
                                            })),
                                    ),
                            ),
                    ),
            )
    }
}
