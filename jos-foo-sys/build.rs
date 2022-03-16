use std::path::PathBuf;

fn main() {
    let out_dir = std::env::var("OUT_DIR").unwrap();
    let out_path = PathBuf::from(out_dir);
    bindgen::builder()
        .header("../jos-foo-build/jos_foo/jos_foo.h")
        .generate()
        .unwrap()
        .write_to_file(out_path.join("bindings.rs"))
        .unwrap();
    println!("cargo:rustc-link-lib=static=jos_foo");
}
