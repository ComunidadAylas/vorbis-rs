extern "C" {
	fn malloc(__size: size_t) -> *mut ::core::ffi::c_void;
	fn calloc(__nmemb: size_t, __size: size_t) -> *mut ::core::ffi::c_void;
	fn realloc(__ptr: *mut ::core::ffi::c_void, __size: size_t) -> *mut ::core::ffi::c_void;
	fn free(__ptr: *mut ::core::ffi::c_void);
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
	fn mdct_init(lookup: *mut mdct_lookup, n: ::core::ffi::c_int);
	fn mdct_clear(l: *mut mdct_lookup);
	fn mdct_forward(
		init: *mut mdct_lookup,
		in_0: *mut ::core::ffi::c_float,
		out: *mut ::core::ffi::c_float
	);
}
use ogg_c2rust_sys::oggpack_buffer;

use super::{
	block::{alloc_chain, vorbis_block, vorbis_dsp_state},
	info::vorbis_info
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
pub struct mdct_lookup {
	pub n: ::core::ffi::c_int,
	pub log2n: ::core::ffi::c_int,
	pub trig: *mut ::core::ffi::c_float,
	pub bitrev: *mut ::core::ffi::c_int,
	pub scale: ::core::ffi::c_float
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
pub union C2RustUnnamed {
	pub i: ogg_uint32_t,
	pub f: ::core::ffi::c_float
}
pub const M_PI: ::core::ffi::c_double = 3.14159265358979323846f64;
pub const VE_PRE: ::core::ffi::c_int = 16 as ::core::ffi::c_int;
pub const VE_WIN: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const VE_POST: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const VE_AMP: ::core::ffi::c_int = VE_PRE + VE_POST - 1 as ::core::ffi::c_int;
pub const VE_BANDS: ::core::ffi::c_int = 7 as ::core::ffi::c_int;
pub const VE_NEARDC: ::core::ffi::c_int = 15 as ::core::ffi::c_int;
pub const VE_MINSTRETCH: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const VE_MAXSTRETCH: ::core::ffi::c_int = 12 as ::core::ffi::c_int;
#[no_mangle]
pub unsafe extern "C" fn _ve_envelope_init(mut e: *mut envelope_lookup, mut vi: *mut vorbis_info) {
	let mut ci: *mut codec_setup_info = (*vi).codec_setup as *mut codec_setup_info;
	let mut gi: *mut vorbis_info_psy_global = &raw mut (*ci).psy_g_param;
	let mut ch: ::core::ffi::c_int = (*vi).channels;
	let mut i: ::core::ffi::c_int = 0;
	let mut j: ::core::ffi::c_int = 0;
	(*e).winlength = 128 as ::core::ffi::c_int;
	let mut n: ::core::ffi::c_int = (*e).winlength;
	(*e).searchstep = 64 as ::core::ffi::c_int;
	(*e).minenergy = (*gi).preecho_minenergy;
	(*e).ch = ch;
	(*e).storage = 128 as ::core::ffi::c_long;
	(*e).cursor = (*ci).blocksizes[1 as ::core::ffi::c_int as usize] / 2 as ::core::ffi::c_long;
	(*e).mdct_win = calloc(
		n as size_t,
		::core::mem::size_of::<::core::ffi::c_float>() as size_t
	) as *mut ::core::ffi::c_float;
	mdct_init(&raw mut (*e).mdct, n);
	i = 0 as ::core::ffi::c_int;
	while i < n {
		*(*e).mdct_win.offset(i as isize) =
			(i as ::core::ffi::c_double / (n as ::core::ffi::c_double - 1.0f64) * M_PI).sin()
				as ::core::ffi::c_float;
		*(*e).mdct_win.offset(i as isize) *= *(*e).mdct_win.offset(i as isize);
		i += 1;
	}
	(*e).band[0 as ::core::ffi::c_int as usize].begin = 2 as ::core::ffi::c_int;
	(*e).band[0 as ::core::ffi::c_int as usize].end = 4 as ::core::ffi::c_int;
	(*e).band[1 as ::core::ffi::c_int as usize].begin = 4 as ::core::ffi::c_int;
	(*e).band[1 as ::core::ffi::c_int as usize].end = 5 as ::core::ffi::c_int;
	(*e).band[2 as ::core::ffi::c_int as usize].begin = 6 as ::core::ffi::c_int;
	(*e).band[2 as ::core::ffi::c_int as usize].end = 6 as ::core::ffi::c_int;
	(*e).band[3 as ::core::ffi::c_int as usize].begin = 9 as ::core::ffi::c_int;
	(*e).band[3 as ::core::ffi::c_int as usize].end = 8 as ::core::ffi::c_int;
	(*e).band[4 as ::core::ffi::c_int as usize].begin = 13 as ::core::ffi::c_int;
	(*e).band[4 as ::core::ffi::c_int as usize].end = 8 as ::core::ffi::c_int;
	(*e).band[5 as ::core::ffi::c_int as usize].begin = 17 as ::core::ffi::c_int;
	(*e).band[5 as ::core::ffi::c_int as usize].end = 8 as ::core::ffi::c_int;
	(*e).band[6 as ::core::ffi::c_int as usize].begin = 22 as ::core::ffi::c_int;
	(*e).band[6 as ::core::ffi::c_int as usize].end = 8 as ::core::ffi::c_int;
	j = 0 as ::core::ffi::c_int;
	while j < VE_BANDS {
		n = (*e).band[j as usize].end;
		(*e).band[j as usize].window = malloc(
			(n as size_t).wrapping_mul(::core::mem::size_of::<::core::ffi::c_float>() as size_t)
		) as *mut ::core::ffi::c_float;
		i = 0 as ::core::ffi::c_int;
		while i < n {
			*(*e).band[j as usize].window.offset(i as isize) =
				((i as ::core::ffi::c_double + 0.5f64) / n as ::core::ffi::c_double * M_PI).sin()
					as ::core::ffi::c_float;
			(*e).band[j as usize].total += *(*e).band[j as usize].window.offset(i as isize);
			i += 1;
		}
		(*e).band[j as usize].total =
			(1.0f64 / (*e).band[j as usize].total as ::core::ffi::c_double) as ::core::ffi::c_float;
		j += 1;
	}
	(*e).filter = calloc(
		(VE_BANDS * ch) as size_t,
		::core::mem::size_of::<envelope_filter_state>() as size_t
	) as *mut envelope_filter_state;
	(*e).mark = calloc(
		(*e).storage as size_t,
		::core::mem::size_of::<::core::ffi::c_int>() as size_t
	) as *mut ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn _ve_envelope_clear(mut e: *mut envelope_lookup) {
	let mut i: ::core::ffi::c_int = 0;
	mdct_clear(&raw mut (*e).mdct);
	i = 0 as ::core::ffi::c_int;
	while i < VE_BANDS {
		free((*e).band[i as usize].window as *mut ::core::ffi::c_void);
		i += 1;
	}
	free((*e).mdct_win as *mut ::core::ffi::c_void);
	free((*e).filter as *mut ::core::ffi::c_void);
	free((*e).mark as *mut ::core::ffi::c_void);
	memset(
		e as *mut ::core::ffi::c_void,
		0 as ::core::ffi::c_int,
		::core::mem::size_of::<envelope_lookup>() as size_t
	);
}
unsafe extern "C" fn _ve_amp(
	mut ve: *mut envelope_lookup,
	mut gi: *mut vorbis_info_psy_global,
	mut data: *mut ::core::ffi::c_float,
	mut bands: *mut envelope_band,
	mut filters: *mut envelope_filter_state
) -> ::core::ffi::c_int {
	// Allocated as `Vec<c_float>` (rather than a byte buffer reinterpreted as `*mut c_float`) so
	// that the buffer is properly aligned for `c_float` access, as required by Miri
	let mut alloca_allocations: Vec<Vec<::core::ffi::c_float>> = Vec::new();
	let mut n: ::core::ffi::c_long = (*ve).winlength as ::core::ffi::c_long;
	let mut ret: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
	let mut i: ::core::ffi::c_long = 0;
	let mut j: ::core::ffi::c_long = 0;
	let mut decay: ::core::ffi::c_float = 0.;
	let mut minV: ::core::ffi::c_float = (*ve).minenergy;
	alloca_allocations.push(::std::vec::from_elem(0.0, n as usize));
	let mut vec: *mut ::core::ffi::c_float = alloca_allocations.last_mut().unwrap().as_mut_ptr();
	let mut stretch: ::core::ffi::c_int =
		if (2 as ::core::ffi::c_int) < (*ve).stretch / 2 as ::core::ffi::c_int {
			(*ve).stretch / 2 as ::core::ffi::c_int
		} else {
			2 as ::core::ffi::c_int
		};
	let mut penalty: ::core::ffi::c_float = (*gi).stretch_penalty
		- ((*ve).stretch / 2 as ::core::ffi::c_int - VE_MINSTRETCH) as ::core::ffi::c_float;
	if penalty < 0.0f32 {
		penalty = 0.0f32;
	}
	if penalty > (*gi).stretch_penalty {
		penalty = (*gi).stretch_penalty;
	}
	i = 0 as ::core::ffi::c_long;
	while i < n {
		*vec.offset(i as isize) = *data.offset(i as isize) * *(*ve).mdct_win.offset(i as isize);
		i += 1;
	}
	mdct_forward(&raw mut (*ve).mdct, vec, vec);
	let mut temp: ::core::ffi::c_float = ((*vec.offset(0 as ::core::ffi::c_int as isize)
		* *vec.offset(0 as ::core::ffi::c_int as isize))
		as ::core::ffi::c_double
		+ 0.7f64
			* *vec.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_double
			* *vec.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_double
		+ 0.2f64
			* *vec.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_double
			* *vec.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_double)
		as ::core::ffi::c_float;
	let mut ptr: ::core::ffi::c_int = (*filters).nearptr;
	if ptr == 0 as ::core::ffi::c_int {
		(*filters).nearDC_acc = (*filters).nearDC_partialacc + temp;
		decay = (*filters).nearDC_acc;
		(*filters).nearDC_partialacc = temp;
	} else {
		(*filters).nearDC_acc += temp;
		decay = (*filters).nearDC_acc;
		(*filters).nearDC_partialacc += temp;
	}
	(*filters).nearDC_acc -= (*filters).nearDC[ptr as usize];
	(*filters).nearDC[ptr as usize] = temp;
	decay = (decay as ::core::ffi::c_double
		* (1.0f64 / (VE_NEARDC + 1 as ::core::ffi::c_int) as ::core::ffi::c_double))
		as ::core::ffi::c_float;
	(*filters).nearptr += 1;
	if (*filters).nearptr >= VE_NEARDC {
		(*filters).nearptr = 0 as ::core::ffi::c_int;
	}
	decay =
		(todB(&raw mut decay) as ::core::ffi::c_double * 0.5f64 - 15.0f64) as ::core::ffi::c_float;
	i = 0 as ::core::ffi::c_long;
	while i < n / 2 as ::core::ffi::c_long {
		let mut val: ::core::ffi::c_float = *vec.offset(i as isize) * *vec.offset(i as isize)
			+ *vec.offset((i + 1 as ::core::ffi::c_long) as isize)
				* *vec.offset((i + 1 as ::core::ffi::c_long) as isize);
		val = todB(&raw mut val) * 0.5f32;
		if val < decay {
			val = decay;
		}
		if val < minV {
			val = minV;
		}
		*vec.offset((i >> 1 as ::core::ffi::c_int) as isize) = val;
		decay = (decay as ::core::ffi::c_double - 8.0f64) as ::core::ffi::c_float;
		i += 2 as ::core::ffi::c_long;
	}
	j = 0 as ::core::ffi::c_long;
	while j < VE_BANDS as ::core::ffi::c_long {
		let mut acc: ::core::ffi::c_float = 0.0f32;
		let mut valmax: ::core::ffi::c_float = 0.;
		let mut valmin: ::core::ffi::c_float = 0.;
		i = 0 as ::core::ffi::c_long;
		while i < (*bands.offset(j as isize)).end as ::core::ffi::c_long {
			acc += *vec
				.offset((i + (*bands.offset(j as isize)).begin as ::core::ffi::c_long) as isize)
				* *(*bands.offset(j as isize)).window.offset(i as isize);
			i += 1;
		}
		acc *= (*bands.offset(j as isize)).total;
		let mut p: ::core::ffi::c_int = 0;
		let mut this: ::core::ffi::c_int = (*filters.offset(j as isize)).ampptr;
		let mut postmax: ::core::ffi::c_float = 0.;
		let mut postmin: ::core::ffi::c_float = 0.;
		let mut premax: ::core::ffi::c_float = -99999.0f32;
		let mut premin: ::core::ffi::c_float = 99999.0f32;
		p = this;
		p -= 1;
		if p < 0 as ::core::ffi::c_int {
			p += VE_AMP;
		}
		postmax = if acc < (*filters.offset(j as isize)).ampbuf[p as usize] {
			(*filters.offset(j as isize)).ampbuf[p as usize]
		} else {
			acc
		};
		postmin = if acc > (*filters.offset(j as isize)).ampbuf[p as usize] {
			(*filters.offset(j as isize)).ampbuf[p as usize]
		} else {
			acc
		};
		i = 0 as ::core::ffi::c_long;
		while i < stretch as ::core::ffi::c_long {
			p -= 1;
			if p < 0 as ::core::ffi::c_int {
				p += VE_AMP;
			}
			premax = if premax < (*filters.offset(j as isize)).ampbuf[p as usize] {
				(*filters.offset(j as isize)).ampbuf[p as usize]
			} else {
				premax
			};
			premin = if premin > (*filters.offset(j as isize)).ampbuf[p as usize] {
				(*filters.offset(j as isize)).ampbuf[p as usize]
			} else {
				premin
			};
			i += 1;
		}
		valmin = postmin - premin;
		valmax = postmax - premax;
		(*filters.offset(j as isize)).ampbuf[this as usize] = acc;
		let ref mut fresh0 = (*filters.offset(j as isize)).ampptr;
		*fresh0 += 1;
		if (*filters.offset(j as isize)).ampptr >= VE_AMP {
			(*filters.offset(j as isize)).ampptr = 0 as ::core::ffi::c_int;
		}
		if valmax > (*gi).preecho_thresh[j as usize] + penalty {
			ret |= 1 as ::core::ffi::c_int;
			ret |= 4 as ::core::ffi::c_int;
		}
		if valmin < (*gi).postecho_thresh[j as usize] - penalty {
			ret |= 2 as ::core::ffi::c_int;
		}
		j += 1;
	}
	return ret;
}
#[no_mangle]
pub unsafe extern "C" fn _ve_envelope_search(mut v: *mut vorbis_dsp_state) -> ::core::ffi::c_long {
	let mut vi: *mut vorbis_info = (*v).vi;
	let mut ci: *mut codec_setup_info = (*vi).codec_setup as *mut codec_setup_info;
	let mut gi: *mut vorbis_info_psy_global = &raw mut (*ci).psy_g_param;
	let mut ve: *mut envelope_lookup = (*((*v).backend_state as *mut private_state)).ve;
	let mut i: ::core::ffi::c_long = 0;
	let mut j: ::core::ffi::c_long = 0;
	let mut first: ::core::ffi::c_int =
		((*ve).current / (*ve).searchstep as ::core::ffi::c_long) as ::core::ffi::c_int;
	let mut last: ::core::ffi::c_int = (*v).pcm_current / (*ve).searchstep - VE_WIN;
	if first < 0 as ::core::ffi::c_int {
		first = 0 as ::core::ffi::c_int;
	}
	if (last + VE_WIN + VE_POST) as ::core::ffi::c_long > (*ve).storage {
		(*ve).storage = (last + VE_WIN + VE_POST) as ::core::ffi::c_long;
		(*ve).mark = realloc(
			(*ve).mark as *mut ::core::ffi::c_void,
			((*ve).storage as size_t)
				.wrapping_mul(::core::mem::size_of::<::core::ffi::c_int>() as size_t)
		) as *mut ::core::ffi::c_int;
	}
	j = first as ::core::ffi::c_long;
	while j < last as ::core::ffi::c_long {
		let mut ret: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
		(*ve).stretch += 1;
		if (*ve).stretch > VE_MAXSTRETCH * 2 as ::core::ffi::c_int {
			(*ve).stretch = VE_MAXSTRETCH * 2 as ::core::ffi::c_int;
		}
		i = 0 as ::core::ffi::c_long;
		while i < (*ve).ch as ::core::ffi::c_long {
			let mut pcm: *mut ::core::ffi::c_float = (*(*v).pcm.offset(i as isize))
				.offset(((*ve).searchstep as ::core::ffi::c_long * j) as isize);
			ret |= _ve_amp(
				ve,
				gi,
				pcm,
				&raw mut (*ve).band as *mut envelope_band,
				(*ve)
					.filter
					.offset((i * VE_BANDS as ::core::ffi::c_long) as isize)
			);
			i += 1;
		}
		*(*ve)
			.mark
			.offset((j + VE_POST as ::core::ffi::c_long) as isize) = 0 as ::core::ffi::c_int;
		if ret & 1 as ::core::ffi::c_int != 0 {
			*(*ve).mark.offset(j as isize) = 1 as ::core::ffi::c_int;
			*(*ve).mark.offset((j + 1 as ::core::ffi::c_long) as isize) = 1 as ::core::ffi::c_int;
		}
		if ret & 2 as ::core::ffi::c_int != 0 {
			*(*ve).mark.offset(j as isize) = 1 as ::core::ffi::c_int;
			if j > 0 as ::core::ffi::c_long {
				*(*ve).mark.offset((j - 1 as ::core::ffi::c_long) as isize) =
					1 as ::core::ffi::c_int;
			}
		}
		if ret & 4 as ::core::ffi::c_int != 0 {
			(*ve).stretch = -(1 as ::core::ffi::c_int);
		}
		j += 1;
	}
	(*ve).current = (last * (*ve).searchstep) as ::core::ffi::c_long;
	let mut centerW: ::core::ffi::c_long = (*v).centerW;
	let mut testW: ::core::ffi::c_long = centerW
		+ (*ci).blocksizes[(*v).W as usize] / 4 as ::core::ffi::c_long
		+ (*ci).blocksizes[1 as ::core::ffi::c_int as usize] / 2 as ::core::ffi::c_long
		+ (*ci).blocksizes[0 as ::core::ffi::c_int as usize] / 4 as ::core::ffi::c_long;
	j = (*ve).cursor;
	while j < (*ve).current - (*ve).searchstep as ::core::ffi::c_long {
		if j >= testW {
			return 1 as ::core::ffi::c_long;
		}
		(*ve).cursor = j;
		if *(*ve)
			.mark
			.offset((j / (*ve).searchstep as ::core::ffi::c_long) as isize)
			!= 0
		{
			if j > centerW {
				(*ve).curmark = j;
				if j >= testW {
					return 1 as ::core::ffi::c_long;
				}
				return 0 as ::core::ffi::c_long;
			}
		}
		j += (*ve).searchstep as ::core::ffi::c_long;
	}
	return -(1 as ::core::ffi::c_int) as ::core::ffi::c_long;
}
#[no_mangle]
pub unsafe extern "C" fn _ve_envelope_mark(mut v: *mut vorbis_dsp_state) -> ::core::ffi::c_int {
	let mut ve: *mut envelope_lookup = (*((*v).backend_state as *mut private_state)).ve;
	let mut vi: *mut vorbis_info = (*v).vi;
	let mut ci: *mut codec_setup_info = (*vi).codec_setup as *mut codec_setup_info;
	let mut centerW: ::core::ffi::c_long = (*v).centerW;
	let mut beginW: ::core::ffi::c_long =
		centerW - (*ci).blocksizes[(*v).W as usize] / 4 as ::core::ffi::c_long;
	let mut endW: ::core::ffi::c_long =
		centerW + (*ci).blocksizes[(*v).W as usize] / 4 as ::core::ffi::c_long;
	if (*v).W != 0 {
		beginW -= (*ci).blocksizes[(*v).lW as usize] / 4 as ::core::ffi::c_long;
		endW += (*ci).blocksizes[(*v).nW as usize] / 4 as ::core::ffi::c_long;
	} else {
		beginW -= (*ci).blocksizes[0 as ::core::ffi::c_int as usize] / 4 as ::core::ffi::c_long;
		endW += (*ci).blocksizes[0 as ::core::ffi::c_int as usize] / 4 as ::core::ffi::c_long;
	}
	if (*ve).curmark >= beginW && (*ve).curmark < endW {
		return 1 as ::core::ffi::c_int;
	}
	let mut first: ::core::ffi::c_long = beginW / (*ve).searchstep as ::core::ffi::c_long;
	let mut last: ::core::ffi::c_long = endW / (*ve).searchstep as ::core::ffi::c_long;
	let mut i: ::core::ffi::c_long = 0;
	i = first;
	while i < last {
		if *(*ve).mark.offset(i as isize) != 0 {
			return 1 as ::core::ffi::c_int;
		}
		i += 1;
	}
	return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn _ve_envelope_shift(
	mut e: *mut envelope_lookup,
	mut shift: ::core::ffi::c_long
) {
	let mut smallsize: ::core::ffi::c_int = ((*e).current / (*e).searchstep as ::core::ffi::c_long
		+ VE_POST as ::core::ffi::c_long)
		as ::core::ffi::c_int;
	let mut smallshift: ::core::ffi::c_int =
		(shift / (*e).searchstep as ::core::ffi::c_long) as ::core::ffi::c_int;
	::core::ptr::copy(
		(*e).mark.offset(smallshift as isize) as *const ::core::ffi::c_int,
		(*e).mark,
		(smallsize - smallshift) as usize
	);
	(*e).current -= shift;
	if (*e).curmark >= 0 as ::core::ffi::c_long {
		(*e).curmark -= shift;
	}
	(*e).cursor -= shift;
}
#[inline]
unsafe extern "C" fn todB(mut x: *const ::core::ffi::c_float) -> ::core::ffi::c_float {
	let mut ix: C2RustUnnamed = C2RustUnnamed { i: 0 };
	ix.f = *x;
	ix.i = ix.i & 0x7fffffff as ogg_uint32_t;
	return ix.i as ::core::ffi::c_float * 7.17711438e-7f32 - 764.6161886f32;
}
