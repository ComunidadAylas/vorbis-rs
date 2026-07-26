#![feature(extern_types)]
#![allow(warnings)]
#![allow(clippy::all)]

#[macro_use]
extern crate c2rust_bitfields;

pub mod lib {
	pub mod analysis;
	pub mod bitrate;
	pub mod block;
	pub mod codebook;
	pub mod envelope;
	pub mod floor0;
	pub mod floor1;
	pub mod info;
	pub mod lookup;
	pub mod lpc;
	pub mod lsp;
	pub mod mapping0;
	pub mod mdct;
	pub mod psy;
	pub mod registry;
	pub mod res0;
	pub mod sharedbook;
	pub mod smallft;
	pub mod synthesis;
	pub mod vorbisenc;
	pub mod vorbisfile;
	pub mod window;
}

// The following `pub use` statements re-export the flat, non-namespaced interface that
// `bindgen`-based bindings (such as the ones previously vended by the `aotuv_lancer_vorbis_sys`
// crate) used to expose for the same underlying C API, adapted to the module layout that
// `c2rust` produced when translating `libvorbis` and `libvorbisenc` on a file-by-file basis.
//
// Because each translated file redeclares the C API structs it needs instead of sharing a
// single definition, the modules below were edited to `use` a single canonical definition of
// each struct shared across translation units (rather than keep their own copy), so that values
// produced by one module's functions can be safely passed to another's, exactly like they could
// when produced by `bindgen`. Definitions of `libogg` structs (e.g. `oggpack_buffer`) are
// likewise shared with, and sourced from, the `ogg_c2rust_sys` crate.
//
// Note that `NOTOPEN`, an internal `libvorbisfile` state constant present in the `bindgen`-based
// bindings, has no equivalent here: `c2rust` does not emit unused constants, and `NOTOPEN` is
// never referenced by `vorbisfile.c`.

// Core data structs, shared verbatim across the modules re-exported below
// Packet analysis (encoding)
// Bitrate management
// `vorbis_block`/`vorbis_dsp_state` life cycle, and analysis/synthesis buffer management
// `vorbis_info`/`vorbis_comment` life cycle, and header packet (de)serialization
// Packet synthesis (decoding)
pub use lib::synthesis::{
	vorbis_packet_blocksize, vorbis_synthesis, vorbis_synthesis_halfrate,
	vorbis_synthesis_halfrate_p, vorbis_synthesis_trackonly, OV_ENOTAUDIO
};
// `libvorbisenc` setup and rate/quality control
pub use lib::vorbisenc::{
	ovectl_ratemanage2_arg, ovectl_ratemanage_arg, vorbis_encode_ctl, vorbis_encode_init,
	vorbis_encode_init_vbr, vorbis_encode_setup_init, vorbis_encode_setup_managed,
	vorbis_encode_setup_vbr, OV_ECTL_COUPLING_GET, OV_ECTL_COUPLING_SET, OV_ECTL_IBLOCK_GET,
	OV_ECTL_IBLOCK_SET, OV_ECTL_LOWPASS_GET, OV_ECTL_LOWPASS_SET, OV_ECTL_RATEMANAGE2_GET,
	OV_ECTL_RATEMANAGE2_SET, OV_ECTL_RATEMANAGE_AVG, OV_ECTL_RATEMANAGE_GET,
	OV_ECTL_RATEMANAGE_HARD, OV_ECTL_RATEMANAGE_SET
};
// `libvorbisfile` high-level stream access
pub use lib::vorbisfile::{
	ov_bitrate, ov_bitrate_instant, ov_callbacks, ov_clear, ov_comment, ov_crosslap, ov_halfrate,
	ov_halfrate_p, ov_info, ov_open_callbacks, ov_pcm_seek, ov_pcm_seek_lap, ov_pcm_seek_page,
	ov_pcm_seek_page_lap, ov_pcm_tell, ov_pcm_total, ov_raw_seek, ov_raw_seek_lap, ov_raw_tell,
	ov_raw_total, ov_read, ov_read_filter, ov_read_float, ov_seekable, ov_serialnumber, ov_streams,
	ov_test_callbacks, ov_test_open, ov_time_seek, ov_time_seek_lap, ov_time_seek_page,
	ov_time_seek_page_lap, ov_time_tell, ov_time_total, OggVorbis_File, INITSET, OPENED,
	OV_EBADHEADER, OV_EBADLINK, OV_EBADPACKET, OV_EFAULT, OV_EINVAL, OV_ENOSEEK, OV_ENOTVORBIS,
	OV_EOF, OV_EREAD, OV_FALSE, OV_HOLE, PARTOPEN, STREAMSET
};
pub use lib::{
	analysis::vorbis_analysis,
	bitrate::{vorbis_bitrate_addblock, vorbis_bitrate_flushpacket},
	block::{
		alloc_chain, vorbis_analysis_blockout, vorbis_analysis_buffer, vorbis_analysis_init,
		vorbis_analysis_wrote, vorbis_block, vorbis_block_clear, vorbis_block_init,
		vorbis_dsp_clear, vorbis_dsp_state, vorbis_synthesis_blockin, vorbis_synthesis_init,
		vorbis_synthesis_lapout, vorbis_synthesis_pcmout, vorbis_synthesis_read,
		vorbis_synthesis_restart
	},
	info::{
		vorbis_analysis_headerout, vorbis_comment, vorbis_comment_add, vorbis_comment_add_tag,
		vorbis_comment_clear, vorbis_comment_init, vorbis_comment_query,
		vorbis_comment_query_count, vorbis_commentheader_out, vorbis_granule_time, vorbis_info,
		vorbis_info_blocksize, vorbis_info_clear, vorbis_info_init, vorbis_synthesis_headerin,
		vorbis_synthesis_idheader, vorbis_version_string, OV_EIMPL, OV_EVERSION
	}
};
pub use ogg_c2rust_sys::oggpack_buffer;
