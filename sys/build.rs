extern crate core;

use std::env;
use std::path::Path;

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

fn main() {
    // Configure
    let version_major = env!("CARGO_PKG_VERSION_MAJOR");
    let version_minor = env!("CARGO_PKG_VERSION_MINOR");
    let version_dlib = format!("{}.{}", version_major, version_minor);

    // Download
    let src = download_and_unzip(&version_dlib);

    build(&src);
}

#[cfg(target_family = "unix")]
fn build(src: &Path) {
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

    // Build
    let dst = cmake::Config::new(src.join("examples"))
        .no_build_target(true)
        .define("DLIB_JPEG_SUPPORT", "1")
        .define("DLIB_PNG_SUPPORT", "1")
        // .define("DLIB_USE_BLAS", "1")
        // .define("DLIB_USE_LAPACK", "1")
        .define("USE_AVX_INSTRUCTIONS", "1")
        .define("USE_SSE2_INSTRUCTIONS", "1")
        .define("USE_SSE4_INSTRUCTIONS", "1")
        .build();

    // Copy the library file
    let dst_lib = &dst;
    let src_lib_dir = dst.join("build").join("dlib_build");
    let src_lib_prefix = if cfg!(windows) { "" } else { "lib" };
    let src_lib_suffix = if cfg!(windows) { "lib" } else { "a" };

    let src_lib = glob::glob(&format!(
        "{}/**/{}dlib*.{}",
        src_lib_dir.display(),
        &src_lib_prefix,
        &src_lib_suffix
    ))
    .expect("Failed to read glob pattern")
    .into_iter()
    .find_map(Result::ok)
    .expect("Failed to find library file");

    std::fs::create_dir_all(dst_lib).unwrap();
    std::fs::copy(
        src_lib,
        dst_lib.join(format!("{}dlib.{}", &src_lib_prefix, &src_lib_suffix)),
    )
    .unwrap();

    // Copy header files
    let dst_include = dst.join("include");
    std::fs::create_dir_all(&dst_include).unwrap();
    fs_extra::dir::copy(
        src.join("dlib"),
        &dst_include,
        &fs_extra::dir::CopyOptions {
            skip_exist: true,
            ..Default::default()
        },
    )
    .unwrap();

    let out_dir = env::var("OUT_DIR").unwrap();

    // Link
    println!("cargo:rustc-flags=-L {}", out_dir);
    println!("cargo:include={}", dst_include.display());
    println!("cargo:rustc-link-lib=static=dlib");
}

#[cfg(target_family = "windows")]
fn build(src: &std::path::Path) {
    use fs_extra::dir::CopyOptions;
    use std::path::PathBuf;

    let dst = cmake::Config::new(src)
        .no_build_target(false)
        .define("CMAKE_INSTALL_PREFIX", "install")
        .build();

    // Copy the library file
    let dst_lib = &dst;
    let src_lib_dir = dst.join("build").join("install");
    std::fs::create_dir_all(dst.join("lib")).unwrap();
    std::fs::create_dir_all(dst.join("include")).unwrap();

    fs_extra::dir::copy(
        src_lib_dir.join("lib"),
        &dst,
        &CopyOptions {
            skip_exist: true,
            ..Default::default()
        },
    )
    .unwrap();

    fs_extra::dir::copy(
        src_lib_dir.join("include"),
        &dst,
        &CopyOptions {
            skip_exist: true,
            ..Default::default()
        },
    )
    .unwrap();

    // modify file name only on windows msvc tool chain.
    modify_dlib_msvc_filename(&dst);

    let out_dir = env::var("OUT_DIR").unwrap();

    std::env::var("CARGO_MANIFEST_DIR").unwrap();
    let dlib = PathBuf::from(out_dir.clone()).join("lib");
    println!("cargo:rustc-link-search=native={}", out_dir);
    println!("cargo:rustc-flags=-L {}", dlib.display());
    println!("cargo:rustc-flags=-L {}", out_dir);
    println!("cargo:root={}", out_dir);
    println!("cargo:include={}", dst_lib.join("include").display());
    println!("cargo:rustc-link-lib=static=dlib");
}

#[cfg(target_family = "windows")]
fn modify_dlib_msvc_filename(dst: &Path) {
    use std::fs::File;
    let src_lib_prefix = if cfg!(windows) { "" } else { "lib" };
    let src_lib_suffix = if cfg!(windows) { "lib" } else { "a" };
    let target = env::var("TARGET").unwrap();
    if target.contains("x86_64-pc-windows-msvc") {
        let src_lib_path = glob::glob(&format!(
            "{}/{}dlib*.{}",
            dst.join("lib").display(),
            &src_lib_prefix,
            &src_lib_suffix
        ))
        .expect("Failed to read glob pattern")
        .into_iter()
        .filter_map(Result::ok)
        .next();
        if let Some(source) = src_lib_path {
            let dlib_modified_name = dst
                .join("lib")
                .join(format!("{}dlib.{}", &src_lib_prefix, &src_lib_suffix));
            std::fs::File::create(dlib_modified_name.clone()).unwrap();
            std::fs::copy(source, dlib_modified_name).unwrap();
        }
    };
}
