use gpui::private::anyhow;
use std::fs;
use std::fs::Permissions;
use std::os::unix::fs::PermissionsExt;
use std::path::PathBuf;

pub fn perform_appimage_self_update(package_path: &PathBuf) -> Result<(), anyhow::Error> {
    let Ok(app_path) = std::env::var("APPIMAGE").map(PathBuf::from) else {
        return Err(anyhow::anyhow!(
            "Attempted to update on Linux but not running from an AppImage"
        ));
    };

    fs::remove_file(&app_path)?;
    fs::copy(package_path, &app_path)?;
    fs::set_permissions(&app_path, Permissions::from_mode(0o755))?;

    Ok(())
}
