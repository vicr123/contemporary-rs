use crate::notification::{
    Notification, NotificationActionEvent, NotificationPriority, PostedNotification,
};
use ashpd::desktop::notification::{Button, NotificationProxy, Priority};
use gpui::{App, AsyncApp, BorrowAppContext, Global};
use smol::stream::StreamExt;
use std::collections::HashMap;
use uuid::Uuid;

struct LinuxPostedNotification {
    id: String,
    summary: Option<String>,
    body: Option<String>,
}

impl PostedNotification for LinuxPostedNotification {
    fn summary(&self) -> Option<&str> {
        self.summary.as_deref()
    }

    fn body(&self) -> Option<&str> {
        self.body.as_deref()
    }

    fn dismiss(&self, cx: &mut App) {
        let id_clone = self.id.clone();
        cx.spawn(async move |cx: &mut AsyncApp| {
            let Ok(notification_portal) = NotificationProxy::new().await else {
                return;
            };

            let _ = notification_portal
                .remove_notification(id_clone.as_str())
                .await;
        })
        .detach();
    }

    fn replace(&self, notification: Notification, cx: &mut App) {
        let id_clone = self.id.clone();

        let ashpd_notification = contemporary_notification_to_ashpd_notification(notification, cx);

        cx.spawn(async move |cx: &mut AsyncApp| {
            let Ok(notification_portal) = NotificationProxy::new().await else {
                return;
            };

            let _ = notification_portal
                .add_notification(id_clone.as_str(), ashpd_notification)
                .await;
        })
        .detach();
    }
}

pub fn post_notification(notification: Notification, cx: &mut App) -> Box<dyn PostedNotification> {
    let id = Uuid::new_v4().to_string();

    let body = notification.body.clone();
    let summary = notification.summary.clone();

    let ashpd_notification = contemporary_notification_to_ashpd_notification(notification, cx);

    let id_clone = id.clone();
    cx.spawn(async move |cx: &mut AsyncApp| {
        // TODO: If the portal is not available, fall back to org.freedesktop.Notifications
        let Ok(notification_portal) = NotificationProxy::new().await else {
            return;
        };

        let _ = notification_portal
            .add_notification(id_clone.as_str(), ashpd_notification)
            .await;
    })
    .detach();

    Box::new(LinuxPostedNotification { id, body, summary })
}

fn contemporary_notification_to_ashpd_notification(
    notification: Notification,
    cx: &mut App,
) -> ashpd::desktop::notification::Notification {
    cx.update_global::<LinuxNotificationGlobal, _>(|linux_notification_global, cx| {
        let mut ashpd_notification = ashpd::desktop::notification::Notification::new(
            notification.summary.unwrap_or_default().as_str(),
        )
        .body(notification.body.unwrap_or_default().as_str())
        .priority(match notification.priority {
            NotificationPriority::Low => Priority::Low,
            NotificationPriority::Normal => Priority::Normal,
            NotificationPriority::High => Priority::High,
            NotificationPriority::Urgent => Priority::Urgent,
        });

        for action in notification.actions {
            let uuid = Uuid::new_v4();
            let on_triggered = action.on_triggered.clone();
            linux_notification_global.action_handlers.insert(
                uuid,
                Box::new(move |_, cx| {
                    on_triggered.clone()(&NotificationActionEvent, cx);
                }),
            );
            ashpd_notification = ashpd_notification
                .button(Button::new(action.text.as_str(), uuid.to_string().as_str()));
        }

        ashpd_notification
    })
}

struct LinuxNotificationGlobal {
    action_handlers: HashMap<Uuid, Box<dyn Fn(Option<String>, &mut App) + 'static>>,
}

impl Global for LinuxNotificationGlobal {}

pub fn setup_linux_notifications(cx: &mut App) {
    cx.set_global(LinuxNotificationGlobal {
        action_handlers: HashMap::new(),
    });

    cx.spawn(async move |cx: &mut AsyncApp| {
        // TODO: If the portal is not available, fall back to org.freedesktop.Notifications
        let Ok(notification_portal) = NotificationProxy::new().await else {
            return;
        };

        let Ok(mut action_invoked) = notification_portal.receive_action_invoked().await else {
            return;
        };

        while let Some(action) = action_invoked.next().await {
            if cx
                .update_global::<LinuxNotificationGlobal, _>(|linux_notification_global, cx| {
                    let Ok(uuid) = Uuid::try_parse(action.name()) else {
                        return;
                    };

                    let Some(handler) = linux_notification_global.action_handlers.get(&uuid) else {
                        return;
                    };

                    handler(None, cx);
                })
                .is_err()
            {
                return;
            }
        }
    })
    .detach();
}
