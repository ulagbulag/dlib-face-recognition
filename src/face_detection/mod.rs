//! Face detection structs.

mod base;
mod cnn;
mod hog;
mod location;

pub use self::base::FaceDetectorTrait;
pub use self::cnn::FaceDetectorCnn;
pub use self::hog::FaceDetector;
pub use self::location::FaceLocations;

/*
//! Face detection structs.

use std::ops::*;
use std::path::*;
use std::{fmt, slice};

use crate::base::path_as_cstring;
use crate::geometry::*;
use crate::image_matrix::*;
*/
