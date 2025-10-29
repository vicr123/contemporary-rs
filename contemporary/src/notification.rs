use gpui::App;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Notification {
    pub summary: Option<String>,
    pub body: Option<String>,
    pub priority: NotificationPriority,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum NotificationPriority {
    Low,

    #[default]
    Normal,
    High,
    Urgent,
}

impl Default for Notification {
    fn default() -> Self {
        Self::new()
    }
}

impl Notification {
    pub fn new() -> Notification {
        Notification {
            summary: None,
            body: None,
            priority: NotificationPriority::Normal,
        }
    }

    pub fn summary(mut self, summary: &str) -> Notification {
        self.summary = Some(summary.to_string());
        self
    }

    pub fn body(mut self, body: &str) -> Notification {
        self.body = Some(body.to_string());
        self
    }

    pub fn priority(mut self, priority: NotificationPriority) -> Notification {
        self.priority = priority;
        self
    }

    pub fn low_priority(self) -> Notification {
        self.priority(NotificationPriority::Low)
    }

    pub fn high_priority(self) -> Notification {
        self.priority(NotificationPriority::High)
    }

    pub fn urgent_priority(self) -> Notification {
        self.priority(NotificationPriority::Urgent)
    }

    pub fn post(self, cx: &mut App) -> Box<dyn PostedNotification> {
        #[cfg(target_os = "macos")]
        {
            return crate::platform_support::macos::notification::post_notification(self, cx);
        }
        #[cfg(target_os = "linux")]
        {
            return crate::platform_support::linux::notification::post_notification(self, cx);
        }

        Box::new(DummyPostedNotification)
    }
}

pub trait PostedNotification {
    fn remove(&mut self, cx: &mut App);
    fn replace(&mut self, notification: Notification, cx: &mut App);
}

struct DummyPostedNotification;

impl PostedNotification for DummyPostedNotification {
    fn remove(&mut self, cx: &mut App) {
        // noop
    }

    fn replace(&mut self, notification: Notification, cx: &mut App) {
        // noop
    }
}
