//! Face detection structs.

mod base;
mod cnn;
mod hog;
mod location;

pub use self::base::FaceDetectorTrait;
pub use self::cnn::FaceDetectorCnn;
pub use self::hog::FaceDetector;
pub use self::location::FaceLocations;
