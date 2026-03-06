use gpui::{App, Window};

pub struct Permissions {}

#[derive(Copy, Clone, PartialEq)]
pub enum PermissionType {
    Microphone,
    Camera,
}

#[derive(Copy, Clone, PartialEq)]
pub enum GrantStatus {
    Granted,
    Denied,
    NotDetermined,

    // Permission checking is not supported, but the permission may still be tried
    PlatformUnsupported,
}

pub struct PermissionRequestCompleteEvent {
    pub grant_status: GrantStatus,
}

impl Permissions {
    pub fn grant_status(permission: PermissionType) -> GrantStatus {
        #[cfg(target_os = "macos")]
        {
            return crate::platform_support::macos::permissions::grant_status(permission);
        }

        GrantStatus::PlatformUnsupported
    }

    pub fn request_permission(
        permission: PermissionType,
        callback: impl FnOnce(&PermissionRequestCompleteEvent, &mut Window, &mut App) + 'static,
        window: &mut Window,
        cx: &mut App,
    ) {
        #[cfg(target_os = "macos")]
        {
            return crate::platform_support::macos::permissions::request_permission(
                permission, callback, window, cx,
            );
        }

        callback(
            &PermissionRequestCompleteEvent {
                grant_status: GrantStatus::PlatformUnsupported,
            },
            window,
            cx,
        );
    }
}
