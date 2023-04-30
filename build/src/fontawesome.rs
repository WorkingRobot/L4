// Partially taken from: https://github.com/emmanueltouzery/cigale/blob/c471c441448c3fcc74f4ce88870652c618ca0b81/build.rs

use flate2::read::GzDecoder;
use std::path::Path;
use std::{
    env,
    io::{BufWriter, Write},
};

const FONTAWESOME_VERSION: &str = "6.4.0";

fn fetch_icons<P: AsRef<Path>>(target_dir: P) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let fontawesome_url = format!(
        "https://registry.npmjs.org/@fortawesome/fontawesome-free/-/fontawesome-free-{}.tgz",
        FONTAWESOME_VERSION
    );
    let resp = reqwest::blocking::get(fontawesome_url)?;
    let mut archive = tar::Archive::new(GzDecoder::new(resp));

    let n = archive
        .entries()
        .unwrap()
        .filter_map(|e| e.ok())
        .filter_map(|mut entry| -> Option<String> {
            let path = entry
                .path()
                .ok()?
                .strip_prefix("package/svgs")
                .ok()?
                .to_owned();
            let icon_style = path.parent()?.to_str()?;
            let icon_name = path.file_name()?.to_str()?.strip_suffix(".svg")?;
            let icon_filename = format!("fa-{}-{}-symbolic.svg", icon_name, icon_style);
            let path = target_dir.as_ref().join(&icon_filename);
            entry.unpack(&path).ok()?;
            Some(icon_filename)
        })
        .collect();

    Ok(n)
}

fn create_gresource_manifest<P: AsRef<Path>, Q: AsRef<str>>(
    target_file: P,
    icons: &[Q],
) -> std::io::Result<()> {
    let target_file = std::fs::OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(target_file.as_ref())?;
    let mut f = BufWriter::new(target_file);
    f.write_all(r#"<?xml version="1.0" encoding="UTF-8"?>"#.as_bytes())?;
    f.write_all(r#"<gresources>"#.as_bytes())?;
    f.write_all(r#"<gresource prefix="/com/fontawesome/icons/scalable/actions">"#.as_bytes())?;
    for icon in icons {
        f.write_fmt(format_args!(
            r#"<file compressed="true">{}</file>"#,
            icon.as_ref()
        ))?;
    }
    f.write_all(r#"</gresource>"#.as_bytes())?;
    f.write_all(r#"</gresources>"#.as_bytes())?;

    Ok(())
}

pub fn create_gresource() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let out_dir = Path::new(&out_dir);

    if !out_dir.join("FontAwesome.gresource").exists() {
        let target_dir = out_dir.join("fontawesome");
        let target_manifest = target_dir.join("fontawesome.gresource.xml");

        if !target_dir.is_dir() {
            std::fs::create_dir_all(&target_dir).unwrap();

            let icons = fetch_icons(&target_dir).unwrap();

            create_gresource_manifest(&target_manifest, &icons).unwrap();
        }

        glib_build_tools::compile_resources(
            &[target_dir],
            target_manifest.to_str().unwrap(),
            "FontAwesome.gresource",
        );
    }
}
