fn main() {
    let mut config = cpp_build::Config::new();

    println!("cargo:rustc-link-lib=dlib");
    println!("cargo:rustc-link-lib=lapack");
    println!("cargo:rustc-link-lib=cblas");

    config.build("src/lib.rs");
}
