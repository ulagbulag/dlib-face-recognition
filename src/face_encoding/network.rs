use std::path::Path;

use super::base::FaceEncoderTrait;
use super::encodings::FaceEncodings;
use crate::base::path_as_cstring;
use crate::landmark_prediction::FaceLandmarks;
use crate::matrix::ImageMatrix;

/// A face encoding network.
#[derive(Clone)]
pub struct FaceEncoderNetwork {
    inner: FaceEncoderNetworkInner,
}

cpp_class!(unsafe struct FaceEncoderNetworkInner as "face_encoding_nn");

impl FaceEncoderNetwork {
    /// Deserialize the face encoding network from a file path.
    pub fn new<P: AsRef<Path>>(filename: P) -> Result<Self, String> {
        let string = path_as_cstring(filename.as_ref())?;

        let inner = FaceEncoderNetworkInner::default();

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
            Err(format!(
                "Failed to deserialize '{}'",
                filename.as_ref().display()
            ))
        } else {
            Ok(Self { inner })
        }
    }
}

#[cfg(feature = "embed-fe-nn")]
impl Default for FaceEncoderNetwork {
    fn default() -> Self {
        Self::new(crate::embed::path_for_file(
            "dlib_face_recognition_resnet_model_v1.dat",
        ))
        .unwrap()
    }
}

impl FaceEncoderTrait for FaceEncoderNetwork {
    fn get_face_encodings(
        &self,
        image: &ImageMatrix,
        landmarks: &[FaceLandmarks],
        num_jitters: u32,
    ) -> FaceEncodings {
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
