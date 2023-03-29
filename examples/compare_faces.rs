#[cfg(feature = "embed-all")]

use dlib_face_recognition::*;
use dlib_face_recognition::FaceDetectorCnn;

fn tick<R>(name: &str, f: impl Fn() -> R) -> R {
    let now = std::time::Instant::now();
    let result = f();
    println!("[{}] elapsed time: {}ms", name, now.elapsed().as_millis());
    result
}

#[cfg(feature="embed-all")]
fn main() {
    let mut args = std::env::args().skip(1);
    let first_photo_path = args.next().unwrap();
    let second_photo_path = args.next().unwrap();

    let first_photo = image::open(first_photo_path).unwrap().to_rgb8();
    let matrix_photo_1 = ImageMatrix::from_image(&first_photo);

    let second_photo = image::open(second_photo_path).unwrap().to_rgb8();
    let matrix_photo_2 = ImageMatrix::from_image(&second_photo);

    //let hog_detector = FaceDetector::default();
    
    let Ok(cnn_detector) = FaceDetectorCnn::default() else {
        panic!("Error loading Face Detector (CNN).");
    };

    let Ok(landmarks) = LandmarkPredictor::default() else {
        panic!("Error loading Landmark Predictor.");
    };

    let Ok(face_encoder) = FaceEncoderNetwork::default() else {
        panic!("Error loading Face Encoder.");
    };

    let face_locations_photo_1 = tick("FaceDetectorCnn", || cnn_detector.face_locations(&matrix_photo_1));

    let face_locations_photo_2 = tick("FaceDetectorCnn", || cnn_detector.face_locations(&matrix_photo_2));

    let face_1 = face_locations_photo_1.first().unwrap();
    let face_2 = face_locations_photo_2.first().unwrap();

    let landmarks_face_1 = landmarks.face_landmarks(&matrix_photo_1, face_1);

    let landmarks_face_2 = landmarks.face_landmarks(&matrix_photo_2, face_2);

    let encodings_face_1 = face_encoder.get_face_encodings(&matrix_photo_1, &[landmarks_face_1], 0);

    let encodings_face_2 = face_encoder.get_face_encodings(&matrix_photo_2, &[landmarks_face_2], 0);

    let first_face_measurements = encodings_face_1.first().unwrap();

    let second_face_measurements = encodings_face_2.first().unwrap();

    let distance = first_face_measurements.distance(second_face_measurements
    );

    println!("Euclidean distance of chosen faces: {distance}");
}

#[cfg(not(feature = "embed-all"))]
fn main() {
    panic!("You need to run this example with '--features embed-all'.");
}