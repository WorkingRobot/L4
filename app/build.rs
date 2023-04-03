use embed_manifest::{embed_manifest, manifest::DpiAwareness, new_manifest};

fn main() {
    build_scripts::blueprint_compile_resources(
        "resources",
        "resources/resources.gresource.xml",
        "L4.gresource",
    );

    build_scripts::include_gresources(
        "resources/Sweet",
        &[
            "Sweet",
            "Sweet-Ambar",
            "Sweet-Ambar-Blue",
            "Sweet-Dark",
            "Sweet-Mars",
        ],
    );

    build_scripts::fontawesome_create_gresource();

    if cfg!(target_os = "windows") {
        embed_manifest(
            new_manifest("WorkingRobot.L4").dpi_awareness(DpiAwareness::PerMonitorV2Only),
        )
        .expect("Unable to embed manifest file");

        let mut res = winres::WindowsResource::new();
        res.set_icon("resources/icon.ico")
            .set("InternalName", "L4.EXE");
        res.compile().unwrap();
    }
}
