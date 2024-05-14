use std::path::PathBuf;

fn main() {
    println!("cargo:rustc-link-search=./ihslib");
    println!("cargo:rustc-link-lib=ihslib");

    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .generate()
        .expect("Unable to generate bindings");

    // Create "dist"
    if std::fs::create_dir("bindings").is_err() {
        println!("bindings/ already exists");
    }

    // Write the bindings to the dist folder
    let out_path = PathBuf::from("./bindings");
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}