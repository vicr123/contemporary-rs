use crate::permissions::{GrantStatus, PermissionType};
use block2::RcBlock;
use objc2::runtime::Bool;
use objc2_av_foundation::{
    AVAuthorizationStatus, AVCaptureDevice, AVMediaTypeAudio, AVMediaTypeVideo,
};
use std::cell::Cell;

pub fn grant_status(permission_type: PermissionType) -> GrantStatus {
    match permission_type {
        PermissionType::Microphone | PermissionType::Camera => grant_status_av(permission_type),
        _ => GrantStatus::PlatformUnsupported,
    }
}

fn grant_status_av(permission_type: PermissionType) -> GrantStatus {
    let av_permission_type = match permission_type {
        PermissionType::Microphone => unsafe { AVMediaTypeAudio.unwrap() },
        PermissionType::Camera => unsafe { AVMediaTypeVideo.unwrap() },
        _ => panic!("Called grant_status_av with unsupported permission type"),
    };

    let result = unsafe { AVCaptureDevice::authorizationStatusForMediaType(av_permission_type) };

    match result {
        AVAuthorizationStatus::Authorized => GrantStatus::Granted,
        AVAuthorizationStatus::Denied => GrantStatus::Denied,
        AVAuthorizationStatus::NotDetermined => GrantStatus::NotDetermined,
        AVAuthorizationStatus::Restricted => GrantStatus::Denied,
        _ => GrantStatus::PlatformUnsupported,
    }
}

pub fn request_permission(
    permission_type: PermissionType,
    on_complete: impl FnOnce(bool) + 'static,
) {
    match permission_type {
        PermissionType::Microphone | PermissionType::Camera => {
            request_permission_av(permission_type, on_complete)
        }
        _ => on_complete(false),
    }
}

fn request_permission_av(
    permission_type: PermissionType,
    on_complete: impl FnOnce(bool) + 'static,
) {
    let av_permission_type = match permission_type {
        PermissionType::Microphone => unsafe { AVMediaTypeAudio.unwrap() },
        PermissionType::Camera => unsafe { AVMediaTypeVideo.unwrap() },
        _ => panic!("Called grant_status_av with unsupported permission type"),
    };

    let on_complete = Cell::new(Some(on_complete));

    unsafe {
        AVCaptureDevice::requestAccessForMediaType_completionHandler(
            av_permission_type,
            &RcBlock::new(move |granted: Bool| {
                let on_complete = on_complete.take().expect("on_complete already called");
                on_complete(granted.into());
            }),
        )
    }
}
