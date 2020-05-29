use std::fmt;
use std::ops::Deref;
use std::slice;

use crate::geometry::Rectangle;

cpp_class!(
    /// A rust wrapper around a `std::vector<rectangle>`.
    pub unsafe struct FaceLocations as "std::vector<dlib::rectangle>"
);

impl Deref for FaceLocations {
    type Target = [Rectangle];

    fn deref(&self) -> &Self::Target {
        let len = unsafe {
            cpp!([self as "std::vector<dlib::rectangle>*"] -> usize as "size_t" {
                return self->size();
            })
        };

        if len == 0 {
            &[]
        } else {
            unsafe {
                let pointer = cpp!([self as "std::vector<dlib::rectangle>*"] -> *const Rectangle as "dlib::rectangle*" {
                    return &(*self)[0];
                });

                slice::from_raw_parts(pointer, len)
            }
        }
    }
}

impl fmt::Debug for FaceLocations {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.deref().fmt(f)
    }
}
