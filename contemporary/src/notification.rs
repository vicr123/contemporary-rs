use gpui::{App, Window};
use std::rc::Rc;
use std::time::Duration;
use uuid::Uuid;

pub struct NotificationDismissEvent;

type DismissListener = Box<dyn Fn(&NotificationDismissEvent, &mut App)>;

#[derive(Clone)]
pub struct Notification {
    pub summary: Option<String>,
    pub body: Option<String>,
    pub priority: NotificationPriority,
    pub sound: NotificationSound,
    pub timeout: NotificationTimeout,

    pub on_dismiss: Vec<Rc<DismissListener>>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum NotificationPriority {
    Low,

    #[default]
    Normal,
    High,
    Urgent,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum NotificationSound {
    #[default]
    Default,
    Silent,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum NotificationTimeout {
    #[default]
    Default,
    Never,
    Duration(Duration),
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
            priority: NotificationPriority::default(),
            sound: NotificationSound::default(),
            timeout: NotificationTimeout::default(),

            on_dismiss: Vec::new(),
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

    pub fn sound(mut self, sound: NotificationSound) -> Notification {
        self.sound = sound;
        self
    }

    pub fn silent(self) -> Notification {
        self.sound(NotificationSound::Silent)
    }

    pub fn on_dismiss(
        mut self,
        listener: impl Fn(&NotificationDismissEvent, &mut App) + 'static,
    ) -> Notification {
        self.on_dismiss.push(Rc::new(Box::new(listener)));
        self
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

        Box::new(DummyPostedNotification {
            summary: self.summary,
            body: self.body,
        })
    }
}

pub trait PostedNotification {
    fn summary(&self) -> Option<&str>;
    fn body(&self) -> Option<&str>;

    fn dismiss(&self, cx: &mut App);
    fn replace(&self, notification: Notification, cx: &mut App);
}

struct DummyPostedNotification {
    summary: Option<String>,
    body: Option<String>,
}

impl PostedNotification for DummyPostedNotification {
    fn summary(&self) -> Option<&str> {
        self.summary.as_deref()
    }

    fn body(&self) -> Option<&str> {
        self.body.as_deref()
    }

    fn dismiss(&self, cx: &mut App) {
        // noop
    }

    fn replace(&self, notification: Notification, cx: &mut App) {
        // noop
    }
}
