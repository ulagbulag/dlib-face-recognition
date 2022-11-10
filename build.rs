use std::env;
use std::path::PathBuf;

#[cfg(feature = "embed-any")]
fn download_path() -> std::path::PathBuf {
    std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("files")
}

#[cfg(feature = "embed-any")]
fn download_and_unzip(client: &reqwest::blocking::Client, url: &str) {
    use bzip2::read::*;

    let url: reqwest::Url = url.parse().unwrap();

    let filename = url
        .path_segments()
        .unwrap()
        .last()
        .unwrap()
        .replace(".bz2", "");

    let path = download_path().join(&filename);

    if path.exists() {
        println!("Already got '{}'", path.display());
        return;
    }

    println!("Downloading '{}'...", url);

    let response = client.get(url).send().unwrap();
    let mut decoded = BzDecoder::new(response);
    let mut file = std::fs::File::create(&path).unwrap();
    std::io::copy(&mut decoded, &mut file).unwrap();
}

#[cfg(feature = "build")]
fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let target = env::var("TARGET").unwrap();
    println!("cargo:warning= target {}", target);
    let mut config = cpp_build::Config::new();
    println!("cargo:rerun-if-changed=./files");

    let root_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    let widows = PathBuf::from(root_dir.clone()).join("external-libs").join("windows");

    // only for windows mingw64/gnu tool chain
    if target.contains("windows-gnu") {
        config.flag("-Os");
        config.flag("-Wa,-mbig-obj");
        println!("cargo:rustc-flags=-L {}", widows.display());
    } else if target.contains("msvc") {
        println!("cargo:rustc-flags=-L {}", widows.display());
    }

    println!("cargo:rustc-link-lib={}", "blas");
    println!("cargo:rustc-link-lib={}", "lapack");
    println!("cargo:rustc-link-lib=static=dlib");

    if let Ok(paths) = std::env::var("DEP_DLIB_INCLUDE") {
        for path in std::env::split_paths(&paths) {
            config.include(path);
        }
    }
    config.build("src/lib.rs");

    #[cfg(feature = "embed-any")]
    {
        if !download_path().exists() {
            std::fs::create_dir(download_path()).unwrap();
        }

        // Download the data files
        // I'm not sure if doing this in the build script is such a good idea, seeing as it happens silently,
        // but I dont think adding the files to the repo is good either

        // Create a client for maintaining connections
        let client = reqwest::blocking::ClientBuilder::new().build().unwrap();

        #[cfg(feature = "embed-fd-nn")]
        download_and_unzip(
            &client,
            "http://dlib.net/files/mmod_human_face_detector.dat.bz2",
        );
        #[cfg(feature = "embed-fe-nn")]
        download_and_unzip(
            &client,
            "http://dlib.net/files/dlib_face_recognition_resnet_model_v1.dat.bz2",
        );
        #[cfg(feature = "embed-lp")]
        download_and_unzip(
            &client,
            "http://dlib.net/files/shape_predictor_68_face_landmarks.dat.bz2",
        );
    }
}

#[cfg(not(feature = "build"))]
fn main() {}
