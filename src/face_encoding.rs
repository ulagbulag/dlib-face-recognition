//! Face encoding structs.

use std::path::*;
use std::ops::*;
use std::slice;
use std::fmt;

use *;
use landmark_prediction::*;
use image_matrix::*;

cpp_class!(unsafe struct FaceEncodingNetworkInner as "face_encoding_nn");

/// A face encoding network.
#[derive(Clone)]
pub struct FaceEncodingNetwork {
    inner: FaceEncodingNetworkInner
}

impl FaceEncodingNetwork {
    /// Deserialize the face encoding network from a file path.
    pub fn new<P: AsRef<Path>>(filename: P) -> Result<Self, String> {
        let string = path_as_cstring(filename.as_ref())?;

        let inner = FaceEncodingNetworkInner::default();

        let deserialized = unsafe {
            let filename = string.as_ptr();
            let network = &inner;

            cpp!([filename as "char*", network as "face_encoding_nn*"] -> bool as "bool" {
                try {
                    deserialize(filename) >> *network;
                    return true;
                } catch (const error& exception) {
                    return false;
                }
            })
        };

        if !deserialized {
            Err(format!("Failed to deserialize '{}'", filename.as_ref().display()))
        } else {
            Ok(Self {inner})
        }
    }

    /// Get a number of face encodings from an image and a list of landmarks, and jitter them a certain amount.
    /// 
    /// It is recommended to keep `num_jitters` at 0 unless you know what you're doing. 
    pub fn get_face_encodings(&self, image: &ImageMatrix, landmarks: &[FaceLandmarks], num_jitters: u32) -> FaceEncodings {
        let num_faces = landmarks.len();
        let landmarks = landmarks.as_ptr();
        let net = &self.inner;

        unsafe {
            cpp!([
                    net as "face_encoding_nn*",
                    image as "matrix<rgb_pixel>*",
                    landmarks as "full_object_detection*",
                    num_faces as "size_t",
                    num_jitters as "uint"
                ] -> FaceEncodings as "std::vector<matrix<double,0,1>>" {
                
                std::vector<matrix<double,0,1>> encodings;
                encodings.reserve(num_faces);

                // first we need to use the landmarks to get image chips for each face

                std::vector<chip_details> dets;
                dets.reserve(num_faces);

                array<matrix<rgb_pixel>> face_chips;
                
                for (size_t offset = 0; offset < num_faces; offset++) {
                    chip_details details = get_face_chip_details(*(landmarks + offset), 150, 0.25);
                    dets.push_back(details);
                }
                
                extract_image_chips(*image, dets, face_chips);

                // extract descriptors and convert from float vectors to double vectors

                if (num_jitters <= 1) {
                    auto network_output = (*net)(face_chips, 16);
                    for (matrix<float,0,1>& float_encoding: network_output) {
                        encodings.push_back((matrix_cast<double>(float_encoding)));
                    }
                } else {
                    for (auto& chip : face_chips) {
                        auto network_output = (*net)(jitter_image(chip, num_jitters), 16);
                        matrix<float,0,1> float_encoding = mean(mat(network_output));

                        encodings.push_back(matrix_cast<double>(float_encoding));
                    }
                }

                return encodings;
            })
        }
    }
}

#[cfg(feature = "download-models")]
impl Default for FaceEncodingNetwork {
    fn default() -> Self {
        Self::new(path_for_file("dlib_face_recognition_resnet_model_v1.dat")).unwrap()
    }
}


cpp_class!(
    /// A wrapper around a `std::vector<matrix<double,0,1>>`, a vector of encodings.
    pub unsafe struct FaceEncodings as "std::vector<matrix<double,0,1>>"
);

impl Deref for FaceEncodings {
    type Target = [FaceEncoding];
    
    fn deref(&self) -> &Self::Target {
        let len = unsafe {
            cpp!([self as "std::vector<matrix<double,0,1>>*"] -> usize as "size_t" {
                return self->size();
            })
        };

        if len == 0 {
            &[]
        } else {
            unsafe {
                let pointer = cpp!([self as "std::vector<matrix<double,0,1>>*"] -> *const FaceEncoding as "matrix<double,0,1>*" {
                    return &(*self)[0];
                });

                slice::from_raw_parts(pointer, len)
            }
        }
    }
}

cpp_class!(unsafe struct FaceEncodingInner as "matrix<double,0,1>");

/// A wrapper around a `matrix<double,0,1>>`, an encoding.
#[derive(Clone)]
pub struct FaceEncoding {
    inner: FaceEncodingInner
}

impl FaceEncoding {
    /// Create a new encoding initialised with a scalar value.
    ///
    /// Mostly used for testing purposes.
    pub fn new_from_scalar(scalar: f64) -> Self {
        let inner = unsafe {
            cpp!([scalar as "double"] -> FaceEncodingInner as "matrix<double,0,1>" {
                auto inner = matrix<double,0,1>(128);
                
                for (int i = 0; i < 128; i++) {
                    inner(i) = scalar;
                }

                return inner;
            })
        };

        Self {
            inner
        }
    }

    /// Calculate the euclidean distance between two encodings.
    ///
    /// This value can be compared to a constant to determine if the faces are the same or not.
    /// A good value for this is `0.6`.
    pub fn distance(&self, other: &Self) -> f64 {
        unsafe {
            cpp!([self as "matrix<double,0,1>*", other as "matrix<double,0,1>*"] -> f64 as "double" {
                return length(*self - *other);
            })
        }
    }
}

impl Deref for FaceEncoding {
    type Target = [f64];

    fn deref(&self) -> &Self::Target {
        let matrix = &self.inner;

        let len = unsafe {
            cpp!([matrix as "matrix<double,0,1>*"] -> usize as "size_t" {
                return matrix->size();
            })
        };

        if len == 0 {
            &[]
        } else {
            unsafe {
                let pointer = cpp!([matrix as "matrix<double,0,1>*"] -> *const f64 as "double*" {
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
    assert_eq!(
        size_of::<FaceEncodings>(),
        size_of::<Vec<FaceEncoding>>()
    );
}