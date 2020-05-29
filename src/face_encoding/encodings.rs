use std::ops::Deref;
use std::slice;

use super::encoding::FaceEncoding;

cpp_class!(
    /// A wrapper around a `std::vector<matrix<double,0,1>>`, a vector of encodings.
    pub unsafe struct FaceEncodings as "std::vector<dlib::matrix<double,0,1>>"
);

impl Deref for FaceEncodings {
    type Target = [FaceEncoding];

    fn deref(&self) -> &Self::Target {
        let len = unsafe {
            cpp!([self as "std::vector<dlib::matrix<double,0,1>>*"] -> usize as "size_t" {
                return self->size();
            })
        };

        if len == 0 {
            &[]
        } else {
            unsafe {
                let pointer = cpp!([self as "std::vector<dlib::matrix<double,0,1>>*"] -> *const FaceEncoding as "dlib::matrix<double,0,1>*" {
                    return &(*self)[0];
                });

                slice::from_raw_parts(pointer, len)
            }
        }
    }
}

#[test]
fn test_default_encoding() {
    let encodings = FaceEncodings::default();

    assert!(encodings.is_empty());
    assert_eq!(encodings.len(), 0);
    assert_eq!(encodings.get(0), None);
}

#[test]
fn test_sizes() {
    use std::mem::*;

    assert_eq!(size_of::<FaceEncodings>(), size_of::<Vec<FaceEncoding>>());
}
