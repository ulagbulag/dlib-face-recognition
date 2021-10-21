use std::ops::Deref;

#[derive(Copy, Clone, Default, Debug, PartialEq, Eq)]
#[repr(C)]
/// A 2D Point.
pub struct Point([i64; 2]);

impl AsRef<[i64]> for Point {
    fn as_ref(&self) -> &[i64] {
        &self.0
    }
}

impl Deref for Point {
    type Target = [i64; 2];

    fn deref(&self) -> &[i64; 2] {
        &self.0
    }
}

impl Point {
    pub fn new(x: i64, y: i64) -> Self {
        Self([x, y])
    }

    pub fn x(&self) -> i64 {
        self.0[0]
    }

    pub fn y(&self) -> i64 {
        self.0[1]
    }
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

impl Rectangle {
    pub fn width(&self) -> i64 {
        self.right - self.left
    }

    pub fn height(&self) -> i64 {
        self.bottom - self.top
    }

    pub fn size(&self) -> Point {
        Point::new(self.width(), self.height())
    }

    pub fn center_x(&self) -> f64 {
        (self.left + self.right) as f64 / 2.0
    }

    pub fn center_y(&self) -> f64 {
        (self.top + self.bottom) as f64 / 2.0
    }

    pub fn center(&self) -> [f64; 2] {
        [self.center_x(), self.center_y()]
    }
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

    //assert_eq!(point, Point { x: 42, y: -1000 });
    assert_eq!(point, Point { 0: [42, -1000] });
}
