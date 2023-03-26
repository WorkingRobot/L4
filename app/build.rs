use std::process::Command;
use std::{
    env,
    path::{Path, PathBuf},
};

fn main() {
    compile_resources_with_blueprints(
        &["dummy.blp"],
        "resources",
        "resources/resources.gresource.xml",
        "L4.gresource",
    );
}

fn get_workspace_root() -> PathBuf {
    let output = Command::new("cargo")
        .arg("locate-project")
        .arg("--workspace")
        .arg("--message-format")
        .arg("plain")
        .output()
        .unwrap();
    let mut path: PathBuf = PathBuf::from(String::from_utf8(output.stdout).unwrap());
    path.pop();
    path
}

fn get_blueprint_compiler_path() -> PathBuf {
    let mut path = get_workspace_root();
    path.push("deps");
    path.push("blueprint-compiler");
    path.push("blueprint-compiler.py");
    path
}

fn blueprint_batch_compile<P: AsRef<Path>>(sources: &[&str], input_dir: P, output_dir: &str) {
    let mut command = Command::new("python");

    command
        .arg(get_blueprint_compiler_path())
        .arg("batch-compile")
        .arg(output_dir)
        .arg(input_dir.as_ref());

    for source in sources {
        let source_path = input_dir.as_ref().join(source);
        command.arg(&source_path);
        println!("cargo:rerun-if-changed={source_path:?}");
    }

    let output = command.output().unwrap();

    assert!(
        output.status.success(),
        "blueprint-compiler failed with exit status {} and stderr:\n{}",
        output.status,
        String::from_utf8_lossy(&output.stderr)
    )
}

fn compile_resources_with_blueprints(
    blueprints: &[&str],
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
