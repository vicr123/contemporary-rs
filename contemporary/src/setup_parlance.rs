use crate::tokio::tokio_helper::TokioHelper;
use cntp_i18n_parlance_source::install_cntp_i18n_parlance_source;
use gpui::{App, AsyncApp};
use tracing::error;
use url::Url;

pub fn setup_parlance_i18n(
    base_url: Url,
    project: String,
    subproject: String,
    crate_name: String,
    cx: &mut App,
) {
    cx.spawn(async move |cx: &mut AsyncApp| {
        if let Err(e) = cx
            .spawn_tokio(async move {
                install_cntp_i18n_parlance_source(base_url, project, subproject, crate_name).await
            })
            .await
        {
            error!("Unable to set up Parlance translation source: {:?}", e);
        }
    })
    .detach();
}

pub fn setup_parlance_i18n_if_enabled(
    base_url: Url,
    project: String,
    subproject: String,
    crate_name: String,
    cx: &mut App,
) {
    if std::env::var("CNTP_PARLANCE_ENABLED").is_ok() {
        setup_parlance_i18n(base_url, project, subproject, crate_name, cx);
    }
}
