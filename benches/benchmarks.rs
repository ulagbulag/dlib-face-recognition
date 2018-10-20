#![feature(test)]

extern crate test;
extern crate face_recognition;
extern crate image;
#[macro_use]
extern crate lazy_static;

use face_recognition::*;
use face_recognition::face_detection::*;
use face_recognition::landmark_prediction::*;
use face_recognition::face_encoding::*;
use image::*;

use std::path::*;

use test::Bencher;

fn load_image(filename: &str) -> RgbImage {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("benches").join(filename);
    image::open(&path).unwrap().to_rgb()
}

lazy_static! {
    static ref DETECTOR: FaceDetector = FaceDetector::default();
    static ref DETECTOR_CNN: FaceDetectorCnn = FaceDetectorCnn::default();
    static ref PREDICTOR: LandmarkPredictor = LandmarkPredictor::default();
    static ref MODEL: FaceEncodingNetwork = FaceEncodingNetwork::default();
    
    static ref OBAMA_1: RgbImage = load_image("obama_1.jpg");
    static ref OBAMA_2: RgbImage = load_image("obama_2.jpg");
    
    static ref OBAMA_1_MATRIX: ImageMatrix = ImageMatrix::from_image(&OBAMA_1);
    static ref OBAMA_2_MATRIX: ImageMatrix = ImageMatrix::from_image(&OBAMA_2);
}

#[cfg(feature = "download-models")]
fn initialize() {
    lazy_static::initialize(&DETECTOR);
    lazy_static::initialize(&DETECTOR_CNN);
    lazy_static::initialize(&PREDICTOR);
    lazy_static::initialize(&MODEL);
    
    lazy_static::initialize(&OBAMA_1);
    lazy_static::initialize(&OBAMA_2);

    lazy_static::initialize(&OBAMA_1_MATRIX);
    lazy_static::initialize(&OBAMA_2_MATRIX);
}

#[cfg(not(feature = "download-models"))]
fn initialize() {
    panic!("You need to run these benchmarks with '--features download-models'.");
}

#[bench]
fn bench_image_matrix_loading(bencher: &mut Bencher) {
    initialize();

    bencher.iter(|| ImageMatrix::from_image(&OBAMA_1));
}

#[bench]
fn bench_face_detection(bencher: &mut Bencher) {
    initialize();
    
    bencher.iter(|| {
        assert_eq!(DETECTOR.face_locations(&OBAMA_1_MATRIX).len(), 1)
    });
}

// This benchmark is super slow to run, so turn it off by default
/*#[bench]
fn bench_face_detection_cnn(bencher: &mut Bencher) {
    initialize();

    bencher.iter(|| {
        assert_eq!(DETECTOR_CNN.face_locations(&OBAMA_1_MATRIX).len(), 1)
    });
}*/

#[bench]
fn bench_face_landmark_detection(bencher: &mut Bencher) {
    initialize();
    
    let rect = DETECTOR.face_locations(&OBAMA_1_MATRIX)[0];

    bencher.iter(|| {
        PREDICTOR.face_landmarks(&OBAMA_1_MATRIX, &rect)
    });
}

#[bench]
fn bench_face_encoding(bencher: &mut Bencher) {
    initialize();

    let rect = DETECTOR.face_locations(&OBAMA_1_MATRIX)[0];
    let landmarks = PREDICTOR.face_landmarks(&OBAMA_1_MATRIX, &rect);

    bencher.iter(|| {
        MODEL.get_face_encodings(&OBAMA_1_MATRIX, &[landmarks.clone()], 0)
    });
}

#[bench]
fn encoding_distances(bencher: &mut Bencher) {
    initialize();

    let a = &OBAMA_1_MATRIX;
    let b = &OBAMA_2_MATRIX;

    let a_rect = DETECTOR.face_locations(&a)[0];
    let b_rect = DETECTOR.face_locations(&b)[0];

    let a_landmarks = PREDICTOR.face_landmarks(&a, &a_rect);
    let b_landmarks = PREDICTOR.face_landmarks(&b, &b_rect);

    let a_encoding = &MODEL.get_face_encodings(&a, &[a_landmarks], 0)[0];
    let b_encoding = &MODEL.get_face_encodings(&b, &[b_landmarks], 0)[0];

    bencher.iter(|| {
        let distance = a_encoding.distance(b_encoding);
        assert!(distance > 0.0 && distance < 0.6);
    });
}