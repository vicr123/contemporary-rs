pub mod bin_chicken_client;

#[cfg(target_os = "linux")]
pub mod appimage;

#[cfg(target_os = "macos")]
mod macos;

use crate::application::Details;
use crate::self_update::bin_chicken_client::BinChickenClient;
use gpui::private::anyhow;
use gpui::private::anyhow::Error;
use gpui::{App, AsyncApp, Global};
use minisign_verify::PublicKey;
use std::path::PathBuf;
use std::time::Duration;
use tracing::{error, info};
use url::Url;

#[derive(Debug, Clone)]
pub struct UpdateInformation {
    pub artifact_number: u64,
    pub artifact_version: Option<String>,
}

pub enum SelfUpdateState {
    Idle,
    UpdateAvailableBackground {
        new_update: UpdateInformation,
    },
    UpdateDownloaded {
        new_update: UpdateInformation,
    },
    UpdateDownloadedInBackground {
        new_update: UpdateInformation,
    },
    UpdateInstalling {
        new_update: UpdateInformation,
    },
    RestartToUpdate {
        new_update: UpdateInformation,
    },
    InstallStepFailed {
        new_update: UpdateInformation,
        error: Error,
    },
}

pub enum VisibleSelfUpdateState<'a> {
    UpdateDownloaded {
        new_update: &'a UpdateInformation,
    },
    UpdateInstalling {
        new_update: &'a UpdateInformation,
    },
    RestartToUpdate {
        new_update: &'a UpdateInformation,
    },
    InstallStepFailed {
        new_update: &'a UpdateInformation,
        error: &'a Error,
    },
}

impl SelfUpdateState {
    pub fn is_visible(&self) -> bool {
        self.to_visible().is_some()
    }

    pub fn to_visible(&'_ self) -> Option<VisibleSelfUpdateState<'_>> {
        match self {
            SelfUpdateState::UpdateDownloaded { new_update } => {
                Some(VisibleSelfUpdateState::UpdateDownloaded { new_update })
            }
            SelfUpdateState::UpdateInstalling { new_update } => {
                Some(VisibleSelfUpdateState::UpdateInstalling { new_update })
            }
            SelfUpdateState::RestartToUpdate { new_update } => {
                Some(VisibleSelfUpdateState::RestartToUpdate { new_update })
            }
            SelfUpdateState::InstallStepFailed { new_update, error } => {
                Some(VisibleSelfUpdateState::InstallStepFailed { new_update, error })
            }
            _ => None,
        }
    }
}

pub struct SelfUpdate {
    bin_chicken_url: Url,
    repository: &'static str,
    our_uuid: &'static str,
    signature_public_key: PublicKey,

    state: SelfUpdateState,
}

impl Global for SelfUpdate {}

impl SelfUpdate {
    pub fn check_for_updates(&mut self, cx: &mut App) {
        let Some(bin_chicken_client) = self.create_bin_chicken_client(cx) else {
            return;
        };

        // TODO: Don't download updates on a metered connection or low data mode

        cx.spawn(async move |cx: &mut AsyncApp| {
            match bin_chicken_client.check_for_updates().await {
                Ok(Some(update_information)) => {
                    let _ = cx.update_global::<SelfUpdate, _>(|this, cx| {
                        this.state = SelfUpdateState::UpdateAvailableBackground {
                            new_update: update_information,
                        };
                        this.start_update_download(cx);
                    });
                }
                Ok(None) => {
                    let _ = cx.update_global::<SelfUpdate, _>(|this, cx| {
                        this.state = SelfUpdateState::Idle
                    });
                }
                Err(e) => {
                    error!("Unable to check for updates: {e}")
                }
            }
        })
        .detach();
    }

    pub fn start_update_download(&mut self, cx: &mut App) {
        let Some(bin_chicken_client) = self.create_bin_chicken_client(cx) else {
            return;
        };

        let update_information = match &self.state {
            SelfUpdateState::UpdateAvailableBackground { new_update } => new_update,
            _ => return,
        }
        .clone();

        cx.spawn(async move |cx: &mut AsyncApp| {
            match bin_chicken_client
                .download_artifact(update_information.artifact_number)
                .await
            {
                Ok(_) => {
                    // TODO: If we can install in the background, do that
                    if self_update_type().supports_in_place_update() {
                        let _ = cx.update_global::<SelfUpdate, _>(|this, cx| {
                            this.state = SelfUpdateState::UpdateDownloadedInBackground {
                                new_update: update_information,
                            };
                            this.install_downloaded_update(cx);
                        });
                    } else {
                        let _ = cx.update_global::<SelfUpdate, _>(|this, cx| {
                            this.state = SelfUpdateState::UpdateDownloaded {
                                new_update: update_information,
                            }
                        });
                    }
                }
                Err(_) => {
                    // TODO
                }
            }
        })
        .detach();
    }

    pub fn install_downloaded_update(&mut self, cx: &mut App) {
        let Some(bin_chicken_client) = self.create_bin_chicken_client(cx) else {
            return;
        };

        let update_information = match &self.state {
            SelfUpdateState::UpdateDownloadedInBackground { new_update } => new_update,
            SelfUpdateState::UpdateDownloaded { new_update } => new_update,
            SelfUpdateState::InstallStepFailed { new_update, .. } => {
                // Pause for 2 seconds in Update Installing state because
                // the install is likely to fail again, and we need to give the user some feedback
                // if it does.
                self.state = SelfUpdateState::UpdateInstalling {
                    new_update: new_update.clone(),
                };

                cx.spawn(async move |cx: &mut AsyncApp| {
                    cx.background_executor().timer(Duration::from_secs(2)).await;
                    let _ = cx.update_global::<SelfUpdate, _>(|this, cx| {
                        this.install_downloaded_update(cx);
                    });
                })
                .detach();
                return;
            }
            SelfUpdateState::UpdateInstalling { new_update } => new_update,
            _ => return,
        }
        .clone();

        let artifact_path =
            bin_chicken_client.artifact_local_path(update_information.artifact_number);

        let update_result = match self_update_type() {
            #[cfg(target_os = "linux")]
            SelfUpdateType::AppImage => appimage::perform_appimage_self_update(&artifact_path),
            #[cfg(target_os = "macos")]
            SelfUpdateType::MacApplicationBundle => {
                macos::perform_macos_self_update(&artifact_path)
            }
            SelfUpdateType::NotSupported => Ok(()),
        };

        match update_result {
            Ok(_) => {
                self.state = SelfUpdateState::RestartToUpdate {
                    new_update: update_information,
                };
            }
            Err(e) => {
                error!("Unable to install update: {e}");
                self.state = SelfUpdateState::InstallStepFailed {
                    new_update: update_information,
                    error: e,
                };
            }
        }
    }

    pub fn restart_application_after_update(&mut self, cx: &mut App) {
        match &self.state {
            SelfUpdateState::RestartToUpdate { .. } => {}
            _ => return,
        };

        let update_result: Result<(), Error> = match self_update_type() {
            #[cfg(target_os = "linux")]
            SelfUpdateType::AppImage => appimage::perform_appimage_self_restart(),
            _ => Ok(()),
        };

        if update_result.is_ok() {
            cx.quit();
        }
    }

    fn create_bin_chicken_client(&self, cx: &App) -> Option<BinChickenClient> {
        let details = cx.global::<Details>();
        let standard_dirs = details.standard_dirs()?;

        let updates_working_directory = standard_dirs.data_dir().join("updates");

        let bin_chicken_client = BinChickenClient::new(
            updates_working_directory,
            self.bin_chicken_url.clone(),
            self.repository,
            self.our_uuid,
            self.signature_public_key.clone(),
        );

        Some(bin_chicken_client)
    }

    pub fn state(&self) -> &SelfUpdateState {
        &self.state
    }
}

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub enum SelfUpdateType {
    #[cfg(target_os = "linux")]
    AppImage,
    #[cfg(target_os = "macos")]
    MacApplicationBundle,
    NotSupported,
}

impl SelfUpdateType {
    pub fn supports_in_place_update(&self) -> bool {
        match self {
            #[cfg(target_os = "linux")]
            SelfUpdateType::AppImage => true,
            #[cfg(target_os = "macos")]
            SelfUpdateType::MacApplicationBundle => false,
            SelfUpdateType::NotSupported => false,
        }
    }
}

pub fn init_self_update(
    bin_chicken_url: Url,
    repository: &'static str,
    our_uuid: Option<&'static str>,
    signature_public_key: Option<&'static str>,
    cx: &mut App,
) {
    let Some(our_uuid) = our_uuid else {
        // UUID isn't available so just don't bother with self-update
        return;
    };
    let Some(signature_public_key) = signature_public_key else {
        // Signature public key isn't available so just don't bother with self-update
        return;
    };
    let Ok(signature_public_key) = PublicKey::from_base64(signature_public_key) else {
        info!("Self-update is disabled because the provided public key during build is invalid.");
        return;
    };
    if self_update_type() == SelfUpdateType::NotSupported {
        info!("Self-update is not supported on this platform or configuration");
        return;
    }
    if std::env::var("CNTP_DISABLE_SELF_UPDATE").is_ok_and(|x| x == "1") {
        info!("Self-update is disabled by CNTP_DISABLE_SELF_UPDATE=1");
        return;
    }

    cx.spawn(async move |cx: &mut AsyncApp| {
        loop {
            if cx
                .update_global::<SelfUpdate, _>(|self_update, cx| {
                    self_update.check_for_updates(cx);
                })
                .is_err()
            {
                return;
            }

            cx.background_executor()
                .timer(Duration::from_hours(1))
                .await;
        }
    })
    .detach();

    cx.set_global(SelfUpdate {
        bin_chicken_url,
        repository,
        our_uuid,
        signature_public_key,

        state: SelfUpdateState::Idle,
    });
}

pub fn self_update_type() -> SelfUpdateType {
    #[cfg(target_os = "linux")]
    if std::env::var("APPIMAGE").map(PathBuf::from).is_ok() {
        return SelfUpdateType::AppImage;
    }

    #[cfg(target_os = "macos")]
    if macos::can_macos_self_update() {
        return SelfUpdateType::MacApplicationBundle;
    }

    SelfUpdateType::NotSupported
}
