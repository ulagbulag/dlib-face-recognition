#[macro_use]
extern crate cpp;

mod wrapper;

use dlib_face_recognition::ImageMatrix;
use opencv::prelude::Mat;

/// Copy a matrix from an opencv mat
pub fn matrix_to_opencv_mat(mat: &Mat) -> ImageMatrix {
    let mat = mat.as_raw_Mat();

    unsafe {
        cpp!([mat as "const cv::Mat*"] -> ImageMatrix as "dlib::matrix<dlib::rgb_pixel>" {
            dlib::cv_image<dlib::bgr_pixel> image(*mat);
            dlib::matrix<dlib::rgb_pixel> out;

            dlib::assign_image(out, image);
            return out;
        })
    }
}
