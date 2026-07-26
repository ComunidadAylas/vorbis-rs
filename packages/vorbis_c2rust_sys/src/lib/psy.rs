extern "C" {
	fn malloc(__size: size_t) -> *mut ::core::ffi::c_void;
	fn calloc(__nmemb: size_t, __size: size_t) -> *mut ::core::ffi::c_void;
	fn free(__ptr: *mut ::core::ffi::c_void);
	fn qsort(
		__base: *mut ::core::ffi::c_void,
		__nmemb: size_t,
		__size: size_t,
		__compar: __compar_fn_t
	);
	fn atan(__x: ::core::ffi::c_double) -> ::core::ffi::c_double;
	fn sqrt(__x: ::core::ffi::c_double) -> ::core::ffi::c_double;
	fn ceil(__x: ::core::ffi::c_double) -> ::core::ffi::c_double;
	fn fabs(__x: ::core::ffi::c_double) -> ::core::ffi::c_double;
	fn floor(__x: ::core::ffi::c_double) -> ::core::ffi::c_double;
	fn rint(__x: ::core::ffi::c_double) -> ::core::ffi::c_double;
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
}
use super::{block::vorbis_dsp_state, info::vorbis_info};
pub type size_t = usize;
pub type __uint16_t = u16;
pub type __int32_t = i32;
pub type __uint32_t = u32;
pub type __int64_t = i64;
pub type int32_t = __int32_t;
pub type int64_t = __int64_t;
pub type __compar_fn_t = Option<
	unsafe extern "C" fn(
		*const ::core::ffi::c_void,
		*const ::core::ffi::c_void
	) -> ::core::ffi::c_int
>;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
pub type ogg_uint16_t = uint16_t;
pub type ogg_int32_t = int32_t;
pub type ogg_uint32_t = uint32_t;
pub type ogg_int64_t = int64_t;
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
pub struct vorbis_info_mode {
	pub blockflag: ::core::ffi::c_int,
	pub windowtype: ::core::ffi::c_int,
	pub transformtype: ::core::ffi::c_int,
	pub mapping: ::core::ffi::c_int
}
pub type vorbis_info_floor = ();
pub type vorbis_info_residue = ();
pub type vorbis_info_mapping = ();
#[derive(Copy, Clone)]
#[repr(C)]
pub struct vorbis_info_mapping0 {
	pub submaps: ::core::ffi::c_int,
	pub chmuxlist: [::core::ffi::c_int; 256],
	pub floorsubmap: [::core::ffi::c_int; 16],
	pub residuesubmap: [::core::ffi::c_int; 16],
	pub coupling_steps: ::core::ffi::c_int,
	pub coupling_mag: [::core::ffi::c_int; 256],
	pub coupling_ang: [::core::ffi::c_int; 256]
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
pub union C2RustUnnamed {
	pub i: ogg_uint32_t,
	pub f: ::core::ffi::c_float
}
pub const P_BANDS: ::core::ffi::c_int = 17 as ::core::ffi::c_int;
pub const P_LEVELS: ::core::ffi::c_int = 8 as ::core::ffi::c_int;
pub const P_LEVEL_0: ::core::ffi::c_double = 30.0f64;
pub const P_NOISECURVES: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
pub const NOISE_COMPAND_LEVELS: ::core::ffi::c_int = 40 as ::core::ffi::c_int;
pub const NEGINF: ::core::ffi::c_float = -9999.0f32;
static mut stereo_threshholds: [::core::ffi::c_double; 9] = [
	0.0f64, 0.5f64, 1.0f64, 1.5f64, 2.5f64, 4.5f64, 8.5f64, 16.5f64, 9e10f64
];
static mut stereo_threshholds_limited: [::core::ffi::c_double; 9] = [
	0.0f64, 0.5f64, 1.0f64, 1.5f64, 2.0f64, 2.5f64, 4.5f64, 8.5f64, 9e10f64
];
#[no_mangle]
pub unsafe extern "C" fn _vp_global_look(mut vi: *mut vorbis_info) -> *mut vorbis_look_psy_global {
	let mut ci: *mut codec_setup_info = (*vi).codec_setup as *mut codec_setup_info;
	let mut gi: *mut vorbis_info_psy_global = &raw mut (*ci).psy_g_param;
	let mut look: *mut vorbis_look_psy_global = calloc(
		1 as size_t,
		::core::mem::size_of::<vorbis_look_psy_global>() as size_t
	) as *mut vorbis_look_psy_global;
	(*look).channels = (*vi).channels;
	(*look).ampmax = -9999.0f64 as ::core::ffi::c_float;
	(*look).gi = gi;
	return look;
}
#[no_mangle]
pub unsafe extern "C" fn _vp_global_free(mut look: *mut vorbis_look_psy_global) {
	if !look.is_null() {
		memset(
			look as *mut ::core::ffi::c_void,
			0 as ::core::ffi::c_int,
			::core::mem::size_of::<vorbis_look_psy_global>() as size_t
		);
		free(look as *mut ::core::ffi::c_void);
	}
}
#[no_mangle]
pub unsafe extern "C" fn _vi_gpsy_free(mut i: *mut vorbis_info_psy_global) {
	if !i.is_null() {
		memset(
			i as *mut ::core::ffi::c_void,
			0 as ::core::ffi::c_int,
			::core::mem::size_of::<vorbis_info_psy_global>() as size_t
		);
		free(i as *mut ::core::ffi::c_void);
	}
}
#[no_mangle]
pub unsafe extern "C" fn _vi_psy_free(mut i: *mut vorbis_info_psy) {
	if !i.is_null() {
		memset(
			i as *mut ::core::ffi::c_void,
			0 as ::core::ffi::c_int,
			::core::mem::size_of::<vorbis_info_psy>() as size_t
		);
		free(i as *mut ::core::ffi::c_void);
	}
}
unsafe extern "C" fn min_curve(
	mut c: *mut ::core::ffi::c_float,
	mut c2: *mut ::core::ffi::c_float
) {
	let mut i: ::core::ffi::c_int = 0;
	i = 0 as ::core::ffi::c_int;
	while i < EHMER_MAX {
		if *c2.offset(i as isize) < *c.offset(i as isize) {
			*c.offset(i as isize) = *c2.offset(i as isize);
		}
		i += 1;
	}
}
unsafe extern "C" fn max_curve(
	mut c: *mut ::core::ffi::c_float,
	mut c2: *mut ::core::ffi::c_float
) {
	let mut i: ::core::ffi::c_int = 0;
	i = 0 as ::core::ffi::c_int;
	while i < EHMER_MAX {
		if *c2.offset(i as isize) > *c.offset(i as isize) {
			*c.offset(i as isize) = *c2.offset(i as isize);
		}
		i += 1;
	}
}
unsafe extern "C" fn attenuate_curve(
	mut c: *mut ::core::ffi::c_float,
	mut att: ::core::ffi::c_float
) {
	let mut i: ::core::ffi::c_int = 0;
	i = 0 as ::core::ffi::c_int;
	while i < EHMER_MAX {
		*c.offset(i as isize) += att;
		i += 1;
	}
}
unsafe extern "C" fn setup_tone_curves(
	mut curveatt_dB: *mut ::core::ffi::c_float,
	mut binHz: ::core::ffi::c_float,
	mut n: ::core::ffi::c_int,
	mut center_boost: ::core::ffi::c_float,
	mut center_decay_rate: ::core::ffi::c_float
) -> *mut *mut *mut ::core::ffi::c_float {
	let mut alloca_allocations: Vec<Vec<::core::ffi::c_float>> = Vec::new();
	let mut i: ::core::ffi::c_int = 0;
	let mut j: ::core::ffi::c_int = 0;
	let mut k: ::core::ffi::c_int = 0;
	let mut m: ::core::ffi::c_int = 0;
	let mut ath: [::core::ffi::c_float; 56] = [0.; 56];
	let mut workc: [[[::core::ffi::c_float; 56]; 8]; 17] = [[[0.; 56]; 8]; 17];
	let mut athc: [[::core::ffi::c_float; 56]; 8] = [[0.; 56]; 8];
	alloca_allocations.push(::std::vec::from_elem(0.0, n as usize));
	let mut brute_buffer: *mut ::core::ffi::c_float =
		alloca_allocations.last_mut().unwrap().as_mut_ptr();
	let mut ret: *mut *mut *mut ::core::ffi::c_float = malloc(
		(::core::mem::size_of::<*mut *mut ::core::ffi::c_float>() as size_t)
			.wrapping_mul(P_BANDS as size_t)
	) as *mut *mut *mut ::core::ffi::c_float;
	memset(
		&raw mut workc as *mut [[::core::ffi::c_float; 56]; 8] as *mut ::core::ffi::c_void,
		0 as ::core::ffi::c_int,
		::core::mem::size_of::<[[[::core::ffi::c_float; 56]; 8]; 17]>() as size_t
	);
	i = 0 as ::core::ffi::c_int;
	while i < P_BANDS {
		let mut ath_offset: ::core::ffi::c_int = i * 4 as ::core::ffi::c_int;
		j = 0 as ::core::ffi::c_int;
		while j < EHMER_MAX {
			let mut min: ::core::ffi::c_float = 999.0f32;
			k = 0 as ::core::ffi::c_int;
			while k < 4 as ::core::ffi::c_int {
				if j + k + ath_offset < MAX_ATH {
					if min > ATH[(j + k + ath_offset) as usize] {
						min = ATH[(j + k + ath_offset) as usize];
					}
				} else if min > ATH[(MAX_ATH - 1 as ::core::ffi::c_int) as usize] {
					min = ATH[(MAX_ATH - 1 as ::core::ffi::c_int) as usize];
				}
				k += 1;
			}
			ath[j as usize] = min;
			j += 1;
		}
		j = 0 as ::core::ffi::c_int;
		while j < 6 as ::core::ffi::c_int {
			memcpy(
				&raw mut *(&raw mut *(&raw mut workc as *mut [[::core::ffi::c_float; 56]; 8])
					.offset(i as isize) as *mut [::core::ffi::c_float; 56])
					.offset((j + 2 as ::core::ffi::c_int) as isize) as *mut ::core::ffi::c_float
					as *mut ::core::ffi::c_void,
				&raw const *(&raw const *(&raw const tonemasks
					as *const [[::core::ffi::c_float; 56]; 6])
					.offset(i as isize) as *const [::core::ffi::c_float; 56])
					.offset(j as isize) as *const ::core::ffi::c_float
					as *const ::core::ffi::c_void,
				(EHMER_MAX as size_t)
					.wrapping_mul(::core::mem::size_of::<::core::ffi::c_float>() as size_t)
			);
			j += 1;
		}
		memcpy(
			&raw mut *(&raw mut *(&raw mut workc as *mut [[::core::ffi::c_float; 56]; 8])
				.offset(i as isize) as *mut [::core::ffi::c_float; 56])
				.offset(0 as ::core::ffi::c_int as isize) as *mut ::core::ffi::c_float
				as *mut ::core::ffi::c_void,
			&raw const *(&raw const *(&raw const tonemasks
				as *const [[::core::ffi::c_float; 56]; 6])
				.offset(i as isize) as *const [::core::ffi::c_float; 56])
				.offset(0 as ::core::ffi::c_int as isize) as *const ::core::ffi::c_float
				as *const ::core::ffi::c_void,
			(EHMER_MAX as size_t)
				.wrapping_mul(::core::mem::size_of::<::core::ffi::c_float>() as size_t)
		);
		memcpy(
			&raw mut *(&raw mut *(&raw mut workc as *mut [[::core::ffi::c_float; 56]; 8])
				.offset(i as isize) as *mut [::core::ffi::c_float; 56])
				.offset(1 as ::core::ffi::c_int as isize) as *mut ::core::ffi::c_float
				as *mut ::core::ffi::c_void,
			&raw const *(&raw const *(&raw const tonemasks
				as *const [[::core::ffi::c_float; 56]; 6])
				.offset(i as isize) as *const [::core::ffi::c_float; 56])
				.offset(0 as ::core::ffi::c_int as isize) as *const ::core::ffi::c_float
				as *const ::core::ffi::c_void,
			(EHMER_MAX as size_t)
				.wrapping_mul(::core::mem::size_of::<::core::ffi::c_float>() as size_t)
		);
		j = 0 as ::core::ffi::c_int;
		while j < P_LEVELS {
			k = 0 as ::core::ffi::c_int;
			while k < EHMER_MAX {
				let mut adj: ::core::ffi::c_float = center_boost
					+ (EHMER_OFFSET - k).abs() as ::core::ffi::c_float * center_decay_rate;
				if (adj as ::core::ffi::c_double) < 0.0f64
					&& center_boost > 0 as ::core::ffi::c_int as ::core::ffi::c_float
				{
					adj = 0.0f32;
				}
				if adj as ::core::ffi::c_double > 0.0f64
					&& center_boost < 0 as ::core::ffi::c_int as ::core::ffi::c_float
				{
					adj = 0.0f32;
				}
				workc[i as usize][j as usize][k as usize] += adj;
				k += 1;
			}
			j += 1;
		}
		j = 0 as ::core::ffi::c_int;
		while j < P_LEVELS {
			attenuate_curve(
				&raw mut *(&raw mut *(&raw mut workc as *mut [[::core::ffi::c_float; 56]; 8])
					.offset(i as isize) as *mut [::core::ffi::c_float; 56])
					.offset(j as isize) as *mut ::core::ffi::c_float,
				(*curveatt_dB.offset(i as isize) as ::core::ffi::c_double + 100.0f64
					- (if j < 2 as ::core::ffi::c_int {
						2 as ::core::ffi::c_int
					} else {
						j
					}) as ::core::ffi::c_double
						* 10.0f64 - P_LEVEL_0) as ::core::ffi::c_float
			);
			memcpy(
				&raw mut *(&raw mut athc as *mut [::core::ffi::c_float; 56]).offset(j as isize)
					as *mut ::core::ffi::c_float as *mut ::core::ffi::c_void,
				&raw mut ath as *mut ::core::ffi::c_float as *const ::core::ffi::c_void,
				(EHMER_MAX as size_t)
					.wrapping_mul(::core::mem::size_of::<::core::ffi::c_float>() as size_t)
			);
			attenuate_curve(
				&raw mut *(&raw mut athc as *mut [::core::ffi::c_float; 56]).offset(j as isize)
					as *mut ::core::ffi::c_float,
				(100.0f64
					- (j as ::core::ffi::c_float * 10.0f32) as ::core::ffi::c_double
					- P_LEVEL_0) as ::core::ffi::c_float
			);
			max_curve(
				&raw mut *(&raw mut athc as *mut [::core::ffi::c_float; 56]).offset(j as isize)
					as *mut ::core::ffi::c_float,
				&raw mut *(&raw mut *(&raw mut workc as *mut [[::core::ffi::c_float; 56]; 8])
					.offset(i as isize) as *mut [::core::ffi::c_float; 56])
					.offset(j as isize) as *mut ::core::ffi::c_float
			);
			j += 1;
		}
		j = 1 as ::core::ffi::c_int;
		while j < P_LEVELS {
			min_curve(
				&raw mut *(&raw mut athc as *mut [::core::ffi::c_float; 56]).offset(j as isize)
					as *mut ::core::ffi::c_float,
				&raw mut *(&raw mut athc as *mut [::core::ffi::c_float; 56])
					.offset((j - 1 as ::core::ffi::c_int) as isize) as *mut ::core::ffi::c_float
			);
			min_curve(
				&raw mut *(&raw mut *(&raw mut workc as *mut [[::core::ffi::c_float; 56]; 8])
					.offset(i as isize) as *mut [::core::ffi::c_float; 56])
					.offset(j as isize) as *mut ::core::ffi::c_float,
				&raw mut *(&raw mut athc as *mut [::core::ffi::c_float; 56]).offset(j as isize)
					as *mut ::core::ffi::c_float
			);
			j += 1;
		}
		i += 1;
	}
	i = 0 as ::core::ffi::c_int;
	while i < P_BANDS {
		let mut hi_curve: ::core::ffi::c_int = 0;
		let mut lo_curve: ::core::ffi::c_int = 0;
		let mut bin: ::core::ffi::c_int = 0;
		let ref mut fresh1 = *ret.offset(i as isize);
		*fresh1 = malloc(
			(::core::mem::size_of::<*mut ::core::ffi::c_float>() as size_t)
				.wrapping_mul(P_LEVELS as size_t)
		) as *mut *mut ::core::ffi::c_float;
		bin = floor(
			((i as ::core::ffi::c_double * 0.5f64 + 5.965784f64) * 0.693147f64).exp()
				/ binHz as ::core::ffi::c_double
		) as ::core::ffi::c_int;
		lo_curve = ceil(
			(((bin as ::core::ffi::c_float * binHz
				+ 1 as ::core::ffi::c_int as ::core::ffi::c_float) as ::core::ffi::c_double)
				.ln() * 1.442695f64
				- 5.965784f64)
				* 2 as ::core::ffi::c_int as ::core::ffi::c_double
		) as ::core::ffi::c_int;
		hi_curve = floor(
			((((bin + 1 as ::core::ffi::c_int) as ::core::ffi::c_float * binHz)
				as ::core::ffi::c_double)
				.ln() * 1.442695f64
				- 5.965784f64)
				* 2 as ::core::ffi::c_int as ::core::ffi::c_double
		) as ::core::ffi::c_int;
		if lo_curve > i {
			lo_curve = i;
		}
		if lo_curve < 0 as ::core::ffi::c_int {
			lo_curve = 0 as ::core::ffi::c_int;
		}
		if hi_curve >= P_BANDS {
			hi_curve = P_BANDS - 1 as ::core::ffi::c_int;
		}
		m = 0 as ::core::ffi::c_int;
		while m < P_LEVELS {
			let ref mut fresh2 = *(*ret.offset(i as isize)).offset(m as isize);
			*fresh2 = malloc(
				(::core::mem::size_of::<::core::ffi::c_float>() as size_t)
					.wrapping_mul((EHMER_MAX + 2 as ::core::ffi::c_int) as size_t)
			) as *mut ::core::ffi::c_float;
			j = 0 as ::core::ffi::c_int;
			while j < n {
				*brute_buffer.offset(j as isize) = 999.0f32;
				j += 1;
			}
			k = lo_curve;
			while k <= hi_curve {
				let mut l: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
				j = 0 as ::core::ffi::c_int;
				while j < EHMER_MAX {
					let mut lo_bin: ::core::ffi::c_int = (((j as ::core::ffi::c_double * 0.125f64
						+ k as ::core::ffi::c_double * 0.5f64
						- 2.0625f64 + 5.965784f64)
						* 0.693147f64)
						.exp() / binHz as ::core::ffi::c_double)
						as ::core::ffi::c_int;
					let mut hi_bin: ::core::ffi::c_int = (((j as ::core::ffi::c_double * 0.125f64
						+ k as ::core::ffi::c_double * 0.5f64
						- 1.9375f64 + 5.965784f64)
						* 0.693147f64)
						.exp() / binHz as ::core::ffi::c_double
						+ 1 as ::core::ffi::c_int as ::core::ffi::c_double)
						as ::core::ffi::c_int;
					if lo_bin < 0 as ::core::ffi::c_int {
						lo_bin = 0 as ::core::ffi::c_int;
					}
					if lo_bin > n {
						lo_bin = n;
					}
					if lo_bin < l {
						l = lo_bin;
					}
					if hi_bin < 0 as ::core::ffi::c_int {
						hi_bin = 0 as ::core::ffi::c_int;
					}
					if hi_bin > n {
						hi_bin = n;
					}
					while l < hi_bin && l < n {
						if *brute_buffer.offset(l as isize)
							> workc[k as usize][m as usize][j as usize]
						{
							*brute_buffer.offset(l as isize) =
								workc[k as usize][m as usize][j as usize];
						}
						l += 1;
					}
					j += 1;
				}
				while l < n {
					if *brute_buffer.offset(l as isize)
						> workc[k as usize][m as usize]
							[(EHMER_MAX - 1 as ::core::ffi::c_int) as usize]
					{
						*brute_buffer.offset(l as isize) = workc[k as usize][m as usize]
							[(EHMER_MAX - 1 as ::core::ffi::c_int) as usize];
					}
					l += 1;
				}
				k += 1;
			}
			if (i + 1 as ::core::ffi::c_int) < P_BANDS {
				let mut l_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
				k = i + 1 as ::core::ffi::c_int;
				j = 0 as ::core::ffi::c_int;
				while j < EHMER_MAX {
					let mut lo_bin_0: ::core::ffi::c_int =
						(((j as ::core::ffi::c_double * 0.125f64
							+ i as ::core::ffi::c_double * 0.5f64
							- 2.0625f64 + 5.965784f64)
							* 0.693147f64)
							.exp() / binHz as ::core::ffi::c_double) as ::core::ffi::c_int;
					let mut hi_bin_0: ::core::ffi::c_int = (((j as ::core::ffi::c_double
						* 0.125f64 + i
						as ::core::ffi::c_double
						* 0.5f64 - 1.9375f64
						+ 5.965784f64) * 0.693147f64)
						.exp() / binHz as ::core::ffi::c_double
						+ 1 as ::core::ffi::c_int as ::core::ffi::c_double)
						as ::core::ffi::c_int;
					if lo_bin_0 < 0 as ::core::ffi::c_int {
						lo_bin_0 = 0 as ::core::ffi::c_int;
					}
					if lo_bin_0 > n {
						lo_bin_0 = n;
					}
					if lo_bin_0 < l_0 {
						l_0 = lo_bin_0;
					}
					if hi_bin_0 < 0 as ::core::ffi::c_int {
						hi_bin_0 = 0 as ::core::ffi::c_int;
					}
					if hi_bin_0 > n {
						hi_bin_0 = n;
					}
					while l_0 < hi_bin_0 && l_0 < n {
						if *brute_buffer.offset(l_0 as isize)
							> workc[k as usize][m as usize][j as usize]
						{
							*brute_buffer.offset(l_0 as isize) =
								workc[k as usize][m as usize][j as usize];
						}
						l_0 += 1;
					}
					j += 1;
				}
				while l_0 < n {
					if *brute_buffer.offset(l_0 as isize)
						> workc[k as usize][m as usize]
							[(EHMER_MAX - 1 as ::core::ffi::c_int) as usize]
					{
						*brute_buffer.offset(l_0 as isize) = workc[k as usize][m as usize]
							[(EHMER_MAX - 1 as ::core::ffi::c_int) as usize];
					}
					l_0 += 1;
				}
			}
			j = 0 as ::core::ffi::c_int;
			while j < EHMER_MAX {
				let mut bin_0: ::core::ffi::c_int = (((j as ::core::ffi::c_double * 0.125f64
					+ i as ::core::ffi::c_double * 0.5f64
					- 2.0f64 + 5.965784f64)
					* 0.693147f64)
					.exp() / binHz as ::core::ffi::c_double)
					as ::core::ffi::c_int;
				if bin_0 < 0 as ::core::ffi::c_int {
					*(*(*ret.offset(i as isize)).offset(m as isize))
						.offset((j + 2 as ::core::ffi::c_int) as isize) = -999.0f64 as ::core::ffi::c_float;
				} else if bin_0 >= n {
					*(*(*ret.offset(i as isize)).offset(m as isize))
						.offset((j + 2 as ::core::ffi::c_int) as isize) = -999.0f64 as ::core::ffi::c_float;
				} else {
					*(*(*ret.offset(i as isize)).offset(m as isize))
						.offset((j + 2 as ::core::ffi::c_int) as isize) = *brute_buffer.offset(bin_0 as isize);
				}
				j += 1;
			}
			j = 0 as ::core::ffi::c_int;
			while j < EHMER_OFFSET {
				if *(*(*ret.offset(i as isize)).offset(m as isize))
					.offset((j + 2 as ::core::ffi::c_int) as isize)
					> -200.0f32
				{
					break;
				}
				j += 1;
			}
			*(*(*ret.offset(i as isize)).offset(m as isize))
				.offset(0 as ::core::ffi::c_int as isize) = j as ::core::ffi::c_float;
			j = EHMER_MAX - 1 as ::core::ffi::c_int;
			while j > EHMER_OFFSET + 1 as ::core::ffi::c_int {
				if *(*(*ret.offset(i as isize)).offset(m as isize))
					.offset((j + 2 as ::core::ffi::c_int) as isize)
					> -200.0f32
				{
					break;
				}
				j -= 1;
			}
			*(*(*ret.offset(i as isize)).offset(m as isize))
				.offset(1 as ::core::ffi::c_int as isize) = j as ::core::ffi::c_float;
			m += 1;
		}
		i += 1;
	}
	return ret;
}
#[no_mangle]
pub unsafe extern "C" fn _vp_psy_init(
	mut p: *mut vorbis_look_psy,
	mut vi: *mut vorbis_info_psy,
	mut gi: *mut vorbis_info_psy_global,
	mut n: ::core::ffi::c_int,
	mut rate: ::core::ffi::c_long
) {
	let mut i: ::core::ffi::c_long = 0;
	let mut j: ::core::ffi::c_long = 0;
	let mut lo: ::core::ffi::c_long = -(99 as ::core::ffi::c_int) as ::core::ffi::c_long;
	let mut hi: ::core::ffi::c_long = 1 as ::core::ffi::c_long;
	let mut maxoc: ::core::ffi::c_long = 0;
	memset(
		p as *mut ::core::ffi::c_void,
		0 as ::core::ffi::c_int,
		::core::mem::size_of::<vorbis_look_psy>() as size_t
	);
	(*p).eighth_octave_lines = (*gi).eighth_octave_lines;
	(*p).shiftoc = (rint(
		(((*gi).eighth_octave_lines as ::core::ffi::c_float * 8.0f32) as ::core::ffi::c_double)
			.ln() / (2.0f64).ln()
	) - 1 as ::core::ffi::c_int as ::core::ffi::c_double) as ::core::ffi::c_long;
	(*p).firstoc = ((((0.25f32 * rate as ::core::ffi::c_float) as ::core::ffi::c_double * 0.5f64
		/ n as ::core::ffi::c_double)
		.ln() * 1.442695f64
		- 5.965784f64)
		* ((1 as ::core::ffi::c_int) << (*p).shiftoc + 1 as ::core::ffi::c_long)
			as ::core::ffi::c_double
		- (*gi).eighth_octave_lines as ::core::ffi::c_double) as ::core::ffi::c_long;
	maxoc = (((((n as ::core::ffi::c_float + 0.25f32) * rate as ::core::ffi::c_float)
		as ::core::ffi::c_double
		* 0.5f64
		/ n as ::core::ffi::c_double)
		.ln() * 1.442695f64
		- 5.965784f64)
		* ((1 as ::core::ffi::c_int) << (*p).shiftoc + 1 as ::core::ffi::c_long)
			as ::core::ffi::c_double
		+ 0.5f64) as ::core::ffi::c_long;
	(*p).total_octave_lines =
		(maxoc - (*p).firstoc + 1 as ::core::ffi::c_long) as ::core::ffi::c_int;
	(*p).ath = malloc(
		(n as size_t).wrapping_mul(::core::mem::size_of::<::core::ffi::c_float>() as size_t)
	) as *mut ::core::ffi::c_float;
	(*p).octave =
		malloc((n as size_t).wrapping_mul(::core::mem::size_of::<::core::ffi::c_long>() as size_t))
			as *mut ::core::ffi::c_long;
	(*p).bark =
		malloc((n as size_t).wrapping_mul(::core::mem::size_of::<::core::ffi::c_long>() as size_t))
			as *mut ::core::ffi::c_long;
	(*p).vi = vi as *mut vorbis_info_psy;
	(*p).n = n;
	(*p).rate = rate;
	(*p).m_val = 1.0f32;
	if rate < 26000 as ::core::ffi::c_long {
		(*p).m_val = 0 as ::core::ffi::c_int as ::core::ffi::c_float;
	} else if rate < 38000 as ::core::ffi::c_long {
		(*p).m_val = 0.94f32;
	} else if rate > 46000 as ::core::ffi::c_long {
		(*p).m_val = 1.275f32;
	}
	i = 0 as ::core::ffi::c_long;
	j = 0 as ::core::ffi::c_long;
	while i < (MAX_ATH - 1 as ::core::ffi::c_int) as ::core::ffi::c_long {
		let mut endpos: ::core::ffi::c_int = rint(
			(((i + 1 as ::core::ffi::c_long) as ::core::ffi::c_double * 0.125f64 - 2.0f64
				+ 5.965784f64)
				* 0.693147f64)
				.exp() * 2 as ::core::ffi::c_int as ::core::ffi::c_double
				* n as ::core::ffi::c_double
				/ rate as ::core::ffi::c_double
		) as ::core::ffi::c_int;
		let mut base: ::core::ffi::c_float = ATH[i as usize];
		if j < endpos as ::core::ffi::c_long {
			let mut delta: ::core::ffi::c_float = (ATH[(i + 1 as ::core::ffi::c_long) as usize]
				- base) / (endpos as ::core::ffi::c_long - j)
				as ::core::ffi::c_float;
			while j < endpos as ::core::ffi::c_long && j < n as ::core::ffi::c_long {
				*(*p).ath.offset(j as isize) =
					(base as ::core::ffi::c_double + 100.0f64) as ::core::ffi::c_float;
				base += delta;
				j += 1;
			}
		}
		i += 1;
	}
	while j < n as ::core::ffi::c_long {
		*(*p).ath.offset(j as isize) = *(*p).ath.offset((j - 1 as ::core::ffi::c_long) as isize);
		j += 1;
	}
	i = 0 as ::core::ffi::c_long;
	while i < n as ::core::ffi::c_long {
		let mut bark: ::core::ffi::c_float = (13.1f64
			* atan(
				(0.00074f32
					* (rate / (2 as ::core::ffi::c_int * n) as ::core::ffi::c_long * i)
						as ::core::ffi::c_float) as ::core::ffi::c_double
			) + 2.24f64
			* atan(
				((rate / (2 as ::core::ffi::c_int * n) as ::core::ffi::c_long
					* i * (rate / (2 as ::core::ffi::c_int * n) as ::core::ffi::c_long * i))
					as ::core::ffi::c_float
					* 1.85e-8f32) as ::core::ffi::c_double
			) + (1e-4f32
			* (rate / (2 as ::core::ffi::c_int * n) as ::core::ffi::c_long * i)
				as ::core::ffi::c_float)
			as ::core::ffi::c_double) as ::core::ffi::c_float;
		while (lo + (*vi).noisewindowlomin as ::core::ffi::c_long) < i
			&& (13.1f64
				* atan(
					(0.00074f32
						* (rate / (2 as ::core::ffi::c_int * n) as ::core::ffi::c_long * lo)
							as ::core::ffi::c_float) as ::core::ffi::c_double
				) + 2.24f64
				* atan(
					((rate / (2 as ::core::ffi::c_int * n) as ::core::ffi::c_long
						* lo * (rate / (2 as ::core::ffi::c_int * n) as ::core::ffi::c_long * lo))
						as ::core::ffi::c_float
						* 1.85e-8f32) as ::core::ffi::c_double
				) + (1e-4f32
				* (rate / (2 as ::core::ffi::c_int * n) as ::core::ffi::c_long * lo)
					as ::core::ffi::c_float) as ::core::ffi::c_double)
				< (bark - (*vi).noisewindowlo) as ::core::ffi::c_double
		{
			lo += 1;
		}
		while hi <= n as ::core::ffi::c_long
			&& (hi < i + (*vi).noisewindowhimin as ::core::ffi::c_long
				|| (13.1f64
					* atan(
						(0.00074f32
							* (rate / (2 as ::core::ffi::c_int * n) as ::core::ffi::c_long * hi)
								as ::core::ffi::c_float) as ::core::ffi::c_double
					) + 2.24f64
					* atan(
						((rate / (2 as ::core::ffi::c_int * n) as ::core::ffi::c_long
							* hi * (rate / (2 as ::core::ffi::c_int * n) as ::core::ffi::c_long * hi))
							as ::core::ffi::c_float
							* 1.85e-8f32) as ::core::ffi::c_double
					) + (1e-4f32
					* (rate / (2 as ::core::ffi::c_int * n) as ::core::ffi::c_long * hi)
						as ::core::ffi::c_float) as ::core::ffi::c_double)
					< (bark + (*vi).noisewindowhi) as ::core::ffi::c_double)
		{
			hi += 1;
		}
		*(*p).bark.offset(i as isize) = (lo - 1 as ::core::ffi::c_long)
			* ((1 as ::core::ffi::c_int) << 16 as ::core::ffi::c_int) as ::core::ffi::c_long
			+ (hi - 1 as ::core::ffi::c_long);
		i += 1;
	}
	i = 0 as ::core::ffi::c_long;
	while i < n as ::core::ffi::c_long {
		*(*p).octave.offset(i as isize) = ((((i as ::core::ffi::c_float + 0.25f32)
			as ::core::ffi::c_double
			* 0.5f64 * rate as ::core::ffi::c_double
			/ n as ::core::ffi::c_double)
			.ln() * 1.442695f64
			- 5.965784f64)
			* ((1 as ::core::ffi::c_int) << (*p).shiftoc + 1 as ::core::ffi::c_long)
				as ::core::ffi::c_double
			+ 0.5f64) as ::core::ffi::c_long;
		i += 1;
	}
	(*p).tonecurves = setup_tone_curves(
		&raw mut (*vi).toneatt as *mut ::core::ffi::c_float,
		(rate as ::core::ffi::c_double * 0.5f64 / n as ::core::ffi::c_double)
			as ::core::ffi::c_float,
		n,
		(*vi).tone_centerboost,
		(*vi).tone_decay
	);
	(*p).noiseoffset = malloc(
		(P_NOISECURVES as size_t)
			.wrapping_mul(::core::mem::size_of::<*mut ::core::ffi::c_float>() as size_t)
	) as *mut *mut ::core::ffi::c_float;
	i = 0 as ::core::ffi::c_long;
	while i < P_NOISECURVES as ::core::ffi::c_long {
		let ref mut fresh0 = *(*p).noiseoffset.offset(i as isize);
		*fresh0 = malloc(
			(n as size_t).wrapping_mul(::core::mem::size_of::<::core::ffi::c_float>() as size_t)
		) as *mut ::core::ffi::c_float;
		i += 1;
	}
	i = 0 as ::core::ffi::c_long;
	while i < n as ::core::ffi::c_long {
		let mut halfoc: ::core::ffi::c_float = ((((i as ::core::ffi::c_double + 0.5f64)
			* rate as ::core::ffi::c_double
			/ (2.0f64 * n as ::core::ffi::c_double))
			.ln() * 1.442695f64
			- 5.965784f64)
			* 2.0f64) as ::core::ffi::c_float;
		let mut inthalfoc: ::core::ffi::c_int = 0;
		let mut del: ::core::ffi::c_float = 0.;
		if halfoc < 0 as ::core::ffi::c_int as ::core::ffi::c_float {
			halfoc = 0 as ::core::ffi::c_int as ::core::ffi::c_float;
		}
		if halfoc >= (P_BANDS - 1 as ::core::ffi::c_int) as ::core::ffi::c_float {
			halfoc = (P_BANDS - 1 as ::core::ffi::c_int) as ::core::ffi::c_float;
		}
		inthalfoc = halfoc as ::core::ffi::c_int;
		if inthalfoc >= P_BANDS - 2 as ::core::ffi::c_int {
			inthalfoc = P_BANDS - 2 as ::core::ffi::c_int;
		}
		del = halfoc - inthalfoc as ::core::ffi::c_float;
		j = 0 as ::core::ffi::c_long;
		while j < P_NOISECURVES as ::core::ffi::c_long {
			*(*(*p).noiseoffset.offset(j as isize)).offset(i as isize) =
				((*(*p).vi).noiseoff[j as usize][inthalfoc as usize] as ::core::ffi::c_double
					* (1.0f64 - del as ::core::ffi::c_double)
					+ ((*(*p).vi).noiseoff[j as usize]
						[(inthalfoc + 1 as ::core::ffi::c_int) as usize]
						* del) as ::core::ffi::c_double) as ::core::ffi::c_float;
			j += 1;
		}
		i += 1;
	}
}
#[no_mangle]
pub unsafe extern "C" fn _vp_psy_clear(mut p: *mut vorbis_look_psy) {
	let mut i: ::core::ffi::c_int = 0;
	let mut j: ::core::ffi::c_int = 0;
	if !p.is_null() {
		if !(*p).ath.is_null() {
			free((*p).ath as *mut ::core::ffi::c_void);
		}
		if !(*p).octave.is_null() {
			free((*p).octave as *mut ::core::ffi::c_void);
		}
		if !(*p).bark.is_null() {
			free((*p).bark as *mut ::core::ffi::c_void);
		}
		if !(*p).tonecurves.is_null() {
			i = 0 as ::core::ffi::c_int;
			while i < P_BANDS {
				j = 0 as ::core::ffi::c_int;
				while j < P_LEVELS {
					free(*(*(*p).tonecurves.offset(i as isize)).offset(j as isize)
						as *mut ::core::ffi::c_void);
					j += 1;
				}
				free(*(*p).tonecurves.offset(i as isize) as *mut ::core::ffi::c_void);
				i += 1;
			}
			free((*p).tonecurves as *mut ::core::ffi::c_void);
		}
		if !(*p).noiseoffset.is_null() {
			i = 0 as ::core::ffi::c_int;
			while i < P_NOISECURVES {
				free(*(*p).noiseoffset.offset(i as isize) as *mut ::core::ffi::c_void);
				i += 1;
			}
			free((*p).noiseoffset as *mut ::core::ffi::c_void);
		}
		memset(
			p as *mut ::core::ffi::c_void,
			0 as ::core::ffi::c_int,
			::core::mem::size_of::<vorbis_look_psy>() as size_t
		);
	}
}
unsafe extern "C" fn seed_curve(
	mut seed: *mut ::core::ffi::c_float,
	mut curves: *mut *const ::core::ffi::c_float,
	mut amp: ::core::ffi::c_float,
	mut oc: ::core::ffi::c_int,
	mut n: ::core::ffi::c_int,
	mut linesper: ::core::ffi::c_int,
	mut dBoffset: ::core::ffi::c_float
) {
	let mut i: ::core::ffi::c_int = 0;
	let mut post1: ::core::ffi::c_int = 0;
	let mut seedptr: ::core::ffi::c_int = 0;
	let mut posts: *const ::core::ffi::c_float = ::core::ptr::null::<::core::ffi::c_float>();
	let mut curve: *const ::core::ffi::c_float = ::core::ptr::null::<::core::ffi::c_float>();
	let mut choice: ::core::ffi::c_int =
		(((amp + dBoffset) as ::core::ffi::c_double - P_LEVEL_0) * 0.1f64) as ::core::ffi::c_int;
	choice = if choice < 0 as ::core::ffi::c_int {
		0 as ::core::ffi::c_int
	} else {
		choice
	};
	choice = if choice > 8 as ::core::ffi::c_int - 1 as ::core::ffi::c_int {
		8 as ::core::ffi::c_int - 1 as ::core::ffi::c_int
	} else {
		choice
	};
	posts = *curves.offset(choice as isize);
	curve = posts.offset(2 as ::core::ffi::c_int as isize);
	post1 = *posts.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int;
	seedptr = (oc as ::core::ffi::c_float
		+ (*posts.offset(0 as ::core::ffi::c_int as isize) - EHMER_OFFSET as ::core::ffi::c_float)
			* linesper as ::core::ffi::c_float
		- (linesper >> 1 as ::core::ffi::c_int) as ::core::ffi::c_float)
		as ::core::ffi::c_int;
	i = *posts.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int;
	while i < post1 {
		if seedptr > 0 as ::core::ffi::c_int {
			let mut lin: ::core::ffi::c_float = amp + *curve.offset(i as isize);
			if *seed.offset(seedptr as isize) < lin {
				*seed.offset(seedptr as isize) = lin;
			}
		}
		seedptr += linesper;
		if seedptr >= n {
			break;
		}
		i += 1;
	}
}
unsafe extern "C" fn seed_loop(
	mut p: *mut vorbis_look_psy,
	mut curves: *mut *mut *const ::core::ffi::c_float,
	mut f: *const ::core::ffi::c_float,
	mut flr: *const ::core::ffi::c_float,
	mut seed: *mut ::core::ffi::c_float,
	mut specmax: ::core::ffi::c_float
) {
	let mut vi: *mut vorbis_info_psy = (*p).vi as *mut vorbis_info_psy;
	let mut n: ::core::ffi::c_long = (*p).n as ::core::ffi::c_long;
	let mut i: ::core::ffi::c_long = 0;
	let mut dBoffset: ::core::ffi::c_float = (*vi).max_curve_dB - specmax;
	i = 0 as ::core::ffi::c_long;
	while i < n {
		let mut max: ::core::ffi::c_float = *f.offset(i as isize);
		let mut oc: ::core::ffi::c_long = *(*p).octave.offset(i as isize);
		while (i + 1 as ::core::ffi::c_long) < n
			&& *(*p).octave.offset((i + 1 as ::core::ffi::c_long) as isize) == oc
		{
			i += 1;
			if *f.offset(i as isize) > max {
				max = *f.offset(i as isize);
			}
		}
		if max + 6.0f32 > *flr.offset(i as isize) {
			oc = oc >> (*p).shiftoc;
			if oc >= P_BANDS as ::core::ffi::c_long {
				oc = (P_BANDS - 1 as ::core::ffi::c_int) as ::core::ffi::c_long;
			}
			if oc < 0 as ::core::ffi::c_long {
				oc = 0 as ::core::ffi::c_long;
			}
			seed_curve(
				seed,
				*curves.offset(oc as isize),
				max,
				(*(*p).octave.offset(i as isize) - (*p).firstoc) as ::core::ffi::c_int,
				(*p).total_octave_lines,
				(*p).eighth_octave_lines,
				dBoffset
			);
		}
		i += 1;
	}
}
unsafe extern "C" fn seed_chase(
	mut seeds: *mut ::core::ffi::c_float,
	mut linesper: ::core::ffi::c_int,
	mut n: ::core::ffi::c_long
) {
	let mut alloca_allocations: Vec<Vec<u8>> = Vec::new();
	alloca_allocations.push(::std::vec::from_elem(
		0,
		(n as usize).wrapping_mul(::core::mem::size_of::<::core::ffi::c_long>() as usize) as usize
	));
	let mut posstack: *mut ::core::ffi::c_long =
		alloca_allocations.last_mut().unwrap().as_mut_ptr() as *mut ::core::ffi::c_long;
	alloca_allocations.push(::std::vec::from_elem(
		0,
		(n as usize).wrapping_mul(::core::mem::size_of::<::core::ffi::c_float>() as usize) as usize
	));
	let mut ampstack: *mut ::core::ffi::c_float =
		alloca_allocations.last_mut().unwrap().as_mut_ptr() as *mut ::core::ffi::c_float;
	let mut stack: ::core::ffi::c_long = 0 as ::core::ffi::c_long;
	let mut pos: ::core::ffi::c_long = 0 as ::core::ffi::c_long;
	let mut i: ::core::ffi::c_long = 0;
	i = 0 as ::core::ffi::c_long;
	while i < n {
		if stack < 2 as ::core::ffi::c_long {
			*posstack.offset(stack as isize) = i;
			let fresh3 = stack;
			stack = stack + 1;
			*ampstack.offset(fresh3 as isize) = *seeds.offset(i as isize);
		} else {
			loop {
				if *seeds.offset(i as isize)
					< *ampstack.offset((stack - 1 as ::core::ffi::c_long) as isize)
				{
					*posstack.offset(stack as isize) = i;
					let fresh4 = stack;
					stack = stack + 1;
					*ampstack.offset(fresh4 as isize) = *seeds.offset(i as isize);
					break;
				} else {
					if i < *posstack.offset((stack - 1 as ::core::ffi::c_long) as isize)
						+ linesper as ::core::ffi::c_long
					{
						if stack > 1 as ::core::ffi::c_long
							&& *ampstack.offset((stack - 1 as ::core::ffi::c_long) as isize)
								<= *ampstack.offset((stack - 2 as ::core::ffi::c_long) as isize)
							&& i < *posstack.offset((stack - 2 as ::core::ffi::c_long) as isize)
								+ linesper as ::core::ffi::c_long
						{
							stack -= 1;
							continue;
						}
					}
					*posstack.offset(stack as isize) = i;
					let fresh5 = stack;
					stack = stack + 1;
					*ampstack.offset(fresh5 as isize) = *seeds.offset(i as isize);
					break;
				}
			}
		}
		i += 1;
	}
	i = 0 as ::core::ffi::c_long;
	while i < stack {
		let mut endpos: ::core::ffi::c_long = 0;
		if i < stack - 1 as ::core::ffi::c_long
			&& *ampstack.offset((i + 1 as ::core::ffi::c_long) as isize)
				> *ampstack.offset(i as isize)
		{
			endpos = *posstack.offset((i + 1 as ::core::ffi::c_long) as isize);
		} else {
			endpos = *posstack.offset(i as isize)
				+ linesper as ::core::ffi::c_long
				+ 1 as ::core::ffi::c_long;
		}
		if endpos > n {
			endpos = n;
		}
		while pos < endpos {
			*seeds.offset(pos as isize) = *ampstack.offset(i as isize);
			pos += 1;
		}
		i += 1;
	}
}
unsafe extern "C" fn max_seeds(
	mut p: *mut vorbis_look_psy,
	mut seed: *mut ::core::ffi::c_float,
	mut flr: *mut ::core::ffi::c_float
) {
	let mut n: ::core::ffi::c_long = (*p).total_octave_lines as ::core::ffi::c_long;
	let mut linesper: ::core::ffi::c_int = (*p).eighth_octave_lines;
	let mut linpos: ::core::ffi::c_long = 0 as ::core::ffi::c_long;
	let mut pos: ::core::ffi::c_long = 0;
	seed_chase(seed, linesper, n);
	pos = *(*p).octave.offset(0 as ::core::ffi::c_int as isize)
		- (*p).firstoc
		- (linesper >> 1 as ::core::ffi::c_int) as ::core::ffi::c_long;
	while (linpos + 1 as ::core::ffi::c_long) < (*p).n as ::core::ffi::c_long {
		let mut minV: ::core::ffi::c_float = *seed.offset(pos as isize);
		let mut end: ::core::ffi::c_long = (*(*p).octave.offset(linpos as isize)
			+ *(*p)
				.octave
				.offset((linpos + 1 as ::core::ffi::c_long) as isize)
			>> 1 as ::core::ffi::c_int)
			- (*p).firstoc;
		if minV > (*(*p).vi).tone_abs_limit {
			minV = (*(*p).vi).tone_abs_limit;
		}
		while pos + 1 as ::core::ffi::c_long <= end {
			pos += 1;
			if *seed.offset(pos as isize) > NEGINF && *seed.offset(pos as isize) < minV
				|| minV == NEGINF
			{
				minV = *seed.offset(pos as isize);
			}
		}
		end = pos + (*p).firstoc;
		while linpos < (*p).n as ::core::ffi::c_long && *(*p).octave.offset(linpos as isize) <= end
		{
			if *flr.offset(linpos as isize) < minV {
				*flr.offset(linpos as isize) = minV;
			}
			linpos += 1;
		}
	}
	let mut minV_0: ::core::ffi::c_float =
		*seed.offset(((*p).total_octave_lines - 1 as ::core::ffi::c_int) as isize);
	while linpos < (*p).n as ::core::ffi::c_long {
		if *flr.offset(linpos as isize) < minV_0 {
			*flr.offset(linpos as isize) = minV_0;
		}
		linpos += 1;
	}
}
unsafe extern "C" fn bark_noise_hybridmp(
	mut n: ::core::ffi::c_int,
	mut b: *const ::core::ffi::c_long,
	mut f: *const ::core::ffi::c_float,
	mut noise: *mut ::core::ffi::c_float,
	offset: ::core::ffi::c_float,
	fixed: ::core::ffi::c_int
) {
	let mut alloca_allocations: Vec<Vec<u8>> = Vec::new();
	alloca_allocations.push(::std::vec::from_elem(
		0,
		(n as usize).wrapping_mul(::core::mem::size_of::<::core::ffi::c_float>() as usize) as usize
	));
	let mut N: *mut ::core::ffi::c_float =
		alloca_allocations.last_mut().unwrap().as_mut_ptr() as *mut ::core::ffi::c_float;
	alloca_allocations.push(::std::vec::from_elem(
		0,
		(n as usize).wrapping_mul(::core::mem::size_of::<::core::ffi::c_float>() as usize) as usize
	));
	let mut X: *mut ::core::ffi::c_float =
		alloca_allocations.last_mut().unwrap().as_mut_ptr() as *mut ::core::ffi::c_float;
	alloca_allocations.push(::std::vec::from_elem(
		0,
		(n as usize).wrapping_mul(::core::mem::size_of::<::core::ffi::c_float>() as usize) as usize
	));
	let mut XX: *mut ::core::ffi::c_float =
		alloca_allocations.last_mut().unwrap().as_mut_ptr() as *mut ::core::ffi::c_float;
	alloca_allocations.push(::std::vec::from_elem(
		0,
		(n as usize).wrapping_mul(::core::mem::size_of::<::core::ffi::c_float>() as usize) as usize
	));
	let mut Y: *mut ::core::ffi::c_float =
		alloca_allocations.last_mut().unwrap().as_mut_ptr() as *mut ::core::ffi::c_float;
	alloca_allocations.push(::std::vec::from_elem(
		0,
		(n as usize).wrapping_mul(::core::mem::size_of::<::core::ffi::c_float>() as usize) as usize
	));
	let mut XY: *mut ::core::ffi::c_float =
		alloca_allocations.last_mut().unwrap().as_mut_ptr() as *mut ::core::ffi::c_float;
	let mut tN: ::core::ffi::c_float = 0.;
	let mut tX: ::core::ffi::c_float = 0.;
	let mut tXX: ::core::ffi::c_float = 0.;
	let mut tY: ::core::ffi::c_float = 0.;
	let mut tXY: ::core::ffi::c_float = 0.;
	let mut i: ::core::ffi::c_int = 0;
	let mut lo: ::core::ffi::c_int = 0;
	let mut hi: ::core::ffi::c_int = 0;
	let mut R: ::core::ffi::c_float = 0.0f32;
	let mut A: ::core::ffi::c_float = 0.0f32;
	let mut B: ::core::ffi::c_float = 0.0f32;
	let mut D: ::core::ffi::c_float = 1.0f32;
	let mut w: ::core::ffi::c_float = 0.;
	let mut x: ::core::ffi::c_float = 0.;
	let mut y: ::core::ffi::c_float = 0.;
	tXY = 0.0f32;
	tY = tXY;
	tXX = tY;
	tX = tXX;
	tN = tX;
	y = *f.offset(0 as ::core::ffi::c_int as isize) + offset;
	if y < 1.0f32 {
		y = 1.0f32;
	}
	w = ((y * y) as ::core::ffi::c_double * 0.5f64) as ::core::ffi::c_float;
	tN += w;
	tX += w;
	tY += w * y;
	*N.offset(0 as ::core::ffi::c_int as isize) = tN;
	*X.offset(0 as ::core::ffi::c_int as isize) = tX;
	*XX.offset(0 as ::core::ffi::c_int as isize) = tXX;
	*Y.offset(0 as ::core::ffi::c_int as isize) = tY;
	*XY.offset(0 as ::core::ffi::c_int as isize) = tXY;
	i = 1 as ::core::ffi::c_int;
	x = 1.0f32;
	while i < n {
		y = *f.offset(i as isize) + offset;
		if y < 1.0f32 {
			y = 1.0f32;
		}
		w = y * y;
		tN += w;
		tX += w * x;
		tXX += w * x * x;
		tY += w * y;
		tXY += w * x * y;
		*N.offset(i as isize) = tN;
		*X.offset(i as isize) = tX;
		*XX.offset(i as isize) = tXX;
		*Y.offset(i as isize) = tY;
		*XY.offset(i as isize) = tXY;
		i += 1;
		x += 1.0f32;
	}
	i = 0 as ::core::ffi::c_int;
	x = 0.0f32;
	while i < n {
		lo = (*b.offset(i as isize) >> 16 as ::core::ffi::c_int) as ::core::ffi::c_int;
		hi = (*b.offset(i as isize) & 0xffff as ::core::ffi::c_long) as ::core::ffi::c_int;
		if lo >= 0 as ::core::ffi::c_int || -lo >= n {
			break;
		}
		if hi >= n {
			break;
		}
		tN = *N.offset(hi as isize) + *N.offset(-lo as isize);
		tX = *X.offset(hi as isize) - *X.offset(-lo as isize);
		tXX = *XX.offset(hi as isize) + *XX.offset(-lo as isize);
		tY = *Y.offset(hi as isize) + *Y.offset(-lo as isize);
		tXY = *XY.offset(hi as isize) - *XY.offset(-lo as isize);
		A = tY * tXX - tX * tXY;
		B = tN * tXY - tX * tY;
		D = tN * tXX - tX * tX;
		R = (A + x * B) / D;
		if R < 0.0f32 {
			R = 0.0f32;
		}
		*noise.offset(i as isize) = R - offset;
		i += 1;
		x += 1.0f32;
	}
	while i < n {
		lo = (*b.offset(i as isize) >> 16 as ::core::ffi::c_int) as ::core::ffi::c_int;
		hi = (*b.offset(i as isize) & 0xffff as ::core::ffi::c_long) as ::core::ffi::c_int;
		if lo < 0 as ::core::ffi::c_int || lo >= n {
			break;
		}
		if hi >= n {
			break;
		}
		tN = *N.offset(hi as isize) - *N.offset(lo as isize);
		tX = *X.offset(hi as isize) - *X.offset(lo as isize);
		tXX = *XX.offset(hi as isize) - *XX.offset(lo as isize);
		tY = *Y.offset(hi as isize) - *Y.offset(lo as isize);
		tXY = *XY.offset(hi as isize) - *XY.offset(lo as isize);
		A = tY * tXX - tX * tXY;
		B = tN * tXY - tX * tY;
		D = tN * tXX - tX * tX;
		R = (A + x * B) / D;
		if R < 0.0f32 {
			R = 0.0f32;
		}
		*noise.offset(i as isize) = R - offset;
		i += 1;
		x += 1.0f32;
	}
	while i < n {
		R = (A + x * B) / D;
		if R < 0.0f32 {
			R = 0.0f32;
		}
		*noise.offset(i as isize) = R - offset;
		i += 1;
		x += 1.0f32;
	}
	if fixed <= 0 as ::core::ffi::c_int {
		return;
	}
	i = 0 as ::core::ffi::c_int;
	x = 0.0f32;
	while i < n {
		hi = i + fixed / 2 as ::core::ffi::c_int;
		lo = hi - fixed;
		if hi >= n {
			break;
		}
		if lo >= 0 as ::core::ffi::c_int {
			break;
		}
		tN = *N.offset(hi as isize) + *N.offset(-lo as isize);
		tX = *X.offset(hi as isize) - *X.offset(-lo as isize);
		tXX = *XX.offset(hi as isize) + *XX.offset(-lo as isize);
		tY = *Y.offset(hi as isize) + *Y.offset(-lo as isize);
		tXY = *XY.offset(hi as isize) - *XY.offset(-lo as isize);
		A = tY * tXX - tX * tXY;
		B = tN * tXY - tX * tY;
		D = tN * tXX - tX * tX;
		R = (A + x * B) / D;
		if R - offset < *noise.offset(i as isize) {
			*noise.offset(i as isize) = R - offset;
		}
		i += 1;
		x += 1.0f32;
	}
	while i < n {
		hi = i + fixed / 2 as ::core::ffi::c_int;
		lo = hi - fixed;
		if hi >= n {
			break;
		}
		if lo < 0 as ::core::ffi::c_int {
			break;
		}
		tN = *N.offset(hi as isize) - *N.offset(lo as isize);
		tX = *X.offset(hi as isize) - *X.offset(lo as isize);
		tXX = *XX.offset(hi as isize) - *XX.offset(lo as isize);
		tY = *Y.offset(hi as isize) - *Y.offset(lo as isize);
		tXY = *XY.offset(hi as isize) - *XY.offset(lo as isize);
		A = tY * tXX - tX * tXY;
		B = tN * tXY - tX * tY;
		D = tN * tXX - tX * tX;
		R = (A + x * B) / D;
		if R - offset < *noise.offset(i as isize) {
			*noise.offset(i as isize) = R - offset;
		}
		i += 1;
		x += 1.0f32;
	}
	while i < n {
		R = (A + x * B) / D;
		if R - offset < *noise.offset(i as isize) {
			*noise.offset(i as isize) = R - offset;
		}
		i += 1;
		x += 1.0f32;
	}
}
#[no_mangle]
pub unsafe extern "C" fn _vp_noisemask(
	mut p: *mut vorbis_look_psy,
	mut logmdct: *mut ::core::ffi::c_float,
	mut logmask: *mut ::core::ffi::c_float
) {
	let mut alloca_allocations: Vec<Vec<u8>> = Vec::new();
	let mut i: ::core::ffi::c_int = 0;
	let mut n: ::core::ffi::c_int = (*p).n;
	alloca_allocations.push(::std::vec::from_elem(
		0,
		(n as usize).wrapping_mul(::core::mem::size_of::<::core::ffi::c_float>() as usize) as usize
	));
	let mut work: *mut ::core::ffi::c_float =
		alloca_allocations.last_mut().unwrap().as_mut_ptr() as *mut ::core::ffi::c_float;
	bark_noise_hybridmp(
		n,
		(*p).bark,
		logmdct,
		logmask,
		140.0f32,
		-(1 as ::core::ffi::c_int)
	);
	i = 0 as ::core::ffi::c_int;
	while i < n {
		*work.offset(i as isize) = *logmdct.offset(i as isize) - *logmask.offset(i as isize);
		i += 1;
	}
	bark_noise_hybridmp(
		n,
		(*p).bark,
		work,
		logmask,
		0.0f32,
		(*(*p).vi).noisewindowfixed
	);
	i = 0 as ::core::ffi::c_int;
	while i < n {
		*work.offset(i as isize) = *logmdct.offset(i as isize) - *work.offset(i as isize);
		i += 1;
	}
	i = 0 as ::core::ffi::c_int;
	while i < n {
		let mut dB: ::core::ffi::c_int =
			(*logmask.offset(i as isize) as ::core::ffi::c_double + 0.5f64) as ::core::ffi::c_int;
		if dB >= NOISE_COMPAND_LEVELS {
			dB = NOISE_COMPAND_LEVELS - 1 as ::core::ffi::c_int;
		}
		if dB < 0 as ::core::ffi::c_int {
			dB = 0 as ::core::ffi::c_int;
		}
		*logmask.offset(i as isize) =
			*work.offset(i as isize) + (*(*p).vi).noisecompand[dB as usize];
		i += 1;
	}
}
#[no_mangle]
pub unsafe extern "C" fn _vp_tonemask(
	mut p: *mut vorbis_look_psy,
	mut logfft: *mut ::core::ffi::c_float,
	mut logmask: *mut ::core::ffi::c_float,
	mut global_specmax: ::core::ffi::c_float,
	mut local_specmax: ::core::ffi::c_float
) {
	let mut alloca_allocations: Vec<Vec<u8>> = Vec::new();
	let mut i: ::core::ffi::c_int = 0;
	let mut n: ::core::ffi::c_int = (*p).n;
	alloca_allocations.push(::std::vec::from_elem(
		0,
		(::core::mem::size_of::<::core::ffi::c_float>() as usize)
			.wrapping_mul((*p).total_octave_lines as usize) as usize
	));
	let mut seed: *mut ::core::ffi::c_float =
		alloca_allocations.last_mut().unwrap().as_mut_ptr() as *mut ::core::ffi::c_float;
	let mut att: ::core::ffi::c_float = local_specmax + (*(*p).vi).ath_adjatt;
	i = 0 as ::core::ffi::c_int;
	while i < (*p).total_octave_lines {
		*seed.offset(i as isize) = NEGINF;
		i += 1;
	}
	if att < (*(*p).vi).ath_maxatt {
		att = (*(*p).vi).ath_maxatt;
	}
	i = 0 as ::core::ffi::c_int;
	while i < n {
		*logmask.offset(i as isize) = *(*p).ath.offset(i as isize) + att;
		i += 1;
	}
	seed_loop(
		p,
		(*p).tonecurves as *mut *mut *const ::core::ffi::c_float,
		logfft,
		logmask,
		seed,
		global_specmax
	);
	max_seeds(p, seed, logmask);
}
#[no_mangle]
pub unsafe extern "C" fn _vp_offset_and_mix(
	mut p: *mut vorbis_look_psy,
	mut noise: *mut ::core::ffi::c_float,
	mut tone: *mut ::core::ffi::c_float,
	mut offset_select: ::core::ffi::c_int,
	mut logmask: *mut ::core::ffi::c_float,
	mut mdct: *mut ::core::ffi::c_float,
	mut logmdct: *mut ::core::ffi::c_float
) {
	let mut i: ::core::ffi::c_int = 0;
	let mut n: ::core::ffi::c_int = (*p).n;
	let mut de: ::core::ffi::c_float = 0.;
	let mut coeffi: ::core::ffi::c_float = 0.;
	let mut cx: ::core::ffi::c_float = 0.;
	let mut toneatt: ::core::ffi::c_float = (*(*p).vi).tone_masteratt[offset_select as usize];
	cx = (*p).m_val;
	i = 0 as ::core::ffi::c_int;
	while i < n {
		let mut val: ::core::ffi::c_float = *noise.offset(i as isize)
			+ *(*(*p).noiseoffset.offset(offset_select as isize)).offset(i as isize);
		if val > (*(*p).vi).noisemaxsupp {
			val = (*(*p).vi).noisemaxsupp;
		}
		*logmask.offset(i as isize) = if val < *tone.offset(i as isize) + toneatt {
			*tone.offset(i as isize) + toneatt
		} else {
			val
		};
		if offset_select == 1 as ::core::ffi::c_int {
			coeffi = -17.2f64 as ::core::ffi::c_float;
			val = val - *logmdct.offset(i as isize);
			if val > coeffi {
				de = (1.0f64
					- (val - coeffi) as ::core::ffi::c_double
						* 0.005f64 * cx as ::core::ffi::c_double) as ::core::ffi::c_float;
				if de < 0 as ::core::ffi::c_int as ::core::ffi::c_float {
					de = 0.0001f32;
				}
			} else {
				de = (1.0f64
					- (val - coeffi) as ::core::ffi::c_double
						* 0.0003f64 * cx as ::core::ffi::c_double) as ::core::ffi::c_float;
			}
			*mdct.offset(i as isize) *= de;
		}
		i += 1;
	}
}
#[no_mangle]
pub unsafe extern "C" fn _vp_ampmax_decay(
	mut amp: ::core::ffi::c_float,
	mut vd: *mut vorbis_dsp_state
) -> ::core::ffi::c_float {
	let mut vi: *mut vorbis_info = (*vd).vi;
	let mut ci: *mut codec_setup_info = (*vi).codec_setup as *mut codec_setup_info;
	let mut gi: *mut vorbis_info_psy_global = &raw mut (*ci).psy_g_param;
	let mut n: ::core::ffi::c_int =
		((*ci).blocksizes[(*vd).W as usize] / 2 as ::core::ffi::c_long) as ::core::ffi::c_int;
	let mut secs: ::core::ffi::c_float =
		n as ::core::ffi::c_float / (*vi).rate as ::core::ffi::c_float;
	amp += secs * (*gi).ampmax_att_per_sec;
	if amp < -(9999 as ::core::ffi::c_int) as ::core::ffi::c_float {
		amp = -(9999 as ::core::ffi::c_int) as ::core::ffi::c_float;
	}
	return amp;
}
static mut FLOOR1_fromdB_LOOKUP: [::core::ffi::c_float; 256] = [
	1.0649863e-07f32,
	1.1341951e-07f32,
	1.2079015e-07f32,
	1.2863978e-07f32,
	1.3699951e-07f32,
	1.4590251e-07f32,
	1.5538408e-07f32,
	1.6548181e-07f32,
	1.7623575e-07f32,
	1.8768855e-07f32,
	1.9988561e-07f32,
	2.128753e-07f32,
	2.2670913e-07f32,
	2.4144197e-07f32,
	2.5713223e-07f32,
	2.7384213e-07f32,
	2.9163793e-07f32,
	3.1059021e-07f32,
	3.3077411e-07f32,
	3.5226968e-07f32,
	3.7516214e-07f32,
	3.9954229e-07f32,
	4.2550680e-07f32,
	4.5315863e-07f32,
	4.8260743e-07f32,
	5.1396998e-07f32,
	5.4737065e-07f32,
	5.8294187e-07f32,
	6.2082472e-07f32,
	6.6116941e-07f32,
	7.0413592e-07f32,
	7.4989464e-07f32,
	7.9862701e-07f32,
	8.5052630e-07f32,
	9.0579828e-07f32,
	9.6466216e-07f32,
	1.0273513e-06f32,
	1.0941144e-06f32,
	1.1652161e-06f32,
	1.2409384e-06f32,
	1.3215816e-06f32,
	1.4074654e-06f32,
	1.4989305e-06f32,
	1.5963394e-06f32,
	1.7000785e-06f32,
	1.8105592e-06f32,
	1.9282195e-06f32,
	2.0535261e-06f32,
	2.1869758e-06f32,
	2.3290978e-06f32,
	2.4804557e-06f32,
	2.6416497e-06f32,
	2.8133190e-06f32,
	2.9961443e-06f32,
	3.1908506e-06f32,
	3.3982101e-06f32,
	3.6190449e-06f32,
	3.8542308e-06f32,
	4.1047004e-06f32,
	4.3714470e-06f32,
	4.6555282e-06f32,
	4.9580707e-06f32,
	5.2802740e-06f32,
	5.6234160e-06f32,
	5.9888572e-06f32,
	6.3780469e-06f32,
	6.7925283e-06f32,
	7.2339451e-06f32,
	7.7040476e-06f32,
	8.2047000e-06f32,
	8.7378876e-06f32,
	9.3057248e-06f32,
	9.9104632e-06f32,
	1.0554501e-05f32,
	1.1240392e-05f32,
	1.1970856e-05f32,
	1.2748789e-05f32,
	1.3577278e-05f32,
	1.4459606e-05f32,
	1.5399272e-05f32,
	1.6400004e-05f32,
	1.7465768e-05f32,
	1.8600792e-05f32,
	1.9809576e-05f32,
	2.1096914e-05f32,
	2.2467911e-05f32,
	2.3928002e-05f32,
	2.5482978e-05f32,
	2.7139006e-05f32,
	2.8902651e-05f32,
	3.0780908e-05f32,
	3.2781225e-05f32,
	3.4911534e-05f32,
	3.7180282e-05f32,
	3.9596466e-05f32,
	4.2169667e-05f32,
	4.4910090e-05f32,
	4.7828601e-05f32,
	5.0936773e-05f32,
	5.4246931e-05f32,
	5.7772202e-05f32,
	6.1526565e-05f32,
	6.5524908e-05f32,
	6.9783085e-05f32,
	7.4317983e-05f32,
	7.9147585e-05f32,
	8.4291040e-05f32,
	8.9768747e-05f32,
	9.5602426e-05f32,
	0.00010181521f32,
	0.00010843174f32,
	0.00011547824f32,
	0.00012298267f32,
	0.00013097477f32,
	0.00013948625f32,
	0.00014855085f32,
	0.00015820453f32,
	0.00016848555f32,
	0.00017943469f32,
	0.00019109536f32,
	0.00020351382f32,
	0.00021673929f32,
	0.00023082423f32,
	0.00024582449f32,
	0.00026179955f32,
	0.00027881276f32,
	0.00029693158f32,
	0.00031622787f32,
	0.00033677814f32,
	0.00035866388f32,
	0.00038197188f32,
	0.00040679456f32,
	0.00043323036f32,
	0.00046138411f32,
	0.00049136745f32,
	0.00052329927f32,
	0.00055730621f32,
	0.00059352311f32,
	0.00063209358f32,
	0.00067317058f32,
	0.00071691700f32,
	0.00076350630f32,
	0.00081312324f32,
	0.00086596457f32,
	0.00092223983f32,
	0.00098217216f32,
	0.0010459992f32,
	0.0011139742f32,
	0.0011863665f32,
	0.0012634633f32,
	0.0013455702f32,
	0.0014330129f32,
	0.0015261382f32,
	0.0016253153f32,
	0.0017309374f32,
	0.0018434235f32,
	0.0019632195f32,
	0.0020908006f32,
	0.0022266726f32,
	0.0023713743f32,
	0.0025254795f32,
	0.0026895994f32,
	0.0028643847f32,
	0.0030505286f32,
	0.0032487691f32,
	0.0034598925f32,
	0.0036847358f32,
	0.0039241906f32,
	0.0041792066f32,
	0.0044507950f32,
	0.0047400328f32,
	0.0050480668f32,
	0.0053761186f32,
	0.0057254891f32,
	0.0060975636f32,
	0.0064938176f32,
	0.0069158225f32,
	0.0073652516f32,
	0.0078438871f32,
	0.0083536271f32,
	0.0088964928f32,
	0.009474637f32,
	0.010090352f32,
	0.010746080f32,
	0.011444421f32,
	0.012188144f32,
	0.012980198f32,
	0.013823725f32,
	0.014722068f32,
	0.015678791f32,
	0.016697687f32,
	0.017782797f32,
	0.018938423f32,
	0.020169149f32,
	0.021479854f32,
	0.022875735f32,
	0.024362330f32,
	0.025945531f32,
	0.027631618f32,
	0.029427276f32,
	0.031339626f32,
	0.033376252f32,
	0.035545228f32,
	0.037855157f32,
	0.040315199f32,
	0.042935108f32,
	0.045725273f32,
	0.048696758f32,
	0.051861348f32,
	0.055231591f32,
	0.058820850f32,
	0.062643361f32,
	0.066714279f32,
	0.071049749f32,
	0.075666962f32,
	0.080584227f32,
	0.085821044f32,
	0.091398179f32,
	0.097337747f32,
	0.10366330f32,
	0.11039993f32,
	0.11757434f32,
	0.12521498f32,
	0.13335215f32,
	0.14201813f32,
	0.15124727f32,
	0.16107617f32,
	0.17154380f32,
	0.18269168f32,
	0.19456402f32,
	0.20720788f32,
	0.22067342f32,
	0.23501402f32,
	0.25028656f32,
	0.26655159f32,
	0.28387361f32,
	0.30232132f32,
	0.32196786f32,
	0.34289114f32,
	0.36517414f32,
	0.38890521f32,
	0.41417847f32,
	0.44109412f32,
	0.46975890f32,
	0.50028648f32,
	0.53279791f32,
	0.56742212f32,
	0.60429640f32,
	0.64356699f32,
	0.68538959f32,
	0.72993007f32,
	0.77736504f32,
	0.82788260f32,
	0.88168307f32,
	0.9389798f32,
	1.0f32
];
unsafe extern "C" fn apsort(
	mut a: *const ::core::ffi::c_void,
	mut b: *const ::core::ffi::c_void
) -> ::core::ffi::c_int {
	let mut f1: ::core::ffi::c_float = **(a as *mut *mut ::core::ffi::c_float);
	let mut f2: ::core::ffi::c_float = **(b as *mut *mut ::core::ffi::c_float);
	return (f1 < f2) as ::core::ffi::c_int - (f1 > f2) as ::core::ffi::c_int;
}
unsafe extern "C" fn flag_lossless(
	mut limit: ::core::ffi::c_int,
	mut prepoint: ::core::ffi::c_float,
	mut postpoint: ::core::ffi::c_float,
	mut mdct: *mut ::core::ffi::c_float,
	mut floor_0: *mut ::core::ffi::c_float,
	mut flag: *mut ::core::ffi::c_int,
	mut i: ::core::ffi::c_int,
	mut jn: ::core::ffi::c_int
) {
	let mut j: ::core::ffi::c_int = 0;
	j = 0 as ::core::ffi::c_int;
	while j < jn {
		let mut point: ::core::ffi::c_float = if j >= limit - i { postpoint } else { prepoint };
		let mut r: ::core::ffi::c_float = (fabs(*mdct.offset(j as isize) as ::core::ffi::c_double)
			/ *floor_0.offset(j as isize) as ::core::ffi::c_double)
			as ::core::ffi::c_float;
		if r < point {
			*flag.offset(j as isize) = 0 as ::core::ffi::c_int;
		} else {
			*flag.offset(j as isize) = 1 as ::core::ffi::c_int;
		}
		j += 1;
	}
}
unsafe extern "C" fn noise_normalize(
	mut p: *mut vorbis_look_psy,
	mut limit: ::core::ffi::c_int,
	mut r: *mut ::core::ffi::c_float,
	mut q: *mut ::core::ffi::c_float,
	mut f: *mut ::core::ffi::c_float,
	mut flags: *mut ::core::ffi::c_int,
	mut acc: ::core::ffi::c_float,
	mut i: ::core::ffi::c_int,
	mut n: ::core::ffi::c_int,
	mut out: *mut ::core::ffi::c_int
) -> ::core::ffi::c_float {
	let mut alloca_allocations: Vec<Vec<u8>> = Vec::new();
	let mut vi: *mut vorbis_info_psy = (*p).vi as *mut vorbis_info_psy;
	alloca_allocations.push(::std::vec::from_elem(
		0,
		(n as usize).wrapping_mul(::core::mem::size_of::<*mut ::core::ffi::c_float>() as usize)
			as usize
	));
	let mut sort: *mut *mut ::core::ffi::c_float =
		alloca_allocations.last_mut().unwrap().as_mut_ptr() as *mut *mut ::core::ffi::c_float;
	let mut j: ::core::ffi::c_int = 0;
	let mut count: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
	let mut start: ::core::ffi::c_int = if (*vi).normal_p != 0 {
		(*vi).normal_start - i
	} else {
		n
	};
	if start > n {
		start = n;
	}
	acc = 0.0f32;
	j = 0 as ::core::ffi::c_int;
	while j < start {
		if flags.is_null() || *flags.offset(j as isize) == 0 {
			let mut ve: ::core::ffi::c_float = *q.offset(j as isize) / *f.offset(j as isize);
			if *r.offset(j as isize) < 0 as ::core::ffi::c_int as ::core::ffi::c_float {
				*out.offset(j as isize) =
					-rint(sqrt(ve as ::core::ffi::c_double)) as ::core::ffi::c_int;
			} else {
				*out.offset(j as isize) =
					rint(sqrt(ve as ::core::ffi::c_double)) as ::core::ffi::c_int;
			}
		}
		j += 1;
	}
	while j < n {
		if flags.is_null() || *flags.offset(j as isize) == 0 {
			let mut ve_0: ::core::ffi::c_float = *q.offset(j as isize) / *f.offset(j as isize);
			if ve_0 < 0.25f32 && (flags.is_null() || j >= limit - i) {
				acc += ve_0;
				let fresh21 = count;
				count = count + 1;
				let ref mut fresh22 = *sort.offset(fresh21 as isize);
				*fresh22 = q.offset(j as isize);
			} else {
				if *r.offset(j as isize) < 0 as ::core::ffi::c_int as ::core::ffi::c_float {
					*out.offset(j as isize) =
						-rint(sqrt(ve_0 as ::core::ffi::c_double)) as ::core::ffi::c_int;
				} else {
					*out.offset(j as isize) =
						rint(sqrt(ve_0 as ::core::ffi::c_double)) as ::core::ffi::c_int;
				}
				*q.offset(j as isize) = (*out.offset(j as isize) * *out.offset(j as isize))
					as ::core::ffi::c_float
					* *f.offset(j as isize);
			}
		}
		j += 1;
	}
	if count != 0 {
		::core::slice::from_raw_parts_mut(sort, count as usize).sort_by(|a, b| unsafe {
			(**a)
				.partial_cmp(&**b)
				.unwrap_or(::core::cmp::Ordering::Equal)
		});
		j = 0 as ::core::ffi::c_int;
		while j < count {
			let mut k: ::core::ffi::c_int = (*sort.offset(j as isize)).offset_from(q)
				as ::core::ffi::c_long as ::core::ffi::c_int;
			if acc as ::core::ffi::c_double >= (*vi).normal_thresh {
				*out.offset(k as isize) = unitnorm(*r.offset(k as isize)) as ::core::ffi::c_int;
				acc -= 1.0f32;
				*q.offset(k as isize) = *f.offset(k as isize);
			} else {
				*out.offset(k as isize) = 0 as ::core::ffi::c_int;
				*q.offset(k as isize) = 0.0f32;
			}
			j += 1;
		}
	}
	return acc;
}
#[no_mangle]
pub unsafe extern "C" fn _vp_couple_quantize_normalize(
	mut blobno: ::core::ffi::c_int,
	mut g: *mut vorbis_info_psy_global,
	mut p: *mut vorbis_look_psy,
	mut vi: *mut vorbis_info_mapping0,
	mut mdct: *mut *mut ::core::ffi::c_float,
	mut iwork: *mut *mut ::core::ffi::c_int,
	mut nonzero: *mut ::core::ffi::c_int,
	mut sliding_lowpass: ::core::ffi::c_int,
	mut ch: ::core::ffi::c_int
) {
	let mut alloca_allocations: Vec<Vec<u8>> = Vec::new();
	let mut i: ::core::ffi::c_int = 0;
	let mut n: ::core::ffi::c_int = (*p).n;
	let mut partition: ::core::ffi::c_int = if (*(*p).vi).normal_p != 0 {
		(*(*p).vi).normal_partition
	} else {
		16 as ::core::ffi::c_int
	};
	let mut limit: ::core::ffi::c_int =
		(*g).coupling_pointlimit[(*(*p).vi).blockflag as usize][blobno as usize];
	let mut prepoint: ::core::ffi::c_float = stereo_threshholds
		[(*g).coupling_prepointamp[blobno as usize] as usize]
		as ::core::ffi::c_float;
	let mut postpoint: ::core::ffi::c_float = stereo_threshholds
		[(*g).coupling_postpointamp[blobno as usize] as usize]
		as ::core::ffi::c_float;
	alloca_allocations.push(::std::vec::from_elem(
		0,
		(ch as usize).wrapping_mul(::core::mem::size_of::<*mut ::core::ffi::c_float>() as usize)
			as usize
	));
	let mut raw: *mut *mut ::core::ffi::c_float =
		alloca_allocations.last_mut().unwrap().as_mut_ptr() as *mut *mut ::core::ffi::c_float;
	alloca_allocations.push(::std::vec::from_elem(
		0,
		(ch as usize).wrapping_mul(::core::mem::size_of::<*mut ::core::ffi::c_float>() as usize)
			as usize
	));
	let mut quant: *mut *mut ::core::ffi::c_float =
		alloca_allocations.last_mut().unwrap().as_mut_ptr() as *mut *mut ::core::ffi::c_float;
	alloca_allocations.push(::std::vec::from_elem(
		0,
		(ch as usize).wrapping_mul(::core::mem::size_of::<*mut ::core::ffi::c_float>() as usize)
			as usize
	));
	let mut floor_0: *mut *mut ::core::ffi::c_float =
		alloca_allocations.last_mut().unwrap().as_mut_ptr() as *mut *mut ::core::ffi::c_float;
	alloca_allocations.push(::std::vec::from_elem(
		0,
		(ch as usize).wrapping_mul(::core::mem::size_of::<*mut ::core::ffi::c_int>() as usize)
			as usize
	));
	let mut flag: *mut *mut ::core::ffi::c_int =
		alloca_allocations.last_mut().unwrap().as_mut_ptr() as *mut *mut ::core::ffi::c_int;
	alloca_allocations.push(::std::vec::from_elem(
		0,
		(ch as usize).wrapping_mul(::core::mem::size_of::<::core::ffi::c_int>() as usize) as usize
	));
	let mut nz: *mut ::core::ffi::c_int =
		alloca_allocations.last_mut().unwrap().as_mut_ptr() as *mut ::core::ffi::c_int;
	alloca_allocations.push(::std::vec::from_elem(
		0,
		((ch + (*vi).coupling_steps) as usize)
			.wrapping_mul(::core::mem::size_of::<::core::ffi::c_float>() as usize) as usize
	));
	let mut acc: *mut ::core::ffi::c_float =
		alloca_allocations.last_mut().unwrap().as_mut_ptr() as *mut ::core::ffi::c_float;
	if n > 1000 as ::core::ffi::c_int {
		postpoint = stereo_threshholds_limited[(*g).coupling_postpointamp[blobno as usize] as usize]
			as ::core::ffi::c_float;
	}
	alloca_allocations.push(::std::vec::from_elem(
		0,
		((ch * partition) as usize)
			.wrapping_mul(::core::mem::size_of::<::core::ffi::c_float>() as usize) as usize
	));
	let ref mut fresh6 = *raw.offset(0 as ::core::ffi::c_int as isize);
	*fresh6 = alloca_allocations.last_mut().unwrap().as_mut_ptr() as *mut ::core::ffi::c_float;
	alloca_allocations.push(::std::vec::from_elem(
		0,
		((ch * partition) as usize)
			.wrapping_mul(::core::mem::size_of::<::core::ffi::c_float>() as usize) as usize
	));
	let ref mut fresh7 = *quant.offset(0 as ::core::ffi::c_int as isize);
	*fresh7 = alloca_allocations.last_mut().unwrap().as_mut_ptr() as *mut ::core::ffi::c_float;
	alloca_allocations.push(::std::vec::from_elem(
		0,
		((ch * partition) as usize)
			.wrapping_mul(::core::mem::size_of::<::core::ffi::c_float>() as usize) as usize
	));
	let ref mut fresh8 = *floor_0.offset(0 as ::core::ffi::c_int as isize);
	*fresh8 = alloca_allocations.last_mut().unwrap().as_mut_ptr() as *mut ::core::ffi::c_float;
	alloca_allocations.push(::std::vec::from_elem(
		0,
		((ch * partition) as usize)
			.wrapping_mul(::core::mem::size_of::<::core::ffi::c_int>() as usize) as usize
	));
	let ref mut fresh9 = *flag.offset(0 as ::core::ffi::c_int as isize);
	*fresh9 = alloca_allocations.last_mut().unwrap().as_mut_ptr() as *mut ::core::ffi::c_int;
	i = 1 as ::core::ffi::c_int;
	while i < ch {
		let ref mut fresh10 = *raw.offset(i as isize);
		*fresh10 = (*raw.offset(0 as ::core::ffi::c_int as isize)).offset((partition * i) as isize)
			as *mut ::core::ffi::c_float;
		let ref mut fresh11 = *quant.offset(i as isize);
		*fresh11 = (*quant.offset(0 as ::core::ffi::c_int as isize))
			.offset((partition * i) as isize) as *mut ::core::ffi::c_float;
		let ref mut fresh12 = *floor_0.offset(i as isize);
		*fresh12 = (*floor_0.offset(0 as ::core::ffi::c_int as isize))
			.offset((partition * i) as isize) as *mut ::core::ffi::c_float;
		let ref mut fresh13 = *flag.offset(i as isize);
		*fresh13 = (*flag.offset(0 as ::core::ffi::c_int as isize)).offset((partition * i) as isize)
			as *mut ::core::ffi::c_int;
		i += 1;
	}
	i = 0 as ::core::ffi::c_int;
	while i < ch + (*vi).coupling_steps {
		*acc.offset(i as isize) = 0.0f32;
		i += 1;
	}
	i = 0 as ::core::ffi::c_int;
	while i < n {
		let mut k: ::core::ffi::c_int = 0;
		let mut j: ::core::ffi::c_int = 0;
		let mut jn: ::core::ffi::c_int = if partition > n - i { n - i } else { partition };
		let mut step: ::core::ffi::c_int = 0;
		let mut track: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
		memcpy(
			nz as *mut ::core::ffi::c_void,
			nonzero as *const ::core::ffi::c_void,
			(::core::mem::size_of::<::core::ffi::c_int>() as size_t).wrapping_mul(ch as size_t)
		);
		memset(
			*flag.offset(0 as ::core::ffi::c_int as isize) as *mut ::core::ffi::c_void,
			0 as ::core::ffi::c_int,
			((ch * partition) as size_t)
				.wrapping_mul(::core::mem::size_of::<::core::ffi::c_int>() as size_t)
		);
		k = 0 as ::core::ffi::c_int;
		while k < ch {
			let mut iout: *mut ::core::ffi::c_int =
				(*iwork.offset(k as isize)).offset(i as isize) as *mut ::core::ffi::c_int;
			if *nz.offset(k as isize) != 0 {
				j = 0 as ::core::ffi::c_int;
				while j < jn {
					*(*floor_0.offset(k as isize)).offset(j as isize) =
						FLOOR1_fromdB_LOOKUP[*iout.offset(j as isize) as usize];
					j += 1;
				}
				flag_lossless(
					limit,
					prepoint,
					postpoint,
					(*mdct.offset(k as isize)).offset(i as isize) as *mut ::core::ffi::c_float,
					*floor_0.offset(k as isize),
					*flag.offset(k as isize),
					i,
					jn
				);
				j = 0 as ::core::ffi::c_int;
				while j < jn {
					let ref mut fresh14 = *(*raw.offset(k as isize)).offset(j as isize);
					*fresh14 = *(*mdct.offset(k as isize)).offset((i + j) as isize)
						* *(*mdct.offset(k as isize)).offset((i + j) as isize);
					*(*quant.offset(k as isize)).offset(j as isize) = *fresh14;
					if *(*mdct.offset(k as isize)).offset((i + j) as isize) < 0.0f32 {
						*(*raw.offset(k as isize)).offset(j as isize) *= -1.0f32;
					}
					*(*floor_0.offset(k as isize)).offset(j as isize) *=
						*(*floor_0.offset(k as isize)).offset(j as isize);
					j += 1;
				}
				*acc.offset(track as isize) = noise_normalize(
					p,
					limit,
					*raw.offset(k as isize),
					*quant.offset(k as isize),
					*floor_0.offset(k as isize),
					::core::ptr::null_mut::<::core::ffi::c_int>(),
					*acc.offset(track as isize),
					i,
					jn,
					iout
				);
			} else {
				j = 0 as ::core::ffi::c_int;
				while j < jn {
					*(*floor_0.offset(k as isize)).offset(j as isize) = 1e-10f32;
					*(*raw.offset(k as isize)).offset(j as isize) = 0.0f32;
					*(*quant.offset(k as isize)).offset(j as isize) = 0.0f32;
					*(*flag.offset(k as isize)).offset(j as isize) = 0 as ::core::ffi::c_int;
					*iout.offset(j as isize) = 0 as ::core::ffi::c_int;
					j += 1;
				}
				*acc.offset(track as isize) = 0.0f32;
			}
			track += 1;
			k += 1;
		}
		step = 0 as ::core::ffi::c_int;
		while step < (*vi).coupling_steps {
			let mut Mi: ::core::ffi::c_int = (*vi).coupling_mag[step as usize];
			let mut Ai: ::core::ffi::c_int = (*vi).coupling_ang[step as usize];
			let mut iM: *mut ::core::ffi::c_int =
				(*iwork.offset(Mi as isize)).offset(i as isize) as *mut ::core::ffi::c_int;
			let mut iA: *mut ::core::ffi::c_int =
				(*iwork.offset(Ai as isize)).offset(i as isize) as *mut ::core::ffi::c_int;
			let mut reM: *mut ::core::ffi::c_float = *raw.offset(Mi as isize);
			let mut reA: *mut ::core::ffi::c_float = *raw.offset(Ai as isize);
			let mut qeM: *mut ::core::ffi::c_float = *quant.offset(Mi as isize);
			let mut qeA: *mut ::core::ffi::c_float = *quant.offset(Ai as isize);
			let mut floorM: *mut ::core::ffi::c_float = *floor_0.offset(Mi as isize);
			let mut floorA: *mut ::core::ffi::c_float = *floor_0.offset(Ai as isize);
			let mut fM: *mut ::core::ffi::c_int = *flag.offset(Mi as isize);
			let mut fA: *mut ::core::ffi::c_int = *flag.offset(Ai as isize);
			if *nz.offset(Mi as isize) != 0 || *nz.offset(Ai as isize) != 0 {
				let ref mut fresh15 = *nz.offset(Ai as isize);
				*fresh15 = 1 as ::core::ffi::c_int;
				*nz.offset(Mi as isize) = *fresh15;
				j = 0 as ::core::ffi::c_int;
				while j < jn {
					if j < sliding_lowpass - i {
						if *fM.offset(j as isize) != 0 || *fA.offset(j as isize) != 0 {
							*reM.offset(j as isize) =
								(fabs(*reM.offset(j as isize) as ::core::ffi::c_double)
									+ fabs(*reA.offset(j as isize) as ::core::ffi::c_double))
									as ::core::ffi::c_float;
							*qeM.offset(j as isize) =
								*qeM.offset(j as isize) + *qeA.offset(j as isize);
							let ref mut fresh16 = *fA.offset(j as isize);
							*fresh16 = 1 as ::core::ffi::c_int;
							*fM.offset(j as isize) = *fresh16;
							let mut A: ::core::ffi::c_int = *iM.offset(j as isize);
							let mut B: ::core::ffi::c_int = *iA.offset(j as isize);
							if A.abs() > B.abs() {
								*iA.offset(j as isize) = if A > 0 as ::core::ffi::c_int {
									A - B
								} else {
									B - A
								};
							} else {
								*iA.offset(j as isize) = if B > 0 as ::core::ffi::c_int {
									A - B
								} else {
									B - A
								};
								*iM.offset(j as isize) = B;
							}
							if *iA.offset(j as isize)
								>= (*iM.offset(j as isize)).abs() * 2 as ::core::ffi::c_int
							{
								*iA.offset(j as isize) = -*iA.offset(j as isize);
								*iM.offset(j as isize) = -*iM.offset(j as isize);
							}
						} else {
							if j < limit - i {
								*reM.offset(j as isize) += *reA.offset(j as isize);
								*qeM.offset(j as isize) =
									fabs(*reM.offset(j as isize) as ::core::ffi::c_double)
										as ::core::ffi::c_float;
							} else if *reM.offset(j as isize) + *reA.offset(j as isize)
								< 0 as ::core::ffi::c_int as ::core::ffi::c_float
							{
								let ref mut fresh17 = *qeM.offset(j as isize);
								*fresh17 = (fabs(*reM.offset(j as isize) as ::core::ffi::c_double)
									+ fabs(*reA.offset(j as isize) as ::core::ffi::c_double))
									as ::core::ffi::c_float;
								*reM.offset(j as isize) = -*fresh17;
							} else {
								let ref mut fresh18 = *qeM.offset(j as isize);
								*fresh18 = (fabs(*reM.offset(j as isize) as ::core::ffi::c_double)
									+ fabs(*reA.offset(j as isize) as ::core::ffi::c_double))
									as ::core::ffi::c_float;
								*reM.offset(j as isize) = *fresh18;
							}
							let ref mut fresh19 = *qeA.offset(j as isize);
							*fresh19 = 0.0f32;
							*reA.offset(j as isize) = *fresh19;
							*fA.offset(j as isize) = 1 as ::core::ffi::c_int;
							*iA.offset(j as isize) = 0 as ::core::ffi::c_int;
						}
					}
					let ref mut fresh20 = *floorA.offset(j as isize);
					*fresh20 = *floorM.offset(j as isize) + *floorA.offset(j as isize);
					*floorM.offset(j as isize) = *fresh20;
					j += 1;
				}
				*acc.offset(track as isize) = noise_normalize(
					p,
					limit,
					*raw.offset(Mi as isize),
					*quant.offset(Mi as isize),
					*floor_0.offset(Mi as isize),
					*flag.offset(Mi as isize),
					*acc.offset(track as isize),
					i,
					jn,
					iM
				);
				track += 1;
			}
			step += 1;
		}
		i += partition;
	}
	i = 0 as ::core::ffi::c_int;
	while i < (*vi).coupling_steps {
		if *nonzero.offset((*vi).coupling_mag[i as usize] as isize) != 0
			|| *nonzero.offset((*vi).coupling_ang[i as usize] as isize) != 0
		{
			*nonzero.offset((*vi).coupling_mag[i as usize] as isize) = 1 as ::core::ffi::c_int;
			*nonzero.offset((*vi).coupling_ang[i as usize] as isize) = 1 as ::core::ffi::c_int;
		}
		i += 1;
	}
}
pub const MAX_ATH: ::core::ffi::c_int = 88 as ::core::ffi::c_int;
static mut ATH: [::core::ffi::c_float; 88] = [
	-(51 as ::core::ffi::c_int) as ::core::ffi::c_float,
	-(52 as ::core::ffi::c_int) as ::core::ffi::c_float,
	-(53 as ::core::ffi::c_int) as ::core::ffi::c_float,
	-(54 as ::core::ffi::c_int) as ::core::ffi::c_float,
	-(55 as ::core::ffi::c_int) as ::core::ffi::c_float,
	-(56 as ::core::ffi::c_int) as ::core::ffi::c_float,
	-(57 as ::core::ffi::c_int) as ::core::ffi::c_float,
	-(58 as ::core::ffi::c_int) as ::core::ffi::c_float,
	-(59 as ::core::ffi::c_int) as ::core::ffi::c_float,
	-(60 as ::core::ffi::c_int) as ::core::ffi::c_float,
	-(61 as ::core::ffi::c_int) as ::core::ffi::c_float,
	-(62 as ::core::ffi::c_int) as ::core::ffi::c_float,
	-(63 as ::core::ffi::c_int) as ::core::ffi::c_float,
	-(64 as ::core::ffi::c_int) as ::core::ffi::c_float,
	-(65 as ::core::ffi::c_int) as ::core::ffi::c_float,
	-(66 as ::core::ffi::c_int) as ::core::ffi::c_float,
	-(67 as ::core::ffi::c_int) as ::core::ffi::c_float,
	-(68 as ::core::ffi::c_int) as ::core::ffi::c_float,
	-(69 as ::core::ffi::c_int) as ::core::ffi::c_float,
	-(70 as ::core::ffi::c_int) as ::core::ffi::c_float,
	-(71 as ::core::ffi::c_int) as ::core::ffi::c_float,
	-(72 as ::core::ffi::c_int) as ::core::ffi::c_float,
	-(73 as ::core::ffi::c_int) as ::core::ffi::c_float,
	-(74 as ::core::ffi::c_int) as ::core::ffi::c_float,
	-(75 as ::core::ffi::c_int) as ::core::ffi::c_float,
	-(76 as ::core::ffi::c_int) as ::core::ffi::c_float,
	-(77 as ::core::ffi::c_int) as ::core::ffi::c_float,
	-(78 as ::core::ffi::c_int) as ::core::ffi::c_float,
	-(80 as ::core::ffi::c_int) as ::core::ffi::c_float,
	-(81 as ::core::ffi::c_int) as ::core::ffi::c_float,
	-(82 as ::core::ffi::c_int) as ::core::ffi::c_float,
	-(83 as ::core::ffi::c_int) as ::core::ffi::c_float,
	-(84 as ::core::ffi::c_int) as ::core::ffi::c_float,
	-(85 as ::core::ffi::c_int) as ::core::ffi::c_float,
	-(86 as ::core::ffi::c_int) as ::core::ffi::c_float,
	-(87 as ::core::ffi::c_int) as ::core::ffi::c_float,
	-(88 as ::core::ffi::c_int) as ::core::ffi::c_float,
	-(88 as ::core::ffi::c_int) as ::core::ffi::c_float,
	-(89 as ::core::ffi::c_int) as ::core::ffi::c_float,
	-(89 as ::core::ffi::c_int) as ::core::ffi::c_float,
	-(90 as ::core::ffi::c_int) as ::core::ffi::c_float,
	-(91 as ::core::ffi::c_int) as ::core::ffi::c_float,
	-(91 as ::core::ffi::c_int) as ::core::ffi::c_float,
	-(92 as ::core::ffi::c_int) as ::core::ffi::c_float,
	-(93 as ::core::ffi::c_int) as ::core::ffi::c_float,
	-(94 as ::core::ffi::c_int) as ::core::ffi::c_float,
	-(95 as ::core::ffi::c_int) as ::core::ffi::c_float,
	-(96 as ::core::ffi::c_int) as ::core::ffi::c_float,
	-(96 as ::core::ffi::c_int) as ::core::ffi::c_float,
	-(97 as ::core::ffi::c_int) as ::core::ffi::c_float,
	-(98 as ::core::ffi::c_int) as ::core::ffi::c_float,
	-(98 as ::core::ffi::c_int) as ::core::ffi::c_float,
	-(99 as ::core::ffi::c_int) as ::core::ffi::c_float,
	-(99 as ::core::ffi::c_int) as ::core::ffi::c_float,
	-(100 as ::core::ffi::c_int) as ::core::ffi::c_float,
	-(100 as ::core::ffi::c_int) as ::core::ffi::c_float,
	-(101 as ::core::ffi::c_int) as ::core::ffi::c_float,
	-(102 as ::core::ffi::c_int) as ::core::ffi::c_float,
	-(103 as ::core::ffi::c_int) as ::core::ffi::c_float,
	-(104 as ::core::ffi::c_int) as ::core::ffi::c_float,
	-(106 as ::core::ffi::c_int) as ::core::ffi::c_float,
	-(107 as ::core::ffi::c_int) as ::core::ffi::c_float,
	-(107 as ::core::ffi::c_int) as ::core::ffi::c_float,
	-(107 as ::core::ffi::c_int) as ::core::ffi::c_float,
	-(107 as ::core::ffi::c_int) as ::core::ffi::c_float,
	-(105 as ::core::ffi::c_int) as ::core::ffi::c_float,
	-(103 as ::core::ffi::c_int) as ::core::ffi::c_float,
	-(102 as ::core::ffi::c_int) as ::core::ffi::c_float,
	-(101 as ::core::ffi::c_int) as ::core::ffi::c_float,
	-(99 as ::core::ffi::c_int) as ::core::ffi::c_float,
	-(98 as ::core::ffi::c_int) as ::core::ffi::c_float,
	-(96 as ::core::ffi::c_int) as ::core::ffi::c_float,
	-(95 as ::core::ffi::c_int) as ::core::ffi::c_float,
	-(95 as ::core::ffi::c_int) as ::core::ffi::c_float,
	-(96 as ::core::ffi::c_int) as ::core::ffi::c_float,
	-(97 as ::core::ffi::c_int) as ::core::ffi::c_float,
	-(96 as ::core::ffi::c_int) as ::core::ffi::c_float,
	-(95 as ::core::ffi::c_int) as ::core::ffi::c_float,
	-(93 as ::core::ffi::c_int) as ::core::ffi::c_float,
	-(90 as ::core::ffi::c_int) as ::core::ffi::c_float,
	-(80 as ::core::ffi::c_int) as ::core::ffi::c_float,
	-(70 as ::core::ffi::c_int) as ::core::ffi::c_float,
	-(50 as ::core::ffi::c_int) as ::core::ffi::c_float,
	-(40 as ::core::ffi::c_int) as ::core::ffi::c_float,
	-(30 as ::core::ffi::c_int) as ::core::ffi::c_float,
	-(30 as ::core::ffi::c_int) as ::core::ffi::c_float,
	-(30 as ::core::ffi::c_int) as ::core::ffi::c_float,
	-(30 as ::core::ffi::c_int) as ::core::ffi::c_float
];
pub const EHMER_OFFSET: ::core::ffi::c_int = 16 as ::core::ffi::c_int;
pub const EHMER_MAX: ::core::ffi::c_int = 56 as ::core::ffi::c_int;
static mut tonemasks: [[[::core::ffi::c_float; 56]; 6]; 17] = [
	[
		[
			-(60 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(60 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(60 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(60 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(60 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(60 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(60 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(60 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(60 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(60 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(60 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(60 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(62 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(62 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(65 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(73 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(69 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(68 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(68 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(67 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(70 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(70 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(72 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(74 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(75 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(79 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(79 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(80 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(83 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(88 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(93 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(100 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(110 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float
		],
		[
			-(48 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(48 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(48 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(48 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(48 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(48 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(48 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(48 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(48 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(48 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(48 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(48 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(48 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(53 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(61 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(66 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(66 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(68 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(67 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(70 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(76 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(76 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(72 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(73 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(75 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(76 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(78 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(79 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(83 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(88 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(93 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(100 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(110 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float
		],
		[
			-(37 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(37 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(37 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(37 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(37 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(37 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(37 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(37 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(38 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(40 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(42 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(46 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(48 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(53 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(55 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(62 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(65 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(58 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(56 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(56 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(61 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(60 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(65 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(67 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(69 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(71 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(77 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(77 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(78 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(80 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(82 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(84 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(88 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(93 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(98 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(106 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(112 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float
		],
		[
			-(25 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(25 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(25 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(25 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(25 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(25 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(25 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(25 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(25 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(26 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(27 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(29 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(32 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(38 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(48 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(52 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(52 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(50 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(48 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(48 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(51 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(52 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(54 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(60 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(67 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(67 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(66 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(68 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(69 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(73 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(73 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(76 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(80 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(81 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(81 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(85 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(85 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(86 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(88 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(93 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(100 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(110 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float
		],
		[
			-(16 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(16 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(16 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(16 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(16 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(16 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(16 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(16 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(17 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(19 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(20 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(22 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(26 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(28 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(31 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(40 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(47 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(39 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(39 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(40 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(42 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(43 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(47 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(51 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(57 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(52 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(55 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(55 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(60 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(58 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(62 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(63 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(70 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(67 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(69 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(72 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(73 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(77 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(80 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(82 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(83 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(87 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(90 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(94 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(98 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(104 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(115 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float
		],
		[
			-(8 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(8 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(8 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(8 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(8 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(8 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(8 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(8 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(8 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(8 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(10 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(11 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(15 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(19 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(25 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(30 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(34 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(31 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(30 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(31 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(29 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(32 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(35 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(42 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(48 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(42 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(44 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(46 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(50 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(50 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(51 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(52 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(59 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(54 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(55 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(55 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(58 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(62 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(63 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(66 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(72 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(73 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(76 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(75 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(78 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(80 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(80 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(81 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(84 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(88 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(90 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(94 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(98 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(101 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(106 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(110 as ::core::ffi::c_int) as ::core::ffi::c_float
		]
	],
	[
		[
			-(66 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(66 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(66 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(66 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(66 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(66 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(66 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(66 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(66 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(66 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(66 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(66 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(66 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(67 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(67 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(67 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(76 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(72 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(71 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(74 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(76 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(76 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(75 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(78 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(79 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(79 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(81 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(83 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(86 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(89 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(93 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(97 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(100 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(105 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(110 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float
		],
		[
			-(47 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(47 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(47 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(47 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(47 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(47 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(47 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(47 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(47 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(47 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(47 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(48 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(51 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(55 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(59 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(66 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(66 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(66 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(67 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(66 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(68 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(69 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(70 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(74 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(79 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(77 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(77 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(78 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(80 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(81 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(82 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(84 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(86 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(88 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(91 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(95 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(100 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(108 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(116 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float
		],
		[
			-(36 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(36 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(36 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(36 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(36 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(36 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(36 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(36 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(36 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(37 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(37 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(41 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(44 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(48 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(51 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(58 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(62 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(60 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(57 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(59 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(59 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(60 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(63 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(65 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(72 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(71 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(70 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(72 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(74 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(77 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(76 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(78 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(81 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(81 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(80 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(83 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(86 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(91 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(96 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(100 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(105 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(110 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float
		],
		[
			-(28 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(28 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(28 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(28 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(28 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(28 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(28 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(28 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(28 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(30 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(32 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(32 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(33 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(35 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(41 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(49 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(50 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(49 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(47 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(48 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(48 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(52 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(51 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(57 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(65 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(61 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(59 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(61 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(64 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(69 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(70 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(74 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(77 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(77 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(78 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(81 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(84 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(85 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(87 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(90 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(92 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(96 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(100 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(107 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(112 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float
		],
		[
			-(19 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(19 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(19 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(19 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(19 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(19 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(19 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(19 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(20 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(21 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(23 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(27 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(30 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(35 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(36 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(41 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(46 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(44 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(42 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(40 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(41 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(41 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(43 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(48 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(55 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(53 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(52 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(53 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(56 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(59 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(58 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(60 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(67 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(66 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(69 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(71 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(72 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(75 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(79 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(81 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(84 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(87 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(90 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(93 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(97 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(101 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(107 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(114 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float
		],
		[
			-(9 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(9 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(9 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(9 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(9 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(9 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(9 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(9 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(11 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(12 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(12 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(15 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(16 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(20 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(23 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(30 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(37 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(34 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(33 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(34 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(31 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(32 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(32 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(38 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(47 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(44 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(41 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(40 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(47 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(49 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(46 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(46 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(58 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(50 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(50 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(54 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(58 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(62 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(64 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(67 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(67 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(70 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(72 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(76 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(79 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(83 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(87 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(91 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(96 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(100 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(104 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(110 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float
		]
	],
	[
		[
			-(62 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(62 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(62 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(62 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(62 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(62 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(62 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(62 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(62 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(62 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(63 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(64 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(66 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(67 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(66 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(68 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(75 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(72 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(76 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(75 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(76 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(78 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(79 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(82 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(84 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(85 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(90 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(94 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(101 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(110 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float
		],
		[
			-(59 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(59 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(59 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(59 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(59 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(59 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(59 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(59 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(59 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(59 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(59 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(60 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(60 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(61 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(63 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(66 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(71 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(68 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(70 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(70 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(71 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(72 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(72 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(75 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(81 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(78 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(79 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(82 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(83 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(86 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(90 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(97 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(103 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(113 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float
		],
		[
			-(53 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(53 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(53 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(53 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(53 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(53 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(53 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(53 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(53 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(54 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(55 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(57 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(56 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(57 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(55 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(61 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(65 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(60 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(60 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(62 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(63 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(63 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(66 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(68 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(74 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(73 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(75 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(75 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(78 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(80 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(80 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(82 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(85 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(90 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(96 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(101 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(108 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float
		],
		[
			-(46 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(46 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(46 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(46 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(46 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(46 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(46 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(46 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(46 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(46 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(47 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(47 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(47 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(47 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(48 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(51 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(57 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(51 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(49 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(50 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(51 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(53 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(54 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(59 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(66 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(60 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(62 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(67 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(67 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(70 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(72 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(75 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(76 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(78 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(81 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(85 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(88 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(94 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(97 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(104 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(112 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float
		],
		[
			-(36 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(36 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(36 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(36 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(36 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(36 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(36 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(36 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(39 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(41 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(42 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(42 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(39 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(38 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(41 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(43 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(52 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(44 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(40 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(39 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(37 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(37 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(40 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(47 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(54 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(50 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(48 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(50 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(55 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(61 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(59 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(62 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(66 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(66 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(66 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(69 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(69 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(73 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(74 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(74 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(75 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(77 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(79 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(82 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(87 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(91 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(95 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(100 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(108 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(115 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float
		],
		[
			-(28 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(26 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(24 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(22 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(20 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(20 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(23 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(29 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(30 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(31 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(28 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(27 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(28 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(28 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(28 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(35 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(40 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(33 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(32 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(29 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(30 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(30 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(30 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(37 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(45 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(41 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(37 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(38 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(45 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(47 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(47 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(48 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(53 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(49 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(48 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(50 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(49 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(49 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(51 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(52 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(58 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(56 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(57 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(56 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(60 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(61 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(62 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(70 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(72 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(74 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(78 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(83 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(88 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(93 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(100 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(106 as ::core::ffi::c_int) as ::core::ffi::c_float
		]
	],
	[
		[
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(110 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(105 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(100 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(95 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(91 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(87 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(83 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(80 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(78 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(76 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(78 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(78 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(81 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(83 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(85 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(86 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(85 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(86 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(87 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(90 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(97 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(107 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float
		],
		[
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(110 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(105 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(100 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(95 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(90 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(85 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(81 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(77 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(73 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(70 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(67 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(67 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(68 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(75 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(73 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(70 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(69 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(70 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(72 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(75 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(79 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(84 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(83 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(84 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(86 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(88 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(89 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(89 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(93 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(98 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(105 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(112 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float
		],
		[
			-(105 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(100 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(95 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(90 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(85 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(80 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(76 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(71 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(68 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(68 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(65 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(63 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(63 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(62 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(62 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(64 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(65 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(64 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(61 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(62 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(63 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(64 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(66 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(68 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(73 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(73 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(74 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(75 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(76 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(81 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(83 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(85 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(88 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(89 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(92 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(95 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(100 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(108 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float
		],
		[
			-(80 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(75 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(71 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(68 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(65 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(63 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(62 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(61 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(61 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(61 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(61 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(59 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(56 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(57 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(53 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(50 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(58 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(52 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(50 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(50 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(52 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(53 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(54 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(58 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(67 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(63 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(67 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(68 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(72 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(75 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(78 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(80 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(81 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(81 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(82 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(85 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(89 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(90 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(93 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(97 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(101 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(107 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(114 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float
		],
		[
			-(65 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(61 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(59 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(57 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(56 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(55 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(55 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(56 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(56 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(57 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(55 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(53 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(52 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(47 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(44 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(44 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(50 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(44 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(41 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(39 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(39 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(42 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(40 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(46 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(51 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(49 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(50 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(53 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(54 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(63 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(60 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(61 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(62 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(66 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(66 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(66 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(70 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(73 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(74 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(75 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(76 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(75 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(79 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(85 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(89 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(91 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(96 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(102 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(110 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float
		],
		[
			-(52 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(50 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(49 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(49 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(48 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(48 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(48 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(49 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(50 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(50 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(49 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(46 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(43 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(39 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(35 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(33 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(38 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(36 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(32 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(29 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(32 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(32 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(32 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(35 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(44 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(39 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(38 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(38 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(46 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(50 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(45 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(46 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(53 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(50 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(50 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(50 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(54 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(54 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(53 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(53 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(56 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(57 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(59 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(66 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(70 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(72 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(74 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(79 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(83 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(85 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(90 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(97 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(114 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float
		]
	],
	[
		[
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(110 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(105 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(100 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(95 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(90 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(86 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(80 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(75 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(75 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(79 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(80 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(79 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(80 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(81 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(82 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(88 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(95 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(103 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(110 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float
		],
		[
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(108 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(103 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(98 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(93 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(88 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(83 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(79 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(78 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(75 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(71 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(67 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(68 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(73 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(73 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(72 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(73 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(75 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(77 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(80 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(82 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(88 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(93 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(100 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(107 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(114 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float
		],
		[
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(110 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(105 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(101 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(96 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(90 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(86 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(81 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(77 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(73 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(69 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(66 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(61 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(62 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(66 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(64 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(62 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(65 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(66 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(70 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(72 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(76 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(81 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(80 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(84 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(90 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(95 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(102 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(110 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float
		],
		[
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(107 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(103 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(97 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(92 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(88 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(83 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(79 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(74 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(70 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(66 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(59 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(53 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(58 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(62 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(55 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(54 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(54 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(54 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(58 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(61 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(62 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(72 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(70 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(72 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(75 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(78 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(80 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(81 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(80 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(83 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(83 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(88 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(93 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(100 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(107 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(115 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float
		],
		[
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(105 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(100 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(95 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(90 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(85 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(80 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(75 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(70 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(66 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(62 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(56 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(48 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(44 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(48 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(46 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(46 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(43 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(46 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(48 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(48 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(51 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(58 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(58 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(59 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(60 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(62 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(62 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(61 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(61 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(65 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(64 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(65 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(68 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(70 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(74 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(75 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(78 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(81 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(86 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(95 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(110 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float
		],
		[
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(105 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(100 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(95 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(90 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(85 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(80 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(75 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(70 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(65 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(61 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(55 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(49 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(39 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(33 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(40 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(35 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(32 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(38 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(40 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(33 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(35 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(37 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(46 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(41 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(45 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(44 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(46 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(42 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(45 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(46 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(52 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(50 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(50 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(50 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(54 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(54 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(55 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(57 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(62 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(64 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(66 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(68 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(70 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(76 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(81 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(90 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(100 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(110 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float
		]
	],
	[
		[
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(105 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(98 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(90 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(85 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(82 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(83 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(80 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(78 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(84 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(79 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(80 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(83 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(87 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(89 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(91 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(93 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(99 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(106 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(117 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float
		],
		[
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(105 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(98 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(90 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(85 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(80 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(75 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(70 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(68 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(74 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(72 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(74 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(77 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(80 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(82 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(85 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(87 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(92 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(89 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(91 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(95 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(100 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(106 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(112 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float
		],
		[
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(105 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(98 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(90 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(83 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(75 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(71 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(63 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(64 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(67 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(62 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(64 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(67 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(70 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(73 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(77 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(81 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(84 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(83 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(85 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(89 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(90 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(93 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(98 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(104 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(109 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(114 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float
		],
		[
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(103 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(96 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(88 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(81 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(75 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(68 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(58 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(54 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(56 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(54 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(56 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(56 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(58 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(60 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(63 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(66 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(74 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(69 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(72 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(72 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(75 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(74 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(77 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(81 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(81 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(82 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(84 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(87 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(93 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(96 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(99 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(104 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(110 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float
		],
		[
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(108 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(102 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(96 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(91 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(85 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(80 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(74 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(68 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(60 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(51 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(46 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(48 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(46 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(43 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(45 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(47 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(47 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(49 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(48 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(56 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(53 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(55 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(58 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(57 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(63 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(58 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(60 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(66 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(64 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(67 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(70 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(70 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(74 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(77 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(84 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(86 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(89 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(91 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(93 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(94 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(101 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(109 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(118 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float
		],
		[
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(108 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(103 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(98 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(93 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(88 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(83 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(78 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(73 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(68 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(60 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(53 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(44 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(35 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(38 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(38 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(34 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(34 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(36 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(40 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(41 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(44 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(51 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(45 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(46 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(47 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(46 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(54 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(50 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(49 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(50 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(50 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(50 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(51 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(54 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(57 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(58 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(60 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(66 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(66 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(66 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(64 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(65 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(68 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(77 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(82 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(87 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(95 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(110 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float
		]
	],
	[
		[
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(107 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(102 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(97 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(92 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(87 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(83 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(78 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(75 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(82 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(79 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(83 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(85 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(89 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(92 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(95 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(98 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(101 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(105 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(109 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(113 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float
		],
		[
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(106 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(100 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(95 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(90 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(86 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(81 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(78 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(74 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(69 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(74 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(74 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(76 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(79 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(83 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(84 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(86 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(89 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(92 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(97 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(93 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(100 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(103 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(107 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(110 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float
		],
		[
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(106 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(100 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(95 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(90 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(87 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(83 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(80 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(75 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(69 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(60 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(66 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(66 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(68 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(70 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(74 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(78 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(79 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(81 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(81 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(83 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(84 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(87 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(93 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(96 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(99 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(103 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(107 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(110 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float
		],
		[
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(108 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(103 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(98 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(93 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(89 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(85 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(82 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(78 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(71 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(62 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(55 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(58 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(58 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(54 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(54 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(55 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(59 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(61 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(62 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(70 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(66 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(66 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(67 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(70 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(72 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(75 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(78 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(84 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(84 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(84 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(88 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(91 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(90 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(95 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(98 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(102 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(103 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(106 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(110 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float
		],
		[
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(108 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(103 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(98 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(94 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(90 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(87 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(82 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(79 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(73 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(67 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(58 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(47 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(50 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(45 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(41 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(45 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(48 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(44 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(44 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(49 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(54 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(51 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(48 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(47 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(49 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(50 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(51 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(57 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(58 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(60 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(63 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(69 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(70 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(69 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(71 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(74 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(78 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(82 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(90 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(95 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(101 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(105 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(110 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float
		],
		[
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(105 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(101 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(97 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(93 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(90 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(85 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(80 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(77 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(72 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(65 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(56 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(48 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(37 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(40 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(36 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(34 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(40 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(50 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(47 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(38 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(41 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(47 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(38 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(35 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(39 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(38 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(43 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(40 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(45 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(50 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(45 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(44 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(47 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(50 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(55 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(48 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(48 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(52 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(66 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(70 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(76 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(82 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(90 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(97 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(105 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(110 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float
		]
	],
	[
		[
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(108 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(103 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(98 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(93 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(86 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(79 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(76 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(83 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(81 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(85 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(87 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(89 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(93 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(98 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(102 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(107 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(112 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float
		],
		[
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(108 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(103 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(98 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(93 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(86 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(79 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(71 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(77 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(74 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(77 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(79 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(81 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(84 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(85 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(90 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(92 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(93 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(92 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(98 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(101 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(108 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(112 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float
		],
		[
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(108 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(103 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(98 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(93 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(87 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(78 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(68 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(65 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(66 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(62 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(65 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(67 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(70 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(73 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(75 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(78 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(82 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(82 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(83 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(84 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(91 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(93 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(98 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(102 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(106 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(110 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float
		],
		[
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(105 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(100 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(95 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(90 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(82 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(74 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(62 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(57 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(58 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(56 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(51 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(52 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(52 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(54 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(54 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(58 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(66 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(59 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(60 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(63 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(66 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(69 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(73 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(79 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(83 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(84 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(80 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(81 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(81 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(82 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(88 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(92 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(98 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(105 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(113 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float
		],
		[
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(107 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(102 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(97 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(92 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(84 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(79 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(69 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(57 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(47 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(52 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(47 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(44 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(45 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(50 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(52 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(42 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(42 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(53 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(43 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(43 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(48 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(51 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(56 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(55 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(52 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(57 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(59 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(61 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(62 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(67 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(71 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(78 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(83 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(86 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(94 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(98 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(103 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(110 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float
		],
		[
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(105 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(100 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(95 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(90 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(84 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(78 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(70 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(61 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(51 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(41 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(40 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(38 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(40 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(46 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(52 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(51 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(41 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(40 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(46 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(40 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(38 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(38 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(41 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(46 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(41 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(46 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(47 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(43 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(43 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(45 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(41 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(45 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(56 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(67 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(68 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(83 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(87 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(90 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(95 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(102 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(107 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(113 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float
		]
	],
	[
		[
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(109 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(105 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(101 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(96 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(91 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(84 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(77 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(82 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(82 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(85 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(89 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(94 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(100 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(106 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(110 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float
		],
		[
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(106 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(103 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(98 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(92 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(85 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(80 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(71 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(75 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(72 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(76 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(80 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(84 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(86 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(89 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(93 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(100 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(107 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(113 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float
		],
		[
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(107 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(104 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(101 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(97 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(92 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(88 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(84 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(80 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(64 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(66 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(63 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(64 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(66 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(69 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(73 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(77 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(83 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(83 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(86 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(91 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(98 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(104 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(111 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float
		],
		[
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(107 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(104 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(101 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(97 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(92 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(90 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(84 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(74 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(57 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(58 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(52 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(55 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(54 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(50 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(52 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(50 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(52 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(63 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(62 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(69 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(76 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(77 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(78 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(78 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(79 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(82 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(88 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(94 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(100 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(106 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(111 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float
		],
		[
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(106 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(102 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(98 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(95 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(90 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(85 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(83 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(78 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(70 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(50 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(50 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(41 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(44 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(49 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(47 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(50 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(50 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(44 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(55 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(46 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(47 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(48 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(48 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(54 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(49 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(49 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(58 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(62 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(71 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(81 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(87 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(92 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(97 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(102 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(108 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(114 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float
		],
		[
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(106 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(102 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(98 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(95 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(90 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(85 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(83 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(78 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(70 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(45 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(43 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(41 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(47 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(50 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(51 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(50 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(49 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(45 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(47 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(41 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(44 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(41 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(39 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(43 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(38 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(37 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(40 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(41 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(44 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(50 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(58 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(65 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(73 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(79 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(85 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(92 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(97 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(101 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(105 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(109 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(113 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float
		]
	],
	[
		[
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(107 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(100 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(95 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(87 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(81 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(85 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(83 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(88 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(93 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(100 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(107 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(114 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float
		],
		[
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(107 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(101 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(95 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(88 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(83 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(76 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(73 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(72 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(79 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(84 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(90 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(95 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(100 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(105 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(110 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(115 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float
		],
		[
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(104 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(98 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(92 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(87 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(81 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(70 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(65 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(62 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(67 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(71 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(74 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(80 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(85 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(91 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(95 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(99 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(103 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(108 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(111 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(114 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float
		],
		[
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(103 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(97 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(90 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(85 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(76 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(60 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(56 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(54 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(60 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(62 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(61 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(56 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(63 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(65 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(73 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(74 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(77 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(75 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(78 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(81 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(86 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(87 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(88 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(91 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(94 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(98 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(103 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(110 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float
		],
		[
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(105 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(100 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(97 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(92 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(86 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(81 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(79 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(70 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(57 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(51 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(47 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(51 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(58 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(60 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(56 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(53 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(50 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(58 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(52 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(50 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(50 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(53 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(55 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(64 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(69 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(71 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(85 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(82 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(78 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(81 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(85 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(95 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(102 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(112 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float
		],
		[
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(105 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(100 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(97 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(92 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(85 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(83 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(79 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(72 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(49 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(40 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(43 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(43 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(54 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(56 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(51 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(50 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(40 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(43 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(38 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(36 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(35 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(37 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(38 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(37 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(44 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(54 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(60 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(57 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(60 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(70 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(75 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(84 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(92 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(103 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(112 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float
		]
	],
	[
		[
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(110 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(102 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(95 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(89 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(82 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(83 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(84 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(90 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(92 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(99 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(107 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(113 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float
		],
		[
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(107 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(101 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(95 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(89 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(83 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(72 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(74 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(78 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(85 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(88 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(88 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(90 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(92 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(98 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(105 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(111 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float
		],
		[
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(109 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(103 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(97 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(93 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(87 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(81 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(70 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(70 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(67 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(75 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(73 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(76 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(79 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(81 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(83 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(88 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(89 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(97 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(103 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(110 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float
		],
		[
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(107 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(100 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(94 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(88 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(83 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(75 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(63 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(59 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(59 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(63 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(66 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(60 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(62 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(67 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(67 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(77 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(76 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(81 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(88 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(86 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(92 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(96 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(102 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(109 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(116 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float
		],
		[
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(105 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(98 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(92 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(86 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(81 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(73 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(56 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(52 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(47 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(55 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(60 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(58 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(52 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(51 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(45 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(49 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(50 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(53 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(54 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(61 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(71 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(70 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(69 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(78 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(79 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(87 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(90 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(96 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(104 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(112 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float
		],
		[
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(103 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(96 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(90 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(86 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(78 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(70 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(51 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(42 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(47 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(48 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(55 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(54 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(54 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(53 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(42 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(35 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(28 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(33 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(38 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(37 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(44 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(47 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(49 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(54 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(63 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(68 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(78 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(82 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(89 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(94 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(99 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(104 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(109 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(114 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float
		]
	],
	[
		[
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(110 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(100 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(90 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(79 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(85 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(81 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(82 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(82 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(89 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(94 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(99 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(103 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(109 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(115 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float
		],
		[
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(105 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(97 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(85 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(72 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(74 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(70 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(70 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(70 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(76 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(85 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(91 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(93 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(97 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(103 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(109 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(115 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float
		],
		[
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(112 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(93 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(81 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(68 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(62 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(60 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(60 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(57 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(63 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(70 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(77 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(82 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(90 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(93 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(98 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(104 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(109 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(113 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float
		],
		[
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(113 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(100 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(93 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(84 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(63 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(58 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(48 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(53 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(54 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(52 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(52 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(57 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(64 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(66 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(76 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(83 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(81 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(85 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(85 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(90 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(95 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(98 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(101 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(103 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(106 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(108 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(111 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float
		],
		[
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(105 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(95 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(86 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(74 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(53 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(50 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(38 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(43 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(49 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(43 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(42 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(39 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(39 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(46 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(52 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(57 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(56 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(72 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(69 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(74 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(81 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(87 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(92 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(94 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(97 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(99 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(102 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(105 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(108 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float
		],
		[
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(108 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(99 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(90 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(76 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(66 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(45 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(43 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(41 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(44 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(47 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(43 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(47 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(40 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(30 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(31 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(31 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(39 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(33 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(40 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(41 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(43 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(53 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(59 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(70 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(73 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(77 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(79 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(82 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(84 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(87 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float
		]
	],
	[
		[
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(110 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(91 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(76 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(75 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(85 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(93 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(98 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(104 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(110 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float
		],
		[
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(110 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(91 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(70 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(70 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(75 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(86 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(89 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(94 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(98 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(101 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(106 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(110 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float
		],
		[
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(110 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(95 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(80 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(60 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(65 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(64 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(74 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(83 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(88 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(91 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(95 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(99 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(103 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(107 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(110 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float
		],
		[
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(110 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(95 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(80 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(58 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(55 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(49 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(66 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(68 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(71 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(78 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(78 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(80 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(88 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(85 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(89 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(97 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(100 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(105 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(110 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float
		],
		[
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(110 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(95 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(80 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(53 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(52 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(41 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(59 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(59 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(49 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(58 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(56 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(63 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(86 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(79 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(90 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(93 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(98 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(103 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(107 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(112 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float
		],
		[
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(110 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(97 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(91 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(73 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(45 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(40 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(33 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(53 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(61 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(49 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(54 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(50 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(50 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(60 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(52 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(67 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(74 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(81 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(92 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(96 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(100 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(105 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(110 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float
		]
	],
	[
		[
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(113 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(106 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(99 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(92 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(77 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(80 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(88 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(97 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(106 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(115 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float
		],
		[
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(116 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(109 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(102 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(95 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(89 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(74 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(72 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(88 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(87 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(95 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(102 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(109 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(116 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float
		],
		[
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(116 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(109 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(102 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(95 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(89 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(75 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(66 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(74 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(77 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(78 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(86 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(87 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(90 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(96 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(105 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(115 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float
		],
		[
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(115 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(108 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(101 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(94 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(88 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(66 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(56 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(61 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(70 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(65 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(78 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(72 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(83 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(84 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(93 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(98 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(105 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(110 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float
		],
		[
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(110 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(105 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(95 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(89 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(82 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(57 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(52 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(52 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(59 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(56 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(59 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(58 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(69 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(67 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(88 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(82 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(82 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(89 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(94 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(100 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(108 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float
		],
		[
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(110 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(101 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(96 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(90 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(83 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(77 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(54 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(43 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(38 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(50 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(48 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(52 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(48 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(42 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(42 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(51 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(52 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(53 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(59 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(65 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(71 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(78 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(85 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(95 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float
		]
	],
	[
		[
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(120 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(105 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(86 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(68 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(78 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(79 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(90 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(100 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(110 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float
		],
		[
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(120 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(105 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(86 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(66 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(73 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(77 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(88 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(96 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(105 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(115 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float
		],
		[
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(120 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(105 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(92 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(80 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(61 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(64 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(68 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(80 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(87 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(92 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(100 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(110 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float
		],
		[
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(120 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(104 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(91 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(79 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(52 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(60 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(54 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(64 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(69 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(77 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(80 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(82 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(84 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(85 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(87 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(88 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(90 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float
		],
		[
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(118 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(100 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(87 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(77 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(49 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(50 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(44 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(58 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(61 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(61 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(67 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(65 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(62 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(62 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(62 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(65 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(68 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float
		],
		[
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(115 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(98 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(84 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(62 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(49 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(44 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(38 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(46 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(49 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(49 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(46 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(39 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(37 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(39 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(40 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(42 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(43 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float
		]
	],
	[
		[
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(110 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(88 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(74 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(77 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(82 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(82 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(85 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(90 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(94 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(99 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(104 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float
		],
		[
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(110 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(88 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(66 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(70 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(81 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(80 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(81 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(84 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(88 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(91 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(93 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float
		],
		[
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(110 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(88 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(61 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(63 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(70 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(71 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(74 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(77 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(80 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(83 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(85 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float
		],
		[
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(110 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(86 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(62 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(63 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(62 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(62 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(58 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(52 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(50 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(50 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(52 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(54 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float
		],
		[
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(118 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(108 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(84 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(53 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(50 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(50 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(50 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(55 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(47 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(45 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(40 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(40 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(40 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float
		],
		[
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(118 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(100 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(73 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(43 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(37 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(42 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(43 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(53 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(38 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(37 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(35 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(35 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(38 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float
		]
	],
	[
		[
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(110 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(100 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(91 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(84 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(74 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(80 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(80 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(80 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(80 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(80 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float
		],
		[
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(110 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(100 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(91 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(84 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(74 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(68 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(68 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(68 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(68 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(68 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float
		],
		[
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(110 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(100 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(86 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(78 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(70 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(60 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(45 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(30 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(21 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float
		],
		[
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(110 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(100 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(87 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(78 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(67 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(48 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(38 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(29 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(21 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float
		],
		[
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(110 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(100 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(86 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(69 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(56 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(45 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(35 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(33 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(29 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float
		],
		[
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(110 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(100 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(83 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(71 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(48 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(27 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(38 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(37 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(34 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float,
			-(999 as ::core::ffi::c_int) as ::core::ffi::c_float
		]
	]
];
#[inline]
unsafe extern "C" fn unitnorm(mut x: ::core::ffi::c_float) -> ::core::ffi::c_float {
	let mut ix: C2RustUnnamed = C2RustUnnamed { i: 0 };
	ix.f = x;
	ix.i = ix.i & 0x80000000 as ogg_uint32_t | 0x3f800000 as ogg_uint32_t;
	return ix.f;
}
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
