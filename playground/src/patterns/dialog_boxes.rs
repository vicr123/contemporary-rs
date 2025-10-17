use cntp_i18n::tr;
use contemporary::components::button::button;
use contemporary::components::constrainer::constrainer;
use contemporary::components::dialog_box::{StandardButton, dialog_box};
use contemporary::components::grandstand::grandstand;
use contemporary::components::layer::layer;
use contemporary::components::subtitle::subtitle;
use contemporary::styling::theme::Theme;
use gpui::{
    App, AppContext, AsyncApp, Context, Entity, IntoElement, ParentElement, Render, Styled,
    WeakEntity, Window, div, px,
};
use std::time::Duration;

pub struct DialogBoxes {
    informational_dialog_open: bool,
    goblin_dialog_box_open: bool,
    nuclear_reactor_dialog_box_open: bool,
    nuclear_reactor_dialog_box_processing: bool,
    error_dialog_box_open: bool,
}

impl DialogBoxes {
    pub fn new(cx: &mut App) -> Entity<Self> {
        cx.new(|_| DialogBoxes {
            informational_dialog_open: false,
            goblin_dialog_box_open: false,
            nuclear_reactor_dialog_box_open: false,
            nuclear_reactor_dialog_box_processing: false,
            error_dialog_box_open: false,
        })
    }
}

impl Render for DialogBoxes {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = cx.global::<Theme>();
        div()
            .bg(theme.background)
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
                        layer()
                            .flex()
                            .flex_col()
                            .p(px(8.))
                            .w_full()
                            .child(subtitle(tr!("DIALOG_BOXES_TITLE")))
                            .child(
                                div().flex().flex_col().gap(px(8.)).child(
                                    tr!("DIALOG_BOXES_DESCRIPTION", "Click on a button to open a dialog box")
                                ).child(
                                    button("informational-dialog-box")
                                        .child(tr!(
                                            "DIALOG_BOX_INFORMATIONAL",
                                            "Informational Dialog Box"
                                        ))
                                        .on_click(cx.listener(|this, _, _, cx| {
                                            this.informational_dialog_open = true;
                                            cx.notify()
                                        })),
                                ).child(
                                    button("goblin-dialog-box")
                                        .child(tr!("DIALOG_BOX_GOBLIN", "Oh no! Goblins!"))
                                        .on_click(cx.listener(|this, _, _, cx| {
                                            this.goblin_dialog_box_open = true;
                                        }))
                                ).child(button("nuclear-reactor-dialog-box")
                                    .child(tr!("DIALOG_BOX_NUCLEAR_REACTOR", "Shut down the nuclear reactor!"))
                                    .on_click(cx.listener(|this, _, _, cx| {
                                        this.nuclear_reactor_dialog_box_open = true;
                                    }))).child(button("error-dialog-box")
                                    .child(tr!("DIALOG_BOX_ERROR", "An error has occurred"))
                                    .on_click(cx.listener(|this, _, _, cx| {
                                        this.error_dialog_box_open = true;
                                    }))),
                            ),
                    ),
            )
            .child(dialog_box("informational-dialog-box").visible(self.informational_dialog_open)
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
                .on_click_outside(cx.listener(|this, _, _, cx| {
                    this.informational_dialog_open = false;
                    cx.notify()
                }))
            )
            .child(dialog_box("goblin-dialog-box").visible(self.goblin_dialog_box_open)
                .content(
                    tr!("DIALOG_BOX_GOBLIN_CONTENT", r#"After battling through hordes of goblins, you finally stand before the throne of the Goblin King himself. The grotesque creature eyes you with contempt from his towering seat.

"So, you're the meddlesome adventurer who's been causing trouble in my kingdom, he growls. I'll give you one chance to save your miserable hide. Swear fealty to me, and I'll let you live as my servant. Refuse, and you'll spend the rest of your days in the darkest pit of my dungeons!"

The Goblin King leans forward, his putrid breath washing over you as he awaits your answer. The weight of your decision could shape the fate of the entire goblin realm."#
                        ))
                .button(
                    button("refuse-button")
                        .child(tr!("DIALOG_BOX_GOBLIN_REFUSE", "Refuse"))
                        .on_click(cx.listener(|this, _, _, cx| {
                            this.goblin_dialog_box_open = false;
                            cx.notify()
                        })),
                ).button(
                button("swear-fealty-button")
                    .child(tr!("DIALOG_BOX_GOBLIN_SWEAR_FEALTY", "Swear Fealty"))
                    .on_click(cx.listener(|this, _, _, cx| {
                        this.goblin_dialog_box_open = false;
                    }))
            )
            )
            .child(dialog_box("nuclear-reactor-dialog-box").visible(self.nuclear_reactor_dialog_box_open)
                .title(
                    tr!("DIALOG_BOX_NUCLEAR_REACTOR_TITLE", "Danger! Extremely Hazardous Operation!").into()
                )
                .content_text_informational(tr!("DIALOG_BOX_NUCLEAR_REACTOR_CONTENT", r#"You are attempting to perform an extremely hazardous operation that could result in catastrophic consequences if not executed with extreme caution.

This operation has the potential to cause:
- Complete data loss
- Irreversible system damage
- Breach of security protocols
- Unrecoverable corruption

Only proceed if you are an expert user and fully understand the risks involved. Improper handling could lead to disastrous and irreparable results."#).into(),
                                            tr!("DIALOG_BOX_NUCLEAR_REACTOR_INFORMATIONAL",
                                                    "This is the informative text displayed in grey below the main text. It provides additional context and warnings about the dangerous operation. Attempting this operation without proper expertise and precautions could lead to permanent and devastating damage to your systems and data. Proceed at your own risk."
                                                ).into(),
                )
                .standard_button(StandardButton::Cancel, cx.listener(|this, _, _, cx| {
                    this.nuclear_reactor_dialog_box_open = false;
                    cx.notify()
                }))
                .button(StandardButton::Ok.button().destructive().on_click(cx.listener(|this, _, _, cx| {
                    this.nuclear_reactor_dialog_box_processing = true;
                    cx.notify();

                    cx.spawn(async move |weak_this: WeakEntity<Self>, cx: &mut AsyncApp| {
                        cx.background_executor().timer(Duration::from_secs(3)).await;
                        weak_this.update(cx, |this, cx| {
                            this.nuclear_reactor_dialog_box_processing = false;
                            this.nuclear_reactor_dialog_box_open = false;
                            cx.notify();
                        }).unwrap();
                    }).detach();
                })))
                .processing(self.nuclear_reactor_dialog_box_processing)
            )
            .child(dialog_box("error-dialog-box").visible(self.error_dialog_box_open)
                .title(
                    tr!("DIALOG_BOX_ERROR_TITLE", "This disc can't be erased.").into()
                )
                .content_text_informational(
                    tr!("DIALOG_BOX_ERROR_CONTENT", "The disc in the drive is not rewritable, so it cannot be erased.").into(),
                    tr!("DIALOG_BOX_ERROR_INFO", "If you need to destroy the data on the disc, you should physically break it in half.").into()
                )
                .standard_button(StandardButton::Sorry, cx.listener(|this, _, _, cx| {
                    this.error_dialog_box_open = false;
                }))
            )
    }
}
