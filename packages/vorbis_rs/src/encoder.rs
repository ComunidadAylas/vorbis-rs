mod encoder_util;
pub use encoder_util::VorbisBitrateManagementStrategy;

mod encoder_impl;
pub use encoder_impl::{VorbisEncoder, VorbisEncoderBuilder};
