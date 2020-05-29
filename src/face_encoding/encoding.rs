use std::fmt;
use std::ops::Deref;
use std::slice;

/// A wrapper around a `matrix<double,0,1>>`, an encoding.
#[derive(Clone)]
pub struct FaceEncoding {
    inner: FaceEncodingInner,
}

cpp_class!(unsafe struct FaceEncodingInner as "dlib::matrix<double,0,1>");

impl FaceEncoding {
    /// Create a new encoding initialised with a scalar value.
    ///
    /// Mostly used for testing purposes.
    pub fn new_from_scalar(scalar: f64) -> Self {
        let inner = unsafe {
            cpp!([scalar as "double"] -> FaceEncodingInner as "dlib::matrix<double,0,1>" {
                auto inner = dlib::matrix<double,0,1>(128);
                for (int i = 0; i < 128; i++) {
                    inner(i) = scalar;
                }

                return inner;
            })
        };

        Self { inner }
    }

    /// Calculate the euclidean distance between two encodings.
    ///
    /// This value can be compared to a constant to determine if the faces are the same or not.
    /// A good value for this is `0.6`.
    pub fn distance(&self, other: &Self) -> f64 {
        unsafe {
            cpp!([self as "const dlib::matrix<double,0,1>*", other as "const dlib::matrix<double,0,1>*"] -> f64 as "double" {
                return dlib::length(*self - *other);
            })
        }
    }
}

impl Deref for FaceEncoding {
    type Target = [f64];

    fn deref(&self) -> &Self::Target {
        let matrix = &self.inner;

        let len = unsafe {
            cpp!([matrix as "const dlib::matrix<double,0,1>*"] -> usize as "size_t" {
                return matrix->size();
            })
        };

        if len == 0 {
            &[]
        } else {
            unsafe {
                let pointer = cpp!([matrix as "dlib::matrix<double,0,1>*"] -> *const f64 as "double*" {
                    return &(*matrix)(0);
                });

                slice::from_raw_parts(pointer, len)
            }
        }
    }
}

impl fmt::Debug for FaceEncoding {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        self.deref().fmt(fmt)
    }
}

impl PartialEq for FaceEncoding {
    fn eq(&self, other: &Self) -> bool {
        self.deref().eq(other.deref())
    }
}

#[test]
fn encoding_test() {
    let encoding_a = FaceEncoding::new_from_scalar(0.0);
    let encoding_b = FaceEncoding::new_from_scalar(1.0);

    assert_eq!(encoding_a, encoding_a);
    assert_ne!(encoding_a, encoding_b);

    assert_eq!(encoding_a.distance(&encoding_b), 128.0_f64.sqrt());
}
