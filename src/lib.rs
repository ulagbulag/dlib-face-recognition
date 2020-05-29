//! Face Recognition
//!
//! Recognising a face via the dlib models provided takes 4 steps:
//!
//! - First, A face has to be detected in an image. This is done by first converting a [image] to dlibs matrix format,
//! then running it through either of the face detectors.
//! - Second, face landmarks have to be predicted. This is called prediction because it only really a guess,
//! and no matter what the number of landmarks returned will be the number of landmarks defined in the model.
//! This takes an image and a face rectangle and generates a series of landmark points on the face,
//! nose, mouth, eyes, etc.
//! - Then the image and these encodings can be run through the face encoding network to generate encodings of the faces.
//! These encodings consist of 128 floating point numbers that represent the face in 128-dimensional space.
//! To determine if two face encodings belong to the same face, the euclideon distance between them can be used.
//! For the dlib encodings, a distance of 0.6 is generally appropriate.
// Ignore the `forget_copy` clippy lint to remove noise from `cargo clippy` output

#![recursion_limit = "1024"]

#[macro_use]
extern crate cpp;

mod wrapper;

mod base;
mod embed;
mod face_detection;
mod face_encoding;
mod geometry;
mod landmark_prediction;
mod matrix;

pub use self::geometry::{Point, Rectangle};
pub use self::matrix::ImageMatrix;

pub use self::face_detection::{FaceDetector, FaceDetectorCnn, FaceDetectorTrait, FaceLocations};
pub use self::face_encoding::{
    FaceComparer, FaceEncoderNetwork, FaceEncoderTrait, FaceEncoding, FaceEncodings,
};
pub use self::landmark_prediction::{LandmarkPredictor, LandmarkPredictorTrait};
