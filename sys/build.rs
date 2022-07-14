extern crate core;

use std::env;
use std::path::PathBuf;
use fs_extra::dir::CopyOptions;

#[cfg(feature = "build")]
fn download_and_unzip(version: &str) -> std::path::PathBuf {
    let url = format!("http://dlib.net/files/dlib-{}.zip", version);
    let url: reqwest::Url = url.parse().unwrap();

    let root_dir = std::path::PathBuf::from(std::env::var("OUT_DIR").unwrap());
    let path = root_dir.join(format!("dlib-{}", version));

    if path.exists() {
        eprintln!("Already got '{}'", path.display());
        return path;
    }

    eprintln!("Downloading '{}'...", url);

    let client = reqwest::blocking::ClientBuilder::new()
        .timeout(std::time::Duration::from_secs(3600))
        .build()
        .unwrap();
    let response = client.get(url).send().unwrap().bytes().unwrap();
    let mut archive = zip::ZipArchive::new(std::io::Cursor::new(response)).unwrap();
    archive.extract(&root_dir).unwrap();

    path
}

#[cfg(feature = "build")]
fn main() {
    // Configure
    let version_major = env!("CARGO_PKG_VERSION_MAJOR");
    let version_minor = env!("CARGO_PKG_VERSION_MINOR");
    let version_dlib = format!("{}.{}", version_major, version_minor);

    // Try probing
    if let Ok(library) = pkg_config::Config::new()
        .print_system_cflags(false)
        // .atleast_version(&version_dlib)
        .probe("dlib-1")
    {
        fn write_paths(key: &str, paths: Vec<std::path::PathBuf>) {
            println!(
                "cargo:{}={}",
                key,
                std::env::join_paths(paths)
                    .unwrap()
                    .as_os_str()
                    .to_str()
                    .unwrap()
            );
        }
        write_paths("root", library.link_paths);
        write_paths("include", library.include_paths);
        return;
    }

    // Download
    let src = download_and_unzip(&version_dlib);
    build_dlib(&src);
}

fn build_dlib(src: &PathBuf) {
    let dst = cmake::Config::new(&src)
        .no_build_target(false)
        .define("JPEG_INCLUDE_DIR", src.join("dlib").join("external").join("libjpeg"))
        .define("JPEG_LIBRARY", src.join("dlib").join("external").join("libjpeg"))
        .define("PNG_PNG_INCLUDE_DIR", src.join("dlib").join("external").join("libpng"))
        .define("PNG_LIBRARY_RELEASE", src.join("dlib").join("external").join("libpng"))
        .define("ZLIB_INCLUDE_DIR", src.join("dlib").join("external").join("zlib"))
        .define("ZLIB_LIBRARY_RELEASE", src.join("dlib").join("external").join("zlib"))
        .define("CMAKE_INSTALL_PREFIX", "install")
        .build();

    // Copy the library file
    let dst_lib = &dst;
    let src_lib_dir = dst.join("build").join("install");
    let src_lib_prefix = if cfg!(windows) { "" } else { "lib" };
    let src_lib_suffix = if cfg!(windows) { "lib" } else { "a" };
    std::fs::create_dir_all(dst.join("lib")).unwrap();
    std::fs::create_dir_all(dst.join("include")).unwrap();


    fs_extra::dir::copy(
        &src_lib_dir.join("lib"),
        &dst,
        &CopyOptions {
            skip_exist: true,
            ..Default::default()
        },
    ).unwrap();

    fs_extra::dir::copy(
        &src_lib_dir.join("include"),
        &dst,
        &CopyOptions {
            skip_exist: true,
            ..Default::default()
        },
    ).unwrap();

    // modify file name only on windows msvc tool chain.
    let src_lib_path = glob::glob(&format!(
        "{}/**/{}dlib*.{}",
        src_lib_dir.join("lib").display(),
        &src_lib_prefix,
        &src_lib_suffix
    ))
        .expect("Failed to read glob pattern")
        .into_iter()
        .filter_map(Result::ok)
        .next();
    match src_lib_path {
        None => {
            // silent ignore
        }
        Some(source) => {
            std::fs::copy(
                source,
                src_lib_dir.join("lib").join(format!("{}dlib.{}", &src_lib_prefix, &src_lib_suffix)))
                .unwrap();
        }
    }

    let out_dir = env::var("OUT_DIR").unwrap();

    std::env::var("CARGO_MANIFEST_DIR").unwrap();
    let dlib = PathBuf::from(out_dir).join("lib");
    println!("cargo:rustc-flags=-L '{}'", dlib.display());
    println!("cargo:root={}", env::var("OUT_DIR").unwrap());
    println!("cargo:include={}", dst_lib.join("include").display());
    println!("cargo:rustc-link-lib=dlib");
}

#[cfg(not(feature = "build"))]
fn main() {}
