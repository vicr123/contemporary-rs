use crate::notification::{Notification, PostedNotification};
use gpui::{App, Global};
use objc2::rc::Retained;
use objc2::runtime::{NSObject, NSObjectProtocol, ProtocolObject};
use objc2::{define_class, msg_send, msg_send_id, MainThreadMarker, MainThreadOnly};
use objc2_foundation::{
    NSBundle, NSString, NSUserNotification, NSUserNotificationCenter,
    NSUserNotificationCenterDelegate,
};

enum MacPostedNotification {
    Failed,
    Posted {
        ns_notification: Retained<NSUserNotification>,
    },
}

impl PostedNotification for MacPostedNotification {
    fn remove(&mut self, cx: &mut App) {
        unsafe {
            let bundle = NSBundle::mainBundle();
            if bundle.bundleIdentifier().is_none() {
                // We don't have a bundle so the notifications won't work.
                // Fail silently.
                return;
            }

            if let MacPostedNotification::Posted { ns_notification } = &self {
                NSUserNotificationCenter::defaultUserNotificationCenter()
                    .removeDeliveredNotification(ns_notification);
            }
        }
    }

    fn replace(&mut self, _: Notification, _: &mut App) {
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
                .map(|summary| NSString::from_str(summary.as_str()))
                .as_deref(),
        );
        ns_notification.setInformativeText(
            notification
                .body
                .map(|summary| NSString::from_str(summary.as_str()))
                .as_deref(),
        );

        NSUserNotificationCenter::defaultUserNotificationCenter()
            .deliverNotification(&ns_notification);

        Box::new(MacPostedNotification::Posted { ns_notification })
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
