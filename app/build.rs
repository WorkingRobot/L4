use embed_manifest::{embed_manifest, manifest::DpiAwareness, new_manifest};
use std::process::{Command, Stdio};
use std::{env, path::Path, path::PathBuf};
use walkdir::WalkDir;

fn main() {
    compile_resources_with_blueprints(
        "resources",
        "resources/resources.gresource.xml",
        "L4.gresource",
    );

    let out_dir = env::var("OUT_DIR").unwrap();
    for theme in &[
        "Sweet",
        "Sweet-Ambar",
        "Sweet-Ambar-Blue",
        "Sweet-Dark",
        "Sweet-Mars",
    ] {
        let mut from_path = PathBuf::new();
        from_path.push("resources");
        from_path.push("Sweet");
        from_path.push(theme);
        from_path.set_extension("gresource");
        let mut to_path = PathBuf::new();
        to_path.push(out_dir.as_str());
        to_path.push(theme);
        to_path.set_extension("gresource");
        std::fs::copy(&from_path, &to_path).unwrap();

        println!("cargo:rerun-if-changed={}", &from_path.display());
    }

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
    println!("cargo:rerun-if-changed=build.rs");
}

fn blueprint_batch_compile<P: AsRef<Path>>(sources: &[P], input_dir: &str, output_dir: &str) {
    let mut command = Command::new("blueprint-compiler");

    command.arg("batch-compile").arg(output_dir).arg(input_dir);

    for source in sources {
        command.arg(source.as_ref());
        println!("cargo:rerun-if-changed={}", source.as_ref().display());
    }

    let output = command
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .output()
        .unwrap();

    assert!(
        output.status.success(),
        "blueprint-compiler failed with exit status {}",
        output.status
    )
}

fn compile_resources_with_blueprints_explicit<P: AsRef<Path>>(
    blueprints: &[P],
    source_dir: &str,
    gresource: &str,
    target: &str,
) {
    let blueprint_out_dir = env::var("OUT_DIR").unwrap();

    blueprint_batch_compile(blueprints, source_dir, blueprint_out_dir.as_str());

    glib_build_tools::compile_resources(
        &[source_dir, blueprint_out_dir.as_str()],
        gresource,
        target,
    );
}

fn compile_resources_with_blueprints(source_dir: &str, gresource: &str, target: &str) {
    let mut blueprints = Vec::new();
    for entry in WalkDir::new(source_dir).into_iter().filter_map(|e| e.ok()) {
        if entry.file_type().is_file()
            && entry
                .file_name()
                .to_str()
                .map(|s| s.ends_with(".blp"))
                .unwrap_or(false)
        {
            blueprints.push(entry.into_path());
        }
    }
    compile_resources_with_blueprints_explicit(
        blueprints.as_slice(),
        source_dir,
        gresource,
        target,
    );
}
