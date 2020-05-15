extern crate cpp_build;
#[cfg(feature = "download-models")]
extern crate reqwest;
#[cfg(feature = "download-models")]
extern crate bzip2;

#[cfg(feature = "download-models")]
fn download_path() -> std::path::PathBuf {
    std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("files")
}

#[cfg(feature = "download-models")]
fn download_and_unzip(client: &reqwest::Client, url: &str) {
    use bzip2::read::*;

    let url: reqwest::Url = url.parse().unwrap();

    let filename = url
        .path_segments().unwrap()
        .last().unwrap()
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

fn main() {
    println!("cargo:rustc-link-lib=dlib");
    println!("cargo:rustc-link-lib=lapack");
    println!("cargo:rustc-link-lib=cblas");

    cpp_build::build("src/lib.rs");

    #[cfg(feature = "download-models")]
    {
        if !download_path().exists() {
            std::fs::create_dir(download_path()).unwrap();
        }

        // Download the data files
        // I'm not sure if doing this in the build script is such a good idea, seeing as it happens silently,
        // but I dont think adding the files to the repo is good either

        // Create a client for maintaining connections
        let client = reqwest::ClientBuilder::new()
            // Turn off gzip decryption
            // See: https://github.com/seanmonstar/reqwest/issues/328
            .gzip(false)
            .build().unwrap();

        download_and_unzip(&client, "http://dlib.net/files/mmod_human_face_detector.dat.bz2");
        download_and_unzip(&client, "http://dlib.net/files/shape_predictor_68_face_landmarks.dat.bz2");
        download_and_unzip(&client, "http://dlib.net/files/dlib_face_recognition_resnet_model_v1.dat.bz2");
    }
}
