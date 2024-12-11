use std::{env, fs, path::Path};

fn main() {
    build_config();
    build_header();
}

fn build_config() {
    let num_samples: usize = env::var("NUM_SAMPLES").map_or(20, |s| s.parse().unwrap());

    let out_dir = env::var("OUT_DIR").unwrap();
    let path = Path::new(&out_dir).join("config.rs");
    fs::write(&path, format!("pub const NUM_SAMPLES: usize = {};", num_samples)).unwrap();

    // Rebuild project if environment variable changed
    println!("cargo:rerun-if-env-changed=NUM_SAMPLES");
}

fn build_header() {
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
