use super::encodings::FaceEncodings;
use crate::landmark_prediction::FaceLandmarks;
use crate::matrix::ImageMatrix;

pub trait FaceEncoderTrait {
    /// Get a number of face encodings from an image and a list of landmarks, and jitter them a certain amount.
    ///
    /// It is recommended to keep `num_jitters` at 0 unless you know what you're doing.
    fn get_face_encodings(
        &self,
        image: &ImageMatrix,
        landmarks: &[FaceLandmarks],
        num_jitters: u32,
    ) -> FaceEncodings;
}
