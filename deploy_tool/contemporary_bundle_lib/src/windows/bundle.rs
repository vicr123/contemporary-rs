use crate::icon::get_svg_icon_contents;
use crate::tool_setup::ToolSetup;
use crate::windows::group_icon::{GroupIcon, GroupIconEntry};
use crate::windows::icon::Icon;
use resvg::render;
use resvg::tiny_skia::{Pixmap, Transform};
use resvg::usvg::{Options, Tree};
use std::collections::HashMap;
use std::env::consts::EXE_EXTENSION;
use std::fs::{copy, create_dir_all};
use std::path::{Path, PathBuf};
use std::process::exit;
use tracing::error;
use winres_edit::{Id, Resource, Resources, resource_type};

pub fn bundle_windows(setup_data: &ToolSetup, executable_path: HashMap<String, PathBuf>) {
    let target_triple = setup_data.targets.first().unwrap();
    let executable_path = executable_path.get(target_triple).unwrap();

    let deployment = setup_data
        .contemporary_config
        .deployment(setup_data.targets.first().unwrap());

    let Ok(_) = create_dir_all(&setup_data.output_directory) else {
        error!("Failed to create output directory");
        exit(1);
    };

    let Some(application_name) = deployment.application_name else {
        error!("No application name specified in config");
        exit(1);
    };

    let output_executable = setup_data
        .output_directory
        .join(application_name.default_value())
        .with_extension(EXE_EXTENSION);
    if copy(executable_path, &output_executable).is_err() {
        error!("Failed to copy executable");
        exit(1);
    };

    let mut resources = Resources::new(&*output_executable);
    resources.load().unwrap();
    resources.open().unwrap();

    let icon_svg = get_svg_icon_contents(
        target_triple,
        &setup_data.base_path,
        &setup_data.contemporary_config,
    );
    let opt = Options::default();
    let tree =
        Tree::from_data(icon_svg.as_bytes(), &opt).expect("Could not interpret built SVG data");

    let mut group_icon = GroupIcon::default();

    for size in [16, 24, 32, 48, 64, 96, 128, 256, 512] {
        let mut pixmap = Pixmap::new(size, size).expect("Could not create pixmap to hold PNG");
        render(
            &tree,
            Transform::from_scale(
                size as f32 / tree.size().width(),
                size as f32 / tree.size().height(),
            ),
            &mut pixmap.as_mut(),
        );

        let icon_rgba_image = Icon::new_from_rgba(size, size, size as u16, pixmap.data().into());
        let Ok(encoded_icon) = icon_rgba_image.encode() else {
            error!("Failed to encode icon");
            exit(1);
        };

        group_icon.push_icon(icon_rgba_image.group_icon_entry().unwrap());

        let res = Resource::new(
            &resources,
            resource_type::ICON.into(),
            Id::Integer(icon_rgba_image.icon_id).into(),
            1033,
            encoded_icon.as_ref(),
        );
        if res.update().is_err() {
            error!("Failed to update icon resource");
            exit(1);
        };
    }

    let group_res = Resource::new(
        &resources,
        Id::Integer(14).into(),
        Id::Integer(1).into(),
        1033,
        &group_icon.encode().unwrap(),
    );
    if group_res.update().is_err() {
        error!("Failed to update group icon resource");
        exit(1);
    };

    resources.close();
}
