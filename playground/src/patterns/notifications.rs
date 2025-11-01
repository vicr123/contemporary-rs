use cntp_i18n::{Quote, tr};
use contemporary::application::Details;
use contemporary::components::button::button;
use contemporary::components::constrainer::constrainer;
use contemporary::components::grandstand::grandstand;
use contemporary::components::icon::icon;
use contemporary::components::icon_text::icon_text;
use contemporary::components::layer::layer;
use contemporary::components::subtitle::subtitle;
use contemporary::components::text_field::TextField;
use contemporary::notification::{Notification, PostedNotification};
use contemporary::styling::theme::Theme;
use gpui::prelude::FluentBuilder;
use gpui::{
    App, AppContext, ClickEvent, Context, Entity, InteractiveElement, IntoElement, ListAlignment,
    ListSizingBehavior, ListState, ParentElement, Render, Styled, Window, div, list, px,
};
use std::rc::Rc;

pub struct Notifications {
    summary_field: Entity<TextField>,
    body_field: Entity<TextField>,

    posted_notifications: Vec<Rc<Box<dyn PostedNotification>>>,
    metadata: Vec<Entity<NotificationMetadata>>,
    list_state: ListState,
    pending_actions: Vec<Entity<TextField>>,
}

struct NotificationMetadata {
    dismissed: bool,
    triggered_action: Option<String>,
}

impl Default for NotificationMetadata {
    fn default() -> Self {
        NotificationMetadata {
            dismissed: false,
            triggered_action: None,
        }
    }
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

            posted_notifications: Vec::new(),
            metadata: Vec::new(),
            list_state: ListState::new(0, ListAlignment::Top, px(200.)),
            pending_actions: Vec::new(),
        })
    }

    fn trigger_notification(
        &mut self,
        _: &ClickEvent,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let meta = cx.new(|_| NotificationMetadata::default());
        let meta_clone = meta.clone();
        let meta_weak = meta.downgrade();

        let summary = self.summary_field.read(cx).text();
        let body = self.body_field.read(cx).text();

        let mut notification =
            Notification::new()
                .summary(summary)
                .body(body)
                .on_dismiss(move |_, cx| {
                    meta_clone.update(cx, |meta, cx| {
                        meta.dismissed = true;
                        cx.notify()
                    })
                });

        for action in self.pending_actions.iter() {
            let meta_weak = meta_weak.clone();
            let action_text = action.read(cx).text().to_string();
            let action_text_2 = action_text.clone();
            notification = notification.on_action(&action_text_2, move |_, cx| {
                let action_text = action_text.clone();
                let _ = meta_weak.update(cx, |meta, cx| meta.triggered_action = Some(action_text));
            })
        }

        let posted = Rc::new(notification.post(cx));

        self.posted_notifications.push(posted.clone());
        self.metadata.push(meta);
        self.list_state.reset(self.posted_notifications.len());
        cx.notify();
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
                                    .child(self.pending_actions.iter().enumerate().fold(
                                        div().flex().flex_col().gap(px(8.)),
                                        |david, (i, text_field)| {
                                            david.child(
                                                div()
                                                    .id(i)
                                                    .flex()
                                                    .gap(px(4.))
                                                    .child(text_field.clone())
                                                    .child(
                                                        button("delete-button")
                                                            .destructive()
                                                            .child(icon("list-remove".into()))
                                                            .on_click(cx.listener(
                                                                move |this, _, _, cx| {
                                                                    this.pending_actions.remove(i);
                                                                    cx.notify();
                                                                },
                                                            )),
                                                    ),
                                            )
                                        },
                                    ))
                                    .child(
                                        button("add-action-button")
                                            .child(icon_text(
                                                "list-add".into(),
                                                tr!("NOTIFICATION_ADD_ACTION", "Add Action").into(),
                                            ))
                                            .on_click(cx.listener(|this, _, _, cx| {
                                                this.pending_actions.push(
                                                    cx.new(|cx| TextField::new("action", cx)),
                                                );
                                                cx.notify()
                                            })),
                                    )
                                    .child(
                                        button("send-notification-button")
                                            .child(icon_text(
                                                "mail-send".into(),
                                                tr!("NOTIFICATION_SEND", "Post Notification")
                                                    .into(),
                                            ))
                                            .on_click(cx.listener(Self::trigger_notification)),
                                    ),
                            ),
                    )
                    .child(
                        layer()
                            .flex()
                            .flex_col()
                            .p(px(8.))
                            .w_full()
                            .child(subtitle(tr!(
                                "POSTED_NOTIFICATIONS_TITLE",
                                "Posted Notifications"
                            )))
                            .child(
                                div()
                                    .flex()
                                    .flex_col()
                                    .gap(px(8.))
                                    .child(tr!(
                                        "POSTED_NOTIFICATIONS_DESCRIPTION",
                                        "Notifications that you have posted will appear below."
                                    ))
                                    .child(
                                        list(
                                            self.list_state.clone(),
                                            cx.processor(|this, i, _, cx| {
                                                let notification: &Rc<Box<dyn PostedNotification>> =
                                                    &this.posted_notifications[i];
                                                let metadata: &Entity<NotificationMetadata> =
                                                    &this.metadata[i];
                                                let notification = notification.clone();
                                                let metadata = metadata.read(cx);

                                                div()
                                                    .id(i)
                                                    .py(px(2.))
                                                    .child(
                                                        layer()
                                                            .flex()
                                                            .w_full()
                                                            .p(px(4.))
                                                            .gap(px(4.))
                                                            .items_center()
                                                            .child(
                                                                div()
                                                                    .flex()
                                                                    .flex_col()
                                                                    .flex_grow()
                                                                    .child(subtitle(
                                                                        notification
                                                                            .summary()
                                                                            .unwrap_or_default()
                                                                            .to_string(),
                                                                    ))
                                                                    .child(
                                                                        notification
                                                                            .body()
                                                                            .unwrap_or_default()
                                                                            .to_string(),
                                                                    ),
                                                            )
                                                            .when_some(
                                                                metadata.triggered_action.clone(),
                                                                |david, action| {
                                                                    david.child(tr!(
                                                                        "NOTIFICATION_ACTION_\
                                                                         TRIGGERED",
                                                                        "Triggered with {{action}}",
                                                                        action:Quote = action
                                                                    ))
                                                                },
                                                            )
                                                            .child(
                                                                button("dismiss-button")
                                                                    .destructive()
                                                                    .child(icon(
                                                                        "edit-delete".into(),
                                                                    ))
                                                                    .when(
                                                                        metadata.dismissed,
                                                                        |button| button.disabled(),
                                                                    )
                                                                    .on_click(move |_, _, cx| {
                                                                        notification.dismiss(cx);
                                                                    }),
                                                            ),
                                                    )
                                                    .into_any_element()
                                            }),
                                        )
                                        .with_sizing_behavior(ListSizingBehavior::Infer),
                                    ),
                            ),
                    ),
            )
    }
}
