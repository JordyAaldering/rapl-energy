#[cfg(not(feature = "libc"))]
fn main() {}

#[cfg(feature = "libc")]
fn main() {
    let lib_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    let profile = std::env::var("PROFILE").unwrap();
    let path = format!("target/{}/rapl_energy.h", profile);

    cbindgen::Builder::new()
        .with_crate(lib_dir)
        .with_language(cbindgen::Language::C)
        .with_include_guard("RAPL_ENERGY_H")
        .with_no_includes()
        .with_sys_include("stdint.h")
        .generate()
        .expect("Unable to generate bindings")
        .write_to_file(path);
}
