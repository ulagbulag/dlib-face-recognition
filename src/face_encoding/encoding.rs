use std::fmt;
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
                for (size_t i = 0; i < 128; i++) {
                    inner(i) = scalar;
                }

                return inner;
            })
        };

        Self { inner }
    }

    /// Create a new encoding using previously stored values
    /// from a f64 Vec.
    pub fn from_vec(values: &Vec<f64>) -> Result<Self, ArraySizeError> {
        match values.len() {
            128 => {
                let values = values.as_ptr();
                let inner = unsafe {
                    cpp!([values as "const double *"] -> FaceEncodingInner as "dlib::matrix<double,0,1>" {

                        auto inner = dlib::matrix<double,0,1>(128);
                        for (int i = 0; i < 128; i++) {
                            inner(i) = values[i];
                        }

                        return inner;
                    })
                };

                Ok(Self { inner })
            }
            _ => Err(ArraySizeError),
        }
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

impl fmt::Debug for FaceEncoding {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        self.as_ref().fmt(fmt)
    }
}

impl PartialEq for FaceEncoding {
    fn eq(&self, other: &Self) -> bool {
        self.as_ref().eq(other.as_ref())
    }
}

impl AsRef<[f64]> for FaceEncoding {
    fn as_ref(&self) -> &[f64] {
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

impl TryFrom<Vec<f64>> for FaceEncoding {
    type Error = ArraySizeError;

    fn try_from(value: Vec<f64>) -> Result<Self, Self::Error> {
        Self::from_vec(&value)
    }
}

pub struct ArraySizeError;

impl fmt::Debug for ArraySizeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{{ file: {}, line: {} }}", file!(), line!()) // programmer-facing output
    }
}

impl fmt::Display for ArraySizeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Invalid array size provided for from_vec.")
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
