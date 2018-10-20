//! Face detection structs.

use {path_as_cstring, path_for_file, Rectangle};
use image_matrix::*;

use std::ops::*;
use std::path::*;
use std::{fmt, slice};


cpp_class!(unsafe struct FaceDetectorInner as "frontal_face_detector");

#[derive(Clone)]
/// A Face detector that uses a HOG feature descriptor.
///
/// Pretty fast (~100ms for test images on my machine), but not as accurate (misses more faces)
/// as the neural network face detector.
pub struct FaceDetector {
    inner: FaceDetectorInner
}

impl FaceDetector {
    pub fn new() -> Self {
        let inner = unsafe {
            cpp!([] -> FaceDetectorInner as "frontal_face_detector" {
                return get_frontal_face_detector();
            })
        };

        Self {
            inner
        }
    }

    pub fn face_locations(&self, image: &ImageMatrix) -> FaceLocations {
        let detector = &self.inner;

        unsafe {
            cpp!([detector as "frontal_face_detector*", image as "matrix<rgb_pixel>*"] -> FaceLocations as "std::vector<rectangle>"  {
                return (*detector)(*image);
            })
        }
    }
}

impl Default for FaceDetector {
    fn default() -> Self {
        Self::new()
    }
}

cpp_class!(unsafe struct FaceDetectorCnnInner as "face_detection_cnn");

/// A face detector that uses a Convulsive Neural Network (CNN).
///
/// This is much slower than the regular face detector (depending on the gpu), but is also much more accurate.
#[derive(Clone)]
pub struct FaceDetectorCnn {
    inner: FaceDetectorCnnInner
}

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
                    deserialize(filename) >> *network;
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

    pub fn face_locations(&self, image: &ImageMatrix) -> FaceLocations {
        let detector = &self.inner;

        unsafe {
            cpp!([detector as "face_detection_cnn*", image as "matrix<rgb_pixel>*"] -> FaceLocations as "std::vector<rectangle>" {
                std::vector<mmod_rect> detections = (*detector)(*image);
                
                // Convert from mmod rectangles
                // see: https://github.com/davisking/dlib/blob/master/dlib/image_processing/full_object_detection.h#L132
                // to regular rectangles

                std::vector<rectangle> rects;
                rects.reserve(detections.size());

                for (mmod_rect &detection: detections) {
                    rects.push_back(detection.rect);
                }

                return rects;
            })
        }
    }
}

impl Default for FaceDetectorCnn {
    fn default() -> Self {
        Self::new(path_for_file("mmod_human_face_detector.dat")).unwrap()
    }
}

cpp_class!(
    /// A rust wrapper around a `std::vector<rectangle>`.
    pub unsafe struct FaceLocations as "std::vector<rectangle>"
);

impl Deref for FaceLocations {
    type Target = [Rectangle];

    fn deref(&self) -> &Self::Target {
        let len = unsafe {
            cpp!([self as "std::vector<rectangle>*"] -> usize as "size_t" {
                return self->size();
            })
        };

        if len == 0 {
            &[]
        } else {
            unsafe {
                let pointer = cpp!([self as "std::vector<rectangle>*"] -> *const Rectangle as "rectangle*" {
                    return &(*self)[0];
                });

                slice::from_raw_parts(pointer, len)
            }
        }
    }
}

impl fmt::Debug for FaceLocations {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        self.deref().fmt(formatter)
    }
}

#[test]
fn face_detection_test() {
    use image;
    let image = image::open("benches/obama_1.jpg").unwrap().to_rgb();
    let matrix = ImageMatrix::from_image(&image);
    let detector = FaceDetector::new();

    let locations = detector.face_locations(&matrix);

    assert_eq!(locations.len(), 1);
    assert_eq!(locations[0], Rectangle { left: 305, top: 113, right: 520, bottom: 328 });
}