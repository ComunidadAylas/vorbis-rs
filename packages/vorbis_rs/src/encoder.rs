mod encoder_util;
pub use encoder_util::{VorbisBitrateManagementStrategy, VorbisEncodingState};

mod encoder_impl;
pub use encoder_impl::{VorbisEncoder, VorbisEncoderBuilder};
