use crate::notification::{
    Notification, NotificationActionEvent, NotificationReplyActionEvent, NotificationSound,
    PostedNotification,
};
use async_channel::Sender;
use gpui::{App, AsyncApp, BorrowAppContext, Global};
use objc2::rc::Retained;
use objc2::runtime::{NSObject, NSObjectProtocol, ProtocolObject};
use objc2::{
    DefinedClass, MainThreadMarker, MainThreadOnly, Message, define_class, msg_send, msg_send_id,
};
use objc2_foundation::{
    NSArray, NSBundle, NSDictionary, NSMutableDictionary, NSObjectNSKeyValueCoding, NSString,
    NSUserNotification, NSUserNotificationAction, NSUserNotificationActivationType,
    NSUserNotificationCenter, NSUserNotificationCenterDelegate, NSUserNotificationDefaultSoundName,
    ns_string,
};
use std::collections::HashMap;
use uuid::Uuid;

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

pub fn post_notification(notification: Notification, cx: &mut App) -> Box<dyn PostedNotification> {
    cx.update_global::<AppleNotificationGlobal, _>(|apple_notification_global, cx| {
        unsafe {
            let bundle = NSBundle::mainBundle();
            if bundle.bundleIdentifier().is_none() {
                // We don't have a bundle so the notifications won't work.
                // Fail silently.
                return Box::new(MacPostedNotification::Failed);
            }

            let user_info = NSMutableDictionary::new();
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

            let mut actions = Vec::new();
            for action in notification.actions {
                let uuid = Uuid::new_v4();
                let on_triggered = action.on_triggered.clone();
                apple_notification_global.action_handlers.insert(
                    uuid,
                    Box::new(move |_, cx| {
                        on_triggered.clone()(&NotificationActionEvent, cx);
                    }),
                );
                actions.push(NSUserNotificationAction::actionWithIdentifier_title(
                    Some(&NSString::from_str(uuid.to_string().as_str())),
                    Some(&NSString::from_str(action.text.as_str())),
                ));
            }
            ns_notification
                .setAdditionalActions(Some(&NSArray::from_retained_slice(actions.as_slice())));

            if let Some(default_action) = notification.default_action {
                let uuid = Uuid::new_v4();
                let on_triggered = default_action.clone();
                apple_notification_global.action_handlers.insert(
                    uuid,
                    Box::new(move |_, cx| {
                        on_triggered.clone()(&NotificationActionEvent, cx);
                    }),
                );

                let uuid = NSString::from_str(uuid.to_string().as_str());
                user_info.setObject_forKey(
                    uuid.as_ref(),
                    ProtocolObject::from_ref(ns_string!("default_action_uuid")),
                );
                ns_notification.setHasActionButton(true);
            } else {
                ns_notification.setHasActionButton(false);
            }

            if !notification.on_reply_action.is_empty() {
                let uuid = Uuid::new_v4();
                let on_triggered = notification.on_reply_action.clone();
                apple_notification_global.action_handlers.insert(
                    uuid,
                    Box::new(move |reply, cx| {
                        let event = &NotificationReplyActionEvent {
                            text: reply.unwrap_or_default(),
                        };

                        for action in on_triggered.clone() {
                            action(&event, cx);
                        }
                    }),
                );

                let uuid = NSString::from_str(uuid.to_string().as_str());
                user_info.setObject_forKey(
                    uuid.as_ref(),
                    ProtocolObject::from_ref(ns_string!("reply_action_uuid")),
                );

                ns_notification.setHasReplyButton(true);
            } else {
                ns_notification.setHasReplyButton(false);
            }

            ns_notification.setUserInfo(Some(&user_info));
            NSUserNotificationCenter::defaultUserNotificationCenter()
                .deliverNotification(&ns_notification);

            Box::new(MacPostedNotification::Posted {
                ns_notification,
                summary: notification.summary,
                body: notification.body,
            })
        }
    })
}

struct AppNotificationDelegateFields {
    tx: Sender<NotificationActionActivation>,
}

// Define the delegate class
define_class!(
    #[unsafe(super(NSObject))]
    #[thread_kind = MainThreadOnly]
    #[ivars = AppNotificationDelegateFields]
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
            match notification.activationType() {
                NSUserNotificationActivationType::ContentsClicked | NSUserNotificationActivationType::ActionButtonClicked => {
                    let Some(user_info) = notification.userInfo() else {
                        return;
                    };

                    let Some(default_action_uuid) = user_info.valueForKey(ns_string!("default_action_uuid")) else {
                        return;
                    };

                    let Ok(default_action_uuid) = default_action_uuid.downcast::<NSString>() else {
                        return;
                    };

                    if let Ok(trigger_uuid) = Uuid::try_parse(default_action_uuid.to_string().as_str()) {
                        _ = smol::block_on(self.ivars().tx.send(NotificationActionActivation::Trigger(trigger_uuid)));
                    }
                }
                NSUserNotificationActivationType::AdditionalActionClicked => {
                    let activated = notification.additionalActivationAction().unwrap();
                    if let Ok(trigger_uuid) = Uuid::try_parse(activated.identifier().unwrap().to_string().as_str()) {
                        _ = smol::block_on(self.ivars().tx.send(NotificationActionActivation::Trigger(trigger_uuid)));
                    }
                }
                NSUserNotificationActivationType::Replied => {
                    let Some(reply) = notification.response() else {
                        return;
                    };
                    let reply = reply.string().to_string();

                    let Some(user_info) = notification.userInfo() else {
                        return;
                    };

                    let Some(reply_action_uuid) = user_info.valueForKey(ns_string!("reply_action_uuid")) else {
                        return;
                    };

                    let Ok(reply_action_uuid) = reply_action_uuid.downcast::<NSString>() else {
                        return;
                    };

                    if let Ok(trigger_uuid) = Uuid::try_parse(reply_action_uuid.to_string().as_str()) {
                        _ = smol::block_on(self.ivars().tx.send(NotificationActionActivation::TriggerReply(trigger_uuid, reply)));
                    }
                }
                _ => {}
            }
        }
    }

    unsafe impl NSObjectProtocol for AppNotificationDelegate {}

    unsafe impl NSUserNotificationCenterDelegate for AppNotificationDelegate {}
);

enum NotificationActionActivation {
    Trigger(Uuid),
    TriggerReply(Uuid, String),
}

impl AppNotificationDelegate {
    fn new(tx: Sender<NotificationActionActivation>) -> Retained<Self> {
        let this = Self::alloc(MainThreadMarker::new().unwrap())
            .set_ivars(AppNotificationDelegateFields { tx });
        unsafe { msg_send![super(this), init] }
    }
}

struct AppleNotificationGlobal {
    globalable_item: Retained<AppNotificationDelegate>,
    action_handlers: HashMap<Uuid, Box<dyn Fn(Option<String>, &mut App) + 'static>>,
}

impl Global for AppleNotificationGlobal {}

pub fn setup_apple_notifications(cx: &mut App) {
    let (tx, rx) = async_channel::bounded(3);

    let app_notification_delegate = unsafe {
        let bundle = NSBundle::mainBundle();
        if bundle.bundleIdentifier().is_none() {
            // We don't have a bundle so the notifications won't work.
            // Fail silently.
            return;
        }

        let app_notification_delegate = AppNotificationDelegate::new(tx);
        let delegate: &ProtocolObject<dyn NSUserNotificationCenterDelegate> =
            ProtocolObject::from_ref(&*app_notification_delegate);
        NSUserNotificationCenter::defaultUserNotificationCenter().setDelegate(Some(delegate));

        app_notification_delegate
    };

    cx.set_global(AppleNotificationGlobal {
        globalable_item: app_notification_delegate,
        action_handlers: HashMap::new(),
    });

    cx.spawn(async move |cx: &mut AsyncApp| {
        loop {
            let Ok(trigger) = rx.recv().await else {
                return;
            };

            if cx
                .update_global::<AppleNotificationGlobal, _>(|apple_notification_global, cx| {
                    match trigger {
                        NotificationActionActivation::Trigger(uuid) => {
                            if let Some(handler) =
                                apple_notification_global.action_handlers.get(&uuid)
                            {
                                handler(None, cx)
                            }
                        }
                        NotificationActionActivation::TriggerReply(uuid, reply) => {
                            if let Some(handler) =
                                apple_notification_global.action_handlers.get(&uuid)
                            {
                                handler(Some(reply), cx)
                            }
                        }
                    }
                })
                .is_err()
            {
                return;
            };
        }
    })
    .detach();
}
