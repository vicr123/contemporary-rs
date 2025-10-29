use gpui::App;

pub struct Notification {
    pub summary: Option<String>,
    pub body: Option<String>,
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

    pub fn post(self, cx: &mut App) -> Box<dyn PostedNotification> {
        #[cfg(target_os = "macos")]
        {
            return crate::platform_support::macos::notification::post_notification(self, cx);
        }

        Box::new(DummyPostedNotification)
    }
}

pub trait PostedNotification {
    fn remove(&mut self, cx: &mut App);
}

struct DummyPostedNotification;

impl PostedNotification for DummyPostedNotification {
    fn remove(&mut self, cx: &mut App) {
        // noop
    }
}
