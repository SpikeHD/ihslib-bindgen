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
    if std::fs::create_dir("dist").is_err() {
        println!("dist already exists");
    }

    // Write the bindings to the dist folder
    let out_path = PathBuf::from("./dist");
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}