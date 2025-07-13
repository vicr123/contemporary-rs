use crate::platform_support::platform_settings::PlatformSettings;
use crate::styling::theme::Theme;
use crate::styling::theme::ThemeType::System;
use async_channel::Sender;
use gpui::{App, AsyncApp, Global};
use objc2::__framework_prelude::Retained;
use objc2::runtime::{NSObject, NSObjectProtocol};
use objc2::{DefinedClass, MainThreadMarker, MainThreadOnly, define_class, msg_send, sel};
use objc2_app_kit::{NSWorkspace, NSWorkspaceAccessibilityDisplayOptionsDidChangeNotification};
use objc2_foundation::{NSDistributedNotificationCenter, NSNotification, ns_string};

struct AppleWorkspaceA11yOptionsChangedListenerFields {
    tx: Sender<()>,
}

define_class!(
    #[unsafe(super(NSObject))]
    #[thread_kind = MainThreadOnly]

    #[ivars = AppleWorkspaceA11yOptionsChangedListenerFields]
    struct AppleWorkspaceA11yOptionsChangedListener;

    impl AppleWorkspaceA11yOptionsChangedListener {
        #[unsafe(method(workspaceAccessibilityOptionsDidChange:))]
        fn __user_defaults_did_change(&self, _: &NSNotification) -> u8 {
            _ = smol::block_on(self.ivars().tx.send(()));
            0
        }
    }

    unsafe impl NSObjectProtocol for AppleWorkspaceA11yOptionsChangedListener {}
);

impl AppleWorkspaceA11yOptionsChangedListener {
    fn new(tx: Sender<()>) -> Retained<Self> {
        let this = Self::alloc(MainThreadMarker::new().unwrap())
            .set_ivars(AppleWorkspaceA11yOptionsChangedListenerFields { tx });
        unsafe { msg_send![super(this), init] }
    }
}

struct AppleWorkspaceA11yOptionsChangedListenerGlobalableWrapperThing {
    globalable_item: Retained<AppleWorkspaceA11yOptionsChangedListener>,
}

impl Global for AppleWorkspaceA11yOptionsChangedListenerGlobalableWrapperThing {}

pub fn setup_apple_workspace_a11y_options_changed_listener(cx: &mut App) {
    let (tx, rx) = async_channel::bounded(3);

    unsafe {
        cx.set_global(
            AppleWorkspaceA11yOptionsChangedListenerGlobalableWrapperThing {
                globalable_item: AppleWorkspaceA11yOptionsChangedListener::new(tx),
            },
        );

        let listener_wrapper =
            cx.global::<AppleWorkspaceA11yOptionsChangedListenerGlobalableWrapperThing>();

        let workspace_notification_center = NSWorkspace::sharedWorkspace().notificationCenter();

        workspace_notification_center.addObserver_selector_name_object(
            &listener_wrapper.globalable_item,
            sel!(workspaceAccessibilityOptionsDidChange:),
            Some(NSWorkspaceAccessibilityDisplayOptionsDidChangeNotification),
            None,
        )
    }

    cx.spawn(async move |cx: &mut AsyncApp| {
        loop {
            _ = rx.recv().await;

            _ = cx.update_global::<PlatformSettings, ()>(|platform_settings, cx| {
                platform_settings.reload()
            });

            _ = cx.refresh();
        }
    })
    .detach()
}
