use std::path::Path;

use super::base::LandmarkPredictorTrait;
use super::landmarks::FaceLandmarks;
use crate::base::path_as_cstring;
use crate::geometry::Rectangle;
use crate::matrix::ImageMatrix;

/// A face landmark predictor.
#[derive(Clone)]
pub struct LandmarkPredictor {
    inner: LandmarkPredictorInner,
}

cpp_class!(unsafe struct LandmarkPredictorInner as "dlib::shape_predictor");

impl LandmarkPredictor {
    /// Deserialize the landmark predictor from a file path.
    pub fn new<P: AsRef<Path>>(filename: P) -> Result<Self, String> {
        let string = path_as_cstring(filename.as_ref())?;
        let inner = LandmarkPredictorInner::default();

        let deserialized = unsafe {
            let filename = string.as_ptr();
            let predictor = &inner;

            cpp!([filename as "char*", predictor as "dlib::shape_predictor*"] -> bool as "bool" {
                try {
                    dlib::deserialize(filename) >> *predictor;
                    return true;
                } catch (const dlib::error& exception) {
                    return false;
                }
            })
        };

        if !deserialized {
            Err(format!(
                "Failed to deserialize '{}'",
                filename.as_ref().display()
            ))
        } else {
            Ok(Self { inner })
        }
    }
}

#[cfg(feature = "embed-lp")]
impl Default for LandmarkPredictor {
    fn default() -> Self {
        Self::new(crate::embed::path_for_file(
            "shape_predictor_68_face_landmarks.dat",
        ))
        .unwrap()
    }
}

impl LandmarkPredictorTrait for LandmarkPredictor {
    fn face_landmarks(&self, image: &ImageMatrix, rect: &Rectangle) -> FaceLandmarks {
        let predictor = &self.inner;

        unsafe {
            cpp!([predictor as "dlib::shape_predictor*", image as "dlib::matrix<dlib::rgb_pixel>*", rect as "dlib::rectangle*"] -> FaceLandmarks as "dlib::full_object_detection" {
                return (*predictor)(*image, *rect);
            })
        }
    }
}
