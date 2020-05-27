//! Face encoding structs.

mod base;
mod encoding;
mod encodings;
mod network;

pub use self::base::FaceEncoderTrait;
pub use self::encoding::FaceEncoding;
pub use self::encodings::FaceEncodings;
pub use self::network::FaceEncoderNetwork;
