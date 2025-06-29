use crate::macos::disk_image::DiskImage;
use crate::macos::ds_store::{DSStore, DSStoreEntry};
use crate::tool_setup::ToolSetup;
use image::{ImageFormat, ImageReader};
use resvg::render;
use resvg::tiny_skia::{Pixmap, Rect};
use resvg::usvg::{Options, Transform, Tree};
use std::fs::{copy, create_dir_all, read_dir, remove_dir_all, remove_file, write, OpenOptions};
use std::io;
use std::io::Read;
use std::os::unix::fs::symlink;
use std::path::Path;
use std::process::exit;
use tempfile::TempDir;
use tracing::{debug, error, info};

pub fn deploy_macos(setup_data: &ToolSetup, output_file: &str) {
    let deployment = setup_data
        .contemporary_config
        .deployment(setup_data.targets.first().unwrap());

    let Some(application_name) = deployment.application_name else {
        error!("No application name specified in config");
        exit(1);
    };

    let Some(disk_image_background) = deployment.disk_image_background else {
        error!("No disk image background specified in config");
        exit(1);
    };

    let app_root = setup_data
        .output_directory
        .join(application_name.default_value())
        .with_extension("app");
    if !app_root.exists() {
        error!("Application bundle does not exist. Please deploy first.");
        exit(1);
    };

    let Ok(mut disk_image_background_file) =
        OpenOptions::new().read(true).open(disk_image_background)
    else {
        error!("Unable to read disk image background file");
        exit(1);
    };

    let mut disk_image_background_contents = Vec::new();
    let Ok(_) = disk_image_background_file.read_to_end(&mut disk_image_background_contents) else {
        error!("Unable to read disk image background file");
        exit(1);
    };

    let opt = Options::default();
    let tree = Tree::from_data(&disk_image_background_contents, &opt)
        .expect("Could not interpret built SVG data");

    let Some(applications_node) = tree.node_by_id("applications") else {
        error!("Disk image background does not contain an element with ID #applications");
        exit(1);
    };

    let Some(app_node) = tree.node_by_id("app") else {
        error!("Disk image background does not contain an element with ID #app");
        exit(1);
    };

    let temp_dir = TempDir::new().expect("Failed to create temporary directory");
    let Ok(editable_disk_image) = DiskImage::new(
        52428800,
        &application_name.default_value(),
        temp_dir.path().join("testdmg").with_extension("dmg"),
        "HFS+",
    ) else {
        error!("Failed to create temporary disk image");
        exit(1);
    };

    let Ok(editable_disk_image_mount) = editable_disk_image.mount() else {
        error!("Failed to mount temporary disk image");
        exit(1);
    };

    let editable_disk_image_background_directory =
        editable_disk_image_mount.mount_point.join(".background");
    let Ok(_) = create_dir_all(&editable_disk_image_background_directory) else {
        error!("Failed to create .background directory in temporary disk image");
        exit(1);
    };

    let mut pixmap = Pixmap::new(tree.size().width() as u32, tree.size().height() as u32)
        .expect("Could not create pixmap to hold PNG");
    render(
        &tree,
        Transform::from_scale(
            pixmap.width() as f32 / tree.size().width(),
            pixmap.height() as f32 / tree.size().height(),
        ),
        &mut pixmap.as_mut(),
    );

    let temp_background_file = temp_dir.path().join("background.png");
    pixmap
        .save_png(&temp_background_file)
        .expect("Could not save PNG");

    // Convert to TIFF
    let image = ImageReader::open(temp_background_file)
        .unwrap()
        .decode()
        .unwrap();
    let Ok(_) = image.save_with_format(
        editable_disk_image_background_directory.join("background.tiff"),
        ImageFormat::Tiff,
    ) else {
        error!("Failed to convert background PNG to TIFF");
        exit(1);
    };

    // Copy the application bundle
    let Ok(_) = copy_dir_all(
        &app_root,
        editable_disk_image_mount
            .mount_point
            .join(app_root.file_name().unwrap()),
    ) else {
        error!("Failed to copy application bundle to temporary disk image");
        exit(1);
    };

    // Create link to Applications folder
    let Ok(_) = symlink(
        "/Applications",
        editable_disk_image_mount.mount_point.join("Applications"),
    ) else {
        error!("Failed to create link to Applications folder");
        exit(1);
    };

    let mut ds_store = DSStore::new();

    // Set the window geometry
    ds_store.push_entry(DSStoreEntry::new_bwsp(
        ".",
        100,
        100,
        tree.size().width() as i32,
        tree.size().height() as i32,
    ));

    // Set the window properties
    ds_store.push_entry(DSStoreEntry::new_icvp(".", 48, vec![]));

    ds_store.push_entry(DSStoreEntry::new_v_srn(".", 1));

    // Move the file icons
    let applications_center = center_of_rect(&applications_node.bounding_box());
    ds_store.push_entry(DSStoreEntry::new_iloc(
        "Applications",
        applications_center.0,
        applications_center.1,
    ));
    let app_center = center_of_rect(&app_node.bounding_box());
    ds_store.push_entry(DSStoreEntry::new_iloc(
        app_root.file_name().unwrap().to_str().unwrap(),
        app_center.0,
        app_center.1,
    ));
    debug!(
        "Applications Icon: {}, {}",
        applications_center.0, applications_center.1
    );
    debug!("App Icon: {}, {}", app_center.0, app_center.1);

    let Ok(_) = write(
        editable_disk_image_mount.mount_point.join(".DS_Store"),
        ds_store.get_bytes(),
    ) else {
        error!("Failed to write .DS_Store file");
        exit(1);
    };

    drop(editable_disk_image_mount);

    let output_file = Path::new(output_file);

    if output_file.exists() {
        remove_file(output_file).unwrap();
    }

    let Ok(_) = editable_disk_image.convert(output_file.into(), "UDZO") else {
        error!("Failed to convert temporary disk image");
        exit(1);
    };

    info!("Disk Image created successfully");
}

fn center_of_rect(rect: &Rect) -> (i32, i32) {
    (
        rect.x() as i32 + rect.width() as i32 / 2,
        rect.y() as i32 + rect.height() as i32 / 2,
    )
}

fn copy_dir_all(src: impl AsRef<Path>, dst: impl AsRef<Path>) -> io::Result<()> {
    create_dir_all(&dst)?;
    for entry in read_dir(src)? {
        let entry = entry?;
        let ty = entry.file_type()?;
        if ty.is_dir() {
            copy_dir_all(entry.path(), dst.as_ref().join(entry.file_name()))?;
        } else {
            copy(entry.path(), dst.as_ref().join(entry.file_name()))?;
        }
    }
    Ok(())
}
