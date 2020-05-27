//! Structs for predicting face landmark locations from images and face rectangles.

mod base;
mod landmarks;
mod model;

pub use self::base::LandmarkPredictorTrait;
pub use self::landmarks::FaceLandmarks;
pub use self::model::LandmarkPredictor;
