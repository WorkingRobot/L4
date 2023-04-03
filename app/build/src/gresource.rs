use std::{env, path::Path, path::PathBuf};

pub fn include_gresources<P: AsRef<Path>>(directory: P, gresources: &[&str]) {
    let out_dir = env::var("OUT_DIR").unwrap();
    for theme in gresources {
        let mut from_path = directory.as_ref().to_path_buf();
        from_path.push(theme);
        from_path.set_extension("gresource");

        let mut to_path = PathBuf::new();
        to_path.push(out_dir.as_str());
        to_path.push(theme);
        to_path.set_extension("gresource");

        std::fs::copy(&from_path, &to_path).unwrap();

        println!("cargo:rerun-if-changed={}", &from_path.display());
    }
}
