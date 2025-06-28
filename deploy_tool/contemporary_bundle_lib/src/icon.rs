use contemporary_config::ContemporaryConfig;
use contemporary_icon_tool_core::contemporary_icon::ContemporaryIcon;
use std::fs::OpenOptions;
use std::io::Read;
use std::path::Path;

pub fn get_svg_icon_contents(
    target_triple: &String,
    base_path: &Path,
    contemporary_config: &ContemporaryConfig,
) -> String {
    let config = contemporary_config.deployment(&target_triple);

    let icon = config.icon;
    if let Some(icon) = icon {
        let path = base_path.join(icon);
        let Ok(mut file) = OpenOptions::new().read(true).open(&path) else {
            panic!("Could not open icon file: {:?}", &path);
        };

        let mut contents = String::new();
        let Ok(_) = file.read_to_string(&mut contents) else {
            panic!("Could not read icon file: {:?}", &path);
        };
        return contents;
    }

    let Some(contemporary_base_icon) = config.contemporary_base_icon else {
        panic!("No icon specified for target triple: {:?}", &target_triple);
    };
    let path = base_path.join(contemporary_base_icon);

    if contemporary_config.application.theme_colors.len() != 2 {
        panic!("theme_colors must contain exactly 2 elements.");
    }

    let icon_generator = ContemporaryIcon::new(
        path,
        matches!(
            target_triple.as_str(),
            "aarch64-apple-darwin" | "x86_64-apple-darwin"
        ),
        false,
    );

    icon_generator.generate(
        &contemporary_config.application.theme_colors[0],
        &contemporary_config.application.theme_colors[1],
    )
}
