use super::landmarks::FaceLandmarks;
use crate::geometry::Rectangle;
use crate::matrix::ImageMatrix;

pub trait LandmarkPredictorTrait {
    /// Detect face landmarks.
    ///
    /// This will generally always return the number of landmarks as defined by the model.
    fn face_landmarks(&self, image: &ImageMatrix, rect: &Rectangle) -> FaceLandmarks;
}
