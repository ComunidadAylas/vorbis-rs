extern "C" {
	fn malloc(__size: size_t) -> *mut ::core::ffi::c_void;
	fn calloc(__nmemb: size_t, __size: size_t) -> *mut ::core::ffi::c_void;
	fn realloc(__ptr: *mut ::core::ffi::c_void, __size: size_t) -> *mut ::core::ffi::c_void;
	fn free(__ptr: *mut ::core::ffi::c_void);
	fn memcpy(
		__dest: *mut ::core::ffi::c_void,
		__src: *const ::core::ffi::c_void,
		__n: size_t
	) -> *mut ::core::ffi::c_void;
	fn memmove(
		__dest: *mut ::core::ffi::c_void,
		__src: *const ::core::ffi::c_void,
		__n: size_t
	) -> *mut ::core::ffi::c_void;
	fn memset(
		__s: *mut ::core::ffi::c_void,
		__c: ::core::ffi::c_int,
		__n: size_t
	) -> *mut ::core::ffi::c_void;
	fn oggpack_writeinit(b: *mut oggpack_buffer);
	fn oggpack_writeclear(b: *mut oggpack_buffer);
	fn mdct_init(lookup: *mut mdct_lookup, n: ::core::ffi::c_int);
	fn mdct_clear(l: *mut mdct_lookup);
	fn _ve_envelope_init(e: *mut envelope_lookup, vi: *mut vorbis_info);
	fn _ve_envelope_clear(e: *mut envelope_lookup);
	fn _ve_envelope_search(v: *mut vorbis_dsp_state) -> ::core::ffi::c_long;
	fn _ve_envelope_shift(e: *mut envelope_lookup, shift: ::core::ffi::c_long);
	fn _ve_envelope_mark(v: *mut vorbis_dsp_state) -> ::core::ffi::c_int;
	fn vorbis_staticbook_destroy(b: *mut static_codebook);
	fn vorbis_book_init_encode(
		dest: *mut codebook,
		source: *const static_codebook
	) -> ::core::ffi::c_int;
	fn vorbis_book_init_decode(c: *mut dec_codebook) -> ::core::ffi::c_int;
	fn vorbis_decbook_clear(c: *mut dec_codebook);
	fn drft_init(l: *mut drft_lookup, n: ::core::ffi::c_int);
	fn drft_clear(l: *mut drft_lookup);
	fn _vp_psy_init(
		p: *mut vorbis_look_psy,
		vi: *mut vorbis_info_psy,
		gi: *mut vorbis_info_psy_global,
		n: ::core::ffi::c_int,
		rate: ::core::ffi::c_long
	);
	fn _vp_psy_clear(p: *mut vorbis_look_psy);
	fn _vp_ampmax_decay(
		amp: ::core::ffi::c_float,
		vd: *mut vorbis_dsp_state
	) -> ::core::ffi::c_float;
	fn ov_ilog(v: ogg_uint32_t) -> ::core::ffi::c_int;
	fn vorbis_lpc_from_data(
		data: *mut ::core::ffi::c_float,
		lpc: *mut ::core::ffi::c_float,
		n: ::core::ffi::c_int,
		m: ::core::ffi::c_int
	) -> ::core::ffi::c_float;
	fn vorbis_lpc_predict(
		coeff: *mut ::core::ffi::c_float,
		prime: *mut ::core::ffi::c_float,
		m: ::core::ffi::c_int,
		data: *mut ::core::ffi::c_float,
		n: ::core::ffi::c_long
	);
	fn _vorbis_window_get(n: ::core::ffi::c_int) -> *const ::core::ffi::c_float;
	fn vorbis_bitrate_init(vi: *mut vorbis_info, bs: *mut bitrate_manager_state);
	fn vorbis_bitrate_clear(bs: *mut bitrate_manager_state);
	fn _vp_global_look(vi: *mut vorbis_info) -> *mut vorbis_look_psy_global;
	fn _vp_global_free(look: *mut vorbis_look_psy_global);
}
use ogg_c2rust_sys::oggpack_buffer;

use super::{
	info::vorbis_info,
	registry::{_floor_P, _residue_P}
};
pub type size_t = usize;
pub type __uint16_t = u16;
pub type __int32_t = i32;
pub type __uint32_t = u32;
pub type __int64_t = i64;
pub type int32_t = __int32_t;
pub type int64_t = __int64_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
pub type ogg_uint16_t = uint16_t;
pub type ogg_int32_t = int32_t;
pub type ogg_uint32_t = uint32_t;
pub type ogg_int64_t = int64_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct vorbis_dsp_state {
	pub analysisp: ::core::ffi::c_int,
	pub vi: *mut vorbis_info,
	pub pcm: *mut *mut ::core::ffi::c_float,
	pub pcmret: *mut *mut ::core::ffi::c_float,
	pub pcm_storage: ::core::ffi::c_int,
	pub pcm_current: ::core::ffi::c_int,
	pub pcm_returned: ::core::ffi::c_int,
	pub preextrapolate: ::core::ffi::c_int,
	pub eofflag: ::core::ffi::c_int,
	pub lW: ::core::ffi::c_long,
	pub W: ::core::ffi::c_long,
	pub nW: ::core::ffi::c_long,
	pub centerW: ::core::ffi::c_long,
	pub granulepos: ogg_int64_t,
	pub sequence: ogg_int64_t,
	pub glue_bits: ogg_int64_t,
	pub time_bits: ogg_int64_t,
	pub floor_bits: ogg_int64_t,
	pub res_bits: ogg_int64_t,
	pub backend_state: *mut ::core::ffi::c_void
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct vorbis_block {
	pub pcm: *mut *mut ::core::ffi::c_float,
	pub opb: oggpack_buffer,
	pub lW: ::core::ffi::c_long,
	pub W: ::core::ffi::c_long,
	pub nW: ::core::ffi::c_long,
	pub pcmend: ::core::ffi::c_int,
	pub mode: ::core::ffi::c_int,
	pub eofflag: ::core::ffi::c_int,
	pub granulepos: ogg_int64_t,
	pub sequence: ogg_int64_t,
	pub vd: *mut vorbis_dsp_state,
	pub localstore: *mut ::core::ffi::c_void,
	pub localtop: ::core::ffi::c_long,
	pub localalloc: ::core::ffi::c_long,
	pub totaluse: ::core::ffi::c_long,
	pub reap: *mut alloc_chain,
	pub glue_bits: ::core::ffi::c_long,
	pub time_bits: ::core::ffi::c_long,
	pub floor_bits: ::core::ffi::c_long,
	pub res_bits: ::core::ffi::c_long,
	pub internal: *mut ::core::ffi::c_void
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct alloc_chain {
	pub ptr: *mut ::core::ffi::c_void,
	pub next: *mut alloc_chain
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct vorbis_block_internal {
	pub pcmdelay: *mut *mut ::core::ffi::c_float,
	pub ampmax: ::core::ffi::c_float,
	pub blocktype: ::core::ffi::c_int,
	pub packetblob: [*mut oggpack_buffer; 15]
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct private_state {
	pub ve: *mut envelope_lookup,
	pub window: [::core::ffi::c_int; 2],
	pub transform: [*mut *mut ::core::ffi::c_void; 2],
	pub fft_look: [drft_lookup; 2],
	pub modebits: ::core::ffi::c_int,
	pub flr: *mut *mut ::core::ffi::c_void,
	pub residue: *mut *mut ::core::ffi::c_void,
	pub psy: *mut vorbis_look_psy,
	pub psy_g_look: *mut vorbis_look_psy_global,
	pub header: *mut ::core::ffi::c_uchar,
	pub header1: *mut ::core::ffi::c_uchar,
	pub header2: *mut ::core::ffi::c_uchar,
	pub bms: bitrate_manager_state,
	pub sample_count: ogg_int64_t
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct bitrate_manager_state {
	pub managed: ::core::ffi::c_int,
	pub avg_reservoir: ::core::ffi::c_long,
	pub minmax_reservoir: ::core::ffi::c_long,
	pub avg_bitsper: ::core::ffi::c_long,
	pub min_bitsper: ::core::ffi::c_long,
	pub max_bitsper: ::core::ffi::c_long,
	pub short_per_long: ::core::ffi::c_long,
	pub avgfloat: ::core::ffi::c_double,
	pub vb: *mut vorbis_block,
	pub choice: ::core::ffi::c_int
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct vorbis_look_psy_global {
	pub ampmax: ::core::ffi::c_float,
	pub channels: ::core::ffi::c_int,
	pub gi: *mut vorbis_info_psy_global,
	pub coupling_pointlimit: [[::core::ffi::c_int; 3]; 2]
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct vorbis_info_psy_global {
	pub eighth_octave_lines: ::core::ffi::c_int,
	pub preecho_thresh: [::core::ffi::c_float; 7],
	pub postecho_thresh: [::core::ffi::c_float; 7],
	pub stretch_penalty: ::core::ffi::c_float,
	pub preecho_minenergy: ::core::ffi::c_float,
	pub ampmax_att_per_sec: ::core::ffi::c_float,
	pub coupling_pkHz: [::core::ffi::c_int; 15],
	pub coupling_pointlimit: [[::core::ffi::c_int; 15]; 2],
	pub coupling_prepointamp: [::core::ffi::c_int; 15],
	pub coupling_postpointamp: [::core::ffi::c_int; 15],
	pub sliding_lowpass: [[::core::ffi::c_int; 15]; 2]
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct vorbis_look_psy {
	pub n: ::core::ffi::c_int,
	pub vi: *mut vorbis_info_psy,
	pub tonecurves: *mut *mut *mut ::core::ffi::c_float,
	pub noiseoffset: *mut *mut ::core::ffi::c_float,
	pub ath: *mut ::core::ffi::c_float,
	pub octave: *mut ::core::ffi::c_long,
	pub bark: *mut ::core::ffi::c_long,
	pub firstoc: ::core::ffi::c_long,
	pub shiftoc: ::core::ffi::c_long,
	pub eighth_octave_lines: ::core::ffi::c_int,
	pub total_octave_lines: ::core::ffi::c_int,
	pub rate: ::core::ffi::c_long,
	pub m_val: ::core::ffi::c_float
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct vorbis_info_psy {
	pub blockflag: ::core::ffi::c_int,
	pub ath_adjatt: ::core::ffi::c_float,
	pub ath_maxatt: ::core::ffi::c_float,
	pub tone_masteratt: [::core::ffi::c_float; 3],
	pub tone_centerboost: ::core::ffi::c_float,
	pub tone_decay: ::core::ffi::c_float,
	pub tone_abs_limit: ::core::ffi::c_float,
	pub toneatt: [::core::ffi::c_float; 17],
	pub noisemaskp: ::core::ffi::c_int,
	pub noisemaxsupp: ::core::ffi::c_float,
	pub noisewindowlo: ::core::ffi::c_float,
	pub noisewindowhi: ::core::ffi::c_float,
	pub noisewindowlomin: ::core::ffi::c_int,
	pub noisewindowhimin: ::core::ffi::c_int,
	pub noisewindowfixed: ::core::ffi::c_int,
	pub noiseoff: [[::core::ffi::c_float; 17]; 3],
	pub noisecompand: [::core::ffi::c_float; 40],
	pub max_curve_dB: ::core::ffi::c_float,
	pub normal_p: ::core::ffi::c_int,
	pub normal_start: ::core::ffi::c_int,
	pub normal_partition: ::core::ffi::c_int,
	pub normal_thresh: ::core::ffi::c_double
}
pub type vorbis_look_residue = ();
pub type vorbis_look_floor = ();
#[derive(Copy, Clone)]
#[repr(C)]
pub struct drft_lookup {
	pub n: ::core::ffi::c_int,
	pub trigcache: *mut ::core::ffi::c_float,
	pub splitcache: *mut ::core::ffi::c_int
}
pub type vorbis_look_transform = ();
#[derive(Copy, Clone)]
#[repr(C)]
pub struct envelope_lookup {
	pub ch: ::core::ffi::c_int,
	pub winlength: ::core::ffi::c_int,
	pub searchstep: ::core::ffi::c_int,
	pub minenergy: ::core::ffi::c_float,
	pub mdct: mdct_lookup,
	pub mdct_win: *mut ::core::ffi::c_float,
	pub band: [envelope_band; 7],
	pub filter: *mut envelope_filter_state,
	pub stretch: ::core::ffi::c_int,
	pub mark: *mut ::core::ffi::c_int,
	pub storage: ::core::ffi::c_long,
	pub current: ::core::ffi::c_long,
	pub curmark: ::core::ffi::c_long,
	pub cursor: ::core::ffi::c_long
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct envelope_filter_state {
	pub ampbuf: [::core::ffi::c_float; 17],
	pub ampptr: ::core::ffi::c_int,
	pub nearDC: [::core::ffi::c_float; 15],
	pub nearDC_acc: ::core::ffi::c_float,
	pub nearDC_partialacc: ::core::ffi::c_float,
	pub nearptr: ::core::ffi::c_int
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct envelope_band {
	pub begin: ::core::ffi::c_int,
	pub end: ::core::ffi::c_int,
	pub window: *mut ::core::ffi::c_float,
	pub total: ::core::ffi::c_float
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mdct_lookup {
	pub n: ::core::ffi::c_int,
	pub log2n: ::core::ffi::c_int,
	pub trig: *mut ::core::ffi::c_float,
	pub bitrev: *mut ::core::ffi::c_int,
	pub scale: ::core::ffi::c_float
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct codec_setup_info {
	pub blocksizes: [::core::ffi::c_long; 2],
	pub modes: ::core::ffi::c_int,
	pub maps: ::core::ffi::c_int,
	pub floors: ::core::ffi::c_int,
	pub residues: ::core::ffi::c_int,
	pub books: ::core::ffi::c_int,
	pub psys: ::core::ffi::c_int,
	pub mode_param: [*mut vorbis_info_mode; 64],
	pub map_type: [::core::ffi::c_int; 64],
	pub map_param: [*mut ::core::ffi::c_void; 64],
	pub floor_type: [::core::ffi::c_int; 64],
	pub floor_param: [*mut ::core::ffi::c_void; 64],
	pub residue_type: [::core::ffi::c_int; 64],
	pub residue_param: [*mut ::core::ffi::c_void; 64],
	pub book_param: [*mut static_codebook; 256],
	pub fullbooks: *mut codebook,
	pub decbooks: *mut dec_codebook,
	pub psy_param: [*mut vorbis_info_psy; 4],
	pub psy_g_param: vorbis_info_psy_global,
	pub bi: bitrate_manager_info,
	pub hi: highlevel_encode_setup,
	pub halfrate_flag: ::core::ffi::c_int
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct highlevel_encode_setup {
	pub set_in_stone: ::core::ffi::c_int,
	pub setup: *const ::core::ffi::c_void,
	pub base_setting: ::core::ffi::c_double,
	pub impulse_noisetune: ::core::ffi::c_double,
	pub req: ::core::ffi::c_float,
	pub managed: ::core::ffi::c_int,
	pub bitrate_min: ::core::ffi::c_long,
	pub bitrate_av: ::core::ffi::c_long,
	pub bitrate_av_damp: ::core::ffi::c_double,
	pub bitrate_max: ::core::ffi::c_long,
	pub bitrate_reservoir: ::core::ffi::c_long,
	pub bitrate_reservoir_bias: ::core::ffi::c_double,
	pub impulse_block_p: ::core::ffi::c_int,
	pub noise_normalize_p: ::core::ffi::c_int,
	pub coupling_p: ::core::ffi::c_int,
	pub stereo_point_setting: ::core::ffi::c_double,
	pub lowpass_kHz: ::core::ffi::c_double,
	pub lowpass_altered: ::core::ffi::c_int,
	pub ath_floating_dB: ::core::ffi::c_double,
	pub ath_absolute_dB: ::core::ffi::c_double,
	pub amplitude_track_dBpersec: ::core::ffi::c_double,
	pub trigger_setting: ::core::ffi::c_double,
	pub block: [highlevel_byblocktype; 4]
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct highlevel_byblocktype {
	pub tone_mask_setting: ::core::ffi::c_double,
	pub tone_peaklimit_setting: ::core::ffi::c_double,
	pub noise_bias_setting: ::core::ffi::c_double,
	pub noise_compand_setting: ::core::ffi::c_double
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct bitrate_manager_info {
	pub avg_rate: ::core::ffi::c_long,
	pub min_rate: ::core::ffi::c_long,
	pub max_rate: ::core::ffi::c_long,
	pub reservoir_bits: ::core::ffi::c_long,
	pub reservoir_bias: ::core::ffi::c_double,
	pub slew_damp: ::core::ffi::c_double
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dec_codebook {
	pub dim: ::core::ffi::c_schar,
	pub minlength: ::core::ffi::c_schar,
	pub maxlength: ::core::ffi::c_schar,
	pub firsttablen: ::core::ffi::c_schar,
	pub entries: ogg_int32_t,
	pub hi_max: ogg_int32_t,
	pub hint_shift: ::core::ffi::c_schar,
	pub maptype: ::core::ffi::c_schar,
	pub q_quant: ::core::ffi::c_schar,
	pub q_sequencep: ::core::ffi::c_schar,
	pub q_min: ogg_uint32_t,
	pub q_delta: ogg_uint32_t,
	pub quantlist: *mut ogg_uint16_t,
	pub firsttable: *mut ogg_uint32_t,
	pub codelist: *mut ogg_uint32_t,
	pub codelengths: *mut ::core::ffi::c_schar,
	pub index: *mut ogg_int32_t,
	pub valuelist: *mut ::core::ffi::c_float
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct codebook {
	pub dim: ::core::ffi::c_long,
	pub entries: ::core::ffi::c_long,
	pub used_entries: ::core::ffi::c_long,
	pub c: *const static_codebook,
	pub valuelist: *mut ::core::ffi::c_float,
	pub codelist: *mut ogg_uint32_t,
	pub quantvals: ::core::ffi::c_int,
	pub minval: ::core::ffi::c_int,
	pub delta: ::core::ffi::c_int
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct static_codebook {
	pub dim: ::core::ffi::c_long,
	pub entries: ::core::ffi::c_long,
	pub lengthlist: *mut ::core::ffi::c_char,
	pub maptype: ::core::ffi::c_int,
	pub q_min: ::core::ffi::c_long,
	pub q_delta: ::core::ffi::c_long,
	pub q_quant: ::core::ffi::c_int,
	pub q_sequencep: ::core::ffi::c_int,
	pub quantlist: *mut ::core::ffi::c_long,
	pub allocedp: ::core::ffi::c_int
}
pub type vorbis_info_residue = ();
pub type vorbis_info_floor = ();
pub type vorbis_info_mapping = ();
#[derive(Copy, Clone)]
#[repr(C)]
pub struct vorbis_info_mode {
	pub blockflag: ::core::ffi::c_int,
	pub windowtype: ::core::ffi::c_int,
	pub transformtype: ::core::ffi::c_int,
	pub mapping: ::core::ffi::c_int
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct vorbis_func_residue {
	pub pack: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void, *mut oggpack_buffer) -> ()>,
	pub unpack: Option<
		unsafe extern "C" fn(*mut vorbis_info, *mut oggpack_buffer) -> *mut ::core::ffi::c_void
	>,
	pub look: Option<
		unsafe extern "C" fn(
			*mut vorbis_dsp_state,
			*mut ::core::ffi::c_void
		) -> *mut ::core::ffi::c_void
	>,
	pub free_info: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
	pub free_look: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
	pub class: Option<
		unsafe extern "C" fn(
			*mut vorbis_block,
			*mut ::core::ffi::c_void,
			*mut *mut ::core::ffi::c_int,
			*mut ::core::ffi::c_int,
			::core::ffi::c_int
		) -> *mut *mut ::core::ffi::c_long
	>,
	pub forward: Option<
		unsafe extern "C" fn(
			*mut oggpack_buffer,
			*mut vorbis_block,
			*mut ::core::ffi::c_void,
			*mut *mut ::core::ffi::c_int,
			*mut ::core::ffi::c_int,
			::core::ffi::c_int,
			*mut *mut ::core::ffi::c_long,
			::core::ffi::c_int
		) -> ::core::ffi::c_int
	>,
	pub inverse: Option<
		unsafe extern "C" fn(
			*mut vorbis_block,
			*mut ::core::ffi::c_void,
			*mut *mut ::core::ffi::c_float,
			*mut ::core::ffi::c_int,
			::core::ffi::c_int
		) -> ::core::ffi::c_int
	>
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct vorbis_func_floor {
	pub pack: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void, *mut oggpack_buffer) -> ()>,
	pub unpack: Option<
		unsafe extern "C" fn(*mut vorbis_info, *mut oggpack_buffer) -> *mut ::core::ffi::c_void
	>,
	pub look: Option<
		unsafe extern "C" fn(
			*mut vorbis_dsp_state,
			*mut ::core::ffi::c_void
		) -> *mut ::core::ffi::c_void
	>,
	pub free_info: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
	pub free_look: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
	pub inverse1: Option<
		unsafe extern "C" fn(
			*mut vorbis_block,
			*mut ::core::ffi::c_void
		) -> *mut ::core::ffi::c_void
	>,
	pub inverse2: Option<
		unsafe extern "C" fn(
			*mut vorbis_block,
			*mut ::core::ffi::c_void,
			*mut ::core::ffi::c_void,
			*mut ::core::ffi::c_float
		) -> ::core::ffi::c_int
	>
}
pub const OV_EINVAL: ::core::ffi::c_int = -(131 as ::core::ffi::c_int);
pub const BLOCKTYPE_IMPULSE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const BLOCKTYPE_PADDING: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const BLOCKTYPE_TRANSITION: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const BLOCKTYPE_LONG: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const PACKETBLOBS: ::core::ffi::c_int = 15 as ::core::ffi::c_int;
pub const VI_TRANSFORMB: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const WORD_ALIGN: ::core::ffi::c_int = 8 as ::core::ffi::c_int;
#[no_mangle]
pub unsafe extern "C" fn vorbis_block_init(
	mut v: *mut vorbis_dsp_state,
	mut vb: *mut vorbis_block
) -> ::core::ffi::c_int {
	let mut i: ::core::ffi::c_int = 0;
	memset(
		vb as *mut ::core::ffi::c_void,
		0 as ::core::ffi::c_int,
		::core::mem::size_of::<vorbis_block>() as size_t
	);
	(*vb).vd = v;
	(*vb).localalloc = 0 as ::core::ffi::c_long;
	(*vb).localstore = NULL;
	if (*v).analysisp != 0 {
		(*vb).internal = calloc(
			1 as size_t,
			::core::mem::size_of::<vorbis_block_internal>() as size_t
		);
		let mut vbi: *mut vorbis_block_internal = (*vb).internal as *mut vorbis_block_internal;
		(*vbi).ampmax = -(9999 as ::core::ffi::c_int) as ::core::ffi::c_float;
		i = 0 as ::core::ffi::c_int;
		while i < PACKETBLOBS {
			if i == PACKETBLOBS / 2 as ::core::ffi::c_int {
				(*vbi).packetblob[i as usize] = &raw mut (*vb).opb;
			} else {
				(*vbi).packetblob[i as usize] = calloc(
					1 as size_t,
					::core::mem::size_of::<oggpack_buffer>() as size_t
				) as *mut oggpack_buffer;
			}
			oggpack_writeinit((*vbi).packetblob[i as usize]);
			i += 1;
		}
	}
	return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn _vorbis_block_alloc(
	mut vb: *mut vorbis_block,
	mut bytes: ::core::ffi::c_long
) -> *mut ::core::ffi::c_void {
	bytes = bytes + (WORD_ALIGN - 1 as ::core::ffi::c_int) as ::core::ffi::c_long
		& !(WORD_ALIGN - 1 as ::core::ffi::c_int) as ::core::ffi::c_long;
	if bytes + (*vb).localtop > (*vb).localalloc {
		if !(*vb).localstore.is_null() {
			let mut link: *mut alloc_chain =
				malloc(::core::mem::size_of::<alloc_chain>() as size_t) as *mut alloc_chain;
			(*vb).totaluse += (*vb).localtop;
			(*link).next = (*vb).reap as *mut alloc_chain;
			(*link).ptr = (*vb).localstore;
			(*vb).reap = link as *mut alloc_chain;
		}
		(*vb).localalloc = bytes;
		(*vb).localstore = malloc((*vb).localalloc as size_t);
		(*vb).localtop = 0 as ::core::ffi::c_long;
	}
	let mut ret: *mut ::core::ffi::c_void = ((*vb).localstore as *mut ::core::ffi::c_char)
		.offset((*vb).localtop as isize)
		as *mut ::core::ffi::c_void;
	(*vb).localtop += bytes;
	return ret;
}
#[no_mangle]
pub unsafe extern "C" fn _vorbis_block_ripcord(mut vb: *mut vorbis_block) {
	let mut reap: *mut alloc_chain = (*vb).reap as *mut alloc_chain;
	while !reap.is_null() {
		let mut next: *mut alloc_chain = (*reap).next;
		free((*reap).ptr);
		memset(
			reap as *mut ::core::ffi::c_void,
			0 as ::core::ffi::c_int,
			::core::mem::size_of::<alloc_chain>() as size_t
		);
		free(reap as *mut ::core::ffi::c_void);
		reap = next;
	}
	if (*vb).totaluse != 0 {
		(*vb).localstore = realloc(
			(*vb).localstore,
			((*vb).totaluse + (*vb).localalloc) as size_t
		);
		(*vb).localalloc += (*vb).totaluse;
		(*vb).totaluse = 0 as ::core::ffi::c_long;
	}
	(*vb).localtop = 0 as ::core::ffi::c_long;
	(*vb).reap = ::core::ptr::null_mut::<alloc_chain>();
}
#[no_mangle]
pub unsafe extern "C" fn vorbis_block_clear(mut vb: *mut vorbis_block) -> ::core::ffi::c_int {
	let mut i: ::core::ffi::c_int = 0;
	let mut vbi: *mut vorbis_block_internal = (*vb).internal as *mut vorbis_block_internal;
	_vorbis_block_ripcord(vb);
	if !(*vb).localstore.is_null() {
		free((*vb).localstore);
	}
	if !vbi.is_null() {
		i = 0 as ::core::ffi::c_int;
		while i < PACKETBLOBS {
			oggpack_writeclear((*vbi).packetblob[i as usize]);
			if i != PACKETBLOBS / 2 as ::core::ffi::c_int {
				free((*vbi).packetblob[i as usize] as *mut ::core::ffi::c_void);
			}
			i += 1;
		}
		free(vbi as *mut ::core::ffi::c_void);
	}
	memset(
		vb as *mut ::core::ffi::c_void,
		0 as ::core::ffi::c_int,
		::core::mem::size_of::<vorbis_block>() as size_t
	);
	return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn _vds_shared_init(
	mut v: *mut vorbis_dsp_state,
	mut vi: *mut vorbis_info,
	mut encp: ::core::ffi::c_int
) -> ::core::ffi::c_int {
	let mut current_block: u64;
	let mut i: ::core::ffi::c_int = 0;
	let mut ci: *mut codec_setup_info = (*vi).codec_setup as *mut codec_setup_info;
	let mut b: *mut private_state = ::core::ptr::null_mut::<private_state>();
	let mut hs: ::core::ffi::c_int = 0;
	if ci.is_null()
		|| (*ci).modes <= 0 as ::core::ffi::c_int
		|| (*ci).blocksizes[0 as ::core::ffi::c_int as usize] < 64 as ::core::ffi::c_long
		|| (*ci).blocksizes[1 as ::core::ffi::c_int as usize]
			< (*ci).blocksizes[0 as ::core::ffi::c_int as usize]
	{
		return 1 as ::core::ffi::c_int;
	}
	hs = (*ci).halfrate_flag;
	memset(
		v as *mut ::core::ffi::c_void,
		0 as ::core::ffi::c_int,
		::core::mem::size_of::<vorbis_dsp_state>() as size_t
	);
	(*v).backend_state = calloc(
		1 as size_t,
		::core::mem::size_of::<private_state>() as size_t
	);
	b = (*v).backend_state as *mut private_state;
	(*v).vi = vi;
	(*b).modebits = ov_ilog(((*ci).modes - 1 as ::core::ffi::c_int) as ogg_uint32_t);
	(*b).transform[0 as ::core::ffi::c_int as usize] = calloc(
		VI_TRANSFORMB as size_t,
		::core::mem::size_of::<*mut ::core::ffi::c_void>() as size_t
	) as *mut *mut ::core::ffi::c_void;
	(*b).transform[1 as ::core::ffi::c_int as usize] = calloc(
		VI_TRANSFORMB as size_t,
		::core::mem::size_of::<*mut ::core::ffi::c_void>() as size_t
	) as *mut *mut ::core::ffi::c_void;
	let ref mut fresh0 =
		*(*b).transform[0 as ::core::ffi::c_int as usize].offset(0 as ::core::ffi::c_int as isize);
	*fresh0 = calloc(1 as size_t, ::core::mem::size_of::<mdct_lookup>() as size_t)
		as *mut ::core::ffi::c_void;
	let ref mut fresh1 =
		*(*b).transform[1 as ::core::ffi::c_int as usize].offset(0 as ::core::ffi::c_int as isize);
	*fresh1 = calloc(1 as size_t, ::core::mem::size_of::<mdct_lookup>() as size_t)
		as *mut ::core::ffi::c_void;
	mdct_init(
		*(*b).transform[0 as ::core::ffi::c_int as usize].offset(0 as ::core::ffi::c_int as isize)
			as *mut mdct_lookup,
		((*ci).blocksizes[0 as ::core::ffi::c_int as usize] >> hs) as ::core::ffi::c_int
	);
	mdct_init(
		*(*b).transform[1 as ::core::ffi::c_int as usize].offset(0 as ::core::ffi::c_int as isize)
			as *mut mdct_lookup,
		((*ci).blocksizes[1 as ::core::ffi::c_int as usize] >> hs) as ::core::ffi::c_int
	);
	(*b).window[0 as ::core::ffi::c_int as usize] =
		ov_ilog((*ci).blocksizes[0 as ::core::ffi::c_int as usize] as ogg_uint32_t)
			- 7 as ::core::ffi::c_int;
	(*b).window[1 as ::core::ffi::c_int as usize] =
		ov_ilog((*ci).blocksizes[1 as ::core::ffi::c_int as usize] as ogg_uint32_t)
			- 7 as ::core::ffi::c_int;
	if encp != 0 {
		drft_init(
			(&raw mut (*b).fft_look as *mut drft_lookup).offset(0 as ::core::ffi::c_int as isize)
				as *mut drft_lookup,
			(*ci).blocksizes[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_int
		);
		drft_init(
			(&raw mut (*b).fft_look as *mut drft_lookup).offset(1 as ::core::ffi::c_int as isize)
				as *mut drft_lookup,
			(*ci).blocksizes[1 as ::core::ffi::c_int as usize] as ::core::ffi::c_int
		);
		if (*ci).fullbooks.is_null() {
			(*ci).fullbooks = calloc(
				(*ci).books as size_t,
				::core::mem::size_of::<codebook>() as size_t
			) as *mut codebook;
			i = 0 as ::core::ffi::c_int;
			while i < (*ci).books {
				vorbis_book_init_encode(
					(*ci).fullbooks.offset(i as isize),
					(*ci).book_param[i as usize]
				);
				i += 1;
			}
		}
		(*b).psy = calloc(
			(*ci).psys as size_t,
			::core::mem::size_of::<vorbis_look_psy>() as size_t
		) as *mut vorbis_look_psy;
		i = 0 as ::core::ffi::c_int;
		while i < (*ci).psys {
			_vp_psy_init(
				(*b).psy.offset(i as isize),
				(*ci).psy_param[i as usize],
				&raw mut (*ci).psy_g_param,
				((*ci).blocksizes[(*(*ci).psy_param[i as usize]).blockflag as usize]
					/ 2 as ::core::ffi::c_long) as ::core::ffi::c_int,
				(*vi).rate
			);
			i += 1;
		}
		(*v).analysisp = 1 as ::core::ffi::c_int;
	} else {
		if (*ci).decbooks.is_null() {
			current_block = 17901472523661443743;
		} else {
			i = 0 as ::core::ffi::c_int;
			loop {
				if !(i < (*ci).books) {
					current_block = 4761528863920922185;
					break;
				}
				if vorbis_book_init_decode((*ci).decbooks.offset(i as isize)) != 0 {
					current_block = 17901472523661443743;
					break;
				}
				i += 1;
			}
		}
		match current_block {
			4761528863920922185 => {}
			_ => {
				i = 0 as ::core::ffi::c_int;
				while i < (*ci).books {
					if !(*ci).book_param[i as usize].is_null() {
						vorbis_staticbook_destroy((*ci).book_param[i as usize]);
						(*ci).book_param[i as usize] = ::core::ptr::null_mut::<static_codebook>();
					}
					if !(*ci).decbooks.is_null() {
						vorbis_decbook_clear((*ci).decbooks.offset(i as isize));
					}
					i += 1;
				}
				if !(*ci).decbooks.is_null() {
					free((*ci).decbooks as *mut ::core::ffi::c_void);
					(*ci).decbooks = ::core::ptr::null_mut::<dec_codebook>();
				}
				vorbis_dsp_clear(v);
				return -(1 as ::core::ffi::c_int);
			}
		}
	}
	(*v).pcm_storage = (*ci).blocksizes[1 as ::core::ffi::c_int as usize] as ::core::ffi::c_int;
	(*v).pcm = malloc(
		((*vi).channels as size_t)
			.wrapping_mul(::core::mem::size_of::<*mut ::core::ffi::c_float>() as size_t)
	) as *mut *mut ::core::ffi::c_float;
	(*v).pcmret = malloc(
		((*vi).channels as size_t)
			.wrapping_mul(::core::mem::size_of::<*mut ::core::ffi::c_float>() as size_t)
	) as *mut *mut ::core::ffi::c_float;
	let mut i_0: ::core::ffi::c_int = 0;
	i_0 = 0 as ::core::ffi::c_int;
	while i_0 < (*vi).channels {
		let ref mut fresh2 = *(*v).pcm.offset(i_0 as isize);
		*fresh2 = calloc(
			(*v).pcm_storage as size_t,
			::core::mem::size_of::<::core::ffi::c_float>() as size_t
		) as *mut ::core::ffi::c_float;
		i_0 += 1;
	}
	(*v).lW = 0 as ::core::ffi::c_long;
	(*v).W = 0 as ::core::ffi::c_long;
	(*v).centerW = (*ci).blocksizes[1 as ::core::ffi::c_int as usize] / 2 as ::core::ffi::c_long;
	(*v).pcm_current = (*v).centerW as ::core::ffi::c_int;
	(*b).flr = calloc(
		(*ci).floors as size_t,
		::core::mem::size_of::<*mut ::core::ffi::c_void>() as size_t
	) as *mut *mut ::core::ffi::c_void;
	(*b).residue = calloc(
		(*ci).residues as size_t,
		::core::mem::size_of::<*mut ::core::ffi::c_void>() as size_t
	) as *mut *mut ::core::ffi::c_void;
	i = 0 as ::core::ffi::c_int;
	while i < (*ci).floors {
		let ref mut fresh3 = *(*b).flr.offset(i as isize);
		*fresh3 = (**(&raw const _floor_P as *const *const vorbis_func_floor)
			.offset((*ci).floor_type[i as usize] as isize))
		.look
		.expect("non-null function pointer")(v, (*ci).floor_param[i as usize]);
		i += 1;
	}
	i = 0 as ::core::ffi::c_int;
	while i < (*ci).residues {
		let ref mut fresh4 = *(*b).residue.offset(i as isize);
		*fresh4 = (**(&raw const _residue_P as *const *const vorbis_func_residue)
			.offset((*ci).residue_type[i as usize] as isize))
		.look
		.expect("non-null function pointer")(v, (*ci).residue_param[i as usize]);
		i += 1;
	}
	return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn vorbis_analysis_init(
	mut v: *mut vorbis_dsp_state,
	mut vi: *mut vorbis_info
) -> ::core::ffi::c_int {
	let mut b: *mut private_state = ::core::ptr::null_mut::<private_state>();
	if _vds_shared_init(v, vi, 1 as ::core::ffi::c_int) != 0 {
		return 1 as ::core::ffi::c_int;
	}
	b = (*v).backend_state as *mut private_state;
	(*b).psy_g_look = _vp_global_look(vi);
	(*b).ve = calloc(
		1 as size_t,
		::core::mem::size_of::<envelope_lookup>() as size_t
	) as *mut envelope_lookup;
	_ve_envelope_init((*b).ve, vi);
	vorbis_bitrate_init(vi, &raw mut (*b).bms);
	(*v).sequence = 3 as ogg_int64_t;
	return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn vorbis_dsp_clear(mut v: *mut vorbis_dsp_state) {
	let mut i: ::core::ffi::c_int = 0;
	if !v.is_null() {
		let mut vi: *mut vorbis_info = (*v).vi;
		let mut ci: *mut codec_setup_info = (if !vi.is_null() {
			(*vi).codec_setup
		} else {
			NULL
		}) as *mut codec_setup_info;
		let mut b: *mut private_state = (*v).backend_state as *mut private_state;
		if !b.is_null() {
			if !(*b).ve.is_null() {
				_ve_envelope_clear((*b).ve);
				free((*b).ve as *mut ::core::ffi::c_void);
			}
			if !(*b).transform[0 as ::core::ffi::c_int as usize].is_null() {
				mdct_clear(
					*(*b).transform[0 as ::core::ffi::c_int as usize]
						.offset(0 as ::core::ffi::c_int as isize) as *mut mdct_lookup
				);
				free(
					*(*b).transform[0 as ::core::ffi::c_int as usize]
						.offset(0 as ::core::ffi::c_int as isize) as *mut ::core::ffi::c_void
				);
				free((*b).transform[0 as ::core::ffi::c_int as usize] as *mut ::core::ffi::c_void);
			}
			if !(*b).transform[1 as ::core::ffi::c_int as usize].is_null() {
				mdct_clear(
					*(*b).transform[1 as ::core::ffi::c_int as usize]
						.offset(0 as ::core::ffi::c_int as isize) as *mut mdct_lookup
				);
				free(
					*(*b).transform[1 as ::core::ffi::c_int as usize]
						.offset(0 as ::core::ffi::c_int as isize) as *mut ::core::ffi::c_void
				);
				free((*b).transform[1 as ::core::ffi::c_int as usize] as *mut ::core::ffi::c_void);
			}
			if !(*b).flr.is_null() {
				if !ci.is_null() {
					i = 0 as ::core::ffi::c_int;
					while i < (*ci).floors {
						(**(&raw const _floor_P as *const *const vorbis_func_floor)
							.offset((*ci).floor_type[i as usize] as isize))
						.free_look
						.expect("non-null function pointer")(*(*b).flr.offset(i as isize));
						i += 1;
					}
				}
				free((*b).flr as *mut ::core::ffi::c_void);
			}
			if !(*b).residue.is_null() {
				if !ci.is_null() {
					i = 0 as ::core::ffi::c_int;
					while i < (*ci).residues {
						(**(&raw const _residue_P as *const *const vorbis_func_residue)
							.offset((*ci).residue_type[i as usize] as isize))
						.free_look
						.expect("non-null function pointer")(*(*b).residue.offset(i as isize));
						i += 1;
					}
				}
				free((*b).residue as *mut ::core::ffi::c_void);
			}
			if !(*b).psy.is_null() {
				if !ci.is_null() {
					i = 0 as ::core::ffi::c_int;
					while i < (*ci).psys {
						_vp_psy_clear((*b).psy.offset(i as isize));
						i += 1;
					}
				}
				free((*b).psy as *mut ::core::ffi::c_void);
			}
			if !(*b).psy_g_look.is_null() {
				_vp_global_free((*b).psy_g_look);
			}
			vorbis_bitrate_clear(&raw mut (*b).bms);
			drft_clear(
				(&raw mut (*b).fft_look as *mut drft_lookup)
					.offset(0 as ::core::ffi::c_int as isize) as *mut drft_lookup
			);
			drft_clear(
				(&raw mut (*b).fft_look as *mut drft_lookup)
					.offset(1 as ::core::ffi::c_int as isize) as *mut drft_lookup
			);
		}
		if !(*v).pcm.is_null() {
			if !vi.is_null() {
				i = 0 as ::core::ffi::c_int;
				while i < (*vi).channels {
					if !(*(*v).pcm.offset(i as isize)).is_null() {
						free(*(*v).pcm.offset(i as isize) as *mut ::core::ffi::c_void);
					}
					i += 1;
				}
			}
			free((*v).pcm as *mut ::core::ffi::c_void);
			if !(*v).pcmret.is_null() {
				free((*v).pcmret as *mut ::core::ffi::c_void);
			}
		}
		if !b.is_null() {
			if !(*b).header.is_null() {
				free((*b).header as *mut ::core::ffi::c_void);
			}
			if !(*b).header1.is_null() {
				free((*b).header1 as *mut ::core::ffi::c_void);
			}
			if !(*b).header2.is_null() {
				free((*b).header2 as *mut ::core::ffi::c_void);
			}
			free(b as *mut ::core::ffi::c_void);
		}
		memset(
			v as *mut ::core::ffi::c_void,
			0 as ::core::ffi::c_int,
			::core::mem::size_of::<vorbis_dsp_state>() as size_t
		);
	}
}
#[no_mangle]
pub unsafe extern "C" fn vorbis_analysis_buffer(
	mut v: *mut vorbis_dsp_state,
	mut vals: ::core::ffi::c_int
) -> *mut *mut ::core::ffi::c_float {
	let mut i: ::core::ffi::c_int = 0;
	let mut vi: *mut vorbis_info = (*v).vi;
	let mut b: *mut private_state = (*v).backend_state as *mut private_state;
	if !(*b).header.is_null() {
		free((*b).header as *mut ::core::ffi::c_void);
		(*b).header = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
	}
	if !(*b).header1.is_null() {
		free((*b).header1 as *mut ::core::ffi::c_void);
		(*b).header1 = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
	}
	if !(*b).header2.is_null() {
		free((*b).header2 as *mut ::core::ffi::c_void);
		(*b).header2 = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
	}
	if (*v).pcm_current + vals >= (*v).pcm_storage {
		(*v).pcm_storage = (*v).pcm_current + vals * 2 as ::core::ffi::c_int;
		i = 0 as ::core::ffi::c_int;
		while i < (*vi).channels {
			let fresh5 = (*v).pcm.offset(i as isize);
			*fresh5 = realloc(
				*fresh5 as *mut ::core::ffi::c_void,
				((*v).pcm_storage as size_t)
					.wrapping_mul(::core::mem::size_of::<::core::ffi::c_float>() as size_t)
			) as *mut ::core::ffi::c_float;
			i += 1;
		}
	}
	i = 0 as ::core::ffi::c_int;
	while i < (*vi).channels {
		let ref mut fresh6 = *(*v).pcmret.offset(i as isize);
		*fresh6 = (*(*v).pcm.offset(i as isize)).offset((*v).pcm_current as isize);
		i += 1;
	}
	return (*v).pcmret;
}
unsafe extern "C" fn _preextrapolate_helper(mut v: *mut vorbis_dsp_state) {
	let mut alloca_allocations: Vec<Vec<u8>> = Vec::new();
	let mut i: ::core::ffi::c_int = 0;
	let mut order: ::core::ffi::c_int = 16 as ::core::ffi::c_int;
	alloca_allocations.push(::std::vec::from_elem(
		0,
		(order as usize).wrapping_mul(::core::mem::size_of::<::core::ffi::c_float>() as usize)
			as usize
	));
	let mut lpc: *mut ::core::ffi::c_float =
		alloca_allocations.last_mut().unwrap().as_mut_ptr() as *mut ::core::ffi::c_float;
	alloca_allocations.push(::std::vec::from_elem(
		0,
		((*v).pcm_current as usize)
			.wrapping_mul(::core::mem::size_of::<::core::ffi::c_float>() as usize) as usize
	));
	let mut work: *mut ::core::ffi::c_float =
		alloca_allocations.last_mut().unwrap().as_mut_ptr() as *mut ::core::ffi::c_float;
	let mut j: ::core::ffi::c_long = 0;
	(*v).preextrapolate = 1 as ::core::ffi::c_int;
	if (*v).pcm_current as ::core::ffi::c_long - (*v).centerW
		> (order * 2 as ::core::ffi::c_int) as ::core::ffi::c_long
	{
		i = 0 as ::core::ffi::c_int;
		while i < (*(*v).vi).channels {
			j = 0 as ::core::ffi::c_long;
			while j < (*v).pcm_current as ::core::ffi::c_long {
				*work.offset(j as isize) = *(*(*v).pcm.offset(i as isize)).offset(
					((*v).pcm_current as ::core::ffi::c_long - j - 1 as ::core::ffi::c_long)
						as isize
				);
				j += 1;
			}
			vorbis_lpc_from_data(
				work,
				lpc,
				((*v).pcm_current as ::core::ffi::c_long - (*v).centerW) as ::core::ffi::c_int,
				order
			);
			vorbis_lpc_predict(
				lpc,
				work.offset((*v).pcm_current as isize)
					.offset(-((*v).centerW as isize))
					.offset(-(order as isize)),
				order,
				work.offset((*v).pcm_current as isize)
					.offset(-((*v).centerW as isize)),
				(*v).centerW
			);
			j = 0 as ::core::ffi::c_long;
			while j < (*v).pcm_current as ::core::ffi::c_long {
				*(*(*v).pcm.offset(i as isize)).offset(
					((*v).pcm_current as ::core::ffi::c_long - j - 1 as ::core::ffi::c_long)
						as isize
				) = *work.offset(j as isize);
				j += 1;
			}
			i += 1;
		}
	}
}
#[no_mangle]
pub unsafe extern "C" fn vorbis_analysis_wrote(
	mut v: *mut vorbis_dsp_state,
	mut vals: ::core::ffi::c_int
) -> ::core::ffi::c_int {
	let mut alloca_allocations: Vec<Vec<u8>> = Vec::new();
	let mut vi: *mut vorbis_info = (*v).vi;
	let mut ci: *mut codec_setup_info = (*vi).codec_setup as *mut codec_setup_info;
	if vals <= 0 as ::core::ffi::c_int {
		let mut order: ::core::ffi::c_int = 32 as ::core::ffi::c_int;
		let mut i: ::core::ffi::c_int = 0;
		alloca_allocations.push(::std::vec::from_elem(
			0,
			(order as usize).wrapping_mul(::core::mem::size_of::<::core::ffi::c_float>() as usize)
				as usize
		));
		let mut lpc: *mut ::core::ffi::c_float =
			alloca_allocations.last_mut().unwrap().as_mut_ptr() as *mut ::core::ffi::c_float;
		if (*v).preextrapolate == 0 {
			_preextrapolate_helper(v);
		}
		vorbis_analysis_buffer(
			v,
			((*ci).blocksizes[1 as ::core::ffi::c_int as usize] * 3 as ::core::ffi::c_long)
				as ::core::ffi::c_int
		);
		(*v).eofflag = (*v).pcm_current;
		(*v).pcm_current = ((*v).pcm_current as ::core::ffi::c_long
			+ (*ci).blocksizes[1 as ::core::ffi::c_int as usize] * 3 as ::core::ffi::c_long)
			as ::core::ffi::c_int;
		i = 0 as ::core::ffi::c_int;
		while i < (*vi).channels {
			if (*v).eofflag > order * 2 as ::core::ffi::c_int {
				let mut n: ::core::ffi::c_long = 0;
				n = (*v).eofflag as ::core::ffi::c_long;
				if n > (*ci).blocksizes[1 as ::core::ffi::c_int as usize] {
					n = (*ci).blocksizes[1 as ::core::ffi::c_int as usize];
				}
				vorbis_lpc_from_data(
					(*(*v).pcm.offset(i as isize))
						.offset((*v).eofflag as isize)
						.offset(-(n as isize)),
					lpc,
					n as ::core::ffi::c_int,
					order
				);
				vorbis_lpc_predict(
					lpc,
					(*(*v).pcm.offset(i as isize))
						.offset((*v).eofflag as isize)
						.offset(-(order as isize)),
					order,
					(*(*v).pcm.offset(i as isize)).offset((*v).eofflag as isize),
					((*v).pcm_current - (*v).eofflag) as ::core::ffi::c_long
				);
			} else {
				memset(
					(*(*v).pcm.offset(i as isize)).offset((*v).eofflag as isize)
						as *mut ::core::ffi::c_void,
					0 as ::core::ffi::c_int,
					(((*v).pcm_current - (*v).eofflag) as size_t)
						.wrapping_mul(::core::mem::size_of::<::core::ffi::c_float>() as size_t)
				);
			}
			i += 1;
		}
	} else {
		if (*v).pcm_current + vals > (*v).pcm_storage {
			return -(131 as ::core::ffi::c_int);
		}
		(*v).pcm_current += vals;
		if (*v).preextrapolate == 0
			&& (*v).pcm_current as ::core::ffi::c_long - (*v).centerW
				> (*ci).blocksizes[1 as ::core::ffi::c_int as usize]
		{
			_preextrapolate_helper(v);
		}
	}
	return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn vorbis_analysis_blockout(
	mut v: *mut vorbis_dsp_state,
	mut vb: *mut vorbis_block
) -> ::core::ffi::c_int {
	let mut i: ::core::ffi::c_int = 0;
	let mut vi: *mut vorbis_info = (*v).vi;
	let mut ci: *mut codec_setup_info = (*vi).codec_setup as *mut codec_setup_info;
	let mut b: *mut private_state = (*v).backend_state as *mut private_state;
	let mut g: *mut vorbis_look_psy_global = (*b).psy_g_look;
	let mut beginW: ::core::ffi::c_long =
		(*v).centerW - (*ci).blocksizes[(*v).W as usize] / 2 as ::core::ffi::c_long;
	let mut centerNext: ::core::ffi::c_long = 0;
	let mut vbi: *mut vorbis_block_internal = (*vb).internal as *mut vorbis_block_internal;
	if (*v).preextrapolate == 0 {
		return 0 as ::core::ffi::c_int;
	}
	if (*v).eofflag == -(1 as ::core::ffi::c_int) {
		return 0 as ::core::ffi::c_int;
	}
	let mut bp: ::core::ffi::c_long = _ve_envelope_search(v);
	if bp == -(1 as ::core::ffi::c_int) as ::core::ffi::c_long {
		if (*v).eofflag == 0 as ::core::ffi::c_int {
			return 0 as ::core::ffi::c_int;
		}
		(*v).nW = 0 as ::core::ffi::c_long;
	} else if (*ci).blocksizes[0 as ::core::ffi::c_int as usize]
		== (*ci).blocksizes[1 as ::core::ffi::c_int as usize]
	{
		(*v).nW = 0 as ::core::ffi::c_long;
	} else {
		(*v).nW = bp;
	}
	centerNext = (*v).centerW
		+ (*ci).blocksizes[(*v).W as usize] / 4 as ::core::ffi::c_long
		+ (*ci).blocksizes[(*v).nW as usize] / 4 as ::core::ffi::c_long;
	let mut blockbound: ::core::ffi::c_long =
		centerNext + (*ci).blocksizes[(*v).nW as usize] / 2 as ::core::ffi::c_long;
	if ((*v).pcm_current as ::core::ffi::c_long) < blockbound {
		return 0 as ::core::ffi::c_int;
	}
	_vorbis_block_ripcord(vb);
	(*vb).lW = (*v).lW;
	(*vb).W = (*v).W;
	(*vb).nW = (*v).nW;
	if (*v).W != 0 {
		if (*v).lW == 0 || (*v).nW == 0 {
			(*vbi).blocktype = BLOCKTYPE_TRANSITION;
		} else {
			(*vbi).blocktype = BLOCKTYPE_LONG;
		}
	} else if _ve_envelope_mark(v) != 0 {
		(*vbi).blocktype = BLOCKTYPE_IMPULSE;
	} else {
		(*vbi).blocktype = BLOCKTYPE_PADDING;
	}
	(*vb).vd = v;
	let fresh7 = (*v).sequence;
	(*v).sequence = (*v).sequence + 1;
	(*vb).sequence = fresh7;
	(*vb).granulepos = (*v).granulepos;
	(*vb).pcmend = (*ci).blocksizes[(*v).W as usize] as ::core::ffi::c_int;
	if (*vbi).ampmax > (*g).ampmax {
		(*g).ampmax = (*vbi).ampmax;
	}
	(*g).ampmax = _vp_ampmax_decay((*g).ampmax, v);
	(*vbi).ampmax = (*g).ampmax;
	(*vb).pcm = _vorbis_block_alloc(
		vb,
		(::core::mem::size_of::<*mut ::core::ffi::c_float>() as usize)
			.wrapping_mul((*vi).channels as usize) as ::core::ffi::c_long
	) as *mut *mut ::core::ffi::c_float;
	(*vbi).pcmdelay = _vorbis_block_alloc(
		vb,
		(::core::mem::size_of::<*mut ::core::ffi::c_float>() as usize)
			.wrapping_mul((*vi).channels as usize) as ::core::ffi::c_long
	) as *mut *mut ::core::ffi::c_float;
	i = 0 as ::core::ffi::c_int;
	while i < (*vi).channels {
		let ref mut fresh8 = *(*vbi).pcmdelay.offset(i as isize);
		*fresh8 = _vorbis_block_alloc(
			vb,
			(((*vb).pcmend as ::core::ffi::c_long + beginW) as usize)
				.wrapping_mul(::core::mem::size_of::<::core::ffi::c_float>() as usize)
				as ::core::ffi::c_long
		) as *mut ::core::ffi::c_float;
		memcpy(
			*(*vbi).pcmdelay.offset(i as isize) as *mut ::core::ffi::c_void,
			*(*v).pcm.offset(i as isize) as *const ::core::ffi::c_void,
			(((*vb).pcmend as ::core::ffi::c_long + beginW) as size_t)
				.wrapping_mul(::core::mem::size_of::<::core::ffi::c_float>() as size_t)
		);
		let ref mut fresh9 = *(*vb).pcm.offset(i as isize);
		*fresh9 = (*(*vbi).pcmdelay.offset(i as isize)).offset(beginW as isize);
		i += 1;
	}
	if (*v).eofflag != 0 {
		if (*v).centerW >= (*v).eofflag as ::core::ffi::c_long {
			(*v).eofflag = -(1 as ::core::ffi::c_int);
			(*vb).eofflag = 1 as ::core::ffi::c_int;
			return 1 as ::core::ffi::c_int;
		}
	}
	let mut new_centerNext: ::core::ffi::c_int =
		((*ci).blocksizes[1 as ::core::ffi::c_int as usize] / 2 as ::core::ffi::c_long)
			as ::core::ffi::c_int;
	let mut movementW: ::core::ffi::c_int =
		(centerNext - new_centerNext as ::core::ffi::c_long) as ::core::ffi::c_int;
	if movementW > 0 as ::core::ffi::c_int {
		_ve_envelope_shift((*b).ve, movementW as ::core::ffi::c_long);
		(*v).pcm_current -= movementW;
		i = 0 as ::core::ffi::c_int;
		while i < (*vi).channels {
			::core::ptr::copy(
				(*(*v).pcm.offset(i as isize)).offset(movementW as isize)
					as *const ::core::ffi::c_float,
				*(*v).pcm.offset(i as isize),
				(*v).pcm_current as usize
			);
			i += 1;
		}
		(*v).lW = (*v).W;
		(*v).W = (*v).nW;
		(*v).centerW = new_centerNext as ::core::ffi::c_long;
		if (*v).eofflag != 0 {
			(*v).eofflag -= movementW;
			if (*v).eofflag <= 0 as ::core::ffi::c_int {
				(*v).eofflag = -(1 as ::core::ffi::c_int);
			}
			if (*v).centerW >= (*v).eofflag as ::core::ffi::c_long {
				(*v).granulepos = ((*v).granulepos as ::core::ffi::c_long
					+ (movementW as ::core::ffi::c_long
						- ((*v).centerW - (*v).eofflag as ::core::ffi::c_long)))
					as ogg_int64_t;
			} else {
				(*v).granulepos += movementW as ogg_int64_t;
			}
		} else {
			(*v).granulepos += movementW as ogg_int64_t;
		}
	}
	return 1 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn vorbis_synthesis_restart(
	mut v: *mut vorbis_dsp_state
) -> ::core::ffi::c_int {
	let mut vi: *mut vorbis_info = (*v).vi;
	let mut ci: *mut codec_setup_info = ::core::ptr::null_mut::<codec_setup_info>();
	let mut hs: ::core::ffi::c_int = 0;
	if (*v).backend_state.is_null() {
		return -(1 as ::core::ffi::c_int);
	}
	if vi.is_null() {
		return -(1 as ::core::ffi::c_int);
	}
	ci = (*vi).codec_setup as *mut codec_setup_info;
	if ci.is_null() {
		return -(1 as ::core::ffi::c_int);
	}
	hs = (*ci).halfrate_flag;
	(*v).centerW =
		(*ci).blocksizes[1 as ::core::ffi::c_int as usize] >> hs + 1 as ::core::ffi::c_int;
	(*v).pcm_current = ((*v).centerW >> hs) as ::core::ffi::c_int;
	(*v).pcm_returned = -(1 as ::core::ffi::c_int);
	(*v).granulepos = -(1 as ::core::ffi::c_int) as ogg_int64_t;
	(*v).sequence = -(1 as ::core::ffi::c_int) as ogg_int64_t;
	(*v).eofflag = 0 as ::core::ffi::c_int;
	(*((*v).backend_state as *mut private_state)).sample_count =
		-(1 as ::core::ffi::c_int) as ogg_int64_t;
	return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn vorbis_synthesis_init(
	mut v: *mut vorbis_dsp_state,
	mut vi: *mut vorbis_info
) -> ::core::ffi::c_int {
	if _vds_shared_init(v, vi, 0 as ::core::ffi::c_int) != 0 {
		vorbis_dsp_clear(v);
		return 1 as ::core::ffi::c_int;
	}
	vorbis_synthesis_restart(v);
	return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn vorbis_synthesis_blockin(
	mut v: *mut vorbis_dsp_state,
	mut vb: *mut vorbis_block
) -> ::core::ffi::c_int {
	let mut vi: *mut vorbis_info = (*v).vi;
	let mut ci: *mut codec_setup_info = (*vi).codec_setup as *mut codec_setup_info;
	let mut b: *mut private_state = (*v).backend_state as *mut private_state;
	let mut hs: ::core::ffi::c_int = (*ci).halfrate_flag;
	let mut i: ::core::ffi::c_int = 0;
	let mut j: ::core::ffi::c_int = 0;
	if vb.is_null() {
		return -(131 as ::core::ffi::c_int);
	}
	if (*v).pcm_current > (*v).pcm_returned && (*v).pcm_returned != -(1 as ::core::ffi::c_int) {
		return -(131 as ::core::ffi::c_int);
	}
	(*v).lW = (*v).W;
	(*v).W = (*vb).W;
	(*v).nW = -(1 as ::core::ffi::c_int) as ::core::ffi::c_long;
	if (*v).sequence == -(1 as ::core::ffi::c_int) as ogg_int64_t
		|| (*v).sequence + 1 as ogg_int64_t != (*vb).sequence
	{
		(*v).granulepos = -(1 as ::core::ffi::c_int) as ogg_int64_t;
		(*b).sample_count = -(1 as ::core::ffi::c_int) as ogg_int64_t;
	}
	(*v).sequence = (*vb).sequence;
	if !(*vb).pcm.is_null() {
		let mut n: ::core::ffi::c_int = ((*ci).blocksizes[(*v).W as usize]
			>> hs + 1 as ::core::ffi::c_int) as ::core::ffi::c_int;
		let mut n0: ::core::ffi::c_int = ((*ci).blocksizes[0 as ::core::ffi::c_int as usize]
			>> hs + 1 as ::core::ffi::c_int) as ::core::ffi::c_int;
		let mut n1: ::core::ffi::c_int = ((*ci).blocksizes[1 as ::core::ffi::c_int as usize]
			>> hs + 1 as ::core::ffi::c_int) as ::core::ffi::c_int;
		let mut thisCenter: ::core::ffi::c_int = 0;
		let mut prevCenter: ::core::ffi::c_int = 0;
		(*v).glue_bits = ((*v).glue_bits as ::core::ffi::c_long + (*vb).glue_bits) as ogg_int64_t;
		(*v).time_bits = ((*v).time_bits as ::core::ffi::c_long + (*vb).time_bits) as ogg_int64_t;
		(*v).floor_bits =
			((*v).floor_bits as ::core::ffi::c_long + (*vb).floor_bits) as ogg_int64_t;
		(*v).res_bits = ((*v).res_bits as ::core::ffi::c_long + (*vb).res_bits) as ogg_int64_t;
		if (*v).centerW != 0 {
			thisCenter = n1;
			prevCenter = 0 as ::core::ffi::c_int;
		} else {
			thisCenter = 0 as ::core::ffi::c_int;
			prevCenter = n1;
		}
		j = 0 as ::core::ffi::c_int;
		while j < (*vi).channels {
			if (*v).lW != 0 {
				if (*v).W != 0 {
					let mut w: *const ::core::ffi::c_float =
						_vorbis_window_get((*b).window[1 as ::core::ffi::c_int as usize] - hs);
					let mut pcm: *mut ::core::ffi::c_float =
						(*(*v).pcm.offset(j as isize)).offset(prevCenter as isize);
					let mut p: *mut ::core::ffi::c_float = *(*vb).pcm.offset(j as isize);
					i = 0 as ::core::ffi::c_int;
					while i < n1 {
						*pcm.offset(i as isize) = *pcm.offset(i as isize)
							* *w.offset((n1 - i - 1 as ::core::ffi::c_int) as isize)
							+ *p.offset(i as isize) * *w.offset(i as isize);
						i += 1;
					}
				} else {
					let mut w_0: *const ::core::ffi::c_float =
						_vorbis_window_get((*b).window[0 as ::core::ffi::c_int as usize] - hs);
					let mut pcm_0: *mut ::core::ffi::c_float = (*(*v).pcm.offset(j as isize))
						.offset(prevCenter as isize)
						.offset((n1 / 2 as ::core::ffi::c_int) as isize)
						.offset(-((n0 / 2 as ::core::ffi::c_int) as isize));
					let mut p_0: *mut ::core::ffi::c_float = *(*vb).pcm.offset(j as isize);
					i = 0 as ::core::ffi::c_int;
					while i < n0 {
						*pcm_0.offset(i as isize) = *pcm_0.offset(i as isize)
							* *w_0.offset((n0 - i - 1 as ::core::ffi::c_int) as isize)
							+ *p_0.offset(i as isize) * *w_0.offset(i as isize);
						i += 1;
					}
				}
			} else if (*v).W != 0 {
				let mut w_1: *const ::core::ffi::c_float =
					_vorbis_window_get((*b).window[0 as ::core::ffi::c_int as usize] - hs);
				let mut pcm_1: *mut ::core::ffi::c_float =
					(*(*v).pcm.offset(j as isize)).offset(prevCenter as isize);
				let mut p_1: *mut ::core::ffi::c_float = (*(*vb).pcm.offset(j as isize))
					.offset((n1 / 2 as ::core::ffi::c_int) as isize)
					.offset(-((n0 / 2 as ::core::ffi::c_int) as isize));
				i = 0 as ::core::ffi::c_int;
				while i < n0 {
					*pcm_1.offset(i as isize) = *pcm_1.offset(i as isize)
						* *w_1.offset((n0 - i - 1 as ::core::ffi::c_int) as isize)
						+ *p_1.offset(i as isize) * *w_1.offset(i as isize);
					i += 1;
				}
				while i < n1 / 2 as ::core::ffi::c_int + n0 / 2 as ::core::ffi::c_int {
					*pcm_1.offset(i as isize) = *p_1.offset(i as isize);
					i += 1;
				}
			} else {
				let mut w_2: *const ::core::ffi::c_float =
					_vorbis_window_get((*b).window[0 as ::core::ffi::c_int as usize] - hs);
				let mut pcm_2: *mut ::core::ffi::c_float =
					(*(*v).pcm.offset(j as isize)).offset(prevCenter as isize);
				let mut p_2: *mut ::core::ffi::c_float = *(*vb).pcm.offset(j as isize);
				i = 0 as ::core::ffi::c_int;
				while i < n0 {
					*pcm_2.offset(i as isize) = *pcm_2.offset(i as isize)
						* *w_2.offset((n0 - i - 1 as ::core::ffi::c_int) as isize)
						+ *p_2.offset(i as isize) * *w_2.offset(i as isize);
					i += 1;
				}
			}
			let mut pcm_3: *mut ::core::ffi::c_float =
				(*(*v).pcm.offset(j as isize)).offset(thisCenter as isize);
			let mut p_3: *mut ::core::ffi::c_float =
				(*(*vb).pcm.offset(j as isize)).offset(n as isize);
			i = 0 as ::core::ffi::c_int;
			while i < n {
				*pcm_3.offset(i as isize) = *p_3.offset(i as isize);
				i += 1;
			}
			j += 1;
		}
		if (*v).centerW != 0 {
			(*v).centerW = 0 as ::core::ffi::c_long;
		} else {
			(*v).centerW = n1 as ::core::ffi::c_long;
		}
		if (*v).pcm_returned == -(1 as ::core::ffi::c_int) {
			(*v).pcm_returned = thisCenter;
			(*v).pcm_current = thisCenter;
		} else {
			(*v).pcm_returned = prevCenter;
			(*v).pcm_current = (prevCenter as ::core::ffi::c_long
				+ ((*ci).blocksizes[(*v).lW as usize] / 4 as ::core::ffi::c_long
					+ (*ci).blocksizes[(*v).W as usize] / 4 as ::core::ffi::c_long
					>> hs)) as ::core::ffi::c_int;
		}
	}
	if (*b).sample_count == -(1 as ::core::ffi::c_int) as ogg_int64_t {
		(*b).sample_count = 0 as ogg_int64_t;
	} else {
		(*b).sample_count = ((*b).sample_count as ::core::ffi::c_long
			+ ((*ci).blocksizes[(*v).lW as usize] / 4 as ::core::ffi::c_long
				+ (*ci).blocksizes[(*v).W as usize] / 4 as ::core::ffi::c_long))
			as ogg_int64_t;
	}
	if (*v).granulepos == -(1 as ::core::ffi::c_int) as ogg_int64_t {
		if (*vb).granulepos != -(1 as ::core::ffi::c_int) as ogg_int64_t {
			(*v).granulepos = (*vb).granulepos;
			if (*b).sample_count > (*v).granulepos {
				let mut extra: ::core::ffi::c_long = (*b).sample_count as ::core::ffi::c_long
					- (*vb).granulepos as ::core::ffi::c_long;
				if extra < 0 as ::core::ffi::c_long {
					extra = 0 as ::core::ffi::c_long;
				}
				if (*vb).eofflag != 0 {
					if extra > ((*v).pcm_current - (*v).pcm_returned << hs) as ::core::ffi::c_long {
						extra = ((*v).pcm_current - (*v).pcm_returned << hs) as ::core::ffi::c_long;
					}
					(*v).pcm_current = ((*v).pcm_current as ::core::ffi::c_long - (extra >> hs))
						as ::core::ffi::c_int;
				} else {
					(*v).pcm_returned = ((*v).pcm_returned as ::core::ffi::c_long + (extra >> hs))
						as ::core::ffi::c_int;
					if (*v).pcm_returned > (*v).pcm_current {
						(*v).pcm_returned = (*v).pcm_current;
					}
				}
			}
		}
	} else {
		(*v).granulepos = ((*v).granulepos as ::core::ffi::c_long
			+ ((*ci).blocksizes[(*v).lW as usize] / 4 as ::core::ffi::c_long
				+ (*ci).blocksizes[(*v).W as usize] / 4 as ::core::ffi::c_long))
			as ogg_int64_t;
		if (*vb).granulepos != -(1 as ::core::ffi::c_int) as ogg_int64_t
			&& (*v).granulepos != (*vb).granulepos
		{
			if (*v).granulepos > (*vb).granulepos {
				let mut extra_0: ::core::ffi::c_long = (*v).granulepos as ::core::ffi::c_long
					- (*vb).granulepos as ::core::ffi::c_long;
				if extra_0 != 0 {
					if (*vb).eofflag != 0 {
						if extra_0
							> ((*v).pcm_current - (*v).pcm_returned << hs) as ::core::ffi::c_long
						{
							extra_0 =
								((*v).pcm_current - (*v).pcm_returned << hs) as ::core::ffi::c_long;
						}
						if extra_0 < 0 as ::core::ffi::c_long {
							extra_0 = 0 as ::core::ffi::c_long;
						}
						(*v).pcm_current = ((*v).pcm_current as ::core::ffi::c_long
							- (extra_0 >> hs)) as ::core::ffi::c_int;
					}
				}
			}
			(*v).granulepos = (*vb).granulepos;
		}
	}
	if (*vb).eofflag != 0 {
		(*v).eofflag = 1 as ::core::ffi::c_int;
	}
	return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn vorbis_synthesis_pcmout(
	mut v: *mut vorbis_dsp_state,
	mut pcm: *mut *mut *mut ::core::ffi::c_float
) -> ::core::ffi::c_int {
	let mut vi: *mut vorbis_info = (*v).vi;
	if (*v).pcm_returned > -(1 as ::core::ffi::c_int) && (*v).pcm_returned < (*v).pcm_current {
		if !pcm.is_null() {
			let mut i: ::core::ffi::c_int = 0;
			i = 0 as ::core::ffi::c_int;
			while i < (*vi).channels {
				let ref mut fresh10 = *(*v).pcmret.offset(i as isize);
				*fresh10 = (*(*v).pcm.offset(i as isize)).offset((*v).pcm_returned as isize);
				i += 1;
			}
			*pcm = (*v).pcmret;
		}
		return (*v).pcm_current - (*v).pcm_returned;
	}
	return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn vorbis_synthesis_read(
	mut v: *mut vorbis_dsp_state,
	mut n: ::core::ffi::c_int
) -> ::core::ffi::c_int {
	if n != 0 && (*v).pcm_returned + n > (*v).pcm_current {
		return -(131 as ::core::ffi::c_int);
	}
	(*v).pcm_returned += n;
	return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn vorbis_synthesis_lapout(
	mut v: *mut vorbis_dsp_state,
	mut pcm: *mut *mut *mut ::core::ffi::c_float
) -> ::core::ffi::c_int {
	let mut vi: *mut vorbis_info = (*v).vi;
	let mut ci: *mut codec_setup_info = (*vi).codec_setup as *mut codec_setup_info;
	let mut hs: ::core::ffi::c_int = (*ci).halfrate_flag;
	let mut n: ::core::ffi::c_int =
		((*ci).blocksizes[(*v).W as usize] >> hs + 1 as ::core::ffi::c_int) as ::core::ffi::c_int;
	let mut n0: ::core::ffi::c_int = ((*ci).blocksizes[0 as ::core::ffi::c_int as usize]
		>> hs + 1 as ::core::ffi::c_int) as ::core::ffi::c_int;
	let mut n1: ::core::ffi::c_int = ((*ci).blocksizes[1 as ::core::ffi::c_int as usize]
		>> hs + 1 as ::core::ffi::c_int) as ::core::ffi::c_int;
	let mut i: ::core::ffi::c_int = 0;
	let mut j: ::core::ffi::c_int = 0;
	if (*v).pcm_returned < 0 as ::core::ffi::c_int {
		return 0 as ::core::ffi::c_int;
	}
	if (*v).centerW == n1 as ::core::ffi::c_long {
		j = 0 as ::core::ffi::c_int;
		while j < (*vi).channels {
			let mut p: *mut ::core::ffi::c_float = *(*v).pcm.offset(j as isize);
			i = 0 as ::core::ffi::c_int;
			while i < n1 {
				let mut temp: ::core::ffi::c_float = *p.offset(i as isize);
				*p.offset(i as isize) = *p.offset((i + n1) as isize);
				*p.offset((i + n1) as isize) = temp;
				i += 1;
			}
			j += 1;
		}
		(*v).pcm_current -= n1;
		(*v).pcm_returned -= n1;
		(*v).centerW = 0 as ::core::ffi::c_long;
	}
	if (*v).lW ^ (*v).W == 1 as ::core::ffi::c_long {
		j = 0 as ::core::ffi::c_int;
		while j < (*vi).channels {
			let mut s: *mut ::core::ffi::c_float = *(*v).pcm.offset(j as isize);
			let mut d: *mut ::core::ffi::c_float = (*(*v).pcm.offset(j as isize))
				.offset(((n1 - n0) / 2 as ::core::ffi::c_int) as isize);
			i = (n1 + n0) / 2 as ::core::ffi::c_int - 1 as ::core::ffi::c_int;
			while i >= 0 as ::core::ffi::c_int {
				*d.offset(i as isize) = *s.offset(i as isize);
				i -= 1;
			}
			j += 1;
		}
		(*v).pcm_returned += (n1 - n0) / 2 as ::core::ffi::c_int;
		(*v).pcm_current += (n1 - n0) / 2 as ::core::ffi::c_int;
	} else if (*v).lW == 0 as ::core::ffi::c_long {
		j = 0 as ::core::ffi::c_int;
		while j < (*vi).channels {
			let mut s_0: *mut ::core::ffi::c_float = *(*v).pcm.offset(j as isize);
			let mut d_0: *mut ::core::ffi::c_float = (*(*v).pcm.offset(j as isize))
				.offset(n1 as isize)
				.offset(-(n0 as isize));
			i = n0 - 1 as ::core::ffi::c_int;
			while i >= 0 as ::core::ffi::c_int {
				*d_0.offset(i as isize) = *s_0.offset(i as isize);
				i -= 1;
			}
			j += 1;
		}
		(*v).pcm_returned += n1 - n0;
		(*v).pcm_current += n1 - n0;
	}
	if !pcm.is_null() {
		let mut i_0: ::core::ffi::c_int = 0;
		i_0 = 0 as ::core::ffi::c_int;
		while i_0 < (*vi).channels {
			let ref mut fresh11 = *(*v).pcmret.offset(i_0 as isize);
			*fresh11 = (*(*v).pcm.offset(i_0 as isize)).offset((*v).pcm_returned as isize);
			i_0 += 1;
		}
		*pcm = (*v).pcmret;
	}
	return n1 + n - (*v).pcm_returned;
}
#[no_mangle]
pub unsafe extern "C" fn vorbis_window(
	mut v: *mut vorbis_dsp_state,
	mut W: ::core::ffi::c_int
) -> *const ::core::ffi::c_float {
	let mut vi: *mut vorbis_info = (*v).vi;
	let mut ci: *mut codec_setup_info = (*vi).codec_setup as *mut codec_setup_info;
	let mut hs: ::core::ffi::c_int = (*ci).halfrate_flag;
	let mut b: *mut private_state = (*v).backend_state as *mut private_state;
	if ((*b).window[W as usize] - 1 as ::core::ffi::c_int) < 0 as ::core::ffi::c_int {
		return ::core::ptr::null::<::core::ffi::c_float>();
	}
	return _vorbis_window_get((*b).window[W as usize] - hs);
}
