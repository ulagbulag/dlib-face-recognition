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
    if pkg_config::Config::new()
        .atleast_version(&version_dlib)
        .probe("dlib")
        .is_ok()
    {
        return;
    }

    // Download
    let src = download_and_unzip(&version_dlib);

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
    .filter_map(Result::ok)
    .next()
    .expect("Failed to find library file");
    std::fs::create_dir_all(&dst_lib).unwrap();
    std::fs::copy(
        &src_lib,
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

    // Link
    println!("cargo:rustc-link-search=native={}", dst.display());
    println!("cargo:rustc-link-lib=dlib");
    println!("cargo:rustc-link-lib=blas");
    println!("cargo:rustc-link-lib=lapack");

    println!("cargo:root={}", dst.display());
    println!("cargo:include={}", dst_include.display());
}

#[cfg(not(feature = "build"))]
fn main() {}
