#[derive(Copy, Clone, Default, Debug, PartialEq)]
#[repr(C)]
/// A 2D Point.
pub struct Point {
    pub x: i64,
    pub y: i64,
}

#[derive(Copy, Clone, Default, Debug, PartialEq)]
#[repr(C)]
/// A Rectangle.
pub struct Rectangle {
    pub left: i64,
    pub top: i64,
    pub right: i64,
    pub bottom: i64,
}

#[test]
fn test_default_image() {
    use crate::face_detection::{FaceDetector, FaceDetectorTrait};
    use crate::matrix::ImageMatrix;

    let matrix = ImageMatrix::default();
    let face_det = FaceDetector::default();

    let locations = face_det.face_locations(&matrix);

    assert!(locations.is_empty());
    assert_eq!(locations.len(), 0);
    assert_eq!(locations.get(0), None);
}

#[test]
fn test_point() {
    let point = unsafe {
        cpp!([] -> Point as "dlib::point" {
            return dlib::point(42, -1000);
        })
    };

    assert_eq!(point, Point { x: 42, y: -1000 });
}
