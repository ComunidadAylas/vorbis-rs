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
	fn memset(
		__s: *mut ::core::ffi::c_void,
		__c: ::core::ffi::c_int,
		__n: size_t
	) -> *mut ::core::ffi::c_void;
	fn memcmp(
		__s1: *const ::core::ffi::c_void,
		__s2: *const ::core::ffi::c_void,
		__n: size_t
	) -> ::core::ffi::c_int;
	fn strcpy(
		__dest: *mut ::core::ffi::c_char,
		__src: *const ::core::ffi::c_char
	) -> *mut ::core::ffi::c_char;
	fn strcat(
		__dest: *mut ::core::ffi::c_char,
		__src: *const ::core::ffi::c_char
	) -> *mut ::core::ffi::c_char;
	fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
	fn oggpack_writeinit(b: *mut oggpack_buffer);
	fn oggpack_reset(b: *mut oggpack_buffer);
	fn oggpack_writeclear(b: *mut oggpack_buffer);
	fn oggpack_readinit(
		b: *mut oggpack_buffer,
		buf: *mut ::core::ffi::c_uchar,
		bytes: ::core::ffi::c_int
	);
	fn oggpack_write(b: *mut oggpack_buffer, value: ::core::ffi::c_ulong, bits: ::core::ffi::c_int);
	fn oggpack_read(b: *mut oggpack_buffer, bits: ::core::ffi::c_int) -> ::core::ffi::c_long;
	fn oggpack_bytes(b: *mut oggpack_buffer) -> ::core::ffi::c_long;
	fn vorbis_staticbook_destroy(b: *mut static_codebook);
	fn vorbis_book_clear(b: *mut codebook);
	fn vorbis_decbook_clear(c: *mut dec_codebook);
	fn vorbis_staticbook_pack(
		c: *const static_codebook,
		b: *mut oggpack_buffer
	) -> ::core::ffi::c_int;
	fn vorbis_decbook_unpack(c: *mut dec_codebook, opb: *mut oggpack_buffer) -> ::core::ffi::c_int;
	fn _vi_psy_free(i: *mut vorbis_info_psy);
	fn ov_ilog(v: ogg_uint32_t) -> ::core::ffi::c_int;
}
use ogg_c2rust_sys::{ogg_packet, oggpack_buffer};

use super::{
	block::{alloc_chain, vorbis_block, vorbis_dsp_state},
	registry::{_floor_P, _mapping_P, _residue_P}
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
pub struct vorbis_info {
	pub version: ::core::ffi::c_int,
	pub channels: ::core::ffi::c_int,
	pub rate: ::core::ffi::c_long,
	pub bitrate_upper: ::core::ffi::c_long,
	pub bitrate_nominal: ::core::ffi::c_long,
	pub bitrate_lower: ::core::ffi::c_long,
	pub bitrate_window: ::core::ffi::c_long,
	pub codec_setup: *mut ::core::ffi::c_void
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct vorbis_comment {
	pub user_comments: *mut *mut ::core::ffi::c_char,
	pub comment_lengths: *mut ::core::ffi::c_int,
	pub comments: ::core::ffi::c_int,
	pub vendor: *mut ::core::ffi::c_char
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
pub type vorbis_look_residue = ();
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
pub type vorbis_look_floor = ();
#[derive(Copy, Clone)]
#[repr(C)]
pub struct vorbis_func_mapping {
	pub pack: Option<
		unsafe extern "C" fn(*mut vorbis_info, *mut ::core::ffi::c_void, *mut oggpack_buffer) -> ()
	>,
	pub unpack: Option<
		unsafe extern "C" fn(*mut vorbis_info, *mut oggpack_buffer) -> *mut ::core::ffi::c_void
	>,
	pub free_info: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
	pub forward: Option<unsafe extern "C" fn(*mut vorbis_block) -> ::core::ffi::c_int>,
	pub inverse: Option<
		unsafe extern "C" fn(*mut vorbis_block, *mut ::core::ffi::c_void) -> ::core::ffi::c_int
	>
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
pub const OV_EFAULT: ::core::ffi::c_int = -(129 as ::core::ffi::c_int);
pub const OV_EIMPL: ::core::ffi::c_int = -(130 as ::core::ffi::c_int);
pub const OV_ENOTVORBIS: ::core::ffi::c_int = -(132 as ::core::ffi::c_int);
pub const OV_EBADHEADER: ::core::ffi::c_int = -(133 as ::core::ffi::c_int);
pub const OV_EVERSION: ::core::ffi::c_int = -(134 as ::core::ffi::c_int);
pub const VI_WINDOWB: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const VI_TIMEB: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const VI_FLOORB: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const VI_RESB: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
pub const VI_MAPB: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const GENERAL_VENDOR_STRING: [::core::ffi::c_char; 25] = unsafe {
	::core::mem::transmute::<[u8; 25], [::core::ffi::c_char; 25]>(*b"Xiph.Org libVorbis 1.3.7\0")
};
pub const ENCODE_VENDOR_STRING: [::core::ffi::c_char; 53] = unsafe {
	::core::mem::transmute::<[u8; 53], [::core::ffi::c_char; 53]>(
		*b"Xiph.Org libVorbis I 20200704 (Reducing Environment)\0"
	)
};
unsafe extern "C" fn _v_writestring(
	mut o: *mut oggpack_buffer,
	mut s: *const ::core::ffi::c_char,
	mut bytes: ::core::ffi::c_int
) {
	loop {
		let fresh2 = bytes;
		bytes = bytes - 1;
		if !(fresh2 != 0) {
			break;
		}
		let fresh3 = s;
		s = s.offset(1);
		oggpack_write(o, *fresh3 as ::core::ffi::c_ulong, 8 as ::core::ffi::c_int);
	}
}
unsafe extern "C" fn _v_readstring(
	mut o: *mut oggpack_buffer,
	mut buf: *mut ::core::ffi::c_char,
	mut bytes: ::core::ffi::c_int
) {
	loop {
		let fresh4 = bytes;
		bytes = bytes - 1;
		if !(fresh4 != 0) {
			break;
		}
		let fresh5 = buf;
		buf = buf.offset(1);
		*fresh5 = oggpack_read(o, 8 as ::core::ffi::c_int) as ::core::ffi::c_char;
	}
}
unsafe extern "C" fn _v_toupper(mut c: ::core::ffi::c_int) -> ::core::ffi::c_int {
	return if c >= 'a' as i32 && c <= 'z' as i32 {
		c & !('a' as i32 - 'A' as i32)
	} else {
		c
	};
}
#[no_mangle]
pub unsafe extern "C" fn vorbis_comment_init(mut vc: *mut vorbis_comment) {
	memset(
		vc as *mut ::core::ffi::c_void,
		0 as ::core::ffi::c_int,
		::core::mem::size_of::<vorbis_comment>() as size_t
	);
}
#[no_mangle]
pub unsafe extern "C" fn vorbis_comment_add(
	mut vc: *mut vorbis_comment,
	mut comment: *const ::core::ffi::c_char
) {
	(*vc).user_comments = realloc(
		(*vc).user_comments as *mut ::core::ffi::c_void,
		(((*vc).comments + 2 as ::core::ffi::c_int) as size_t)
			.wrapping_mul(::core::mem::size_of::<*mut ::core::ffi::c_char>() as size_t)
	) as *mut *mut ::core::ffi::c_char;
	(*vc).comment_lengths = realloc(
		(*vc).comment_lengths as *mut ::core::ffi::c_void,
		(((*vc).comments + 2 as ::core::ffi::c_int) as size_t)
			.wrapping_mul(::core::mem::size_of::<::core::ffi::c_int>() as size_t)
	) as *mut ::core::ffi::c_int;
	*(*vc).comment_lengths.offset((*vc).comments as isize) = strlen(comment) as ::core::ffi::c_int;
	let ref mut fresh0 = *(*vc).user_comments.offset((*vc).comments as isize);
	*fresh0 = malloc(
		(*(*vc).comment_lengths.offset((*vc).comments as isize) + 1 as ::core::ffi::c_int)
			as size_t
	) as *mut ::core::ffi::c_char;
	strcpy(
		*(*vc).user_comments.offset((*vc).comments as isize),
		comment
	);
	(*vc).comments += 1;
	let ref mut fresh1 = *(*vc).user_comments.offset((*vc).comments as isize);
	*fresh1 = ::core::ptr::null_mut::<::core::ffi::c_char>();
}
#[no_mangle]
pub unsafe extern "C" fn vorbis_comment_add_tag(
	mut vc: *mut vorbis_comment,
	mut tag: *const ::core::ffi::c_char,
	mut contents: *const ::core::ffi::c_char
) {
	let mut comment: *mut ::core::ffi::c_char = malloc(
		strlen(tag)
			.wrapping_add(strlen(contents))
			.wrapping_add(2 as size_t)
	) as *mut ::core::ffi::c_char;
	strcpy(comment, tag);
	strcat(comment, b"=\0" as *const u8 as *const ::core::ffi::c_char);
	strcat(comment, contents);
	vorbis_comment_add(vc, comment);
	free(comment as *mut ::core::ffi::c_void);
}
unsafe extern "C" fn tagcompare(
	mut s1: *const ::core::ffi::c_char,
	mut s2: *const ::core::ffi::c_char,
	mut n: ::core::ffi::c_int
) -> ::core::ffi::c_int {
	let mut c: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
	while c < n {
		if _v_toupper(*s1.offset(c as isize) as ::core::ffi::c_int)
			!= _v_toupper(*s2.offset(c as isize) as ::core::ffi::c_int)
		{
			return (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int;
		}
		c += 1;
	}
	return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn vorbis_comment_query(
	mut vc: *mut vorbis_comment,
	mut tag: *const ::core::ffi::c_char,
	mut count: ::core::ffi::c_int
) -> *mut ::core::ffi::c_char {
	let mut i: ::core::ffi::c_long = 0;
	let mut found: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
	let mut taglen: ::core::ffi::c_int =
		strlen(tag).wrapping_add(1 as size_t) as ::core::ffi::c_int;
	let mut fulltag: *mut ::core::ffi::c_char =
		malloc((taglen + 1 as ::core::ffi::c_int) as size_t) as *mut ::core::ffi::c_char;
	strcpy(fulltag, tag);
	strcat(fulltag, b"=\0" as *const u8 as *const ::core::ffi::c_char);
	i = 0 as ::core::ffi::c_long;
	while i < (*vc).comments as ::core::ffi::c_long {
		if tagcompare(*(*vc).user_comments.offset(i as isize), fulltag, taglen) == 0 {
			if count == found {
				free(fulltag as *mut ::core::ffi::c_void);
				return (*(*vc).user_comments.offset(i as isize)).offset(taglen as isize);
			} else {
				found += 1;
			}
		}
		i += 1;
	}
	free(fulltag as *mut ::core::ffi::c_void);
	return ::core::ptr::null_mut::<::core::ffi::c_char>();
}
#[no_mangle]
pub unsafe extern "C" fn vorbis_comment_query_count(
	mut vc: *mut vorbis_comment,
	mut tag: *const ::core::ffi::c_char
) -> ::core::ffi::c_int {
	let mut i: ::core::ffi::c_int = 0;
	let mut count: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
	let mut taglen: ::core::ffi::c_int =
		strlen(tag).wrapping_add(1 as size_t) as ::core::ffi::c_int;
	let mut fulltag: *mut ::core::ffi::c_char =
		malloc((taglen + 1 as ::core::ffi::c_int) as size_t) as *mut ::core::ffi::c_char;
	strcpy(fulltag, tag);
	strcat(fulltag, b"=\0" as *const u8 as *const ::core::ffi::c_char);
	i = 0 as ::core::ffi::c_int;
	while i < (*vc).comments {
		if tagcompare(*(*vc).user_comments.offset(i as isize), fulltag, taglen) == 0 {
			count += 1;
		}
		i += 1;
	}
	free(fulltag as *mut ::core::ffi::c_void);
	return count;
}
#[no_mangle]
pub unsafe extern "C" fn vorbis_comment_clear(mut vc: *mut vorbis_comment) {
	if !vc.is_null() {
		let mut i: ::core::ffi::c_long = 0;
		if !(*vc).user_comments.is_null() {
			i = 0 as ::core::ffi::c_long;
			while i < (*vc).comments as ::core::ffi::c_long {
				if !(*(*vc).user_comments.offset(i as isize)).is_null() {
					free(*(*vc).user_comments.offset(i as isize) as *mut ::core::ffi::c_void);
				}
				i += 1;
			}
			free((*vc).user_comments as *mut ::core::ffi::c_void);
		}
		if !(*vc).comment_lengths.is_null() {
			free((*vc).comment_lengths as *mut ::core::ffi::c_void);
		}
		if !(*vc).vendor.is_null() {
			free((*vc).vendor as *mut ::core::ffi::c_void);
		}
		memset(
			vc as *mut ::core::ffi::c_void,
			0 as ::core::ffi::c_int,
			::core::mem::size_of::<vorbis_comment>() as size_t
		);
	}
}
#[no_mangle]
pub unsafe extern "C" fn vorbis_info_blocksize(
	mut vi: *mut vorbis_info,
	mut zo: ::core::ffi::c_int
) -> ::core::ffi::c_int {
	let mut ci: *mut codec_setup_info = (*vi).codec_setup as *mut codec_setup_info;
	return (if !ci.is_null() {
		(*ci).blocksizes[zo as usize]
	} else {
		-(1 as ::core::ffi::c_int) as ::core::ffi::c_long
	}) as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn vorbis_info_init(mut vi: *mut vorbis_info) {
	memset(
		vi as *mut ::core::ffi::c_void,
		0 as ::core::ffi::c_int,
		::core::mem::size_of::<vorbis_info>() as size_t
	);
	(*vi).codec_setup = calloc(
		1 as size_t,
		::core::mem::size_of::<codec_setup_info>() as size_t
	);
}
#[no_mangle]
pub unsafe extern "C" fn vorbis_info_clear(mut vi: *mut vorbis_info) {
	let mut ci: *mut codec_setup_info = (*vi).codec_setup as *mut codec_setup_info;
	let mut i: ::core::ffi::c_int = 0;
	if !ci.is_null() {
		i = 0 as ::core::ffi::c_int;
		while i < (*ci).modes {
			if !(*ci).mode_param[i as usize].is_null() {
				free((*ci).mode_param[i as usize] as *mut ::core::ffi::c_void);
			}
			i += 1;
		}
		i = 0 as ::core::ffi::c_int;
		while i < (*ci).maps {
			if !(*ci).map_param[i as usize].is_null() {
				(**(&raw const _mapping_P as *const *const vorbis_func_mapping)
					.offset((*ci).map_type[i as usize] as isize))
				.free_info
				.expect("non-null function pointer")((*ci).map_param[i as usize]);
			}
			i += 1;
		}
		i = 0 as ::core::ffi::c_int;
		while i < (*ci).floors {
			if !(*ci).floor_param[i as usize].is_null() {
				(**(&raw const _floor_P as *const *const vorbis_func_floor)
					.offset((*ci).floor_type[i as usize] as isize))
				.free_info
				.expect("non-null function pointer")((*ci).floor_param[i as usize]);
			}
			i += 1;
		}
		i = 0 as ::core::ffi::c_int;
		while i < (*ci).residues {
			if !(*ci).residue_param[i as usize].is_null() {
				(**(&raw const _residue_P as *const *const vorbis_func_residue)
					.offset((*ci).residue_type[i as usize] as isize))
				.free_info
				.expect("non-null function pointer")((*ci).residue_param[i as usize]);
			}
			i += 1;
		}
		i = 0 as ::core::ffi::c_int;
		while i < (*ci).books {
			if !(*ci).book_param[i as usize].is_null() {
				vorbis_staticbook_destroy((*ci).book_param[i as usize]);
			}
			if !(*ci).fullbooks.is_null() {
				vorbis_book_clear((*ci).fullbooks.offset(i as isize));
			}
			if !(*ci).decbooks.is_null() {
				vorbis_decbook_clear((*ci).decbooks.offset(i as isize));
			}
			i += 1;
		}
		if !(*ci).fullbooks.is_null() {
			free((*ci).fullbooks as *mut ::core::ffi::c_void);
		}
		if !(*ci).decbooks.is_null() {
			free((*ci).decbooks as *mut ::core::ffi::c_void);
		}
		i = 0 as ::core::ffi::c_int;
		while i < (*ci).psys {
			_vi_psy_free((*ci).psy_param[i as usize]);
			i += 1;
		}
		free(ci as *mut ::core::ffi::c_void);
	}
	memset(
		vi as *mut ::core::ffi::c_void,
		0 as ::core::ffi::c_int,
		::core::mem::size_of::<vorbis_info>() as size_t
	);
}
unsafe extern "C" fn _vorbis_unpack_info(
	mut vi: *mut vorbis_info,
	mut opb: *mut oggpack_buffer
) -> ::core::ffi::c_int {
	let mut ci: *mut codec_setup_info = (*vi).codec_setup as *mut codec_setup_info;
	let mut bs: ::core::ffi::c_int = 0;
	if ci.is_null() {
		return -(129 as ::core::ffi::c_int);
	}
	(*vi).version = oggpack_read(opb, 32 as ::core::ffi::c_int) as ::core::ffi::c_int;
	if (*vi).version != 0 as ::core::ffi::c_int {
		return -(134 as ::core::ffi::c_int);
	}
	(*vi).channels = oggpack_read(opb, 8 as ::core::ffi::c_int) as ::core::ffi::c_int;
	(*vi).rate = oggpack_read(opb, 32 as ::core::ffi::c_int);
	(*vi).bitrate_upper =
		oggpack_read(opb, 32 as ::core::ffi::c_int) as ogg_int32_t as ::core::ffi::c_long;
	(*vi).bitrate_nominal =
		oggpack_read(opb, 32 as ::core::ffi::c_int) as ogg_int32_t as ::core::ffi::c_long;
	(*vi).bitrate_lower =
		oggpack_read(opb, 32 as ::core::ffi::c_int) as ogg_int32_t as ::core::ffi::c_long;
	bs = oggpack_read(opb, 4 as ::core::ffi::c_int) as ::core::ffi::c_int;
	if !(bs < 0 as ::core::ffi::c_int) {
		(*ci).blocksizes[0 as ::core::ffi::c_int as usize] =
			((1 as ::core::ffi::c_int) << bs) as ::core::ffi::c_long;
		bs = oggpack_read(opb, 4 as ::core::ffi::c_int) as ::core::ffi::c_int;
		if !(bs < 0 as ::core::ffi::c_int) {
			(*ci).blocksizes[1 as ::core::ffi::c_int as usize] =
				((1 as ::core::ffi::c_int) << bs) as ::core::ffi::c_long;
			if !((*vi).rate < 1 as ::core::ffi::c_long) {
				if !((*vi).channels < 1 as ::core::ffi::c_int) {
					if !((*ci).blocksizes[0 as ::core::ffi::c_int as usize]
						< 64 as ::core::ffi::c_long)
					{
						if !((*ci).blocksizes[1 as ::core::ffi::c_int as usize]
							< (*ci).blocksizes[0 as ::core::ffi::c_int as usize])
						{
							if !((*ci).blocksizes[1 as ::core::ffi::c_int as usize]
								> 8192 as ::core::ffi::c_long)
							{
								if !(oggpack_read(opb, 1 as ::core::ffi::c_int)
									!= 1 as ::core::ffi::c_long)
								{
									return 0 as ::core::ffi::c_int;
								}
							}
						}
					}
				}
			}
		}
	}
	vorbis_info_clear(vi);
	return -(133 as ::core::ffi::c_int);
}
unsafe extern "C" fn _vorbis_unpack_comment(
	mut vc: *mut vorbis_comment,
	mut opb: *mut oggpack_buffer
) -> ::core::ffi::c_int {
	let mut current_block: u64;
	let mut i: ::core::ffi::c_int = 0;
	let mut vendorlen: ::core::ffi::c_int =
		oggpack_read(opb, 32 as ::core::ffi::c_int) as ::core::ffi::c_int;
	if !(vendorlen < 0 as ::core::ffi::c_int) {
		if !(vendorlen as ::core::ffi::c_long > (*opb).storage - 8 as ::core::ffi::c_long) {
			(*vc).vendor = calloc((vendorlen + 1 as ::core::ffi::c_int) as size_t, 1 as size_t)
				as *mut ::core::ffi::c_char;
			_v_readstring(opb, (*vc).vendor, vendorlen);
			i = oggpack_read(opb, 32 as ::core::ffi::c_int) as ::core::ffi::c_int;
			if !(i < 0 as ::core::ffi::c_int) {
				if !(i as ::core::ffi::c_long
					> (*opb).storage - oggpack_bytes(opb) >> 2 as ::core::ffi::c_int)
				{
					(*vc).comments = i;
					(*vc).user_comments = calloc(
						((*vc).comments + 1 as ::core::ffi::c_int) as size_t,
						::core::mem::size_of::<*mut ::core::ffi::c_char>() as size_t
					) as *mut *mut ::core::ffi::c_char;
					(*vc).comment_lengths = calloc(
						((*vc).comments + 1 as ::core::ffi::c_int) as size_t,
						::core::mem::size_of::<::core::ffi::c_int>() as size_t
					) as *mut ::core::ffi::c_int;
					i = 0 as ::core::ffi::c_int;
					loop {
						if !(i < (*vc).comments) {
							current_block = 17860125682698302841;
							break;
						}
						let mut len: ::core::ffi::c_int =
							oggpack_read(opb, 32 as ::core::ffi::c_int) as ::core::ffi::c_int;
						if len < 0 as ::core::ffi::c_int {
							current_block = 3632844414725951152;
							break;
						}
						if len as ::core::ffi::c_long > (*opb).storage - oggpack_bytes(opb) {
							current_block = 3632844414725951152;
							break;
						}
						*(*vc).comment_lengths.offset(i as isize) = len;
						let ref mut fresh6 = *(*vc).user_comments.offset(i as isize);
						*fresh6 = calloc((len + 1 as ::core::ffi::c_int) as size_t, 1 as size_t)
							as *mut ::core::ffi::c_char;
						_v_readstring(opb, *(*vc).user_comments.offset(i as isize), len);
						i += 1;
					}
					match current_block {
						3632844414725951152 => {}
						_ => {
							if !(oggpack_read(opb, 1 as ::core::ffi::c_int)
								!= 1 as ::core::ffi::c_long)
							{
								return 0 as ::core::ffi::c_int;
							}
						}
					}
				}
			}
		}
	}
	vorbis_comment_clear(vc);
	return -(133 as ::core::ffi::c_int);
}
unsafe extern "C" fn _vorbis_unpack_books(
	mut vi: *mut vorbis_info,
	mut opb: *mut oggpack_buffer
) -> ::core::ffi::c_int {
	let mut current_block: u64;
	let mut ci: *mut codec_setup_info = (*vi).codec_setup as *mut codec_setup_info;
	let mut i: ::core::ffi::c_int = 0;
	(*ci).books = (oggpack_read(opb, 8 as ::core::ffi::c_int) + 1 as ::core::ffi::c_long)
		as ::core::ffi::c_int;
	if !((*ci).books <= 0 as ::core::ffi::c_int) {
		(*ci).decbooks = calloc(
			(*ci).books as size_t,
			::core::mem::size_of::<dec_codebook>() as size_t
		) as *mut dec_codebook;
		if !(*ci).decbooks.is_null() {
			i = 0 as ::core::ffi::c_int;
			loop {
				if !(i < (*ci).books) {
					current_block = 7815301370352969686;
					break;
				}
				if vorbis_decbook_unpack((*ci).decbooks.offset(i as isize), opb)
					< 0 as ::core::ffi::c_int
				{
					current_block = 12206915172539270337;
					break;
				}
				i += 1;
			}
			match current_block {
				12206915172539270337 => {}
				_ => {
					let mut times: ::core::ffi::c_int = (oggpack_read(opb, 6 as ::core::ffi::c_int)
						+ 1 as ::core::ffi::c_long)
						as ::core::ffi::c_int;
					if !(times <= 0 as ::core::ffi::c_int) {
						i = 0 as ::core::ffi::c_int;
						loop {
							if !(i < times) {
								current_block = 12800627514080957624;
								break;
							}
							let mut test: ::core::ffi::c_int =
								oggpack_read(opb, 16 as ::core::ffi::c_int) as ::core::ffi::c_int;
							if test < 0 as ::core::ffi::c_int || test >= VI_TIMEB {
								current_block = 12206915172539270337;
								break;
							}
							i += 1;
						}
						match current_block {
							12206915172539270337 => {}
							_ => {
								(*ci).floors = (oggpack_read(opb, 6 as ::core::ffi::c_int)
									+ 1 as ::core::ffi::c_long)
									as ::core::ffi::c_int;
								if !((*ci).floors <= 0 as ::core::ffi::c_int) {
									i = 0 as ::core::ffi::c_int;
									loop {
										if !(i < (*ci).floors) {
											current_block = 12147880666119273379;
											break;
										}
										(*ci).floor_type[i as usize] =
											oggpack_read(opb, 16 as ::core::ffi::c_int)
												as ::core::ffi::c_int;
										if (*ci).floor_type[i as usize] < 0 as ::core::ffi::c_int
											|| (*ci).floor_type[i as usize] >= VI_FLOORB
										{
											current_block = 12206915172539270337;
											break;
										}
										(*ci).floor_param[i as usize] =
											(**(&raw const _floor_P
												as *const *const vorbis_func_floor)
												.offset((*ci).floor_type[i as usize] as isize))
											.unpack
											.expect("non-null function pointer")(vi, opb);
										if (*ci).floor_param[i as usize].is_null() {
											current_block = 12206915172539270337;
											break;
										}
										i += 1;
									}
									match current_block {
										12206915172539270337 => {}
										_ => {
											(*ci).residues =
												(oggpack_read(opb, 6 as ::core::ffi::c_int)
													+ 1 as ::core::ffi::c_long)
													as ::core::ffi::c_int;
											if !((*ci).residues <= 0 as ::core::ffi::c_int) {
												i = 0 as ::core::ffi::c_int;
												loop {
													if !(i < (*ci).residues) {
														current_block = 14648156034262866959;
														break;
													}
													(*ci).residue_type[i as usize] =
														oggpack_read(opb, 16 as ::core::ffi::c_int)
															as ::core::ffi::c_int;
													if (*ci).residue_type[i as usize]
														< 0 as ::core::ffi::c_int || (*ci)
														.residue_type[i as usize]
														>= VI_RESB
													{
														current_block = 12206915172539270337;
														break;
													}
													(*ci).residue_param[i as usize] =
														(**(&raw const _residue_P
															as *const *const vorbis_func_residue)
															.offset(
																(*ci).residue_type[i as usize]
																	as isize
															))
														.unpack
														.expect("non-null function pointer")(
															vi, opb
														);
													if (*ci).residue_param[i as usize].is_null() {
														current_block = 12206915172539270337;
														break;
													}
													i += 1;
												}
												match current_block {
													12206915172539270337 => {}
													_ => {
														(*ci).maps = (oggpack_read(
															opb,
															6 as ::core::ffi::c_int
														) + 1 as ::core::ffi::c_long)
															as ::core::ffi::c_int;
														if !((*ci).maps <= 0 as ::core::ffi::c_int)
														{
															i = 0 as ::core::ffi::c_int;
															loop {
																if !(i < (*ci).maps) {
																	current_block =
																		7828949454673616476;
																	break;
																}
																(*ci).map_type[i as usize] =
																	oggpack_read(
																		opb,
																		16 as ::core::ffi::c_int
																	)
																		as ::core::ffi::c_int;
																if (*ci).map_type[i as usize]
																	< 0 as ::core::ffi::c_int
																	|| (*ci).map_type[i as usize]
																		>= VI_MAPB
																{
																	current_block =
																		12206915172539270337;
																	break;
																}
																(*ci).map_param[i as usize] = (**(&raw const _mapping_P
                                                                    as *const *const vorbis_func_mapping)
                                                                    .offset((*ci).map_type[i as usize] as isize))
                                                                    .unpack
                                                                    .expect("non-null function pointer")(vi, opb);
																if (*ci).map_param[i as usize]
																	.is_null()
																{
																	current_block =
																		12206915172539270337;
																	break;
																}
																i += 1;
															}
															match current_block {
																12206915172539270337 => {}
																_ => {
																	(*ci).modes = (oggpack_read(
																		opb,
																		6 as ::core::ffi::c_int
																	) + 1
																		as ::core::ffi::c_long)
																		as ::core::ffi::c_int;
																	if !((*ci).modes
																		<= 0 as ::core::ffi::c_int)
																	{
																		i = 0 as ::core::ffi::c_int;
																		loop {
																			if !(i < (*ci).modes) {
																				current_block = 3160140712158701372;
																				break;
																			}
																			(*ci).mode_param[i as usize] = calloc(
                                                                                1 as size_t,
                                                                                ::core::mem::size_of::<vorbis_info_mode>() as size_t,
                                                                            ) as *mut vorbis_info_mode;
																			(*(*ci).mode_param[i as usize]).blockflag = oggpack_read(
                                                                                opb,
                                                                                1 as ::core::ffi::c_int,
                                                                            ) as ::core::ffi::c_int;
																			(*(*ci).mode_param[i as usize]).windowtype = oggpack_read(
                                                                                opb,
                                                                                16 as ::core::ffi::c_int,
                                                                            ) as ::core::ffi::c_int;
																			(*(*ci).mode_param[i as usize]).transformtype = oggpack_read(
                                                                                opb,
                                                                                16 as ::core::ffi::c_int,
                                                                            ) as ::core::ffi::c_int;
																			(*(*ci).mode_param[i as usize]).mapping = oggpack_read(
                                                                                opb,
                                                                                8 as ::core::ffi::c_int,
                                                                            ) as ::core::ffi::c_int;
																			if (*(*ci).mode_param
																				[i as usize])
																				.windowtype
																				>= VI_WINDOWB
																			{
																				current_block = 12206915172539270337;
																				break;
																			}
																			if (*(*ci).mode_param
																				[i as usize])
																				.transformtype
																				>= VI_WINDOWB
																			{
																				current_block = 12206915172539270337;
																				break;
																			}
																			if (*(*ci).mode_param
																				[i as usize])
																				.mapping >= (*ci)
																				.maps
																			{
																				current_block = 12206915172539270337;
																				break;
																			}
																			if (*(*ci).mode_param[i as usize]).mapping
                                                                                < 0 as ::core::ffi::c_int
                                                                            {
                                                                                current_block = 12206915172539270337;
                                                                                break;
                                                                            }
																			i += 1;
																		}
																		match current_block {
                                                                            12206915172539270337 => {}
                                                                            _ => {
                                                                                if !(oggpack_read(opb, 1 as ::core::ffi::c_int)
                                                                                    != 1 as ::core::ffi::c_long)
                                                                                {
                                                                                    return 0 as ::core::ffi::c_int;
                                                                                }
                                                                            }
                                                                        }
																	}
																}
															}
														}
													}
												}
											}
										}
									}
								}
							}
						}
					}
				}
			}
		}
	}
	vorbis_info_clear(vi);
	return -(133 as ::core::ffi::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn vorbis_synthesis_idheader(mut op: *mut ogg_packet) -> ::core::ffi::c_int {
	let mut opb: oggpack_buffer = oggpack_buffer {
		endbyte: 0,
		endbit: 0,
		buffer: ::core::ptr::null_mut::<::core::ffi::c_uchar>(),
		ptr: ::core::ptr::null_mut::<::core::ffi::c_uchar>(),
		storage: 0
	};
	let mut buffer: [::core::ffi::c_char; 6] = [0; 6];
	if !op.is_null() {
		oggpack_readinit(
			&raw mut opb,
			(*op).packet,
			(*op).bytes as ::core::ffi::c_int
		);
		if (*op).b_o_s == 0 {
			return 0 as ::core::ffi::c_int;
		}
		if oggpack_read(&raw mut opb, 8 as ::core::ffi::c_int) != 1 as ::core::ffi::c_long {
			return 0 as ::core::ffi::c_int;
		}
		memset(
			&raw mut buffer as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void,
			0 as ::core::ffi::c_int,
			6 as size_t
		);
		_v_readstring(
			&raw mut opb,
			&raw mut buffer as *mut ::core::ffi::c_char,
			6 as ::core::ffi::c_int
		);
		if memcmp(
			&raw mut buffer as *mut ::core::ffi::c_char as *const ::core::ffi::c_void,
			b"vorbis\0" as *const u8 as *const ::core::ffi::c_char as *const ::core::ffi::c_void,
			6 as size_t
		) != 0
		{
			return 0 as ::core::ffi::c_int;
		}
		return 1 as ::core::ffi::c_int;
	}
	return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn vorbis_synthesis_headerin(
	mut vi: *mut vorbis_info,
	mut vc: *mut vorbis_comment,
	mut op: *mut ogg_packet
) -> ::core::ffi::c_int {
	let mut opb: oggpack_buffer = oggpack_buffer {
		endbyte: 0,
		endbit: 0,
		buffer: ::core::ptr::null_mut::<::core::ffi::c_uchar>(),
		ptr: ::core::ptr::null_mut::<::core::ffi::c_uchar>(),
		storage: 0
	};
	if !op.is_null() {
		oggpack_readinit(
			&raw mut opb,
			(*op).packet,
			(*op).bytes as ::core::ffi::c_int
		);
		let mut buffer: [::core::ffi::c_char; 6] = [0; 6];
		let mut packtype: ::core::ffi::c_int =
			oggpack_read(&raw mut opb, 8 as ::core::ffi::c_int) as ::core::ffi::c_int;
		memset(
			&raw mut buffer as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void,
			0 as ::core::ffi::c_int,
			6 as size_t
		);
		_v_readstring(
			&raw mut opb,
			&raw mut buffer as *mut ::core::ffi::c_char,
			6 as ::core::ffi::c_int
		);
		if memcmp(
			&raw mut buffer as *mut ::core::ffi::c_char as *const ::core::ffi::c_void,
			b"vorbis\0" as *const u8 as *const ::core::ffi::c_char as *const ::core::ffi::c_void,
			6 as size_t
		) != 0
		{
			return -(132 as ::core::ffi::c_int);
		}
		match packtype {
			1 => {
				if (*op).b_o_s == 0 {
					return -(133 as ::core::ffi::c_int);
				}
				if (*vi).rate != 0 as ::core::ffi::c_long {
					return -(133 as ::core::ffi::c_int);
				}
				return _vorbis_unpack_info(vi, &raw mut opb);
			}
			3 => {
				if (*vi).rate == 0 as ::core::ffi::c_long {
					return -(133 as ::core::ffi::c_int);
				}
				if !(*vc).vendor.is_null() {
					return -(133 as ::core::ffi::c_int);
				}
				return _vorbis_unpack_comment(vc, &raw mut opb);
			}
			5 => {
				if (*vi).rate == 0 as ::core::ffi::c_long || (*vc).vendor.is_null() {
					return -(133 as ::core::ffi::c_int);
				}
				if (*vi).codec_setup.is_null() {
					return -(129 as ::core::ffi::c_int);
				}
				if (*((*vi).codec_setup as *mut codec_setup_info)).books > 0 as ::core::ffi::c_int {
					return -(133 as ::core::ffi::c_int);
				}
				return _vorbis_unpack_books(vi, &raw mut opb);
			}
			_ => return -(133 as ::core::ffi::c_int)
		}
	}
	return -(133 as ::core::ffi::c_int);
}
unsafe extern "C" fn _vorbis_pack_info(
	mut opb: *mut oggpack_buffer,
	mut vi: *mut vorbis_info
) -> ::core::ffi::c_int {
	let mut ci: *mut codec_setup_info = (*vi).codec_setup as *mut codec_setup_info;
	if ci.is_null()
		|| (*ci).blocksizes[0 as ::core::ffi::c_int as usize] < 64 as ::core::ffi::c_long
		|| (*ci).blocksizes[1 as ::core::ffi::c_int as usize]
			< (*ci).blocksizes[0 as ::core::ffi::c_int as usize]
	{
		return -(129 as ::core::ffi::c_int);
	}
	oggpack_write(opb, 0x1 as ::core::ffi::c_ulong, 8 as ::core::ffi::c_int);
	_v_writestring(
		opb,
		b"vorbis\0" as *const u8 as *const ::core::ffi::c_char,
		6 as ::core::ffi::c_int
	);
	oggpack_write(opb, 0 as ::core::ffi::c_ulong, 32 as ::core::ffi::c_int);
	oggpack_write(
		opb,
		(*vi).channels as ::core::ffi::c_ulong,
		8 as ::core::ffi::c_int
	);
	oggpack_write(
		opb,
		(*vi).rate as ::core::ffi::c_ulong,
		32 as ::core::ffi::c_int
	);
	oggpack_write(
		opb,
		(*vi).bitrate_upper as ::core::ffi::c_ulong,
		32 as ::core::ffi::c_int
	);
	oggpack_write(
		opb,
		(*vi).bitrate_nominal as ::core::ffi::c_ulong,
		32 as ::core::ffi::c_int
	);
	oggpack_write(
		opb,
		(*vi).bitrate_lower as ::core::ffi::c_ulong,
		32 as ::core::ffi::c_int
	);
	oggpack_write(
		opb,
		ov_ilog(
			((*ci).blocksizes[0 as ::core::ffi::c_int as usize] - 1 as ::core::ffi::c_long)
				as ogg_uint32_t
		) as ::core::ffi::c_ulong,
		4 as ::core::ffi::c_int
	);
	oggpack_write(
		opb,
		ov_ilog(
			((*ci).blocksizes[1 as ::core::ffi::c_int as usize] - 1 as ::core::ffi::c_long)
				as ogg_uint32_t
		) as ::core::ffi::c_ulong,
		4 as ::core::ffi::c_int
	);
	oggpack_write(opb, 1 as ::core::ffi::c_ulong, 1 as ::core::ffi::c_int);
	return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn _vorbis_pack_comment(
	mut opb: *mut oggpack_buffer,
	mut vc: *mut vorbis_comment
) -> ::core::ffi::c_int {
	let mut bytes: ::core::ffi::c_int = strlen(ENCODE_VENDOR_STRING.as_ptr()) as ::core::ffi::c_int;
	oggpack_write(opb, 0x3 as ::core::ffi::c_ulong, 8 as ::core::ffi::c_int);
	_v_writestring(
		opb,
		b"vorbis\0" as *const u8 as *const ::core::ffi::c_char,
		6 as ::core::ffi::c_int
	);
	oggpack_write(opb, bytes as ::core::ffi::c_ulong, 32 as ::core::ffi::c_int);
	_v_writestring(opb, ENCODE_VENDOR_STRING.as_ptr(), bytes);
	oggpack_write(
		opb,
		(*vc).comments as ::core::ffi::c_ulong,
		32 as ::core::ffi::c_int
	);
	if (*vc).comments != 0 {
		let mut i: ::core::ffi::c_int = 0;
		i = 0 as ::core::ffi::c_int;
		while i < (*vc).comments {
			if !(*(*vc).user_comments.offset(i as isize)).is_null() {
				oggpack_write(
					opb,
					*(*vc).comment_lengths.offset(i as isize) as ::core::ffi::c_ulong,
					32 as ::core::ffi::c_int
				);
				_v_writestring(
					opb,
					*(*vc).user_comments.offset(i as isize),
					*(*vc).comment_lengths.offset(i as isize)
				);
			} else {
				oggpack_write(opb, 0 as ::core::ffi::c_ulong, 32 as ::core::ffi::c_int);
			}
			i += 1;
		}
	}
	oggpack_write(opb, 1 as ::core::ffi::c_ulong, 1 as ::core::ffi::c_int);
	return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn _vorbis_pack_books(
	mut opb: *mut oggpack_buffer,
	mut vi: *mut vorbis_info
) -> ::core::ffi::c_int {
	let mut current_block: u64;
	let mut ci: *mut codec_setup_info = (*vi).codec_setup as *mut codec_setup_info;
	let mut i: ::core::ffi::c_int = 0;
	if ci.is_null() {
		return -(129 as ::core::ffi::c_int);
	}
	oggpack_write(opb, 0x5 as ::core::ffi::c_ulong, 8 as ::core::ffi::c_int);
	_v_writestring(
		opb,
		b"vorbis\0" as *const u8 as *const ::core::ffi::c_char,
		6 as ::core::ffi::c_int
	);
	oggpack_write(
		opb,
		((*ci).books - 1 as ::core::ffi::c_int) as ::core::ffi::c_ulong,
		8 as ::core::ffi::c_int
	);
	i = 0 as ::core::ffi::c_int;
	loop {
		if !(i < (*ci).books) {
			current_block = 4906268039856690917;
			break;
		}
		if vorbis_staticbook_pack((*ci).book_param[i as usize], opb) != 0 {
			current_block = 11449587397151675551;
			break;
		}
		i += 1;
	}
	match current_block {
		4906268039856690917 => {
			oggpack_write(opb, 0 as ::core::ffi::c_ulong, 6 as ::core::ffi::c_int);
			oggpack_write(opb, 0 as ::core::ffi::c_ulong, 16 as ::core::ffi::c_int);
			oggpack_write(
				opb,
				((*ci).floors - 1 as ::core::ffi::c_int) as ::core::ffi::c_ulong,
				6 as ::core::ffi::c_int
			);
			i = 0 as ::core::ffi::c_int;
			loop {
				if !(i < (*ci).floors) {
					current_block = 10599921512955367680;
					break;
				}
				oggpack_write(
					opb,
					(*ci).floor_type[i as usize] as ::core::ffi::c_ulong,
					16 as ::core::ffi::c_int
				);
				if !(**(&raw const _floor_P as *const *const vorbis_func_floor)
					.offset((*ci).floor_type[i as usize] as isize))
				.pack
				.is_some()
				{
					current_block = 11449587397151675551;
					break;
				}
				(**(&raw const _floor_P as *const *const vorbis_func_floor)
					.offset((*ci).floor_type[i as usize] as isize))
				.pack
				.expect("non-null function pointer")((*ci).floor_param[i as usize], opb);
				i += 1;
			}
			match current_block {
				11449587397151675551 => {}
				_ => {
					oggpack_write(
						opb,
						((*ci).residues - 1 as ::core::ffi::c_int) as ::core::ffi::c_ulong,
						6 as ::core::ffi::c_int
					);
					i = 0 as ::core::ffi::c_int;
					while i < (*ci).residues {
						oggpack_write(
							opb,
							(*ci).residue_type[i as usize] as ::core::ffi::c_ulong,
							16 as ::core::ffi::c_int
						);
						(**(&raw const _residue_P as *const *const vorbis_func_residue)
							.offset((*ci).residue_type[i as usize] as isize))
						.pack
						.expect("non-null function pointer")(
							(*ci).residue_param[i as usize], opb
						);
						i += 1;
					}
					oggpack_write(
						opb,
						((*ci).maps - 1 as ::core::ffi::c_int) as ::core::ffi::c_ulong,
						6 as ::core::ffi::c_int
					);
					i = 0 as ::core::ffi::c_int;
					while i < (*ci).maps {
						oggpack_write(
							opb,
							(*ci).map_type[i as usize] as ::core::ffi::c_ulong,
							16 as ::core::ffi::c_int
						);
						(**(&raw const _mapping_P as *const *const vorbis_func_mapping)
							.offset((*ci).map_type[i as usize] as isize))
						.pack
						.expect("non-null function pointer")(
							vi, (*ci).map_param[i as usize], opb
						);
						i += 1;
					}
					oggpack_write(
						opb,
						((*ci).modes - 1 as ::core::ffi::c_int) as ::core::ffi::c_ulong,
						6 as ::core::ffi::c_int
					);
					i = 0 as ::core::ffi::c_int;
					while i < (*ci).modes {
						oggpack_write(
							opb,
							(*(*ci).mode_param[i as usize]).blockflag as ::core::ffi::c_ulong,
							1 as ::core::ffi::c_int
						);
						oggpack_write(
							opb,
							(*(*ci).mode_param[i as usize]).windowtype as ::core::ffi::c_ulong,
							16 as ::core::ffi::c_int
						);
						oggpack_write(
							opb,
							(*(*ci).mode_param[i as usize]).transformtype as ::core::ffi::c_ulong,
							16 as ::core::ffi::c_int
						);
						oggpack_write(
							opb,
							(*(*ci).mode_param[i as usize]).mapping as ::core::ffi::c_ulong,
							8 as ::core::ffi::c_int
						);
						i += 1;
					}
					oggpack_write(opb, 1 as ::core::ffi::c_ulong, 1 as ::core::ffi::c_int);
					return 0 as ::core::ffi::c_int;
				}
			}
		}
		_ => {}
	}
	return -(1 as ::core::ffi::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn vorbis_commentheader_out(
	mut vc: *mut vorbis_comment,
	mut op: *mut ogg_packet
) -> ::core::ffi::c_int {
	let mut opb: oggpack_buffer = oggpack_buffer {
		endbyte: 0,
		endbit: 0,
		buffer: ::core::ptr::null_mut::<::core::ffi::c_uchar>(),
		ptr: ::core::ptr::null_mut::<::core::ffi::c_uchar>(),
		storage: 0
	};
	oggpack_writeinit(&raw mut opb);
	if _vorbis_pack_comment(&raw mut opb, vc) != 0 {
		oggpack_writeclear(&raw mut opb);
		return OV_EIMPL;
	}
	(*op).packet = malloc(oggpack_bytes(&raw mut opb) as size_t) as *mut ::core::ffi::c_uchar;
	memcpy(
		(*op).packet as *mut ::core::ffi::c_void,
		opb.buffer as *const ::core::ffi::c_void,
		oggpack_bytes(&raw mut opb) as size_t
	);
	(*op).bytes = oggpack_bytes(&raw mut opb);
	(*op).b_o_s = 0 as ::core::ffi::c_long;
	(*op).e_o_s = 0 as ::core::ffi::c_long;
	(*op).granulepos = 0 as ogg_int64_t;
	(*op).packetno = 1 as ogg_int64_t;
	oggpack_writeclear(&raw mut opb);
	return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn vorbis_analysis_headerout(
	mut v: *mut vorbis_dsp_state,
	mut vc: *mut vorbis_comment,
	mut op: *mut ogg_packet,
	mut op_comm: *mut ogg_packet,
	mut op_code: *mut ogg_packet
) -> ::core::ffi::c_int {
	let mut ret: ::core::ffi::c_int = OV_EIMPL;
	let mut vi: *mut vorbis_info = (*v).vi;
	let mut opb: oggpack_buffer = oggpack_buffer {
		endbyte: 0,
		endbit: 0,
		buffer: ::core::ptr::null_mut::<::core::ffi::c_uchar>(),
		ptr: ::core::ptr::null_mut::<::core::ffi::c_uchar>(),
		storage: 0
	};
	let mut b: *mut private_state = (*v).backend_state as *mut private_state;
	if b.is_null()
		|| (*vi).channels <= 0 as ::core::ffi::c_int
		|| (*vi).channels > 256 as ::core::ffi::c_int
	{
		b = ::core::ptr::null_mut::<private_state>();
		ret = OV_EFAULT;
	} else {
		oggpack_writeinit(&raw mut opb);
		if !(_vorbis_pack_info(&raw mut opb, vi) != 0) {
			if !(*b).header.is_null() {
				free((*b).header as *mut ::core::ffi::c_void);
			}
			(*b).header =
				malloc(oggpack_bytes(&raw mut opb) as size_t) as *mut ::core::ffi::c_uchar;
			memcpy(
				(*b).header as *mut ::core::ffi::c_void,
				opb.buffer as *const ::core::ffi::c_void,
				oggpack_bytes(&raw mut opb) as size_t
			);
			(*op).packet = (*b).header;
			(*op).bytes = oggpack_bytes(&raw mut opb);
			(*op).b_o_s = 1 as ::core::ffi::c_long;
			(*op).e_o_s = 0 as ::core::ffi::c_long;
			(*op).granulepos = 0 as ogg_int64_t;
			(*op).packetno = 0 as ogg_int64_t;
			oggpack_reset(&raw mut opb);
			if !(_vorbis_pack_comment(&raw mut opb, vc) != 0) {
				if !(*b).header1.is_null() {
					free((*b).header1 as *mut ::core::ffi::c_void);
				}
				(*b).header1 =
					malloc(oggpack_bytes(&raw mut opb) as size_t) as *mut ::core::ffi::c_uchar;
				memcpy(
					(*b).header1 as *mut ::core::ffi::c_void,
					opb.buffer as *const ::core::ffi::c_void,
					oggpack_bytes(&raw mut opb) as size_t
				);
				(*op_comm).packet = (*b).header1;
				(*op_comm).bytes = oggpack_bytes(&raw mut opb);
				(*op_comm).b_o_s = 0 as ::core::ffi::c_long;
				(*op_comm).e_o_s = 0 as ::core::ffi::c_long;
				(*op_comm).granulepos = 0 as ogg_int64_t;
				(*op_comm).packetno = 1 as ogg_int64_t;
				oggpack_reset(&raw mut opb);
				if !(_vorbis_pack_books(&raw mut opb, vi) != 0) {
					if !(*b).header2.is_null() {
						free((*b).header2 as *mut ::core::ffi::c_void);
					}
					(*b).header2 =
						malloc(oggpack_bytes(&raw mut opb) as size_t) as *mut ::core::ffi::c_uchar;
					memcpy(
						(*b).header2 as *mut ::core::ffi::c_void,
						opb.buffer as *const ::core::ffi::c_void,
						oggpack_bytes(&raw mut opb) as size_t
					);
					(*op_code).packet = (*b).header2;
					(*op_code).bytes = oggpack_bytes(&raw mut opb);
					(*op_code).b_o_s = 0 as ::core::ffi::c_long;
					(*op_code).e_o_s = 0 as ::core::ffi::c_long;
					(*op_code).granulepos = 0 as ogg_int64_t;
					(*op_code).packetno = 2 as ogg_int64_t;
					oggpack_writeclear(&raw mut opb);
					return 0 as ::core::ffi::c_int;
				}
			}
		}
	}
	memset(
		op as *mut ::core::ffi::c_void,
		0 as ::core::ffi::c_int,
		::core::mem::size_of::<ogg_packet>() as size_t
	);
	memset(
		op_comm as *mut ::core::ffi::c_void,
		0 as ::core::ffi::c_int,
		::core::mem::size_of::<ogg_packet>() as size_t
	);
	memset(
		op_code as *mut ::core::ffi::c_void,
		0 as ::core::ffi::c_int,
		::core::mem::size_of::<ogg_packet>() as size_t
	);
	if !b.is_null() {
		if (*vi).channels > 0 as ::core::ffi::c_int {
			oggpack_writeclear(&raw mut opb);
		}
		if !(*b).header.is_null() {
			free((*b).header as *mut ::core::ffi::c_void);
		}
		if !(*b).header1.is_null() {
			free((*b).header1 as *mut ::core::ffi::c_void);
		}
		if !(*b).header2.is_null() {
			free((*b).header2 as *mut ::core::ffi::c_void);
		}
		(*b).header = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
		(*b).header1 = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
		(*b).header2 = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
	}
	return ret;
}
#[no_mangle]
pub unsafe extern "C" fn vorbis_granule_time(
	mut v: *mut vorbis_dsp_state,
	mut granulepos: ogg_int64_t
) -> ::core::ffi::c_double {
	if granulepos == -(1 as ::core::ffi::c_int) as ogg_int64_t {
		return -(1 as ::core::ffi::c_int) as ::core::ffi::c_double;
	}
	if granulepos >= 0 as ogg_int64_t {
		return granulepos as ::core::ffi::c_double / (*(*v).vi).rate as ::core::ffi::c_double;
	} else {
		let mut granuleoff: ogg_int64_t = 0xffffffff as ogg_int64_t;
		granuleoff <<= 31 as ::core::ffi::c_int;
		granuleoff =
			(granuleoff as ::core::ffi::c_long | 0x7ffffffff as ::core::ffi::c_long) as ogg_int64_t;
		return (granulepos as ::core::ffi::c_double
			+ 2 as ::core::ffi::c_int as ::core::ffi::c_double
			+ granuleoff as ::core::ffi::c_double
			+ granuleoff as ::core::ffi::c_double)
			/ (*(*v).vi).rate as ::core::ffi::c_double;
	};
}
#[no_mangle]
pub unsafe extern "C" fn vorbis_version_string() -> *const ::core::ffi::c_char {
	return GENERAL_VENDOR_STRING.as_ptr();
}
