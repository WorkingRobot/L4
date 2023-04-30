fn main() {
    println!("cargo:rustc-env=RUSTC_VERSION={}", rustc_version::version().unwrap());
}
