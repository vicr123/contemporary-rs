use crate::icon::get_svg_icon_contents;
use crate::tool_setup::ToolSetup;
use contemporary_config::{ContemporaryConfig, LocalisedString};
use resvg::render;
use resvg::tiny_skia::{Pixmap, Transform};
use resvg::usvg::{Options, Tree};
use std::collections::HashMap;
use std::fmt::{Error, Write};
use std::fs::{copy, create_dir_all, remove_dir_all, write};
use std::os::unix::fs::symlink;
use std::path::{Path, PathBuf};
use std::process::exit;
use tracing::error;

pub fn deploy_linux(setup_data: &ToolSetup, executable_path: HashMap<String, PathBuf>) {
    let target_triple = setup_data.targets.first().unwrap();
    let executable_path = executable_path.get(target_triple).unwrap();

    let deployment = setup_data.contemporary_config.deployment(&target_triple);

    let Some(desktop_entry) = deployment.desktop_entry else {
        error!("No desktop entry specified in config");
        exit(1);
    };

    let desktop_entry_with_desktop_extension = desktop_entry.clone() + ".desktop";
    let desktop_entry_with_svg_extension = desktop_entry.clone() + ".svg";

    let Ok(_) = create_dir_all(&setup_data.output_directory) else {
        error!("Failed to create output directory");
        exit(1);
    };

    let appdir_root = setup_data.output_directory.join("appdir");
    if appdir_root.exists() {
        let Ok(_) = remove_dir_all(&appdir_root) else {
            error!("Failed to remove existing appdir");
            exit(1);
        };
    }

    let appdir_usr = appdir_root.join("usr");
    let appdir_bin = appdir_usr.join("bin");
    let Ok(_) = create_dir_all(&appdir_bin) else {
        error!("Failed to create appdir bin folder");
        exit(1);
    };

    let Ok(_) = copy(
        &executable_path,
        appdir_bin.join(executable_path.file_name().unwrap()),
    ) else {
        error!("Failed to copy executable to bin directory");
        exit(1);
    };

    let appdir_share = appdir_usr.join("share");
    let appdir_share_applications = appdir_share.join("applications");
    let Ok(_) = create_dir_all(&appdir_share_applications) else {
        error!("Failed to create appdir applications folder");
        exit(1);
    };

    let appdir_scalable_app_icons = appdir_share
        .join("icons")
        .join("hicolor")
        .join("scalable")
        .join("apps");
    let Ok(_) = create_dir_all(&appdir_scalable_app_icons) else {
        error!("Failed to create appdir icons folder");
        exit(1);
    };

    let apprun_path = appdir_root.join("AppRun");
    let Ok(_) = symlink(
        PathBuf::from("usr/bin").join(executable_path.file_name().unwrap()),
        apprun_path,
    ) else {
        error!("Failed to create AppRun symlink");
        exit(1);
    };

    let Ok(desktop_entry_contents) = generate_desktop_entry(
        &target_triple,
        executable_path,
        &setup_data.contemporary_config,
    ) else {
        error!("Failed to generate desktop entry");
        exit(1);
    };

    let desktop_entry_path = appdir_share_applications.join(&desktop_entry_with_desktop_extension);
    let Ok(_) = write(&desktop_entry_path, desktop_entry_contents) else {
        error!("Failed to write desktop entry");
        exit(1);
    };

    let root_desktop_entry_path = appdir_root.join(&desktop_entry_with_desktop_extension);
    let Ok(_) = symlink(
        PathBuf::from("usr/share/applications").join(&desktop_entry_with_desktop_extension),
        root_desktop_entry_path,
    ) else {
        error!("Failed to create desktop entry symlink");
        exit(1);
    };

    let icon_svg = get_svg_icon_contents(
        target_triple,
        &setup_data.base_path,
        &setup_data.contemporary_config,
    );
    let Ok(_) = write(
        appdir_scalable_app_icons.join(&desktop_entry_with_svg_extension),
        &icon_svg,
    ) else {
        error!("Failed to write SVG icon");
        exit(1);
    };

    let diricon_path = appdir_root.join(".DirIcon");
    {
        let opt = Options::default();
        let tree =
            Tree::from_data(icon_svg.as_bytes(), &opt).expect("Could not interpret built SVG data");
        let mut pixmap = Pixmap::new(256, 256).expect("Could not create pixmap to hold PNG");
        render(
            &tree,
            Transform::from_scale(256. / tree.size().width(), 256. / tree.size().height()),
            &mut pixmap.as_mut(),
        );
        pixmap.save_png(diricon_path).expect("Could not save PNG");
    }

    let root_icon_path = appdir_root.join(&desktop_entry_with_svg_extension);
    let Ok(_) = symlink(
        PathBuf::from("usr/share/icons/hicolor/scalable/apps")
            .join(&desktop_entry_with_svg_extension),
        root_icon_path,
    ) else {
        error!("Failed to create icon symlink");
        exit(1);
    };
}

fn generate_desktop_entry(
    target_triple: &str,
    executable_path: &Path,
    contemporary_config: &ContemporaryConfig,
) -> Result<String, Error> {
    let deployment = contemporary_config.deployment(target_triple);

    let Some(application_name) = deployment.application_name else {
        error!("No application name specified in config");
        exit(1);
    };

    let Some(desktop_entry) = deployment.desktop_entry else {
        error!("No desktop entry specified in config");
        exit(1);
    };

    let Some(desktop_entry_categories) = deployment.desktop_entry_categories else {
        error!("No desktop entry categories specified in config");
        exit(1);
    };

    let mut entry = DesktopEntry::new();
    entry.push_line_invariant("Type", "Application")?;
    entry.push_line_invariant("Version", "1.0")?;
    entry.push_line_invariant(
        "Exec",
        executable_path.file_name().unwrap().to_str().unwrap(),
    )?;
    entry.push_line_invariant("Icon", &desktop_entry)?;
    entry.push_line("Name", &application_name)?;

    if let Some(generic_name) = deployment.application_generic_name {
        entry.push_line("GenericName", &generic_name)?;
    }

    entry.push_line_invariant("Categories", &(desktop_entry_categories.join(";") + ";"))?;

    Ok(entry.contents)
}

struct DesktopEntry {
    pub contents: String,
}

impl DesktopEntry {
    fn new() -> Self {
        Self {
            contents: "#!/usr/bin/env xdg-open\n[Desktop Entry]\n".to_string(),
        }
    }

    fn push_line(&mut self, key: &str, value: &LocalisedString) -> Result<(), Error> {
        match value {
            LocalisedString::Hardcoded(value) => {
                writeln!(&mut self.contents, "{key}={value}")?;
            }
            LocalisedString::Localised(languages) => {
                self.push_line_invariant(key, &value.default_value())?;
                for (language, value) in languages {
                    writeln!(&mut self.contents, "{key}[{language}]={value}")?;
                }
            }
        }
        Ok(())
    }

    fn push_line_invariant(&mut self, key: &str, value: &str) -> Result<(), Error> {
        self.push_line(key, &LocalisedString::Hardcoded(value.into()))
    }
}
