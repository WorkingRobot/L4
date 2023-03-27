use std::process::Command;
use std::{env, path::Path};
use walkdir::WalkDir;

fn main() {
    compile_resources_with_blueprints(
        "resources",
        "resources/resources.gresource.xml",
        "L4.gresource",
    );
}

fn blueprint_batch_compile<P: AsRef<Path>>(sources: &[P], input_dir: &str, output_dir: &str) {
    let mut command = Command::new("blueprint-compiler");

    command.arg("batch-compile").arg(output_dir).arg(input_dir);

    for source in sources {
        command.arg(source.as_ref());
        println!("cargo:rerun-if-changed={:?}", source.as_ref());
    }

    let output = command.output().unwrap();

    assert!(
        output.status.success(),
        "blueprint-compiler failed with exit status {} and stderr:\n{}",
        output.status,
        String::from_utf8_lossy(&output.stderr)
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
