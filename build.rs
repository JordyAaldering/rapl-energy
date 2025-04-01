use std::env;

fn main() -> Result<(), cbindgen::Error> {
    let lib_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let profile = env::var("PROFILE").unwrap();
    let path = format!("target/{}/rapl_energy.h", profile);

    cbindgen::Builder::new()
        .with_crate(lib_dir)
        .with_language(cbindgen::Language::C)
        .with_include_guard("RAPL_ENERGY_H")
        .with_no_includes()
        .with_sys_include("stdint.h")
        .generate()?
        .write_to_file(path);

    Ok(())
}
