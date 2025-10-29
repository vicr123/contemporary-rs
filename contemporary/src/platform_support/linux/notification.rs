use crate::notification::{Notification, NotificationPriority, PostedNotification};
use ashpd::desktop::notification::{NotificationProxy, Priority};
use gpui::{App, AsyncApp};
use uuid::Uuid;

struct LinuxPostedNotification {
    id: String,
}

impl PostedNotification for LinuxPostedNotification {
    fn remove(&mut self, cx: &mut App) {
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

    fn replace(&mut self, notification: Notification, cx: &mut App) {
        let id_clone = self.id.clone();
        cx.spawn(async move |cx: &mut AsyncApp| {
            let Ok(notification_portal) = NotificationProxy::new().await else {
                return;
            };

            let _ = notification_portal
                .add_notification(
                    id_clone.as_str(),
                    contemporary_notification_to_ashpd_notification(notification),
                )
                .await;
        })
        .detach();
    }
}

pub fn post_notification(notification: Notification, cx: &mut App) -> Box<dyn PostedNotification> {
    let id = Uuid::new_v4().to_string();

    let id_clone = id.clone();
    cx.spawn(async move |cx: &mut AsyncApp| {
        // TODO: If the portal is not available, fall back to org.freedesktop.Notifications
        let Ok(notification_portal) = NotificationProxy::new().await else {
            return;
        };

        let _ = notification_portal
            .add_notification(
                id_clone.as_str(),
                contemporary_notification_to_ashpd_notification(notification),
            )
            .await;
    })
    .detach();

    Box::new(LinuxPostedNotification { id })
}

fn contemporary_notification_to_ashpd_notification(
    notification: Notification,
) -> ashpd::desktop::notification::Notification {
    ashpd::desktop::notification::Notification::new(
        notification.summary.unwrap_or_default().as_str(),
    )
    .body(notification.body.unwrap_or_default().as_str())
    .priority(match notification.priority {
        NotificationPriority::Low => Priority::Low,
        NotificationPriority::Normal => Priority::Normal,
        NotificationPriority::High => Priority::High,
        NotificationPriority::Urgent => Priority::Urgent,
    })
}
