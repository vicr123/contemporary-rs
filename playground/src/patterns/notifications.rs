use cntp_i18n::tr;
use contemporary::application::Details;
use contemporary::components::button::button;
use contemporary::components::constrainer::constrainer;
use contemporary::components::grandstand::grandstand;
use contemporary::components::icon_text::icon_text;
use contemporary::components::layer::layer;
use contemporary::components::subtitle::subtitle;
use contemporary::components::text_field::TextField;
use contemporary::notification::Notification;
use contemporary::styling::theme::Theme;
use gpui::{
    App, AppContext, Context, Entity, IntoElement, ParentElement, Render, Styled, Window, div, px,
};

pub struct Notifications {
    summary_field: Entity<TextField>,
    body_field: Entity<TextField>,
}

impl Notifications {
    pub fn new(cx: &mut App) -> Entity<Self> {
        cx.new(|cx| Self {
            summary_field: cx.new(|cx| {
                let mut text_field = TextField::new("summary-field", cx);
                text_field
                    .set_placeholder(tr!("NOTIFICATIONS_SUMMARY", "Summary").to_string().as_str());
                text_field
            }),
            body_field: cx.new(|cx| {
                let mut text_field = TextField::new("body-field", cx);
                text_field.set_placeholder(tr!("NOTIFICATIONS_BODY", "Body").to_string().as_str());
                text_field
            }),
        })
    }
}

impl Render for Notifications {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = cx.global::<Theme>();
        let details = cx.global::<Details>();

        let directories = details.standard_dirs().unwrap();

        div()
            .bg(theme.background)
            .w_full()
            .h_full()
            .flex()
            .flex_col()
            .child(
                grandstand("notifications-grandstand")
                    .text(tr!("NOTIFICATIONS_TITLE", "Notifications"))
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
                        layer()
                            .flex()
                            .flex_col()
                            .p(px(8.))
                            .w_full()
                            .child(subtitle(tr!("NOTIFICATIONS_TITLE")))
                            .child(
                                div()
                                    .flex()
                                    .flex_col()
                                    .gap(px(8.))
                                    .child(tr!(
                                        "NOTIFICATIONS_DESCRIPTION",
                                        "Notifications throw up an alert on the desktop to let \
                                        the user know about something that happened in the \
                                        background."
                                    ))
                                    .child(self.summary_field.clone())
                                    .child(self.body_field.clone())
                                    .child(
                                        button("send-notification-button")
                                            .child(icon_text(
                                                "mail-send".into(),
                                                tr!("NOTIFICATION_SEND", "Send Notification")
                                                    .into(),
                                            ))
                                            .on_click(cx.listener(|this, _, _, cx| {
                                                // TODO
                                                let summary = this.summary_field.read(cx).text();
                                                let body = this.body_field.read(cx).text();
                                                Notification::new()
                                                    .summary(summary)
                                                    .body(body)
                                                    .post(cx);
                                            })),
                                    ),
                            ),
                    ),
            )
    }
}
