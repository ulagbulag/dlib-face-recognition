fn main() {
    println!("cargo:rustc-link-lib=dlib");
    println!("cargo:rustc-link-lib=lapack");
    println!("cargo:rustc-link-lib=cblas");

    cpp_build::Config::new()
        .include("/usr/include/opencv4")
        .build("src/lib.rs");
}
