use std::path::PathBuf;
use std::fmt;

pub enum ModelFile{
    FaceDetectorCnn,
    FaceEncoderNetwork,
    LandmarkPredictor
}

impl fmt::Display for ModelFile {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
       match self {
           ModelFile::FaceDetectorCnn => write!(f, "mmod_human_face_detector.dat"),
           ModelFile::FaceEncoderNetwork => write!(f, "dlib_face_recognition_resnet_model_v1.dat"),
           ModelFile::LandmarkPredictor => write!(f, "shape_predictor_68_face_landmarks.dat")
       }
    }
}

pub fn download_path() -> std::path::PathBuf {
    std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("files")
}

pub fn path_for_file(filename: &ModelFile) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("files")
        .join(filename.to_string())
}

pub fn check_file_or_download(file: &ModelFile){
    if !download_path().exists() {
        std::fs::create_dir(download_path()).unwrap();
    }

    let filename = crate::embed::path_for_file(file);

    if !filename.exists() {
        let client = reqwest::blocking::ClientBuilder::new().build().unwrap();

        download_and_unzip(&client, file);
    } else {
        println!("{} Already exists in files/ folder, skipping download.", file);
    }
}

pub fn download_and_unzip(client: &reqwest::blocking::Client,file: &ModelFile) {
    use bzip2::read::*;

    let base_url: &str = "http://dlib.net/files/";
    let compressed_file_extension: &str = ".bz2";

    let url = format!("{base_url}{file}{compressed_file_extension}");

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
