use crate::styling::theme::Theme;
use crate::styling::theme::ThemeType::System;
use async_channel::Sender;
use gpui::{App, AsyncApp, Global};
use objc2::__framework_prelude::Retained;
use objc2::runtime::{NSObject, NSObjectProtocol};
use objc2::{DefinedClass, MainThreadMarker, MainThreadOnly, define_class, msg_send, sel};
use objc2_foundation::{NSDistributedNotificationCenter, NSNotification, ns_string};

struct AppleInterfaceThemeChangedListenerFields {
    tx: Sender<()>,
}

define_class!(
    #[unsafe(super(NSObject))]
    #[thread_kind = MainThreadOnly]

    #[ivars = AppleInterfaceThemeChangedListenerFields]
    struct AppleInterfaceThemeChangedListener;

    impl AppleInterfaceThemeChangedListener {
        #[unsafe(method(userDefaultsDidChange:))]
        fn __user_defaults_did_change(&self, _: &NSNotification) -> u8 {
            _ = smol::block_on(self.ivars().tx.send(()));
            0
        }
    }

    unsafe impl NSObjectProtocol for AppleInterfaceThemeChangedListener {}
);

impl AppleInterfaceThemeChangedListener {
    fn new(tx: Sender<()>) -> Retained<Self> {
        let this = Self::alloc(MainThreadMarker::new().unwrap())
            .set_ivars(AppleInterfaceThemeChangedListenerFields { tx });
        unsafe { msg_send![super(this), init] }
    }
}

struct AppleInterfaceThemeChangedListenerGlobalableWrapperThing {
    globalable_item: Retained<AppleInterfaceThemeChangedListener>,
}

impl Global for AppleInterfaceThemeChangedListenerGlobalableWrapperThing {}

pub fn setup_apple_interface_theme_changed_listener(cx: &mut App) {
    let (tx, rx) = async_channel::bounded(3);

    unsafe {
        cx.set_global(AppleInterfaceThemeChangedListenerGlobalableWrapperThing {
            globalable_item: AppleInterfaceThemeChangedListener::new(tx),
        });

        let listener_wrapper =
            cx.global::<AppleInterfaceThemeChangedListenerGlobalableWrapperThing>();

        let distributed_notification_center = NSDistributedNotificationCenter::defaultCenter();
        distributed_notification_center.addObserver_selector_name_object(
            &listener_wrapper.globalable_item,
            sel!(userDefaultsDidChange:),
            Some(ns_string!("AppleInterfaceThemeChangedNotification")),
            None,
        )
    }

    cx.spawn(async move |cx: &mut AsyncApp| {
        loop {
            _ = rx.recv().await;

            _ = cx.update_global::<Theme, ()>(|theme, cx| {
                if theme.theme_type == System {
                    theme.set_theme(Theme::default());
                }
            });

            _ = cx.refresh();
        }
    })
    .detach()
}
