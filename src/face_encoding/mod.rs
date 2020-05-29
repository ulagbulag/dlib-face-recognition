//! Face encoding structs.

mod base;
mod compare;
mod encoding;
mod encodings;
mod nn;

pub use self::base::FaceEncoderTrait;
pub use self::compare::FaceComparer;
pub use self::encoding::FaceEncoding;
pub use self::encodings::FaceEncodings;
pub use self::nn::FaceEncoderNetwork;
