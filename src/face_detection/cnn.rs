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
    /// Loss layers don't specify whether thei are thread safe, so we asume they
    /// need to be held behind a mutex as stated in the dlib
    /// [documentation](http://dlib.net/intro.html)
    ///
    /// [`UnsafeCell`] is not [`Sync`] which forbids access to a
    /// shared reference (&Self) from multiple threads (requires a mutex),
    /// but implements [`Send`]
    data: std::marker::PhantomData<std::cell::UnsafeCell<()>>,
}

cpp_class!(unsafe struct FaceDetectorCnnInner as "face_detection_cnn");

impl FaceDetectorCnn {
    #[cfg(feature = "embed-fd-nn")]
    pub fn default() -> Result<Self, String> {
        use crate::embed::{check_file_or_download, ModelFile};

        let filename = ModelFile::FaceDetectorCnn;

        let default_filepath = crate::embed::path_for_file(&filename);

        check_file_or_download(&filename);

        Self::open(default_filepath)
    }

    /// Create a new face detector from a filename
    pub fn open<P: AsRef<Path>>(filename: P) -> Result<Self, String> {
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
            Ok(Self {
                inner,
                data: std::marker::PhantomData::default(),
            })
        }
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
