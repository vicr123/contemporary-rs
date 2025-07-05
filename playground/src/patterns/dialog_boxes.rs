use contemporary::components::button::button;
use contemporary::components::constrainer::constrainer;
use contemporary::components::dialog_box::{dialog_box, StandardButton};
use contemporary::components::grandstand::grandstand;
use contemporary::components::layer::layer;
use contemporary::components::subtitle::subtitle;
use contemporary_i18n::tr;
use gpui::prelude::FluentBuilder;
use gpui::{
    div, px, App, AppContext, Context, Entity, IntoElement, ParentElement, Render, Styled, Window,
};

pub struct DialogBoxes {
    informational_dialog_open: bool,
}

impl DialogBoxes {
    pub fn new(cx: &mut App) -> Entity<Self> {
        cx.new(|_| DialogBoxes {
            informational_dialog_open: false,
        })
    }
}

impl Render for DialogBoxes {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .w_full()
            .h_full()
            .flex()
            .flex_col()
            .child(
                grandstand("dialog-boxes-grandstand")
                    .text(tr!("DIALOG_BOXES_TITLE", "Dialog Boxes"))
                    .pt(px(36.)),
            )
            .child(
                constrainer("dialog-boxes")
                    .flex()
                    .flex_col()
                    .w_full()
                    .p(px(8.))
                    .gap(px(8.))
                    .child(
                        layer("normal-dialog-boxes")
                            .flex()
                            .flex_col()
                            .p(px(8.))
                            .w_full()
                            .child(subtitle(tr!("DIALOG_BOXES_TITLE")))
                            .child(
                                div().flex().flex_col().gap(px(8.)).child(
                                    button("informational-dialog-box")
                                        .child(tr!(
                                            "DIALOG_BOX_INFORMATIONAL",
                                            "Informational Dialog Box"
                                        ))
                                        .on_click(cx.listener(|this, _, _, cx| {
                                            this.informational_dialog_open = true;
                                            cx.notify()
                                        })),
                                ),
                            ),
                    ),
            )
            .when(self.informational_dialog_open, |div| {
                div.child(dialog_box()
                    .title(
                        tr!("DIALOG_BOX_INFORMATIONAL_TITLE", "Message Box Title").into()
                    )
                    .content(
                        tr!("DIALOG_BOX_INFORMATIONAL_CONTENT", "This is the main text of the message box. It conveys the primary information or message that needs to be communicated to the user.")
                    )
                    .standard_button(StandardButton::Ok, cx.listener(|this, _, _, cx| {
                        this.informational_dialog_open = false;
                        cx.notify()
                    }))
                )
            })
    }
}
