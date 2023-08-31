use super::base::FaceDetectorTrait;
use super::location::FaceLocations;
use crate::matrix::ImageMatrix;
use std::cell::UnsafeCell;
use std::marker::PhantomData;

#[derive(Clone)]
/// A Face detector that uses a HOG feature descriptor.
///
/// Pretty fast (~100ms for test images on my machine), but not as accurate (misses more faces)
/// as the neural network face detector.
pub struct FaceDetector {
    inner: FaceDetectorInner,
    /// Face detector relies on
    /// [scan_fhog_pyramid](http://dlib.net/dlib/image_processing/scan_fhog_pyramid_abstract.h.html#scan_fhog_pyramid)
    /// which is not thread safe, this hack makes Rust aware of that. On nightly, we could use
    /// [negative impls](https://github.com/rust-lang/rust/issues/68318)
    ///
    /// Because [`UnsafeCell`] is [`Send`] (doesn't have thread local data), but not [`Sync`]
    /// (has interior mutabilitin in a not thread safe way), which captures the sutiation with
    /// scan_fhog_pyramid perfectly (you need a mutex for concurrent access to it)
    data: PhantomData<UnsafeCell<()>>,
}

cpp_class!(unsafe struct FaceDetectorInner as "dlib::frontal_face_detector");

impl FaceDetector {
    /// Create a new face detector.
    ///
    /// This is handles by dlib internally, so you do not need to worry about file paths.
    pub fn new() -> Self {
        let inner = unsafe {
            cpp!([] -> FaceDetectorInner as "dlib::frontal_face_detector" {
                return dlib::get_frontal_face_detector();
            })
        };

        Self {
            inner,
            data: std::marker::PhantomData::default(),
        }
    }
}

impl Default for FaceDetector {
    fn default() -> Self {
        Self::new()
    }
}

impl FaceDetectorTrait for FaceDetector {
    fn face_locations(&self, image: &ImageMatrix) -> FaceLocations {
        let detector = &self.inner;

        unsafe {
            cpp!([detector as "dlib::frontal_face_detector*", image as "dlib::matrix<dlib::rgb_pixel>*"] -> FaceLocations as "std::vector<dlib::rectangle>"  {
                return (*detector)(*image);
            })
        }
    }
}

#[test]
fn test_face_detection() {
    use crate::geometry::Rectangle;

    let image = image::open("assets/obama_1.jpg").unwrap().to_rgb8();
    let matrix = ImageMatrix::from_image(&image);
    let detector = FaceDetector::new();

    let locations = detector.face_locations(&matrix);

    assert_eq!(locations.len(), 1);
    assert_eq!(
        locations[0],
        Rectangle {
            left: 305,
            top: 113,
            right: 520,
            bottom: 328
        }
    );
}
