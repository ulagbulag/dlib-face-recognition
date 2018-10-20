use std::path::*;
use {path_as_cstring, path_for_file, Rectangle, Point};
use image_matrix::*;

use std::ops::*;
use std::slice;

cpp_class!(unsafe struct LandmarkPredictorInner as "shape_predictor");

#[derive(Clone)]
pub struct LandmarkPredictor {
    inner: LandmarkPredictorInner
}

impl LandmarkPredictor {
    pub fn new<P: AsRef<Path>>(filename: P) -> Result<Self, String> {
        let string = path_as_cstring(filename.as_ref())?;
        
        let inner = LandmarkPredictorInner::default();

        let deserialized = unsafe {
            let filename = string.as_ptr();
            let predictor = &inner;

            cpp!([filename as "char*", predictor as "shape_predictor*"] -> bool as "bool" {
                try {
                    deserialize(filename) >> *predictor;
                    return true;
                } catch (const error& exception) {
                    return false;
                }
            })
        };

        if !deserialized {
            Err(format!("Failed to deserialize '{}'", filename.as_ref().display()))
        } else {
            Ok(Self {inner})
        }
    }

    pub fn face_landmarks(&self, image: &ImageMatrix, rect: &Rectangle) -> FaceLandmarks {
        let predictor = &self.inner;

        unsafe {
            cpp!([predictor as "shape_predictor*", image as "matrix<rgb_pixel>*", rect as "rectangle*"] -> FaceLandmarks as "full_object_detection" {
                return (*predictor)(*image, *rect);
            })
        }
    }
}

impl Default for LandmarkPredictor {
    fn default() -> Self {
        Self::new(path_for_file("shape_predictor_68_face_landmarks.dat")).unwrap()
    }
}

/// https://github.com/davisking/dlib/blob/master/dlib/image_processing/full_object_detection.h#L21
cpp_class!(pub unsafe struct FaceLandmarks as "full_object_detection");

impl Deref for FaceLandmarks {
    type Target = [Point];

    fn deref(&self) -> &Self::Target {
        let len = unsafe {
            cpp!([self as "full_object_detection*"] -> usize as "size_t" {
                return self->num_parts();
            })
        };

        if len == 0 {
            &[]
        } else {
            unsafe {
                // We can do this because we know that it uses a std::vector internally and part(0) is the first item
                let pointer = cpp!([self as "full_object_detection*"] -> *const Point as "point*" {
                    return &self->part(0);
                });

                slice::from_raw_parts(pointer, len)
            }
        }
    }
}

#[test]
fn test_default_landmarks() {
    // ensure that FaceLandmarks::default() doesnt allow memory violations in safe code
    let landmarks = FaceLandmarks::default();
    assert!(landmarks.is_empty());
    assert_eq!(landmarks.len(), 0);
    assert_eq!(landmarks.get(0), None);
}