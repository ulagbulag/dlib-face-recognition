//! Face Recognition
//!
//! Recognising a face via the dlib models provided takes 4 steps:
//!
//! - First, A face has to be detected in an image. This is done by first converting a [image] to dlibs matrix format,
//! then running it through either of the face detectors.
//! - Second, face landmarks have to be predicted. This is called prediction because it only really a guess,
//! and no matter what the number of landmarks returned will be the number of landmarks defined in the model.
//! This takes an image and a face rectangle and generates a series of landmark points on the face,
//! nose, mouth, eyes, etc.
//! - Then the image and these encodings can be run through the face encoding network to generate encodings of the faces.
//! These encodings consist of 128 floating point numbers that represent the face in 128-dimensional space.
//! To determine if two face encodings belong to the same face, the euclideon distance between them can be used.
//! For the dlib encodings, a distance of 0.6 is generally appropriate.

// Ignore the `forget_copy` clippy lint to remove noise from `cargo clippy` output
#![cfg_attr(feature = "cargo-clippy", allow(forget_copy))]

#![recursion_limit="1024"]
#[macro_use]
extern crate cpp;
extern crate image;

mod image_matrix;
pub mod face_detection;
pub mod landmark_prediction;
pub mod face_encoding;

pub use image_matrix::*;

use std::path::*;
use std::ffi::*;

#[cfg(feature = "download-models")]
fn path_for_file(filename: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("files").join(filename)
}

fn path_as_cstring(path: &Path) -> Result<CString, String> {    
    if !path.exists() {
        Err(format!("File not found: '{}'", path.display()))
    } else {
        let string = path.to_str().unwrap();
        Ok(CString::new(string).unwrap())
    }
}

#[derive(Debug, Default, Copy, Clone, PartialEq)]
#[repr(C)]
/// A 2D Point.
pub struct Point {
    pub x: i64,
    pub y: i64
}

#[derive(Debug, Default, Clone, Copy, PartialEq)]
#[repr(C)]
/// A Rectangle.
pub struct Rectangle {
    pub left: u64,
    pub top: u64,
    pub right: u64,
    pub bottom: u64
}

cpp!{{
    #include <dlib/image_processing/frontal_face_detector.h>
    #include <dlib/image_processing/full_object_detection.h>
    #include <dlib/dnn.h>

    using namespace dlib;

    // face encoding network definition from
    // https://github.com/davisking/dlib/blob/master/tools/python/src/face_recognition.cpp

    template <template <int,template<typename>class,int,typename> class block, int N, template<typename>class BN, typename SUBNET>
    using residual = add_prev1<block<N,BN,1,tag1<SUBNET>>>;

    template <template <int,template<typename>class,int,typename> class block, int N, template<typename>class BN, typename SUBNET>
    using residual_down = add_prev2<avg_pool<2,2,2,2,skip1<tag2<block<N,BN,2,tag1<SUBNET>>>>>>;

    template <int N, template <typename> class BN, int stride, typename SUBNET> 
    using block  = BN<con<N,3,3,1,1,relu<BN<con<N,3,3,stride,stride,SUBNET>>>>>;

    template <int N, typename SUBNET> using ares      = relu<residual<block,N,affine,SUBNET>>;
    template <int N, typename SUBNET> using ares_down = relu<residual_down<block,N,affine,SUBNET>>;

    template <typename SUBNET> using alevel0 = ares_down<256,SUBNET>;
    template <typename SUBNET> using alevel1 = ares<256,ares<256,ares_down<256,SUBNET>>>;
    template <typename SUBNET> using alevel2 = ares<128,ares<128,ares_down<128,SUBNET>>>;
    template <typename SUBNET> using alevel3 = ares<64,ares<64,ares<64,ares_down<64,SUBNET>>>>;
    template <typename SUBNET> using alevel4 = ares<32,ares<32,ares<32,SUBNET>>>;

    using face_encoding_nn = loss_metric<fc_no_bias<128,avg_pool_everything<
                                alevel0<
                                alevel1<
                                alevel2<
                                alevel3<
                                alevel4<
                                max_pool<3,3,2,2,relu<affine<con<32,7,7,2,2,
                                input_rgb_image_sized<150>
                                >>>>>>>>>>>>;

    // cnn face detector definition from
    // https://github.com/davisking/dlib/blob/master/tools/python/src/cnn_face_detector.cpp#L121

    template <long num_filters, typename SUBNET> using con5d = con<num_filters,5,5,2,2,SUBNET>;
    template <long num_filters, typename SUBNET> using con5  = con<num_filters,5,5,1,1,SUBNET>;

    template <typename SUBNET> using downsampler  = relu<affine<con5d<32, relu<affine<con5d<32, relu<affine<con5d<16,SUBNET>>>>>>>>>;
    template <typename SUBNET> using rcon5  = relu<affine<con5<45,SUBNET>>>;

    using face_detection_cnn = loss_mmod<con<1,9,9,1,1,rcon5<rcon5<rcon5<downsampler<input_rgb_image_pyramid<pyramid_down<6>>>>>>>>;

    // misc

    // todo: I am unsure if having rnd as a global here is thread safe.

    dlib::rand rnd;

    // https://github.com/davisking/dlib/blob/master/tools/python/src/face_recognition.cpp#L131
    std::vector<matrix<rgb_pixel>> jitter_image(const matrix<rgb_pixel>& img, const int num_jitters) {
        std::vector<matrix<rgb_pixel>> crops; 
        for (int i = 0; i < num_jitters; ++i) {
            crops.push_back(dlib::jitter_image(img, rnd));
        }
        return crops;
    }
}}

#[test]
fn test_default_image() {
    use face_detection::*;

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
            return point(-1000, -1000);
        })
    };

    assert_eq!(point, Point {x: -1000, y: -1000});
}