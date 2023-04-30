use std::process::{Command, Stdio};
use std::{env, path::Path};
use walkdir::WalkDir;

fn batch_compile<P: AsRef<Path>>(sources: &[P], input_dir: &str, output_dir: &str) {
    if sources.is_empty() {
        return;
    }

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

fn compile_resources_explicit<P: AsRef<Path>>(
    blueprints: &[P],
    source_dir: &str,
    gresource: &str,
    target: &str,
) {
    let out_dir = env::var("OUT_DIR").unwrap();

    batch_compile(blueprints, source_dir, out_dir.as_str());

    glib_build_tools::compile_resources(&[source_dir, out_dir.as_str()], gresource, target);
}

pub fn compile_resources(source_dir: &str, gresource: &str, target: &str) {
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
    compile_resources_explicit(blueprints.as_slice(), source_dir, gresource, target);
}
