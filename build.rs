/// Ref @ https://doc.rust-lang.org/cargo/reference/build-scripts.html

fn main() {
    println!("cargo:rustc-link-search=libs/usr/local/lib/");
    println!("cargo:rustc-link-lib=static=glfw3");
    println!("cargo:rustc-link-lib=static=glad");
}
