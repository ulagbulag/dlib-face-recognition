#[allow(dead_code)]
fn system_is_debian() -> bool {
    cfg!(target_os = "linux")
        && ::sys_info::linux_os_release()
            .map(|os| os.id_like.map(|id| id == "debian").unwrap_or_default())
            .unwrap_or_default()
}

#[allow(dead_code)]
enum BlasLibrary {
    #[cfg(feature = "openblas")]
    Openblas,

    Blas,
    Cblas,
}

impl BlasLibrary {
    fn as_str(&self) -> &'static str {
        match self {
            #[cfg(feature = "openblas")]
            BlasLibrary::Openblas => "openblas",

            BlasLibrary::Cblas => "cblas",
            BlasLibrary::Blas => "blas",
        }
    }
}

fn set_blas_library() -> BlasLibrary {
    #[cfg(feature = "openblas")]
    {
        BlasLibrary::Openblas
    }

    #[cfg(not(feature = "openblas"))]
    {
        if system_is_debian() {
            BlasLibrary::Blas
        } else {
            BlasLibrary::Cblas
        }
    }
}

fn main() {
    let mut config = cpp_build::Config::new();
    println!("cargo:rerun-if-changed=./files");

    let blas_library = set_blas_library().as_str();

    #[cfg(target_family = "windows")]
    {
        let root_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
        let windows = std::path::PathBuf::from(root_dir)
            .join("external-libs")
            .join("windows");

        #[cfg(target_env = "gnu")]
        {
            config.flag("-Os");
            config.flag("-Wa,-mbig-obj");
        }

        println!("cargo:rustc-flags=-L {}", windows.display());

        println!("cargo:rustc-link-lib={blas_library}");
        println!("cargo:rustc-link-lib=lapack");
        println!("cargo:rustc-link-lib=static=dlib");
    }

    #[cfg(not(target_family = "windows"))]
    {
        println!("cargo:rustc-link-lib={blas_library}");
        println!("cargo:rustc-link-lib=dlib");
        println!("cargo:rustc-link-lib=lapack");
    }

    if let Ok(paths) = std::env::var("DEP_DLIB_INCLUDE") {
        for path in std::env::split_paths(&paths) {
            config.include(path);
        }
    }
    config.flag("-std=c++14").build("src/lib.rs");
}
