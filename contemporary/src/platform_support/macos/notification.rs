use crate::notification::{Notification, NotificationSound, PostedNotification};
use gpui::{App, Global};
use objc2::rc::Retained;
use objc2::runtime::{NSObject, NSObjectProtocol, ProtocolObject};
use objc2::{MainThreadMarker, MainThreadOnly, define_class, msg_send, msg_send_id};
use objc2_foundation::{
    NSBundle, NSString, NSUserNotification, NSUserNotificationCenter,
    NSUserNotificationCenterDelegate, NSUserNotificationDefaultSoundName,
};

enum MacPostedNotification {
    Failed,
    Posted {
        ns_notification: Retained<NSUserNotification>,
        summary: Option<String>,
        body: Option<String>,
    },
}

impl PostedNotification for MacPostedNotification {
    fn summary(&self) -> Option<&str> {
        match self {
            MacPostedNotification::Failed => None,
            MacPostedNotification::Posted { summary, .. } => summary.as_deref(),
        }
    }

    fn body(&self) -> Option<&str> {
        match self {
            MacPostedNotification::Failed => None,
            MacPostedNotification::Posted { body, .. } => body.as_deref(),
        }
    }

    fn dismiss(&self, cx: &mut App) {
        unsafe {
            let bundle = NSBundle::mainBundle();
            if bundle.bundleIdentifier().is_none() {
                // We don't have a bundle so the notifications won't work.
                // Fail silently.
                return;
            }

            if let MacPostedNotification::Posted {
                ns_notification, ..
            } = &self
            {
                NSUserNotificationCenter::defaultUserNotificationCenter()
                    .removeDeliveredNotification(ns_notification);
            }
        }
    }

    fn replace(&self, _: Notification, _: &mut App) {
        // TODO
    }
}

pub fn post_notification(notification: Notification, _: &mut App) -> Box<dyn PostedNotification> {
    unsafe {
        let bundle = NSBundle::mainBundle();
        if bundle.bundleIdentifier().is_none() {
            // We don't have a bundle so the notifications won't work.
            // Fail silently.
            return Box::new(MacPostedNotification::Failed);
        }

        let ns_notification = NSUserNotification::new();
        ns_notification.setTitle(
            notification
                .summary
                .clone()
                .map(|summary| NSString::from_str(summary.as_str()))
                .as_deref(),
        );
        ns_notification.setInformativeText(
            notification
                .body
                .clone()
                .map(|summary| NSString::from_str(summary.as_str()))
                .as_deref(),
        );
        ns_notification.setSoundName(match notification.sound {
            NotificationSound::Default => Some(NSUserNotificationDefaultSoundName),
            NotificationSound::Silent => None,
        });

        NSUserNotificationCenter::defaultUserNotificationCenter()
            .deliverNotification(&ns_notification);

        Box::new(MacPostedNotification::Posted {
            ns_notification,
            summary: notification.summary,
            body: notification.body,
        })
    }
}

// Define the delegate class
define_class!(
    #[unsafe(super(NSObject))]
    #[thread_kind = MainThreadOnly]
    struct AppNotificationDelegate;

    impl AppNotificationDelegate {
        #[unsafe(method(userNotificationCenter:shouldPresentNotification:))]
        unsafe fn __user_notification_center_should_present_notification(
            &self,
            center: &NSUserNotificationCenter,
            notification: &NSUserNotification,
        ) -> bool {
            true
        }

        #[unsafe(method(userNotificationCenter:didActivateNotification:))]
        unsafe fn __user_notification_center_did_activate_notification(
            &self,
            center: &NSUserNotificationCenter,
            notification: &NSUserNotification,
        ) {
            // TODO: Actions
        }
    }

    unsafe impl NSObjectProtocol for AppNotificationDelegate {}

    unsafe impl NSUserNotificationCenterDelegate for AppNotificationDelegate {}
);

impl AppNotificationDelegate {
    fn new() -> Retained<Self> {
        let this = Self::alloc(MainThreadMarker::new().unwrap());
        unsafe { msg_send![this, init] }
    }
}

struct AppleAppNotificationDelegateGlobalableWrapperThing {
    globalable_item: Retained<AppNotificationDelegate>,
}

impl Global for AppleAppNotificationDelegateGlobalableWrapperThing {}

pub fn setup_apple_notifications(cx: &mut App) {
    let app_notification_delegate = unsafe {
        let bundle = NSBundle::mainBundle();
        if bundle.bundleIdentifier().is_none() {
            // We don't have a bundle so the notifications won't work.
            // Fail silently.
            return;
        }

        let app_notification_delegate = AppNotificationDelegate::new();
        let delegate: &ProtocolObject<dyn NSUserNotificationCenterDelegate> =
            ProtocolObject::from_ref(&*app_notification_delegate);
        NSUserNotificationCenter::defaultUserNotificationCenter().setDelegate(Some(delegate));

        app_notification_delegate
    };

    cx.set_global(AppleAppNotificationDelegateGlobalableWrapperThing {
        globalable_item: app_notification_delegate,
    })
}
