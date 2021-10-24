#![allow(dead_code)]
#![allow(unused_imports)]

#[macro_use]
extern crate lazy_static;

use std::ops::Deref;
use std::path::*;

use dlib_face_recognition::*;

use image::*;

fn load_image(filename: &str) -> RgbImage {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("assets")
        .join(filename);
    image::open(&path).unwrap().to_rgb8()
}

lazy_static! {
    static ref DETECTOR: FaceDetector = FaceDetector::default();
    static ref DETECTOR_CNN: FaceDetectorCnn = FaceDetectorCnn::default().unwrap();
    static ref PREDICTOR: LandmarkPredictor = LandmarkPredictor::default().unwrap();
    static ref MODEL: FaceEncoderNetwork = FaceEncoderNetwork::default().unwrap();
    static ref OBAMA_1: RgbImage = load_image("obama_1.jpg");
    static ref OBAMA_2: RgbImage = load_image("obama_2.jpg");
    static ref OBAMA_1_MATRIX: ImageMatrix = ImageMatrix::from_image(&OBAMA_1);
    static ref OBAMA_2_MATRIX: ImageMatrix = ImageMatrix::from_image(&OBAMA_2);
}

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

#[test]
fn test_image_matrix_loading() {
    initialize();

    ImageMatrix::from_image(&OBAMA_1);
}

#[test]
fn test_face_detection() {
    initialize();

    assert_eq!(DETECTOR.face_locations(&OBAMA_1_MATRIX).len(), 1);
}

#[test]
fn test_face_landmark_detection() {
    initialize();
    let rect = DETECTOR.face_locations(&OBAMA_1_MATRIX)[0];

    PREDICTOR.face_landmarks(&OBAMA_1_MATRIX, &rect);
}

#[test]
fn test_face_encoding() {
    initialize();

    let rect = DETECTOR.face_locations(&OBAMA_1_MATRIX)[0];
    let landmarks = PREDICTOR.face_landmarks(&OBAMA_1_MATRIX, &rect);

    MODEL.get_face_encodings(&OBAMA_1_MATRIX, &[landmarks], 0);
}

#[test]
fn encoding_distances() {
    initialize();

    let a = &OBAMA_1_MATRIX;
    let b = &OBAMA_2_MATRIX;

    let a_rect = DETECTOR.face_locations(&a)[0];
    let b_rect = DETECTOR.face_locations(&b)[0];

    let a_landmarks = PREDICTOR.face_landmarks(&a, &a_rect);
    let b_landmarks = PREDICTOR.face_landmarks(&b, &b_rect);

    let a_encoding = &MODEL.get_face_encodings(&a, &[a_landmarks], 0)[0];
    let b_encoding = &MODEL.get_face_encodings(&b, &[b_landmarks], 0)[0];

    let distance = a_encoding.distance(b_encoding);
    assert!(distance > 0.0 && distance < 0.6);
}
