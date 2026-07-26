extern "C" {
	fn malloc(__size: size_t) -> *mut ::core::ffi::c_void;
	fn calloc(__nmemb: size_t, __size: size_t) -> *mut ::core::ffi::c_void;
	fn free(__ptr: *mut ::core::ffi::c_void);
	fn memset(
		__s: *mut ::core::ffi::c_void,
		__c: ::core::ffi::c_int,
		__n: size_t
	) -> *mut ::core::ffi::c_void;
	fn atan(__x: ::core::ffi::c_double) -> ::core::ffi::c_double;
	fn floor(__x: ::core::ffi::c_double) -> ::core::ffi::c_double;
	fn oggpack_read(b: *mut oggpack_buffer, bits: ::core::ffi::c_int) -> ::core::ffi::c_long;
	fn vorbis_book_decodev_set(
		book: *mut dec_codebook,
		a: *mut ::core::ffi::c_float,
		b: *mut oggpack_buffer,
		n: ::core::ffi::c_int
	) -> ::core::ffi::c_long;
	fn _vorbis_block_alloc(
		vb: *mut vorbis_block,
		bytes: ::core::ffi::c_long
	) -> *mut ::core::ffi::c_void;
	fn ov_ilog(v: ogg_uint32_t) -> ::core::ffi::c_int;
	fn vorbis_lsp_to_curve(
		curve: *mut ::core::ffi::c_float,
		map: *mut ::core::ffi::c_int,
		n: ::core::ffi::c_int,
		ln: ::core::ffi::c_int,
		lsp: *mut ::core::ffi::c_float,
		m: ::core::ffi::c_int,
		amp: ::core::ffi::c_float,
		ampoffset: ::core::ffi::c_float
	);
}
use ogg_c2rust_sys::oggpack_buffer;

use super::{
	block::{alloc_chain, vorbis_block, vorbis_dsp_state},
	info::vorbis_info,
	registry::vorbis_func_floor
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
pub type vorbis_look_floor = ();
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
pub struct vorbis_info_floor0 {
	pub order: ::core::ffi::c_int,
	pub rate: ::core::ffi::c_long,
	pub barkmap: ::core::ffi::c_long,
	pub ampbits: ::core::ffi::c_int,
	pub ampdB: ::core::ffi::c_int,
	pub numbooks: ::core::ffi::c_int,
	pub books: [::core::ffi::c_int; 16],
	pub lessthan: ::core::ffi::c_float,
	pub greaterthan: ::core::ffi::c_float
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
pub struct highlevel_byblocktype {
	pub tone_mask_setting: ::core::ffi::c_double,
	pub tone_peaklimit_setting: ::core::ffi::c_double,
	pub noise_bias_setting: ::core::ffi::c_double,
	pub noise_compand_setting: ::core::ffi::c_double
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
pub struct vorbis_look_floor0 {
	pub ln: ::core::ffi::c_int,
	pub m: ::core::ffi::c_int,
	pub linearmap: *mut *mut ::core::ffi::c_int,
	pub n: [::core::ffi::c_int; 2],
	pub vi: *mut vorbis_info_floor0,
	pub bits: ::core::ffi::c_long,
	pub frames: ::core::ffi::c_long
}
unsafe extern "C" fn floor0_free_info(mut i: *mut ::core::ffi::c_void) {
	let mut info: *mut vorbis_info_floor0 = i as *mut vorbis_info_floor0;
	if !info.is_null() {
		memset(
			info as *mut ::core::ffi::c_void,
			0 as ::core::ffi::c_int,
			::core::mem::size_of::<vorbis_info_floor0>() as size_t
		);
		free(info as *mut ::core::ffi::c_void);
	}
}
unsafe extern "C" fn floor0_free_look(mut i: *mut ::core::ffi::c_void) {
	let mut look: *mut vorbis_look_floor0 = i as *mut vorbis_look_floor0;
	if !look.is_null() {
		if !(*look).linearmap.is_null() {
			if !(*(*look).linearmap.offset(0 as ::core::ffi::c_int as isize)).is_null() {
				free(*(*look).linearmap.offset(0 as ::core::ffi::c_int as isize)
					as *mut ::core::ffi::c_void);
			}
			if !(*(*look).linearmap.offset(1 as ::core::ffi::c_int as isize)).is_null() {
				free(*(*look).linearmap.offset(1 as ::core::ffi::c_int as isize)
					as *mut ::core::ffi::c_void);
			}
			free((*look).linearmap as *mut ::core::ffi::c_void);
		}
		memset(
			look as *mut ::core::ffi::c_void,
			0 as ::core::ffi::c_int,
			::core::mem::size_of::<vorbis_look_floor0>() as size_t
		);
		free(look as *mut ::core::ffi::c_void);
	}
}
unsafe extern "C" fn floor0_unpack(
	mut vi: *mut vorbis_info,
	mut opb: *mut oggpack_buffer
) -> *mut ::core::ffi::c_void {
	let mut current_block: u64;
	let mut ci: *mut codec_setup_info = (*vi).codec_setup as *mut codec_setup_info;
	let mut j: ::core::ffi::c_int = 0;
	let mut info: *mut vorbis_info_floor0 =
		malloc(::core::mem::size_of::<vorbis_info_floor0>() as size_t) as *mut vorbis_info_floor0;
	(*info).order = oggpack_read(opb, 8 as ::core::ffi::c_int) as ::core::ffi::c_int;
	(*info).rate = oggpack_read(opb, 16 as ::core::ffi::c_int);
	(*info).barkmap = oggpack_read(opb, 16 as ::core::ffi::c_int);
	(*info).ampbits = oggpack_read(opb, 6 as ::core::ffi::c_int) as ::core::ffi::c_int;
	(*info).ampdB = oggpack_read(opb, 8 as ::core::ffi::c_int) as ::core::ffi::c_int;
	(*info).numbooks = (oggpack_read(opb, 4 as ::core::ffi::c_int) + 1 as ::core::ffi::c_long)
		as ::core::ffi::c_int;
	if !((*info).order < 1 as ::core::ffi::c_int) {
		if !((*info).rate < 1 as ::core::ffi::c_long) {
			if !((*info).barkmap < 1 as ::core::ffi::c_long) {
				if !((*info).numbooks < 1 as ::core::ffi::c_int) {
					j = 0 as ::core::ffi::c_int;
					loop {
						if !(j < (*info).numbooks) {
							current_block = 8236137900636309791;
							break;
						}
						(*info).books[j as usize] =
							oggpack_read(opb, 8 as ::core::ffi::c_int) as ::core::ffi::c_int;
						if (*info).books[j as usize] < 0 as ::core::ffi::c_int
							|| (*info).books[j as usize] >= (*ci).books
						{
							current_block = 6554656252626946067;
							break;
						}
						if (*(*ci).decbooks.offset((*info).books[j as usize] as isize)).maptype
							as ::core::ffi::c_int == 0 as ::core::ffi::c_int
						{
							current_block = 6554656252626946067;
							break;
						}
						if ((*(*ci).decbooks.offset((*info).books[j as usize] as isize)).dim
							as ::core::ffi::c_int)
							< 1 as ::core::ffi::c_int
						{
							current_block = 6554656252626946067;
							break;
						}
						j += 1;
					}
					match current_block {
						6554656252626946067 => {}
						_ => return info as *mut ::core::ffi::c_void
					}
				}
			}
		}
	}
	floor0_free_info(info as *mut ::core::ffi::c_void);
	return ::core::ptr::null_mut::<::core::ffi::c_void>();
}
unsafe extern "C" fn floor0_map_lazy_init(
	mut vb: *mut vorbis_block,
	mut infoX: *mut ::core::ffi::c_void,
	mut look: *mut vorbis_look_floor0
) {
	if (*(*look).linearmap.offset((*vb).W as isize)).is_null() {
		let mut vd: *mut vorbis_dsp_state = (*vb).vd;
		let mut vi: *mut vorbis_info = (*vd).vi;
		let mut ci: *mut codec_setup_info = (*vi).codec_setup as *mut codec_setup_info;
		let mut info: *mut vorbis_info_floor0 = infoX as *mut vorbis_info_floor0;
		let mut W: ::core::ffi::c_int = (*vb).W as ::core::ffi::c_int;
		let mut n: ::core::ffi::c_int =
			((*ci).blocksizes[W as usize] / 2 as ::core::ffi::c_long) as ::core::ffi::c_int;
		let mut j: ::core::ffi::c_int = 0;
		let mut scale: ::core::ffi::c_float = ((*look).ln as ::core::ffi::c_double
			/ (13.1f64
				* atan(
					(0.00074f32 * ((*info).rate as ::core::ffi::c_float / 2.0f32))
						as ::core::ffi::c_double
				) + 2.24f64
				* atan(
					((*info).rate as ::core::ffi::c_float / 2.0f32
						* ((*info).rate as ::core::ffi::c_float / 2.0f32)
						* 1.85e-8f32) as ::core::ffi::c_double
				) + (1e-4f32 * ((*info).rate as ::core::ffi::c_float / 2.0f32))
				as ::core::ffi::c_double))
			as ::core::ffi::c_float;
		let ref mut fresh0 = *(*look).linearmap.offset(W as isize);
		*fresh0 = malloc(
			((n + 1 as ::core::ffi::c_int) as size_t)
				.wrapping_mul(::core::mem::size_of::<::core::ffi::c_int>() as size_t)
		) as *mut ::core::ffi::c_int;
		j = 0 as ::core::ffi::c_int;
		while j < n {
			let mut val: ::core::ffi::c_int = floor(
				(13.1f64
					* atan(
						(0.00074f32
							* ((*info).rate as ::core::ffi::c_float
								/ 2.0f32 / n as ::core::ffi::c_float
								* j as ::core::ffi::c_float)) as ::core::ffi::c_double
					) + 2.24f64
					* atan(
						((*info).rate as ::core::ffi::c_float / 2.0f32 / n as ::core::ffi::c_float
							* j as ::core::ffi::c_float
							* ((*info).rate as ::core::ffi::c_float
								/ 2.0f32 / n as ::core::ffi::c_float
								* j as ::core::ffi::c_float)
							* 1.85e-8f32) as ::core::ffi::c_double
					) + (1e-4f32
					* ((*info).rate as ::core::ffi::c_float / 2.0f32 / n as ::core::ffi::c_float
						* j as ::core::ffi::c_float)) as ::core::ffi::c_double)
					* scale as ::core::ffi::c_double
			) as ::core::ffi::c_int;
			if val >= (*look).ln {
				val = (*look).ln - 1 as ::core::ffi::c_int;
			}
			*(*(*look).linearmap.offset(W as isize)).offset(j as isize) = val;
			j += 1;
		}
		*(*(*look).linearmap.offset(W as isize)).offset(j as isize) = -(1 as ::core::ffi::c_int);
		(*look).n[W as usize] = n;
	}
}
unsafe extern "C" fn floor0_look(
	mut vd: *mut vorbis_dsp_state,
	mut i: *mut ::core::ffi::c_void
) -> *mut ::core::ffi::c_void {
	let mut info: *mut vorbis_info_floor0 = i as *mut vorbis_info_floor0;
	let mut look: *mut vorbis_look_floor0 = calloc(
		1 as size_t,
		::core::mem::size_of::<vorbis_look_floor0>() as size_t
	) as *mut vorbis_look_floor0;
	(*look).m = (*info).order;
	(*look).ln = (*info).barkmap as ::core::ffi::c_int;
	(*look).vi = info;
	(*look).linearmap = calloc(
		2 as size_t,
		::core::mem::size_of::<*mut ::core::ffi::c_int>() as size_t
	) as *mut *mut ::core::ffi::c_int;
	return look as *mut ::core::ffi::c_void;
}
unsafe extern "C" fn floor0_inverse1(
	mut vb: *mut vorbis_block,
	mut i: *mut ::core::ffi::c_void
) -> *mut ::core::ffi::c_void {
	let mut look: *mut vorbis_look_floor0 = i as *mut vorbis_look_floor0;
	let mut info: *mut vorbis_info_floor0 = (*look).vi;
	let mut j: ::core::ffi::c_int = 0;
	let mut k: ::core::ffi::c_int = 0;
	let mut ampraw: ::core::ffi::c_int =
		oggpack_read(&raw mut (*vb).opb, (*info).ampbits) as ::core::ffi::c_int;
	if ampraw > 0 as ::core::ffi::c_int {
		let mut maxval: ::core::ffi::c_long = (((1 as ::core::ffi::c_int) << (*info).ampbits)
			- 1 as ::core::ffi::c_int) as ::core::ffi::c_long;
		let mut amp: ::core::ffi::c_float = ampraw as ::core::ffi::c_float
			/ maxval as ::core::ffi::c_float
			* (*info).ampdB as ::core::ffi::c_float;
		let mut booknum: ::core::ffi::c_int = oggpack_read(
			&raw mut (*vb).opb,
			ov_ilog((*info).numbooks as ogg_uint32_t)
		) as ::core::ffi::c_int;
		if booknum != -(1 as ::core::ffi::c_int) && booknum < (*info).numbooks {
			let mut ci: *mut codec_setup_info =
				(*(*(*vb).vd).vi).codec_setup as *mut codec_setup_info;
			let mut b: *mut dec_codebook = (*ci)
				.decbooks
				.offset((*info).books[booknum as usize] as isize);
			let mut last: ::core::ffi::c_float = 0.0f32;
			let mut lsp: *mut ::core::ffi::c_float = _vorbis_block_alloc(
				vb,
				(::core::mem::size_of::<::core::ffi::c_float>() as usize).wrapping_mul(
					((*look).m + (*b).dim as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as usize
				) as ::core::ffi::c_long
			) as *mut ::core::ffi::c_float;
			if !(vorbis_book_decodev_set(b, lsp, &raw mut (*vb).opb, (*look).m)
				== -(1 as ::core::ffi::c_int) as ::core::ffi::c_long)
			{
				j = 0 as ::core::ffi::c_int;
				while j < (*look).m {
					k = 0 as ::core::ffi::c_int;
					while j < (*look).m && k < (*b).dim as ::core::ffi::c_int {
						*lsp.offset(j as isize) += last;
						k += 1;
						j += 1;
					}
					last = *lsp.offset((j - 1 as ::core::ffi::c_int) as isize);
				}
				*lsp.offset((*look).m as isize) = amp;
				return lsp as *mut ::core::ffi::c_void;
			}
		}
	}
	return ::core::ptr::null_mut::<::core::ffi::c_void>();
}
unsafe extern "C" fn floor0_inverse2(
	mut vb: *mut vorbis_block,
	mut i: *mut ::core::ffi::c_void,
	mut memo: *mut ::core::ffi::c_void,
	mut out: *mut ::core::ffi::c_float
) -> ::core::ffi::c_int {
	let mut look: *mut vorbis_look_floor0 = i as *mut vorbis_look_floor0;
	let mut info: *mut vorbis_info_floor0 = (*look).vi;
	floor0_map_lazy_init(vb, info as *mut ::core::ffi::c_void, look);
	if !memo.is_null() {
		let mut lsp: *mut ::core::ffi::c_float = memo as *mut ::core::ffi::c_float;
		let mut amp: ::core::ffi::c_float = *lsp.offset((*look).m as isize);
		vorbis_lsp_to_curve(
			out,
			*(*look).linearmap.offset((*vb).W as isize),
			(*look).n[(*vb).W as usize],
			(*look).ln,
			lsp,
			(*look).m,
			amp,
			(*info).ampdB as ::core::ffi::c_float
		);
		return 1 as ::core::ffi::c_int;
	}
	memset(
		out as *mut ::core::ffi::c_void,
		0 as ::core::ffi::c_int,
		(::core::mem::size_of::<::core::ffi::c_float>() as size_t)
			.wrapping_mul((*look).n[(*vb).W as usize] as size_t)
	);
	return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub static mut floor0_exportbundle: vorbis_func_floor = vorbis_func_floor {
	pack: None,
	unpack: Some(
		floor0_unpack
			as unsafe extern "C" fn(
				*mut vorbis_info,
				*mut oggpack_buffer
			) -> *mut ::core::ffi::c_void
	),
	look: Some(
		floor0_look
			as unsafe extern "C" fn(
				*mut vorbis_dsp_state,
				*mut ::core::ffi::c_void
			) -> *mut ::core::ffi::c_void
	),
	free_info: Some(floor0_free_info as unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()),
	free_look: Some(floor0_free_look as unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()),
	inverse1: Some(
		floor0_inverse1
			as unsafe extern "C" fn(
				*mut vorbis_block,
				*mut ::core::ffi::c_void
			) -> *mut ::core::ffi::c_void
	),
	inverse2: Some(
		floor0_inverse2
			as unsafe extern "C" fn(
				*mut vorbis_block,
				*mut ::core::ffi::c_void,
				*mut ::core::ffi::c_void,
				*mut ::core::ffi::c_float
			) -> ::core::ffi::c_int
	)
};
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
