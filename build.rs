use std::env;

fn main() {
    let lib_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let profile = env::var("PROFILE").unwrap();
    let path = format!("target/{}/mtdynamic.h", profile);

    let config = cbindgen::Config {
        usize_is_size_t: true,
        ..Default::default()
    };

    cbindgen::Builder::new()
        .with_config(config)
        .with_crate(lib_dir)
        .with_cpp_compat(true)
        .with_language(cbindgen::Language::C)
        .with_include_guard("MTD_MTDYNAMIC")
        .with_sys_include("semaphore.h")
        .generate()
        .expect("Unable to generate bindings")
        .write_to_file(path);
}
