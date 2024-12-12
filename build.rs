use std::{env, fs, path::Path};

fn main() {
    let delta_based = env::var("CARGO_FEATURE_DELTA_BASED").is_ok();
    let corridor_based = env::var("CARGO_FEATURE_CORRIDOR_BASED").is_ok();
    assert!(delta_based ^ corridor_based);

    let num_letterboxes: usize = env::var("NUM_LETTERBOXES").map_or(256, |s| s.parse().unwrap());
    let num_samples: usize = env::var("NUM_SAMPLES").map_or(10, |s| s.parse().unwrap());

    // Build configuration file
    let out_dir = env::var("OUT_DIR").unwrap();
    let path = Path::new(&out_dir).join("config.rs");

    fs::write(
        &path,
        [
            format!("pub const NUM_LETTERBOXES: usize = {};", num_letterboxes),
            format!("pub const NUM_SAMPLES: usize = {};", num_samples),
        ].join("\n")
    ).unwrap();

    // Rebuild project if environment variable changed
    println!("cargo:rerun-if-env-changed=NUM_LETTERBOXES");
    println!("cargo:rerun-if-env-changed=NUM_SAMPLES");

    // Build header file
    let lib_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let profile = env::var("PROFILE").unwrap();
    let path = format!("target/{}/mtdynamic.h", profile);

    let mut config = cbindgen::Config::from_file("cbindgen.toml").unwrap();
    config.defines.insert("delta-based".to_string(), "delta-based".to_string());

    let defines = [
        String::new(),
        format!("#define {}", if delta_based { "DELTA_BASED" } else { "CORRIDOR_BASED" }),
        format!("#define NUM_LETTERBOXES {}", num_letterboxes),
        format!("#define NUM_SAMPLES {}", num_samples),
    ].join("\n");

    cbindgen::Builder::new()
        .with_config(config)
        .with_crate(lib_dir)
        .with_sys_include("semaphore.h")
        .with_after_include(defines)
        .generate()
        .expect("Unable to generate bindings")
        .write_to_file(path);
}
