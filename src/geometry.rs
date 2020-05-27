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
    pub left: u64,
    pub top: u64,
    pub right: u64,
    pub bottom: u64,
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
        cpp!([] -> Point as "point" {
            return point(42, -1000);
        })
    };

    assert_eq!(point, Point { x: 42, y: -1000 });
}
