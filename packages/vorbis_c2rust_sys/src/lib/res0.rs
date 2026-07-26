extern "C" {
	fn malloc(__size: size_t) -> *mut ::core::ffi::c_void;
	fn calloc(__nmemb: size_t, __size: size_t) -> *mut ::core::ffi::c_void;
	fn free(__ptr: *mut ::core::ffi::c_void);
	fn abs(__x: ::core::ffi::c_int) -> ::core::ffi::c_int;
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
	fn oggpack_write(b: *mut oggpack_buffer, value: ::core::ffi::c_ulong, bits: ::core::ffi::c_int);
	fn oggpack_read(b: *mut oggpack_buffer, bits: ::core::ffi::c_int) -> ::core::ffi::c_long;
	fn vorbis_book_encode(
		book: *mut codebook,
		a: ::core::ffi::c_int,
		b: *mut oggpack_buffer
	) -> ::core::ffi::c_int;
	fn vorbis_book_decode(book: *mut dec_codebook, b: *mut oggpack_buffer) -> ::core::ffi::c_long;
	fn vorbis_book_decodevs_add(
		book: *mut dec_codebook,
		a: *mut ::core::ffi::c_float,
		b: *mut oggpack_buffer,
		n: ::core::ffi::c_int
	) -> ::core::ffi::c_long;
	fn vorbis_book_decodev_add(
		book: *mut dec_codebook,
		a: *mut ::core::ffi::c_float,
		b: *mut oggpack_buffer,
		n: ::core::ffi::c_int
	) -> ::core::ffi::c_long;
	fn vorbis_book_decodevv_add(
		book: *mut dec_codebook,
		a: *mut *mut ::core::ffi::c_float,
		off: ::core::ffi::c_long,
		ch: ::core::ffi::c_int,
		b: *mut oggpack_buffer,
		n: ::core::ffi::c_int
	) -> ::core::ffi::c_long;
	fn _vorbis_block_alloc(
		vb: *mut vorbis_block,
		bytes: ::core::ffi::c_long
	) -> *mut ::core::ffi::c_void;
	fn ov_ilog(v: ogg_uint32_t) -> ::core::ffi::c_int;
}
use ogg_c2rust_sys::oggpack_buffer;

use super::{
	block::{alloc_chain, vorbis_block, vorbis_dsp_state},
	info::vorbis_info,
	registry::vorbis_func_residue
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
pub type vorbis_look_residue = ();
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
pub struct vorbis_info_residue0 {
	pub begin: ::core::ffi::c_long,
	pub end: ::core::ffi::c_long,
	pub grouping: ::core::ffi::c_int,
	pub partitions: ::core::ffi::c_int,
	pub partvals: ::core::ffi::c_int,
	pub groupbook: ::core::ffi::c_int,
	pub secondstages: [::core::ffi::c_int; 64],
	pub booklist: [::core::ffi::c_int; 512],
	pub classmetric1: [::core::ffi::c_int; 64],
	pub classmetric2: [::core::ffi::c_int; 64]
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
pub struct vorbis_look_residue0 {
	pub info: *mut vorbis_info_residue0,
	pub parts: ::core::ffi::c_int,
	pub stages: ::core::ffi::c_int,
	pub fullbooks: *mut codebook,
	pub decbooks: *mut dec_codebook,
	pub phrasebook: ::core::ffi::c_int,
	pub partbooks: *mut *mut ::core::ffi::c_int,
	pub partvals: ::core::ffi::c_int,
	pub decodemap: *mut *mut ::core::ffi::c_int,
	pub postbits: ::core::ffi::c_long,
	pub phrasebits: ::core::ffi::c_long,
	pub frames: ::core::ffi::c_long
}
#[no_mangle]
pub unsafe extern "C" fn res0_free_info(mut i: *mut ::core::ffi::c_void) {
	let mut info: *mut vorbis_info_residue0 = i as *mut vorbis_info_residue0;
	if !info.is_null() {
		memset(
			info as *mut ::core::ffi::c_void,
			0 as ::core::ffi::c_int,
			::core::mem::size_of::<vorbis_info_residue0>() as size_t
		);
		free(info as *mut ::core::ffi::c_void);
	}
}
#[no_mangle]
pub unsafe extern "C" fn res0_free_look(mut i: *mut ::core::ffi::c_void) {
	let mut j: ::core::ffi::c_int = 0;
	if !i.is_null() {
		let mut look: *mut vorbis_look_residue0 = i as *mut vorbis_look_residue0;
		j = 0 as ::core::ffi::c_int;
		while j < (*look).parts {
			if !(*(*look).partbooks.offset(j as isize)).is_null() {
				free(*(*look).partbooks.offset(j as isize) as *mut ::core::ffi::c_void);
			}
			j += 1;
		}
		free((*look).partbooks as *mut ::core::ffi::c_void);
		j = 0 as ::core::ffi::c_int;
		while j < (*look).partvals {
			free(*(*look).decodemap.offset(j as isize) as *mut ::core::ffi::c_void);
			j += 1;
		}
		free((*look).decodemap as *mut ::core::ffi::c_void);
		memset(
			look as *mut ::core::ffi::c_void,
			0 as ::core::ffi::c_int,
			::core::mem::size_of::<vorbis_look_residue0>() as size_t
		);
		free(look as *mut ::core::ffi::c_void);
	}
}
unsafe extern "C" fn icount(mut v: ::core::ffi::c_uint) -> ::core::ffi::c_int {
	let mut ret: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
	while v != 0 {
		ret = (ret as ::core::ffi::c_uint).wrapping_add(v & 1 as ::core::ffi::c_uint)
			as ::core::ffi::c_int as ::core::ffi::c_int;
		v >>= 1 as ::core::ffi::c_int;
	}
	return ret;
}
#[no_mangle]
pub unsafe extern "C" fn res0_pack(mut vr: *mut ::core::ffi::c_void, mut opb: *mut oggpack_buffer) {
	let mut info: *mut vorbis_info_residue0 = vr as *mut vorbis_info_residue0;
	let mut j: ::core::ffi::c_int = 0;
	let mut acc: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
	oggpack_write(
		opb,
		(*info).begin as ::core::ffi::c_ulong,
		24 as ::core::ffi::c_int
	);
	oggpack_write(
		opb,
		(*info).end as ::core::ffi::c_ulong,
		24 as ::core::ffi::c_int
	);
	oggpack_write(
		opb,
		((*info).grouping - 1 as ::core::ffi::c_int) as ::core::ffi::c_ulong,
		24 as ::core::ffi::c_int
	);
	oggpack_write(
		opb,
		((*info).partitions - 1 as ::core::ffi::c_int) as ::core::ffi::c_ulong,
		6 as ::core::ffi::c_int
	);
	oggpack_write(
		opb,
		(*info).groupbook as ::core::ffi::c_ulong,
		8 as ::core::ffi::c_int
	);
	j = 0 as ::core::ffi::c_int;
	while j < (*info).partitions {
		if ov_ilog((*info).secondstages[j as usize] as ogg_uint32_t) > 3 as ::core::ffi::c_int {
			oggpack_write(
				opb,
				(*info).secondstages[j as usize] as ::core::ffi::c_ulong,
				3 as ::core::ffi::c_int
			);
			oggpack_write(opb, 1 as ::core::ffi::c_ulong, 1 as ::core::ffi::c_int);
			oggpack_write(
				opb,
				((*info).secondstages[j as usize] >> 3 as ::core::ffi::c_int)
					as ::core::ffi::c_ulong,
				5 as ::core::ffi::c_int
			);
		} else {
			oggpack_write(
				opb,
				(*info).secondstages[j as usize] as ::core::ffi::c_ulong,
				4 as ::core::ffi::c_int
			);
		}
		acc += icount((*info).secondstages[j as usize] as ::core::ffi::c_uint);
		j += 1;
	}
	j = 0 as ::core::ffi::c_int;
	while j < acc {
		oggpack_write(
			opb,
			(*info).booklist[j as usize] as ::core::ffi::c_ulong,
			8 as ::core::ffi::c_int
		);
		j += 1;
	}
}
#[no_mangle]
pub unsafe extern "C" fn res0_unpack(
	mut vi: *mut vorbis_info,
	mut opb: *mut oggpack_buffer
) -> *mut ::core::ffi::c_void {
	let mut current_block: u64;
	let mut j: ::core::ffi::c_int = 0;
	let mut acc: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
	let mut info: *mut vorbis_info_residue0 = calloc(
		1 as size_t,
		::core::mem::size_of::<vorbis_info_residue0>() as size_t
	) as *mut vorbis_info_residue0;
	let mut ci: *mut codec_setup_info = (*vi).codec_setup as *mut codec_setup_info;
	(*info).begin = oggpack_read(opb, 24 as ::core::ffi::c_int);
	(*info).end = oggpack_read(opb, 24 as ::core::ffi::c_int);
	(*info).grouping = (oggpack_read(opb, 24 as ::core::ffi::c_int) + 1 as ::core::ffi::c_long)
		as ::core::ffi::c_int;
	(*info).partitions = (oggpack_read(opb, 6 as ::core::ffi::c_int) + 1 as ::core::ffi::c_long)
		as ::core::ffi::c_int;
	(*info).groupbook = oggpack_read(opb, 8 as ::core::ffi::c_int) as ::core::ffi::c_int;
	if !((*info).groupbook < 0 as ::core::ffi::c_int) {
		j = 0 as ::core::ffi::c_int;
		loop {
			if !(j < (*info).partitions) {
				current_block = 12599329904712511516;
				break;
			}
			let mut cascade: ::core::ffi::c_int =
				oggpack_read(opb, 3 as ::core::ffi::c_int) as ::core::ffi::c_int;
			let mut cflag: ::core::ffi::c_int =
				oggpack_read(opb, 1 as ::core::ffi::c_int) as ::core::ffi::c_int;
			if cflag < 0 as ::core::ffi::c_int {
				current_block = 10546635418203619763;
				break;
			}
			if cflag != 0 {
				let mut c: ::core::ffi::c_int =
					oggpack_read(opb, 5 as ::core::ffi::c_int) as ::core::ffi::c_int;
				if c < 0 as ::core::ffi::c_int {
					current_block = 10546635418203619763;
					break;
				}
				cascade |= c << 3 as ::core::ffi::c_int;
			}
			(*info).secondstages[j as usize] = cascade;
			acc += icount(cascade as ::core::ffi::c_uint);
			j += 1;
		}
		match current_block {
			10546635418203619763 => {}
			_ => {
				j = 0 as ::core::ffi::c_int;
				loop {
					if !(j < acc) {
						current_block = 15904375183555213903;
						break;
					}
					let mut book: ::core::ffi::c_int =
						oggpack_read(opb, 8 as ::core::ffi::c_int) as ::core::ffi::c_int;
					if book < 0 as ::core::ffi::c_int {
						current_block = 10546635418203619763;
						break;
					}
					(*info).booklist[j as usize] = book;
					j += 1;
				}
				match current_block {
					10546635418203619763 => {}
					_ => {
						if !((*info).groupbook >= (*ci).books) {
							j = 0 as ::core::ffi::c_int;
							loop {
								if !(j < acc) {
									current_block = 13472856163611868459;
									break;
								}
								if (*info).booklist[j as usize] >= (*ci).books {
									current_block = 10546635418203619763;
									break;
								}
								if (*(*ci).decbooks.offset((*info).booklist[j as usize] as isize))
									.maptype as ::core::ffi::c_int
									== 0 as ::core::ffi::c_int
								{
									current_block = 10546635418203619763;
									break;
								}
								j += 1;
							}
							match current_block {
								10546635418203619763 => {}
								_ => {
									let mut entries: ::core::ffi::c_int = (*(*ci)
										.decbooks
										.offset((*info).groupbook as isize))
									.entries
										as ::core::ffi::c_int;
									let mut dim: ::core::ffi::c_int =
										(*(*ci).decbooks.offset((*info).groupbook as isize)).dim
											as ::core::ffi::c_int;
									let mut partvals: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
									if !(dim < 1 as ::core::ffi::c_int) {
										loop {
											if !(dim > 0 as ::core::ffi::c_int) {
												current_block = 17788412896529399552;
												break;
											}
											partvals *= (*info).partitions;
											if partvals > entries {
												current_block = 10546635418203619763;
												break;
											}
											dim -= 1;
										}
										match current_block {
											10546635418203619763 => {}
											_ => {
												(*info).partvals = partvals;
												return info as *mut ::core::ffi::c_void;
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
	res0_free_info(info as *mut ::core::ffi::c_void);
	return ::core::ptr::null_mut::<::core::ffi::c_void>();
}
#[no_mangle]
pub unsafe extern "C" fn res0_look(
	mut vd: *mut vorbis_dsp_state,
	mut vr: *mut ::core::ffi::c_void
) -> *mut ::core::ffi::c_void {
	let mut info: *mut vorbis_info_residue0 = vr as *mut vorbis_info_residue0;
	let mut look: *mut vorbis_look_residue0 = calloc(
		1 as size_t,
		::core::mem::size_of::<vorbis_look_residue0>() as size_t
	) as *mut vorbis_look_residue0;
	let mut ci: *mut codec_setup_info = (*(*vd).vi).codec_setup as *mut codec_setup_info;
	let mut j: ::core::ffi::c_int = 0;
	let mut k: ::core::ffi::c_int = 0;
	let mut acc: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
	let mut dim: ::core::ffi::c_int = 0;
	let mut maxstage: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
	(*look).info = info;
	(*look).parts = (*info).partitions;
	(*look).fullbooks = (*ci).fullbooks;
	(*look).decbooks = (*ci).decbooks;
	(*look).phrasebook = (*info).groupbook;
	dim = (if !(*look).fullbooks.is_null() {
		(*(*look).fullbooks.offset((*look).phrasebook as isize)).dim
	} else {
		(*(*look).decbooks.offset((*look).phrasebook as isize)).dim as ::core::ffi::c_long
	}) as ::core::ffi::c_int;
	(*look).partbooks = calloc(
		(*look).parts as size_t,
		::core::mem::size_of::<*mut ::core::ffi::c_int>() as size_t
	) as *mut *mut ::core::ffi::c_int;
	j = 0 as ::core::ffi::c_int;
	while j < (*look).parts {
		let mut stages: ::core::ffi::c_int =
			ov_ilog((*info).secondstages[j as usize] as ogg_uint32_t);
		if stages != 0 {
			if stages > maxstage {
				maxstage = stages;
			}
			let ref mut fresh0 = *(*look).partbooks.offset(j as isize);
			*fresh0 = calloc(
				stages as size_t,
				::core::mem::size_of::<::core::ffi::c_int>() as size_t
			) as *mut ::core::ffi::c_int;
			k = 0 as ::core::ffi::c_int;
			while k < stages {
				if (*info).secondstages[j as usize] & (1 as ::core::ffi::c_int) << k != 0 {
					let fresh1 = acc;
					acc = acc + 1;
					*(*(*look).partbooks.offset(j as isize)).offset(k as isize) =
						(*info).booklist[fresh1 as usize];
				} else {
					*(*(*look).partbooks.offset(j as isize)).offset(k as isize) =
						-(1 as ::core::ffi::c_int);
				}
				k += 1;
			}
		}
		j += 1;
	}
	(*look).partvals = 1 as ::core::ffi::c_int;
	j = 0 as ::core::ffi::c_int;
	while j < dim {
		(*look).partvals *= (*look).parts;
		j += 1;
	}
	(*look).stages = maxstage;
	(*look).decodemap = malloc(
		((*look).partvals as size_t)
			.wrapping_mul(::core::mem::size_of::<*mut ::core::ffi::c_int>() as size_t)
	) as *mut *mut ::core::ffi::c_int;
	j = 0 as ::core::ffi::c_int;
	while j < (*look).partvals {
		let mut val: ::core::ffi::c_long = j as ::core::ffi::c_long;
		let mut mult: ::core::ffi::c_long =
			((*look).partvals / (*look).parts) as ::core::ffi::c_long;
		let ref mut fresh2 = *(*look).decodemap.offset(j as isize);
		*fresh2 = malloc(
			(dim as size_t).wrapping_mul(::core::mem::size_of::<::core::ffi::c_int>() as size_t)
		) as *mut ::core::ffi::c_int;
		k = 0 as ::core::ffi::c_int;
		while k < dim {
			let mut deco: ::core::ffi::c_long = val / mult;
			val -= deco * mult;
			mult /= (*look).parts as ::core::ffi::c_long;
			*(*(*look).decodemap.offset(j as isize)).offset(k as isize) =
				deco as ::core::ffi::c_int;
			k += 1;
		}
		j += 1;
	}
	return look as *mut ::core::ffi::c_void;
}
unsafe extern "C" fn local_book_besterror(
	mut book: *mut codebook,
	mut a: *mut ::core::ffi::c_int
) -> ::core::ffi::c_int {
	let mut dim: ::core::ffi::c_int = (*book).dim as ::core::ffi::c_int;
	let mut i: ::core::ffi::c_int = 0;
	let mut j: ::core::ffi::c_int = 0;
	let mut o: ::core::ffi::c_int = 0;
	let mut minval: ::core::ffi::c_int = (*book).minval;
	let mut del: ::core::ffi::c_int = (*book).delta;
	let mut qv: ::core::ffi::c_int = (*book).quantvals;
	let mut ze: ::core::ffi::c_int = qv >> 1 as ::core::ffi::c_int;
	let mut index: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
	let mut p: [::core::ffi::c_int; 8] = [
		0 as ::core::ffi::c_int,
		0 as ::core::ffi::c_int,
		0 as ::core::ffi::c_int,
		0 as ::core::ffi::c_int,
		0 as ::core::ffi::c_int,
		0 as ::core::ffi::c_int,
		0 as ::core::ffi::c_int,
		0 as ::core::ffi::c_int
	];
	if del != 1 as ::core::ffi::c_int {
		i = 0 as ::core::ffi::c_int;
		o = dim;
		while i < dim {
			o -= 1;
			let mut v: ::core::ffi::c_int =
				(*a.offset(o as isize) - minval + (del >> 1 as ::core::ffi::c_int)) / del;
			let mut m: ::core::ffi::c_int = if v < ze {
				(ze - v << 1 as ::core::ffi::c_int) - 1 as ::core::ffi::c_int
			} else {
				v - ze << 1 as ::core::ffi::c_int
			};
			index = index * qv
				+ (if m < 0 as ::core::ffi::c_int {
					0 as ::core::ffi::c_int
				} else {
					(if m >= qv {
						qv - 1 as ::core::ffi::c_int
					} else {
						m
					})
				});
			p[o as usize] = v * del + minval;
			i += 1;
		}
	} else {
		i = 0 as ::core::ffi::c_int;
		o = dim;
		while i < dim {
			o -= 1;
			let mut v_0: ::core::ffi::c_int = *a.offset(o as isize) - minval;
			let mut m_0: ::core::ffi::c_int = if v_0 < ze {
				(ze - v_0 << 1 as ::core::ffi::c_int) - 1 as ::core::ffi::c_int
			} else {
				v_0 - ze << 1 as ::core::ffi::c_int
			};
			index = index * qv
				+ (if m_0 < 0 as ::core::ffi::c_int {
					0 as ::core::ffi::c_int
				} else {
					(if m_0 >= qv {
						qv - 1 as ::core::ffi::c_int
					} else {
						m_0
					})
				});
			p[o as usize] = v_0 * del + minval;
			i += 1;
		}
	}
	if *(*(*book).c).lengthlist.offset(index as isize) as ::core::ffi::c_int
		<= 0 as ::core::ffi::c_int
	{
		let mut c: *const static_codebook = (*book).c;
		let mut best: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
		let mut e: [::core::ffi::c_int; 8] = [
			0 as ::core::ffi::c_int,
			0 as ::core::ffi::c_int,
			0 as ::core::ffi::c_int,
			0 as ::core::ffi::c_int,
			0 as ::core::ffi::c_int,
			0 as ::core::ffi::c_int,
			0 as ::core::ffi::c_int,
			0 as ::core::ffi::c_int
		];
		let mut maxval: ::core::ffi::c_int =
			(*book).minval + (*book).delta * ((*book).quantvals - 1 as ::core::ffi::c_int);
		i = 0 as ::core::ffi::c_int;
		while (i as ::core::ffi::c_long) < (*book).entries {
			if *(*c).lengthlist.offset(i as isize) as ::core::ffi::c_int > 0 as ::core::ffi::c_int {
				let mut this: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
				j = 0 as ::core::ffi::c_int;
				while j < dim {
					let mut val: ::core::ffi::c_int = e[j as usize] - *a.offset(j as isize);
					this += val * val;
					j += 1;
				}
				if best == -(1 as ::core::ffi::c_int) || this < best {
					memcpy(
						&raw mut p as *mut ::core::ffi::c_int as *mut ::core::ffi::c_void,
						&raw mut e as *mut ::core::ffi::c_int as *const ::core::ffi::c_void,
						::core::mem::size_of::<[::core::ffi::c_int; 8]>() as size_t
					);
					best = this;
					index = i;
				}
			}
			j = 0 as ::core::ffi::c_int;
			while e[j as usize] >= maxval {
				let fresh3 = j;
				j = j + 1;
				e[fresh3 as usize] = 0 as ::core::ffi::c_int;
			}
			if e[j as usize] >= 0 as ::core::ffi::c_int {
				e[j as usize] += (*book).delta;
			}
			e[j as usize] = -e[j as usize];
			i += 1;
		}
	}
	if index > -(1 as ::core::ffi::c_int) {
		i = 0 as ::core::ffi::c_int;
		while i < dim {
			let fresh4 = a;
			a = a.offset(1);
			*fresh4 -= p[i as usize];
			i += 1;
		}
	}
	return index;
}
unsafe extern "C" fn _encodepart(
	mut opb: *mut oggpack_buffer,
	mut vec: *mut ::core::ffi::c_int,
	mut n: ::core::ffi::c_int,
	mut book: *mut codebook
) -> ::core::ffi::c_int {
	let mut i: ::core::ffi::c_int = 0;
	let mut bits: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
	let mut dim: ::core::ffi::c_int = (*book).dim as ::core::ffi::c_int;
	let mut step: ::core::ffi::c_int = n / dim;
	i = 0 as ::core::ffi::c_int;
	while i < step {
		let mut entry: ::core::ffi::c_int =
			local_book_besterror(book, vec.offset((i * dim) as isize));
		bits += vorbis_book_encode(book, entry, opb);
		i += 1;
	}
	return bits;
}
unsafe extern "C" fn _01class(
	mut vb: *mut vorbis_block,
	mut vl: *mut ::core::ffi::c_void,
	mut in_0: *mut *mut ::core::ffi::c_int,
	mut ch: ::core::ffi::c_int
) -> *mut *mut ::core::ffi::c_long {
	let mut i: ::core::ffi::c_long = 0;
	let mut j: ::core::ffi::c_long = 0;
	let mut k: ::core::ffi::c_long = 0;
	let mut look: *mut vorbis_look_residue0 = vl as *mut vorbis_look_residue0;
	let mut info: *mut vorbis_info_residue0 = (*look).info;
	let mut samples_per_partition: ::core::ffi::c_int = (*info).grouping;
	let mut possible_partitions: ::core::ffi::c_int = (*info).partitions;
	let mut n: ::core::ffi::c_int = ((*info).end - (*info).begin) as ::core::ffi::c_int;
	let mut partvals: ::core::ffi::c_int = n / samples_per_partition;
	let mut partword: *mut *mut ::core::ffi::c_long = _vorbis_block_alloc(
		vb,
		(ch as usize).wrapping_mul(::core::mem::size_of::<*mut ::core::ffi::c_long>() as usize)
			as ::core::ffi::c_long
	) as *mut *mut ::core::ffi::c_long;
	let mut scale: ::core::ffi::c_float =
		(100.0f64 / samples_per_partition as ::core::ffi::c_double) as ::core::ffi::c_float;
	i = 0 as ::core::ffi::c_long;
	while i < ch as ::core::ffi::c_long {
		let ref mut fresh5 = *partword.offset(i as isize);
		*fresh5 = _vorbis_block_alloc(
			vb,
			((n / samples_per_partition) as usize)
				.wrapping_mul(::core::mem::size_of::<::core::ffi::c_long>() as usize)
				as ::core::ffi::c_long
		) as *mut ::core::ffi::c_long;
		memset(
			*partword.offset(i as isize) as *mut ::core::ffi::c_void,
			0 as ::core::ffi::c_int,
			((n / samples_per_partition) as size_t)
				.wrapping_mul(::core::mem::size_of::<::core::ffi::c_long>() as size_t)
		);
		i += 1;
	}
	i = 0 as ::core::ffi::c_long;
	while i < partvals as ::core::ffi::c_long {
		let mut offset: ::core::ffi::c_int = (i * samples_per_partition as ::core::ffi::c_long
			+ (*info).begin) as ::core::ffi::c_int;
		j = 0 as ::core::ffi::c_long;
		while j < ch as ::core::ffi::c_long {
			let mut max: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
			let mut ent: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
			k = 0 as ::core::ffi::c_long;
			while k < samples_per_partition as ::core::ffi::c_long {
				if abs(*(*in_0.offset(j as isize))
					.offset((offset as ::core::ffi::c_long + k) as isize))
					> max
				{
					max = abs(*(*in_0.offset(j as isize))
						.offset((offset as ::core::ffi::c_long + k) as isize));
				}
				ent += abs(*(*in_0.offset(j as isize))
					.offset((offset as ::core::ffi::c_long + k) as isize));
				k += 1;
			}
			ent = (ent as ::core::ffi::c_float * scale) as ::core::ffi::c_int;
			k = 0 as ::core::ffi::c_long;
			while k < (possible_partitions - 1 as ::core::ffi::c_int) as ::core::ffi::c_long {
				if max <= (*info).classmetric1[k as usize]
					&& ((*info).classmetric2[k as usize] < 0 as ::core::ffi::c_int
						|| ent < (*info).classmetric2[k as usize])
				{
					break;
				}
				k += 1;
			}
			*(*partword.offset(j as isize)).offset(i as isize) = k;
			j += 1;
		}
		i += 1;
	}
	(*look).frames += 1;
	return partword;
}
unsafe extern "C" fn _2class(
	mut vb: *mut vorbis_block,
	mut vl: *mut ::core::ffi::c_void,
	mut in_0: *mut *mut ::core::ffi::c_int,
	mut ch: ::core::ffi::c_int
) -> *mut *mut ::core::ffi::c_long {
	let mut i: ::core::ffi::c_long = 0;
	let mut j: ::core::ffi::c_long = 0;
	let mut k: ::core::ffi::c_long = 0;
	let mut l: ::core::ffi::c_long = 0;
	let mut look: *mut vorbis_look_residue0 = vl as *mut vorbis_look_residue0;
	let mut info: *mut vorbis_info_residue0 = (*look).info;
	let mut samples_per_partition: ::core::ffi::c_int = (*info).grouping;
	let mut possible_partitions: ::core::ffi::c_int = (*info).partitions;
	let mut n: ::core::ffi::c_int = ((*info).end - (*info).begin) as ::core::ffi::c_int;
	let mut partvals: ::core::ffi::c_int = n / samples_per_partition;
	let mut partword: *mut *mut ::core::ffi::c_long = _vorbis_block_alloc(
		vb,
		::core::mem::size_of::<*mut ::core::ffi::c_long>() as ::core::ffi::c_long
	) as *mut *mut ::core::ffi::c_long;
	let ref mut fresh6 = *partword.offset(0 as ::core::ffi::c_int as isize);
	*fresh6 = _vorbis_block_alloc(
		vb,
		(partvals as usize).wrapping_mul(::core::mem::size_of::<::core::ffi::c_long>() as usize)
			as ::core::ffi::c_long
	) as *mut ::core::ffi::c_long;
	memset(
		*partword.offset(0 as ::core::ffi::c_int as isize) as *mut ::core::ffi::c_void,
		0 as ::core::ffi::c_int,
		(partvals as size_t).wrapping_mul(::core::mem::size_of::<::core::ffi::c_long>() as size_t)
	);
	i = 0 as ::core::ffi::c_long;
	l = (*info).begin / ch as ::core::ffi::c_long;
	while i < partvals as ::core::ffi::c_long {
		let mut magmax: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
		let mut angmax: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
		j = 0 as ::core::ffi::c_long;
		while j < samples_per_partition as ::core::ffi::c_long {
			if abs(*(*in_0.offset(0 as ::core::ffi::c_int as isize)).offset(l as isize)) > magmax {
				magmax = abs(*(*in_0.offset(0 as ::core::ffi::c_int as isize)).offset(l as isize));
			}
			k = 1 as ::core::ffi::c_long;
			while k < ch as ::core::ffi::c_long {
				if abs(*(*in_0.offset(k as isize)).offset(l as isize)) > angmax {
					angmax = abs(*(*in_0.offset(k as isize)).offset(l as isize));
				}
				k += 1;
			}
			l += 1;
			j += ch as ::core::ffi::c_long;
		}
		j = 0 as ::core::ffi::c_long;
		while j < (possible_partitions - 1 as ::core::ffi::c_int) as ::core::ffi::c_long {
			if magmax <= (*info).classmetric1[j as usize]
				&& angmax <= (*info).classmetric2[j as usize]
			{
				break;
			}
			j += 1;
		}
		*(*partword.offset(0 as ::core::ffi::c_int as isize)).offset(i as isize) = j;
		i += 1;
	}
	(*look).frames += 1;
	return partword;
}
unsafe extern "C" fn _01forward(
	mut opb: *mut oggpack_buffer,
	mut vl: *mut ::core::ffi::c_void,
	mut in_0: *mut *mut ::core::ffi::c_int,
	mut ch: ::core::ffi::c_int,
	mut partword: *mut *mut ::core::ffi::c_long,
	mut encode: Option<
		unsafe extern "C" fn(
			*mut oggpack_buffer,
			*mut ::core::ffi::c_int,
			::core::ffi::c_int,
			*mut codebook
		) -> ::core::ffi::c_int
	>
) -> ::core::ffi::c_int {
	let mut i: ::core::ffi::c_long = 0;
	let mut j: ::core::ffi::c_long = 0;
	let mut k: ::core::ffi::c_long = 0;
	let mut s: ::core::ffi::c_long = 0;
	let mut look: *mut vorbis_look_residue0 = vl as *mut vorbis_look_residue0;
	let mut info: *mut vorbis_info_residue0 = (*look).info;
	let mut phrasebook: *mut codebook = (*look).fullbooks.offset((*look).phrasebook as isize);
	let mut samples_per_partition: ::core::ffi::c_int = (*info).grouping;
	let mut possible_partitions: ::core::ffi::c_int = (*info).partitions;
	let mut partitions_per_word: ::core::ffi::c_int = (*phrasebook).dim as ::core::ffi::c_int;
	let mut n: ::core::ffi::c_int = ((*info).end - (*info).begin) as ::core::ffi::c_int;
	let mut partvals: ::core::ffi::c_int = n / samples_per_partition;
	let mut resbits: [::core::ffi::c_long; 128] = [0; 128];
	let mut resvals: [::core::ffi::c_long; 128] = [0; 128];
	memset(
		&raw mut resbits as *mut ::core::ffi::c_long as *mut ::core::ffi::c_void,
		0 as ::core::ffi::c_int,
		::core::mem::size_of::<[::core::ffi::c_long; 128]>() as size_t
	);
	memset(
		&raw mut resvals as *mut ::core::ffi::c_long as *mut ::core::ffi::c_void,
		0 as ::core::ffi::c_int,
		::core::mem::size_of::<[::core::ffi::c_long; 128]>() as size_t
	);
	s = 0 as ::core::ffi::c_long;
	while s < (*look).stages as ::core::ffi::c_long {
		i = 0 as ::core::ffi::c_long;
		while i < partvals as ::core::ffi::c_long {
			if s == 0 as ::core::ffi::c_long {
				j = 0 as ::core::ffi::c_long;
				while j < ch as ::core::ffi::c_long {
					let mut val: ::core::ffi::c_long =
						*(*partword.offset(j as isize)).offset(i as isize);
					k = 1 as ::core::ffi::c_long;
					while k < partitions_per_word as ::core::ffi::c_long {
						val *= possible_partitions as ::core::ffi::c_long;
						if i + k < partvals as ::core::ffi::c_long {
							val += *(*partword.offset(j as isize)).offset((i + k) as isize);
						}
						k += 1;
					}
					if val < (*phrasebook).entries {
						(*look).phrasebits +=
							vorbis_book_encode(phrasebook, val as ::core::ffi::c_int, opb)
								as ::core::ffi::c_long;
					}
					j += 1;
				}
			}
			k = 0 as ::core::ffi::c_long;
			while k < partitions_per_word as ::core::ffi::c_long
				&& i < partvals as ::core::ffi::c_long
			{
				let mut offset: ::core::ffi::c_long =
					i * samples_per_partition as ::core::ffi::c_long + (*info).begin;
				j = 0 as ::core::ffi::c_long;
				while j < ch as ::core::ffi::c_long {
					if s == 0 as ::core::ffi::c_long {
						resvals[*(*partword.offset(j as isize)).offset(i as isize) as usize] +=
							samples_per_partition as ::core::ffi::c_long;
					}
					if (*info).secondstages
						[*(*partword.offset(j as isize)).offset(i as isize) as usize]
						& (1 as ::core::ffi::c_int) << s
						!= 0
					{
						let mut statebooknum: ::core::ffi::c_int = *(*(*look)
							.partbooks
							.offset(*(*partword.offset(j as isize)).offset(i as isize) as isize))
						.offset(s as isize);
						if statebooknum >= 0 as ::core::ffi::c_int {
							let mut statebook: *mut codebook =
								(*look).fullbooks.offset(statebooknum as isize);
							let mut ret: ::core::ffi::c_int = 0;
							ret = encode.expect("non-null function pointer")(
								opb,
								(*in_0.offset(j as isize)).offset(offset as isize),
								samples_per_partition,
								statebook
							);
							(*look).postbits += ret as ::core::ffi::c_long;
							resbits[*(*partword.offset(j as isize)).offset(i as isize) as usize] +=
								ret as ::core::ffi::c_long;
						}
					}
					j += 1;
				}
				k += 1;
				i += 1;
			}
		}
		s += 1;
	}
	return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn _01inverse(
	mut vb: *mut vorbis_block,
	mut vl: *mut ::core::ffi::c_void,
	mut in_0: *mut *mut ::core::ffi::c_float,
	mut ch: ::core::ffi::c_int,
	mut decodepart: Option<
		unsafe extern "C" fn(
			*mut dec_codebook,
			*mut ::core::ffi::c_float,
			*mut oggpack_buffer,
			::core::ffi::c_int
		) -> ::core::ffi::c_long
	>
) -> ::core::ffi::c_int {
	let mut alloca_allocations: Vec<Vec<u8>> = Vec::new();
	let mut i: ::core::ffi::c_long = 0;
	let mut j: ::core::ffi::c_long = 0;
	let mut k: ::core::ffi::c_long = 0;
	let mut l: ::core::ffi::c_long = 0;
	let mut s: ::core::ffi::c_long = 0;
	let mut look: *mut vorbis_look_residue0 = vl as *mut vorbis_look_residue0;
	let mut info: *mut vorbis_info_residue0 = (*look).info;
	let mut phrasebook: *mut dec_codebook = (*look).decbooks.offset((*look).phrasebook as isize);
	let mut samples_per_partition: ::core::ffi::c_int = (*info).grouping;
	let mut partitions_per_word: ::core::ffi::c_int = (*phrasebook).dim as ::core::ffi::c_int;
	let mut max: ::core::ffi::c_int = (*vb).pcmend >> 1 as ::core::ffi::c_int;
	let mut end: ::core::ffi::c_int = (if (*info).end < max as ::core::ffi::c_long {
		(*info).end
	} else {
		max as ::core::ffi::c_long
	}) as ::core::ffi::c_int;
	let mut n: ::core::ffi::c_int =
		(end as ::core::ffi::c_long - (*info).begin) as ::core::ffi::c_int;
	if n > 0 as ::core::ffi::c_int {
		let mut partvals: ::core::ffi::c_int = n / samples_per_partition;
		let mut partwords: ::core::ffi::c_int =
			(partvals + partitions_per_word - 1 as ::core::ffi::c_int) / partitions_per_word;
		alloca_allocations.push(::std::vec::from_elem(
			0,
			(ch as usize)
				.wrapping_mul(::core::mem::size_of::<*mut *mut ::core::ffi::c_int>() as usize)
				as usize
		));
		let mut partword: *mut *mut *mut ::core::ffi::c_int =
			alloca_allocations.last_mut().unwrap().as_mut_ptr()
				as *mut *mut *mut ::core::ffi::c_int;
		j = 0 as ::core::ffi::c_long;
		while j < ch as ::core::ffi::c_long {
			let ref mut fresh7 = *partword.offset(j as isize);
			*fresh7 = _vorbis_block_alloc(
				vb,
				(partwords as usize)
					.wrapping_mul(::core::mem::size_of::<*mut ::core::ffi::c_int>() as usize)
					as ::core::ffi::c_long
			) as *mut *mut ::core::ffi::c_int;
			j += 1;
		}
		s = 0 as ::core::ffi::c_long;
		's_43: while s < (*look).stages as ::core::ffi::c_long {
			i = 0 as ::core::ffi::c_long;
			l = 0 as ::core::ffi::c_long;
			while i < partvals as ::core::ffi::c_long {
				if s == 0 as ::core::ffi::c_long {
					j = 0 as ::core::ffi::c_long;
					while j < ch as ::core::ffi::c_long {
						let mut temp: ::core::ffi::c_int =
							vorbis_book_decode(phrasebook, &raw mut (*vb).opb)
								as ::core::ffi::c_int;
						if temp == -(1 as ::core::ffi::c_int) || temp >= (*info).partvals {
							break 's_43;
						}
						let ref mut fresh8 = *(*partword.offset(j as isize)).offset(l as isize);
						*fresh8 = *(*look).decodemap.offset(temp as isize);
						if (*(*partword.offset(j as isize)).offset(l as isize)).is_null() {
							break 's_43;
						}
						j += 1;
					}
				}
				k = 0 as ::core::ffi::c_long;
				while k < partitions_per_word as ::core::ffi::c_long
					&& i < partvals as ::core::ffi::c_long
				{
					j = 0 as ::core::ffi::c_long;
					while j < ch as ::core::ffi::c_long {
						let mut offset: ::core::ffi::c_long =
							(*info).begin + i * samples_per_partition as ::core::ffi::c_long;
						if (*info).secondstages[*(*(*partword.offset(j as isize))
							.offset(l as isize))
						.offset(k as isize) as usize]
							& (1 as ::core::ffi::c_int) << s
							!= 0
						{
							let mut stagebooknum: ::core::ffi::c_int =
								*(*(*look).partbooks.offset(
									*(*(*partword.offset(j as isize)).offset(l as isize))
										.offset(k as isize) as isize
								))
								.offset(s as isize);
							if stagebooknum >= 0 as ::core::ffi::c_int {
								let mut stagebook: *mut dec_codebook =
									(*look).decbooks.offset(stagebooknum as isize);
								if decodepart.expect("non-null function pointer")(
									stagebook,
									(*in_0.offset(j as isize)).offset(offset as isize),
									&raw mut (*vb).opb,
									samples_per_partition
								) == -(1 as ::core::ffi::c_int) as ::core::ffi::c_long
								{
									break 's_43;
								}
							}
						}
						j += 1;
					}
					k += 1;
					i += 1;
				}
				l += 1;
			}
			s += 1;
		}
	}
	return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn res0_inverse(
	mut vb: *mut vorbis_block,
	mut vl: *mut ::core::ffi::c_void,
	mut in_0: *mut *mut ::core::ffi::c_float,
	mut nonzero: *mut ::core::ffi::c_int,
	mut ch: ::core::ffi::c_int
) -> ::core::ffi::c_int {
	let mut i: ::core::ffi::c_int = 0;
	let mut used: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
	i = 0 as ::core::ffi::c_int;
	while i < ch {
		if *nonzero.offset(i as isize) != 0 {
			let fresh9 = used;
			used = used + 1;
			let ref mut fresh10 = *in_0.offset(fresh9 as isize);
			*fresh10 = *in_0.offset(i as isize);
		}
		i += 1;
	}
	if used != 0 {
		return _01inverse(
			vb,
			vl,
			in_0,
			used,
			Some(
				vorbis_book_decodevs_add
					as unsafe extern "C" fn(
						*mut dec_codebook,
						*mut ::core::ffi::c_float,
						*mut oggpack_buffer,
						::core::ffi::c_int
					) -> ::core::ffi::c_long
			)
		);
	} else {
		return 0 as ::core::ffi::c_int;
	};
}
#[no_mangle]
pub unsafe extern "C" fn res1_forward(
	mut opb: *mut oggpack_buffer,
	mut vb: *mut vorbis_block,
	mut vl: *mut ::core::ffi::c_void,
	mut in_0: *mut *mut ::core::ffi::c_int,
	mut nonzero: *mut ::core::ffi::c_int,
	mut ch: ::core::ffi::c_int,
	mut partword: *mut *mut ::core::ffi::c_long,
	mut submap: ::core::ffi::c_int
) -> ::core::ffi::c_int {
	let mut i: ::core::ffi::c_int = 0;
	let mut used: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
	i = 0 as ::core::ffi::c_int;
	while i < ch {
		if *nonzero.offset(i as isize) != 0 {
			let fresh11 = used;
			used = used + 1;
			let ref mut fresh12 = *in_0.offset(fresh11 as isize);
			*fresh12 = *in_0.offset(i as isize);
		}
		i += 1;
	}
	if used != 0 {
		return _01forward(
			opb,
			vl,
			in_0,
			used,
			partword,
			Some(
				_encodepart
					as unsafe extern "C" fn(
						*mut oggpack_buffer,
						*mut ::core::ffi::c_int,
						::core::ffi::c_int,
						*mut codebook
					) -> ::core::ffi::c_int
			)
		);
	} else {
		return 0 as ::core::ffi::c_int;
	};
}
#[no_mangle]
pub unsafe extern "C" fn res1_class(
	mut vb: *mut vorbis_block,
	mut vl: *mut ::core::ffi::c_void,
	mut in_0: *mut *mut ::core::ffi::c_int,
	mut nonzero: *mut ::core::ffi::c_int,
	mut ch: ::core::ffi::c_int
) -> *mut *mut ::core::ffi::c_long {
	let mut i: ::core::ffi::c_int = 0;
	let mut used: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
	i = 0 as ::core::ffi::c_int;
	while i < ch {
		if *nonzero.offset(i as isize) != 0 {
			let fresh13 = used;
			used = used + 1;
			let ref mut fresh14 = *in_0.offset(fresh13 as isize);
			*fresh14 = *in_0.offset(i as isize);
		}
		i += 1;
	}
	if used != 0 {
		return _01class(vb, vl, in_0, used);
	} else {
		return ::core::ptr::null_mut::<*mut ::core::ffi::c_long>();
	};
}
#[no_mangle]
pub unsafe extern "C" fn res1_inverse(
	mut vb: *mut vorbis_block,
	mut vl: *mut ::core::ffi::c_void,
	mut in_0: *mut *mut ::core::ffi::c_float,
	mut nonzero: *mut ::core::ffi::c_int,
	mut ch: ::core::ffi::c_int
) -> ::core::ffi::c_int {
	let mut i: ::core::ffi::c_int = 0;
	let mut used: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
	i = 0 as ::core::ffi::c_int;
	while i < ch {
		if *nonzero.offset(i as isize) != 0 {
			let fresh15 = used;
			used = used + 1;
			let ref mut fresh16 = *in_0.offset(fresh15 as isize);
			*fresh16 = *in_0.offset(i as isize);
		}
		i += 1;
	}
	if used != 0 {
		return _01inverse(
			vb,
			vl,
			in_0,
			used,
			Some(
				vorbis_book_decodev_add
					as unsafe extern "C" fn(
						*mut dec_codebook,
						*mut ::core::ffi::c_float,
						*mut oggpack_buffer,
						::core::ffi::c_int
					) -> ::core::ffi::c_long
			)
		);
	} else {
		return 0 as ::core::ffi::c_int;
	};
}
#[no_mangle]
pub unsafe extern "C" fn res2_class(
	mut vb: *mut vorbis_block,
	mut vl: *mut ::core::ffi::c_void,
	mut in_0: *mut *mut ::core::ffi::c_int,
	mut nonzero: *mut ::core::ffi::c_int,
	mut ch: ::core::ffi::c_int
) -> *mut *mut ::core::ffi::c_long {
	let mut i: ::core::ffi::c_int = 0;
	let mut used: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
	i = 0 as ::core::ffi::c_int;
	while i < ch {
		if *nonzero.offset(i as isize) != 0 {
			used += 1;
		}
		i += 1;
	}
	if used != 0 {
		return _2class(vb, vl, in_0, ch);
	} else {
		return ::core::ptr::null_mut::<*mut ::core::ffi::c_long>();
	};
}
#[no_mangle]
pub unsafe extern "C" fn res2_forward(
	mut opb: *mut oggpack_buffer,
	mut vb: *mut vorbis_block,
	mut vl: *mut ::core::ffi::c_void,
	mut in_0: *mut *mut ::core::ffi::c_int,
	mut nonzero: *mut ::core::ffi::c_int,
	mut ch: ::core::ffi::c_int,
	mut partword: *mut *mut ::core::ffi::c_long,
	mut submap: ::core::ffi::c_int
) -> ::core::ffi::c_int {
	let mut i: ::core::ffi::c_long = 0;
	let mut j: ::core::ffi::c_long = 0;
	let mut k: ::core::ffi::c_long = 0;
	let mut n: ::core::ffi::c_long =
		((*vb).pcmend / 2 as ::core::ffi::c_int) as ::core::ffi::c_long;
	let mut used: ::core::ffi::c_long = 0 as ::core::ffi::c_long;
	let mut work: *mut ::core::ffi::c_int = _vorbis_block_alloc(
		vb,
		((ch as ::core::ffi::c_long * n) as usize)
			.wrapping_mul(::core::mem::size_of::<::core::ffi::c_int>() as usize)
			as ::core::ffi::c_long
	) as *mut ::core::ffi::c_int;
	i = 0 as ::core::ffi::c_long;
	while i < ch as ::core::ffi::c_long {
		let mut pcm: *mut ::core::ffi::c_int = *in_0.offset(i as isize);
		if *nonzero.offset(i as isize) != 0 {
			used += 1;
		}
		j = 0 as ::core::ffi::c_long;
		k = i;
		while j < n {
			*work.offset(k as isize) = *pcm.offset(j as isize);
			j += 1;
			k += ch as ::core::ffi::c_long;
		}
		i += 1;
	}
	if used != 0 {
		return _01forward(
			opb,
			vl,
			&raw mut work,
			1 as ::core::ffi::c_int,
			partword,
			Some(
				_encodepart
					as unsafe extern "C" fn(
						*mut oggpack_buffer,
						*mut ::core::ffi::c_int,
						::core::ffi::c_int,
						*mut codebook
					) -> ::core::ffi::c_int
			)
		);
	} else {
		return 0 as ::core::ffi::c_int;
	};
}
#[no_mangle]
pub unsafe extern "C" fn res2_inverse(
	mut vb: *mut vorbis_block,
	mut vl: *mut ::core::ffi::c_void,
	mut in_0: *mut *mut ::core::ffi::c_float,
	mut nonzero: *mut ::core::ffi::c_int,
	mut ch: ::core::ffi::c_int
) -> ::core::ffi::c_int {
	let mut i: ::core::ffi::c_long = 0;
	let mut k: ::core::ffi::c_long = 0;
	let mut l: ::core::ffi::c_long = 0;
	let mut s: ::core::ffi::c_long = 0;
	let mut look: *mut vorbis_look_residue0 = vl as *mut vorbis_look_residue0;
	let mut info: *mut vorbis_info_residue0 = (*look).info;
	let mut phrasebook: *mut dec_codebook = (*look).decbooks.offset((*look).phrasebook as isize);
	let mut samples_per_partition: ::core::ffi::c_int = (*info).grouping;
	let mut partitions_per_word: ::core::ffi::c_int = (*phrasebook).dim as ::core::ffi::c_int;
	let mut max: ::core::ffi::c_int = (*vb).pcmend * ch >> 1 as ::core::ffi::c_int;
	let mut end: ::core::ffi::c_int = (if (*info).end < max as ::core::ffi::c_long {
		(*info).end
	} else {
		max as ::core::ffi::c_long
	}) as ::core::ffi::c_int;
	let mut n: ::core::ffi::c_int =
		(end as ::core::ffi::c_long - (*info).begin) as ::core::ffi::c_int;
	if n > 0 as ::core::ffi::c_int {
		let mut partvals: ::core::ffi::c_int = n / samples_per_partition;
		let mut partwords: ::core::ffi::c_int =
			(partvals + partitions_per_word - 1 as ::core::ffi::c_int) / partitions_per_word;
		let mut partword: *mut *mut ::core::ffi::c_int = _vorbis_block_alloc(
			vb,
			(partwords as usize)
				.wrapping_mul(::core::mem::size_of::<*mut ::core::ffi::c_int>() as usize)
				as ::core::ffi::c_long
		) as *mut *mut ::core::ffi::c_int;
		i = 0 as ::core::ffi::c_long;
		while i < ch as ::core::ffi::c_long {
			if *nonzero.offset(i as isize) != 0 {
				break;
			}
			i += 1;
		}
		if i == ch as ::core::ffi::c_long {
			return 0 as ::core::ffi::c_int;
		}
		s = 0 as ::core::ffi::c_long;
		's_52: while s < (*look).stages as ::core::ffi::c_long {
			i = 0 as ::core::ffi::c_long;
			l = 0 as ::core::ffi::c_long;
			while i < partvals as ::core::ffi::c_long {
				if s == 0 as ::core::ffi::c_long {
					let mut temp: ::core::ffi::c_int =
						vorbis_book_decode(phrasebook, &raw mut (*vb).opb) as ::core::ffi::c_int;
					if temp == -(1 as ::core::ffi::c_int) || temp >= (*info).partvals {
						break 's_52;
					}
					let ref mut fresh17 = *partword.offset(l as isize);
					*fresh17 = *(*look).decodemap.offset(temp as isize);
					if (*partword.offset(l as isize)).is_null() {
						break 's_52;
					}
				}
				k = 0 as ::core::ffi::c_long;
				while k < partitions_per_word as ::core::ffi::c_long
					&& i < partvals as ::core::ffi::c_long
				{
					if (*info).secondstages
						[*(*partword.offset(l as isize)).offset(k as isize) as usize]
						& (1 as ::core::ffi::c_int) << s
						!= 0
					{
						let mut stagebooknum: ::core::ffi::c_int = *(*(*look)
							.partbooks
							.offset(*(*partword.offset(l as isize)).offset(k as isize) as isize))
						.offset(s as isize);
						if stagebooknum >= 0 as ::core::ffi::c_int {
							let mut stagebook: *mut dec_codebook =
								(*look).decbooks.offset(stagebooknum as isize);
							if vorbis_book_decodevv_add(
								stagebook,
								in_0,
								i * samples_per_partition as ::core::ffi::c_long + (*info).begin,
								ch,
								&raw mut (*vb).opb,
								samples_per_partition
							) == -(1 as ::core::ffi::c_int) as ::core::ffi::c_long
							{
								break 's_52;
							}
						}
					}
					k += 1;
					i += 1;
				}
				l += 1;
			}
			s += 1;
		}
	}
	return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub static mut residue0_exportbundle: vorbis_func_residue = vorbis_func_residue {
	pack: None,
	unpack: Some(
		res0_unpack
			as unsafe extern "C" fn(
				*mut vorbis_info,
				*mut oggpack_buffer
			) -> *mut ::core::ffi::c_void
	),
	look: Some(
		res0_look
			as unsafe extern "C" fn(
				*mut vorbis_dsp_state,
				*mut ::core::ffi::c_void
			) -> *mut ::core::ffi::c_void
	),
	free_info: Some(res0_free_info as unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()),
	free_look: Some(res0_free_look as unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()),
	class: None,
	forward: None,
	inverse: Some(
		res0_inverse
			as unsafe extern "C" fn(
				*mut vorbis_block,
				*mut ::core::ffi::c_void,
				*mut *mut ::core::ffi::c_float,
				*mut ::core::ffi::c_int,
				::core::ffi::c_int
			) -> ::core::ffi::c_int
	)
};
#[no_mangle]
pub static mut residue1_exportbundle: vorbis_func_residue = vorbis_func_residue {
	pack: Some(
		res0_pack as unsafe extern "C" fn(*mut ::core::ffi::c_void, *mut oggpack_buffer) -> ()
	),
	unpack: Some(
		res0_unpack
			as unsafe extern "C" fn(
				*mut vorbis_info,
				*mut oggpack_buffer
			) -> *mut ::core::ffi::c_void
	),
	look: Some(
		res0_look
			as unsafe extern "C" fn(
				*mut vorbis_dsp_state,
				*mut ::core::ffi::c_void
			) -> *mut ::core::ffi::c_void
	),
	free_info: Some(res0_free_info as unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()),
	free_look: Some(res0_free_look as unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()),
	class: Some(
		res1_class
			as unsafe extern "C" fn(
				*mut vorbis_block,
				*mut ::core::ffi::c_void,
				*mut *mut ::core::ffi::c_int,
				*mut ::core::ffi::c_int,
				::core::ffi::c_int
			) -> *mut *mut ::core::ffi::c_long
	),
	forward: Some(
		res1_forward
			as unsafe extern "C" fn(
				*mut oggpack_buffer,
				*mut vorbis_block,
				*mut ::core::ffi::c_void,
				*mut *mut ::core::ffi::c_int,
				*mut ::core::ffi::c_int,
				::core::ffi::c_int,
				*mut *mut ::core::ffi::c_long,
				::core::ffi::c_int
			) -> ::core::ffi::c_int
	),
	inverse: Some(
		res1_inverse
			as unsafe extern "C" fn(
				*mut vorbis_block,
				*mut ::core::ffi::c_void,
				*mut *mut ::core::ffi::c_float,
				*mut ::core::ffi::c_int,
				::core::ffi::c_int
			) -> ::core::ffi::c_int
	)
};
#[no_mangle]
pub static mut residue2_exportbundle: vorbis_func_residue = vorbis_func_residue {
	pack: Some(
		res0_pack as unsafe extern "C" fn(*mut ::core::ffi::c_void, *mut oggpack_buffer) -> ()
	),
	unpack: Some(
		res0_unpack
			as unsafe extern "C" fn(
				*mut vorbis_info,
				*mut oggpack_buffer
			) -> *mut ::core::ffi::c_void
	),
	look: Some(
		res0_look
			as unsafe extern "C" fn(
				*mut vorbis_dsp_state,
				*mut ::core::ffi::c_void
			) -> *mut ::core::ffi::c_void
	),
	free_info: Some(res0_free_info as unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()),
	free_look: Some(res0_free_look as unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()),
	class: Some(
		res2_class
			as unsafe extern "C" fn(
				*mut vorbis_block,
				*mut ::core::ffi::c_void,
				*mut *mut ::core::ffi::c_int,
				*mut ::core::ffi::c_int,
				::core::ffi::c_int
			) -> *mut *mut ::core::ffi::c_long
	),
	forward: Some(
		res2_forward
			as unsafe extern "C" fn(
				*mut oggpack_buffer,
				*mut vorbis_block,
				*mut ::core::ffi::c_void,
				*mut *mut ::core::ffi::c_int,
				*mut ::core::ffi::c_int,
				::core::ffi::c_int,
				*mut *mut ::core::ffi::c_long,
				::core::ffi::c_int
			) -> ::core::ffi::c_int
	),
	inverse: Some(
		res2_inverse
			as unsafe extern "C" fn(
				*mut vorbis_block,
				*mut ::core::ffi::c_void,
				*mut *mut ::core::ffi::c_float,
				*mut ::core::ffi::c_int,
				::core::ffi::c_int
			) -> ::core::ffi::c_int
	)
};
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
