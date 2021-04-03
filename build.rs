/// Ref @ https://doc.rust-lang.org/cargo/reference/build-scripts.html
extern crate bindgen;

fn main() {
    println!("cargo:rustc-link-search=libs/usr/local/lib/");
    println!("cargo:rustc-link-lib=static=glfw3");
    println!("cargo:rustc-link-lib=static=glad");

    // bindgen uses `libclang` & the custom path need to be defined
    std::env::set_var("LIBCLANG_PATH", "libs/usr/local/lib/");

    // GLAD bindgen
    println!("cargo:rerun-if-changed=libs/usr/local/include/glad/glad.h");
    bindgen::Builder::default()
        .header("libs/usr/local/include/glad/glad.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .clang_arg("-I/usr/include/")
        .clang_arg("-I/usr/include/linux/")
        .generate()
        .expect("Unable to generate bindings")
        .write_to_file("src/glfw/glad_bindgen.rs")
        .expect("Couldn't write bindings!");

    // GLFW bindgen
    println!("cargo:rerun-if-changed=libs/usr/local/include/GLFW/glfw3.h");
    bindgen::Builder::default()
        .header("libs/usr/local/include/GLFW/glfw3.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .clang_arg("-I/usr/include/")
        .clang_arg("-I/usr/include/linux/")
        .generate()
        .expect("Unable to generate bindings")
        .write_to_file("src/glfw/glfw_bindgen.rs")
        .expect("Couldn't write bindings!");
}
