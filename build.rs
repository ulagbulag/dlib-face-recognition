fn main() {
    let mut config = cpp_build::Config::new();
    println!("cargo:rerun-if-changed=./files");

    #[cfg(target_family="windows")] {
        let root_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
        let windows = std::path::PathBuf::from(root_dir)
            .join("external-libs")
            .join("windows");

        #[cfg(target_env="gnu")] {
            config.flag("-Os");
            config.flag("-Wa,-mbig-obj");
        }

        println!("cargo:rustc-flags=-L {}", windows.display());

        println!("cargo:rustc-link-lib=blas");
        println!("cargo:rustc-link-lib=lapack");
        println!("cargo:rustc-link-lib=static=dlib");
    } 

    #[cfg(not(target_family="windows"))] {
        println!("cargo:rustc-link-lib=dlib");
        println!("cargo:rustc-link-lib=blas");
        println!("cargo:rustc-link-lib=lapack");
    } 

    if let Ok(paths) = std::env::var("DEP_DLIB_INCLUDE") {
        for path in std::env::split_paths(&paths) {
            config.include(path);
        }
    }
    config.build("src/lib.rs");
}
