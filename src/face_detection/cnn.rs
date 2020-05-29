use std::path::Path;

use super::base::FaceDetectorTrait;
use super::location::FaceLocations;
use crate::base::path_as_cstring;
use crate::matrix::ImageMatrix;

/// A face detector that uses a Convulsive Neural Network (CNN).
///
/// This is much slower than the regular face detector (depending on the gpu), but is also much more accurate.
#[derive(Clone)]
pub struct FaceDetectorCnn {
    inner: FaceDetectorCnnInner,
}

cpp_class!(unsafe struct FaceDetectorCnnInner as "face_detection_cnn");

impl FaceDetectorCnn {
    /// Create a new face detector from a filename
    pub fn new<P: AsRef<Path>>(filename: P) -> Result<Self, String> {
        let string = path_as_cstring(filename.as_ref())?;
        let inner = FaceDetectorCnnInner::default();

        let deserialized = unsafe {
            let filename = string.as_ptr();
            let network = &inner;

            cpp!([filename as "char*", network as "face_detection_cnn*"] -> bool as "bool" {
                try {
                    dlib::deserialize(filename) >> *network;
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

#[cfg(feature = "embed-fd-nn")]
impl Default for FaceDetectorCnn {
    fn default() -> Self {
        Self::new(crate::embed::path_for_file("mmod_human_face_detector.dat")).unwrap()
    }
}

impl FaceDetectorTrait for FaceDetectorCnn {
    fn face_locations(&self, image: &ImageMatrix) -> FaceLocations {
        let detector = &self.inner;

        unsafe {
            cpp!([detector as "face_detection_cnn*", image as "dlib::matrix<dlib::rgb_pixel>*"] -> FaceLocations as "std::vector<dlib::rectangle>" {
                std::vector<dlib::mmod_rect> detections = (*detector)(*image);
                // Convert from mmod rectangles
                // see: https://github.com/davisking/dlib/blob/master/dlib/image_processing/full_object_detection.h#L132
                // to regular rectangles

                std::vector<dlib::rectangle> rects;
                rects.reserve(detections.size());

                for (auto &detection: detections) {
                    rects.push_back(detection.rect);
                }

                return rects;
            })
        }
    }
}
