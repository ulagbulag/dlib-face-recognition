use std::ops::Deref;
use std::slice;

use crate::geometry::Point;

cpp_class!(
    /// A wrapper around the dlib `full_object_detection` class, which internally has a `std::vector<point>`.
    /// https://github.com/davisking/dlib/blob/master/dlib/image_processing/full_object_detection.h#L21
    pub unsafe struct FaceLandmarks as "dlib::full_object_detection"
);

impl Deref for FaceLandmarks {
    type Target = [Point];

    fn deref(&self) -> &Self::Target {
        let len = unsafe {
            cpp!([self as "dlib::full_object_detection*"] -> usize as "size_t" {
                return self->num_parts();
            })
        };

        if len == 0 {
            &[]
        } else {
            unsafe {
                // We can do this because we know that it uses a std::vector internally and part(0) is the first item
                let pointer = cpp!([self as "dlib::full_object_detection*"] -> *const Point as "dlib::point*" {
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
