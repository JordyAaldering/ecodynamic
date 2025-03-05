use std::env;

fn main() {
    let lib_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    cbindgen::Builder::new()
        .with_crate(lib_dir)
        .with_tab_width(4)
        .with_no_includes()
        .with_sys_include("stdint.h")
        .with_include_guard("_MTD_LETTERBOX_H_")
        .include_item("Incoming")
        .include_item("Outgoing")
        .generate()
        .expect("Unable to generate bindings")
        .write_to_file("../letterbox.h");
}
