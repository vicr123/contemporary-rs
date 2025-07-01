use crate::macos::alias::Alias;
use crate::macos::disk_image::DiskImage;
use crate::macos::ds_store::{DSStore, DSStoreEntry};
use crate::tool_setup::ToolSetup;
use resvg::render;
use resvg::tiny_skia::{Pixmap, Rect};
use resvg::usvg::{Options, Transform, Tree};
use std::fs::{copy, create_dir_all, read_dir, remove_file, write, OpenOptions};
use std::io;
use std::io::Read;
use std::os::unix::fs::symlink;
use std::path::Path;
use std::process::exit;
use tempfile::TempDir;
use tiff::encoder::{colortype, Rational, TiffEncoder};
use tiff::tags::ResolutionUnit;
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
        error!("Application bundle does not exist. Please bundle first.");
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

    // TODO: Calculate an approximate size for the DMG on the fly
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

    let mut pixmap = Pixmap::new(
        tree.size().width() as u32 * 2,
        tree.size().height() as u32 * 2,
    )
    .expect("Could not create pixmap to hold PNG");
    render(
        &tree,
        Transform::from_scale(
            pixmap.width() as f32 / tree.size().width(),
            pixmap.height() as f32 / tree.size().height(),
        ),
        &mut pixmap.as_mut(),
    );

    // Render to TIFF
    let finished_background_file_path =
        editable_disk_image_background_directory.join("background.tiff");

    {
        let Ok(mut output_tiff_file) = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(&finished_background_file_path)
        else {
            error!("Failed to open output background TIFF file");
            exit(1);
        };

        let mut encoder = TiffEncoder::new(&mut output_tiff_file).unwrap();
        let mut encoder_image = encoder
            .new_image::<colortype::RGBA8>(pixmap.width(), pixmap.height())
            .unwrap();
        encoder_image.resolution(ResolutionUnit::Inch, Rational { n: 144, d: 1 });
        let Ok(_) = encoder_image.write_data(pixmap.data()) else {
            error!("Failed to write TIFF data");
            exit(1);
        };
    }

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
        tree.size().width() as u32,
        tree.size().height() as u32,
    ));

    // Set the window properties
    ds_store.push_entry(DSStoreEntry::new_icvp(
        ".",
        48,
        Alias::alias_for(
            finished_background_file_path.clone(),
            editable_disk_image_mount.mount_point.clone(),
        )
        .unwrap()
        .data(),
    ));

    ds_store.push_entry(DSStoreEntry::new_v_srn(".", 1));

    // Move the file icons
    let applications_center = center_of_rect(&applications_node.abs_stroke_bounding_box());
    ds_store.push_entry(DSStoreEntry::new_iloc(
        "Applications",
        applications_center.0,
        applications_center.1,
    ));
    let app_center = center_of_rect(&app_node.abs_stroke_bounding_box());
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

fn center_of_rect(rect: &Rect) -> (u32, u32) {
    (
        rect.x() as u32 + rect.width() as u32 / 2,
        rect.y() as u32 + rect.height() as u32 / 2,
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
