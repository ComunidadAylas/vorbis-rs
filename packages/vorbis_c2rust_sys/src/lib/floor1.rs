extern "C" {
	fn calloc(__nmemb: size_t, __size: size_t) -> *mut ::core::ffi::c_void;
	fn free(__ptr: *mut ::core::ffi::c_void);
	fn exit(__status: ::core::ffi::c_int) -> !;
	fn qsort(
		__base: *mut ::core::ffi::c_void,
		__nmemb: size_t,
		__size: size_t,
		__compar: __compar_fn_t
	);
	fn memset(
		__s: *mut ::core::ffi::c_void,
		__c: ::core::ffi::c_int,
		__n: size_t
	) -> *mut ::core::ffi::c_void;
	fn rint(__x: ::core::ffi::c_double) -> ::core::ffi::c_double;
	fn oggpack_write(b: *mut oggpack_buffer, value: ::core::ffi::c_ulong, bits: ::core::ffi::c_int);
	fn oggpack_read(b: *mut oggpack_buffer, bits: ::core::ffi::c_int) -> ::core::ffi::c_long;
	fn vorbis_book_encode(
		book: *mut codebook,
		a: ::core::ffi::c_int,
		b: *mut oggpack_buffer
	) -> ::core::ffi::c_int;
	fn vorbis_book_decode(book: *mut dec_codebook, b: *mut oggpack_buffer) -> ::core::ffi::c_long;
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
	registry::vorbis_func_floor
};
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
pub struct vorbis_info_floor1 {
	pub partitions: ::core::ffi::c_int,
	pub partitionclass: [::core::ffi::c_int; 31],
	pub class_dim: [::core::ffi::c_int; 16],
	pub class_subs: [::core::ffi::c_int; 16],
	pub class_book: [::core::ffi::c_int; 16],
	pub class_subbook: [[::core::ffi::c_int; 8]; 16],
	pub mult: ::core::ffi::c_int,
	pub postlist: [::core::ffi::c_int; 65],
	pub maxover: ::core::ffi::c_float,
	pub maxunder: ::core::ffi::c_float,
	pub maxerr: ::core::ffi::c_float,
	pub twofitweight: ::core::ffi::c_float,
	pub twofitatten: ::core::ffi::c_float,
	pub n: ::core::ffi::c_int
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
pub struct vorbis_look_floor1 {
	pub sorted_index: [::core::ffi::c_int; 65],
	pub forward_index: [::core::ffi::c_int; 65],
	pub reverse_index: [::core::ffi::c_int; 65],
	pub hineighbor: [::core::ffi::c_int; 63],
	pub loneighbor: [::core::ffi::c_int; 63],
	pub posts: ::core::ffi::c_int,
	pub n: ::core::ffi::c_int,
	pub quant_q: ::core::ffi::c_int,
	pub vi: *mut vorbis_info_floor1,
	pub phrasebits: ::core::ffi::c_long,
	pub postbits: ::core::ffi::c_long,
	pub frames: ::core::ffi::c_long
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct lsfit_acc {
	pub x0: ::core::ffi::c_int,
	pub x1: ::core::ffi::c_int,
	pub xa: ::core::ffi::c_int,
	pub ya: ::core::ffi::c_int,
	pub x2a: ::core::ffi::c_int,
	pub y2a: ::core::ffi::c_int,
	pub xya: ::core::ffi::c_int,
	pub an: ::core::ffi::c_int,
	pub xb: ::core::ffi::c_int,
	pub yb: ::core::ffi::c_int,
	pub x2b: ::core::ffi::c_int,
	pub y2b: ::core::ffi::c_int,
	pub xyb: ::core::ffi::c_int,
	pub bn: ::core::ffi::c_int
}
pub const VIF_POSIT: ::core::ffi::c_int = 63 as ::core::ffi::c_int;
unsafe extern "C" fn floor1_free_info(mut i: *mut ::core::ffi::c_void) {
	let mut info: *mut vorbis_info_floor1 = i as *mut vorbis_info_floor1;
	if !info.is_null() {
		memset(
			info as *mut ::core::ffi::c_void,
			0 as ::core::ffi::c_int,
			::core::mem::size_of::<vorbis_info_floor1>() as size_t
		);
		free(info as *mut ::core::ffi::c_void);
	}
}
unsafe extern "C" fn floor1_free_look(mut i: *mut ::core::ffi::c_void) {
	let mut look: *mut vorbis_look_floor1 = i as *mut vorbis_look_floor1;
	if !look.is_null() {
		memset(
			look as *mut ::core::ffi::c_void,
			0 as ::core::ffi::c_int,
			::core::mem::size_of::<vorbis_look_floor1>() as size_t
		);
		free(look as *mut ::core::ffi::c_void);
	}
}
unsafe extern "C" fn floor1_pack(mut i: *mut ::core::ffi::c_void, mut opb: *mut oggpack_buffer) {
	let mut info: *mut vorbis_info_floor1 = i as *mut vorbis_info_floor1;
	let mut j: ::core::ffi::c_int = 0;
	let mut k: ::core::ffi::c_int = 0;
	let mut count: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
	let mut rangebits: ::core::ffi::c_int = 0;
	let mut maxposit: ::core::ffi::c_int = (*info).postlist[1 as ::core::ffi::c_int as usize];
	let mut maxclass: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
	oggpack_write(
		opb,
		(*info).partitions as ::core::ffi::c_ulong,
		5 as ::core::ffi::c_int
	);
	j = 0 as ::core::ffi::c_int;
	while j < (*info).partitions {
		oggpack_write(
			opb,
			(*info).partitionclass[j as usize] as ::core::ffi::c_ulong,
			4 as ::core::ffi::c_int
		);
		if maxclass < (*info).partitionclass[j as usize] {
			maxclass = (*info).partitionclass[j as usize];
		}
		j += 1;
	}
	j = 0 as ::core::ffi::c_int;
	while j < maxclass + 1 as ::core::ffi::c_int {
		oggpack_write(
			opb,
			((*info).class_dim[j as usize] - 1 as ::core::ffi::c_int) as ::core::ffi::c_ulong,
			3 as ::core::ffi::c_int
		);
		oggpack_write(
			opb,
			(*info).class_subs[j as usize] as ::core::ffi::c_ulong,
			2 as ::core::ffi::c_int
		);
		if (*info).class_subs[j as usize] != 0 {
			oggpack_write(
				opb,
				(*info).class_book[j as usize] as ::core::ffi::c_ulong,
				8 as ::core::ffi::c_int
			);
		}
		k = 0 as ::core::ffi::c_int;
		while k < (1 as ::core::ffi::c_int) << (*info).class_subs[j as usize] {
			oggpack_write(
				opb,
				((*info).class_subbook[j as usize][k as usize] + 1 as ::core::ffi::c_int)
					as ::core::ffi::c_ulong,
				8 as ::core::ffi::c_int
			);
			k += 1;
		}
		j += 1;
	}
	oggpack_write(
		opb,
		((*info).mult - 1 as ::core::ffi::c_int) as ::core::ffi::c_ulong,
		2 as ::core::ffi::c_int
	);
	oggpack_write(
		opb,
		ov_ilog((maxposit - 1 as ::core::ffi::c_int) as ogg_uint32_t) as ::core::ffi::c_ulong,
		4 as ::core::ffi::c_int
	);
	rangebits = ov_ilog((maxposit - 1 as ::core::ffi::c_int) as ogg_uint32_t);
	j = 0 as ::core::ffi::c_int;
	k = 0 as ::core::ffi::c_int;
	while j < (*info).partitions {
		count += (*info).class_dim[(*info).partitionclass[j as usize] as usize];
		while k < count {
			oggpack_write(
				opb,
				(*info).postlist[(k + 2 as ::core::ffi::c_int) as usize] as ::core::ffi::c_ulong,
				rangebits
			);
			k += 1;
		}
		j += 1;
	}
}
unsafe extern "C" fn icomp(
	mut a: *const ::core::ffi::c_void,
	mut b: *const ::core::ffi::c_void
) -> ::core::ffi::c_int {
	return **(a as *mut *mut ::core::ffi::c_int) - **(b as *mut *mut ::core::ffi::c_int);
}
unsafe extern "C" fn floor1_unpack(
	mut vi: *mut vorbis_info,
	mut opb: *mut oggpack_buffer
) -> *mut ::core::ffi::c_void {
	let mut current_block: u64;
	let mut ci: *mut codec_setup_info = (*vi).codec_setup as *mut codec_setup_info;
	let mut j: ::core::ffi::c_int = 0;
	let mut k: ::core::ffi::c_int = 0;
	let mut count: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
	let mut maxclass: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
	let mut rangebits: ::core::ffi::c_int = 0;
	let mut info: *mut vorbis_info_floor1 = calloc(
		1 as size_t,
		::core::mem::size_of::<vorbis_info_floor1>() as size_t
	) as *mut vorbis_info_floor1;
	(*info).partitions = oggpack_read(opb, 5 as ::core::ffi::c_int) as ::core::ffi::c_int;
	j = 0 as ::core::ffi::c_int;
	loop {
		if !(j < (*info).partitions) {
			current_block = 7815301370352969686;
			break;
		}
		(*info).partitionclass[j as usize] =
			oggpack_read(opb, 4 as ::core::ffi::c_int) as ::core::ffi::c_int;
		if (*info).partitionclass[j as usize] < 0 as ::core::ffi::c_int {
			current_block = 3799606739964546207;
			break;
		}
		if maxclass < (*info).partitionclass[j as usize] {
			maxclass = (*info).partitionclass[j as usize];
		}
		j += 1;
	}
	match current_block {
		7815301370352969686 => {
			j = 0 as ::core::ffi::c_int;
			's_40: loop {
				if !(j < maxclass + 1 as ::core::ffi::c_int) {
					current_block = 2370887241019905314;
					break;
				}
				(*info).class_dim[j as usize] = (oggpack_read(opb, 3 as ::core::ffi::c_int)
					+ 1 as ::core::ffi::c_long)
					as ::core::ffi::c_int;
				(*info).class_subs[j as usize] =
					oggpack_read(opb, 2 as ::core::ffi::c_int) as ::core::ffi::c_int;
				if (*info).class_subs[j as usize] < 0 as ::core::ffi::c_int {
					current_block = 3799606739964546207;
					break;
				}
				if (*info).class_subs[j as usize] != 0 {
					(*info).class_book[j as usize] =
						oggpack_read(opb, 8 as ::core::ffi::c_int) as ::core::ffi::c_int;
				}
				if (*info).class_book[j as usize] < 0 as ::core::ffi::c_int
					|| (*info).class_book[j as usize] >= (*ci).books
				{
					current_block = 3799606739964546207;
					break;
				}
				k = 0 as ::core::ffi::c_int;
				while k < (1 as ::core::ffi::c_int) << (*info).class_subs[j as usize] {
					(*info).class_subbook[j as usize][k as usize] =
						(oggpack_read(opb, 8 as ::core::ffi::c_int) - 1 as ::core::ffi::c_long)
							as ::core::ffi::c_int;
					if (*info).class_subbook[j as usize][k as usize] < -(1 as ::core::ffi::c_int)
						|| (*info).class_subbook[j as usize][k as usize] >= (*ci).books
					{
						current_block = 3799606739964546207;
						break 's_40;
					}
					k += 1;
				}
				j += 1;
			}
			match current_block {
				3799606739964546207 => {}
				_ => {
					(*info).mult = (oggpack_read(opb, 2 as ::core::ffi::c_int)
						+ 1 as ::core::ffi::c_long) as ::core::ffi::c_int;
					rangebits = oggpack_read(opb, 4 as ::core::ffi::c_int) as ::core::ffi::c_int;
					if !(rangebits < 0 as ::core::ffi::c_int) {
						j = 0 as ::core::ffi::c_int;
						k = 0 as ::core::ffi::c_int;
						's_109: loop {
							if !(j < (*info).partitions) {
								current_block = 14359455889292382949;
								break;
							}
							count += (*info).class_dim[(*info).partitionclass[j as usize] as usize];
							if count > VIF_POSIT {
								current_block = 3799606739964546207;
								break;
							}
							while k < count {
								(*info).postlist[(k + 2 as ::core::ffi::c_int) as usize] =
									oggpack_read(opb, rangebits) as ::core::ffi::c_int;
								let mut t: ::core::ffi::c_int =
									(*info).postlist[(k + 2 as ::core::ffi::c_int) as usize];
								if t < 0 as ::core::ffi::c_int
									|| t >= (1 as ::core::ffi::c_int) << rangebits
								{
									current_block = 3799606739964546207;
									break 's_109;
								}
								k += 1;
							}
							j += 1;
						}
						match current_block {
							3799606739964546207 => {}
							_ => {
								(*info).postlist[0 as ::core::ffi::c_int as usize] =
									0 as ::core::ffi::c_int;
								(*info).postlist[1 as ::core::ffi::c_int as usize] =
									(1 as ::core::ffi::c_int) << rangebits;
								let mut sortpointer: [*mut ::core::ffi::c_int; 65] =
									[::core::ptr::null_mut::<::core::ffi::c_int>(); 65];
								j = 0 as ::core::ffi::c_int;
								while j < count + 2 as ::core::ffi::c_int {
									sortpointer[j as usize] = (&raw mut (*info).postlist
										as *mut ::core::ffi::c_int)
										.offset(j as isize);
									j += 1;
								}
								sortpointer[..(count + 2 as ::core::ffi::c_int) as usize]
									.sort_by(|a, b| unsafe { (**a).cmp(&**b) });
								j = 1 as ::core::ffi::c_int;
								loop {
									if !(j < count + 2 as ::core::ffi::c_int) {
										current_block = 8845338526596852646;
										break;
									}
									if *sortpointer[(j - 1 as ::core::ffi::c_int) as usize]
										== *sortpointer[j as usize]
									{
										current_block = 3799606739964546207;
										break;
									}
									j += 1;
								}
								match current_block {
									3799606739964546207 => {}
									_ => return info as *mut ::core::ffi::c_void
								}
							}
						}
					}
				}
			}
		}
		_ => {}
	}
	floor1_free_info(info as *mut ::core::ffi::c_void);
	return ::core::ptr::null_mut::<::core::ffi::c_void>();
}
unsafe extern "C" fn floor1_look(
	mut vd: *mut vorbis_dsp_state,
	mut in_0: *mut ::core::ffi::c_void
) -> *mut ::core::ffi::c_void {
	let mut sortpointer: [*mut ::core::ffi::c_int; 65] =
		[::core::ptr::null_mut::<::core::ffi::c_int>(); 65];
	let mut info: *mut vorbis_info_floor1 = in_0 as *mut vorbis_info_floor1;
	let mut look: *mut vorbis_look_floor1 = calloc(
		1 as size_t,
		::core::mem::size_of::<vorbis_look_floor1>() as size_t
	) as *mut vorbis_look_floor1;
	let mut i: ::core::ffi::c_int = 0;
	let mut j: ::core::ffi::c_int = 0;
	let mut n: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
	(*look).vi = info;
	(*look).n = (*info).postlist[1 as ::core::ffi::c_int as usize];
	i = 0 as ::core::ffi::c_int;
	while i < (*info).partitions {
		n += (*info).class_dim[(*info).partitionclass[i as usize] as usize];
		i += 1;
	}
	n += 2 as ::core::ffi::c_int;
	(*look).posts = n;
	i = 0 as ::core::ffi::c_int;
	while i < n {
		sortpointer[i as usize] =
			(&raw mut (*info).postlist as *mut ::core::ffi::c_int).offset(i as isize);
		i += 1;
	}
	sortpointer[..n as usize].sort_by(|a, b| unsafe { (**a).cmp(&**b) });
	i = 0 as ::core::ffi::c_int;
	while i < n {
		(*look).forward_index[i as usize] = sortpointer[i as usize]
			.offset_from(&raw mut (*info).postlist as *mut ::core::ffi::c_int)
			as ::core::ffi::c_long as ::core::ffi::c_int;
		i += 1;
	}
	i = 0 as ::core::ffi::c_int;
	while i < n {
		(*look).reverse_index[(*look).forward_index[i as usize] as usize] = i;
		i += 1;
	}
	i = 0 as ::core::ffi::c_int;
	while i < n {
		(*look).sorted_index[i as usize] =
			(*info).postlist[(*look).forward_index[i as usize] as usize];
		i += 1;
	}
	match (*info).mult {
		1 => {
			(*look).quant_q = 256 as ::core::ffi::c_int;
		}
		2 => {
			(*look).quant_q = 128 as ::core::ffi::c_int;
		}
		3 => {
			(*look).quant_q = 86 as ::core::ffi::c_int;
		}
		4 => {
			(*look).quant_q = 64 as ::core::ffi::c_int;
		}
		_ => {}
	}
	i = 0 as ::core::ffi::c_int;
	while i < n - 2 as ::core::ffi::c_int {
		let mut lo: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
		let mut hi: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
		let mut lx: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
		let mut hx: ::core::ffi::c_int = (*look).n;
		let mut currentx: ::core::ffi::c_int =
			(*info).postlist[(i + 2 as ::core::ffi::c_int) as usize];
		j = 0 as ::core::ffi::c_int;
		while j < i + 2 as ::core::ffi::c_int {
			let mut x: ::core::ffi::c_int = (*info).postlist[j as usize];
			if x > lx && x < currentx {
				lo = j;
				lx = x;
			}
			if x < hx && x > currentx {
				hi = j;
				hx = x;
			}
			j += 1;
		}
		(*look).loneighbor[i as usize] = lo;
		(*look).hineighbor[i as usize] = hi;
		i += 1;
	}
	return look as *mut ::core::ffi::c_void;
}
unsafe extern "C" fn render_point(
	mut x0: ::core::ffi::c_int,
	mut x1: ::core::ffi::c_int,
	mut y0: ::core::ffi::c_int,
	mut y1: ::core::ffi::c_int,
	mut x: ::core::ffi::c_int
) -> ::core::ffi::c_int {
	y0 &= 0x7fff as ::core::ffi::c_int;
	y1 &= 0x7fff as ::core::ffi::c_int;
	let mut dy: ::core::ffi::c_int = y1 - y0;
	let mut adx: ::core::ffi::c_int = x1 - x0;
	let mut ady: ::core::ffi::c_int = dy.abs();
	let mut err: ::core::ffi::c_int = ady * (x - x0);
	let mut off: ::core::ffi::c_int = err / adx;
	if dy < 0 as ::core::ffi::c_int {
		return y0 - off;
	}
	return y0 + off;
}
unsafe extern "C" fn vorbis_dBquant(mut x: *const ::core::ffi::c_float) -> ::core::ffi::c_int {
	let mut i: ::core::ffi::c_int = (*x * 7.3142857f32 + 1023.5f32) as ::core::ffi::c_int;
	if i > 1023 as ::core::ffi::c_int {
		return 1023 as ::core::ffi::c_int;
	}
	if i < 0 as ::core::ffi::c_int {
		return 0 as ::core::ffi::c_int;
	}
	return i;
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
unsafe extern "C" fn render_line(
	mut n: ::core::ffi::c_int,
	mut x0: ::core::ffi::c_int,
	mut x1: ::core::ffi::c_int,
	mut y0: ::core::ffi::c_int,
	mut y1: ::core::ffi::c_int,
	mut d: *mut ::core::ffi::c_float
) {
	let mut dy: ::core::ffi::c_int = y1 - y0;
	let mut adx: ::core::ffi::c_int = x1 - x0;
	let mut ady: ::core::ffi::c_int = dy.abs();
	let mut base: ::core::ffi::c_int = dy / adx;
	let mut sy: ::core::ffi::c_int = if dy < 0 as ::core::ffi::c_int {
		base - 1 as ::core::ffi::c_int
	} else {
		base + 1 as ::core::ffi::c_int
	};
	let mut x: ::core::ffi::c_int = x0;
	let mut y: ::core::ffi::c_int = y0;
	let mut err: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
	ady -= (base * adx).abs();
	if n > x1 {
		n = x1;
	}
	if x < n {
		*d.offset(x as isize) *= FLOOR1_fromdB_LOOKUP[y as usize];
	}
	loop {
		x += 1;
		if !(x < n) {
			break;
		}
		err = err + ady;
		if err >= adx {
			err -= adx;
			y += sy;
		} else {
			y += base;
		}
		*d.offset(x as isize) *= FLOOR1_fromdB_LOOKUP[y as usize];
	}
}
unsafe extern "C" fn render_line0(
	mut n: ::core::ffi::c_int,
	mut x0: ::core::ffi::c_int,
	mut x1: ::core::ffi::c_int,
	mut y0: ::core::ffi::c_int,
	mut y1: ::core::ffi::c_int,
	mut d: *mut ::core::ffi::c_int
) {
	let mut dy: ::core::ffi::c_int = y1 - y0;
	let mut adx: ::core::ffi::c_int = x1 - x0;
	let mut ady: ::core::ffi::c_int = dy.abs();
	let mut base: ::core::ffi::c_int = dy / adx;
	let mut sy: ::core::ffi::c_int = if dy < 0 as ::core::ffi::c_int {
		base - 1 as ::core::ffi::c_int
	} else {
		base + 1 as ::core::ffi::c_int
	};
	let mut x: ::core::ffi::c_int = x0;
	let mut y: ::core::ffi::c_int = y0;
	let mut err: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
	ady -= (base * adx).abs();
	if n > x1 {
		n = x1;
	}
	if x < n {
		*d.offset(x as isize) = y;
	}
	loop {
		x += 1;
		if !(x < n) {
			break;
		}
		err = err + ady;
		if err >= adx {
			err -= adx;
			y += sy;
		} else {
			y += base;
		}
		*d.offset(x as isize) = y;
	}
}
unsafe extern "C" fn accumulate_fit(
	mut flr: *const ::core::ffi::c_float,
	mut mdct: *const ::core::ffi::c_float,
	mut x0: ::core::ffi::c_int,
	mut x1: ::core::ffi::c_int,
	mut a: *mut lsfit_acc,
	mut n: ::core::ffi::c_int,
	mut info: *mut vorbis_info_floor1
) -> ::core::ffi::c_int {
	let mut i: ::core::ffi::c_long = 0;
	let mut xa: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
	let mut ya: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
	let mut x2a: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
	let mut y2a: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
	let mut xya: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
	let mut na: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
	let mut xb: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
	let mut yb: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
	let mut x2b: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
	let mut y2b: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
	let mut xyb: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
	let mut nb: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
	memset(
		a as *mut ::core::ffi::c_void,
		0 as ::core::ffi::c_int,
		::core::mem::size_of::<lsfit_acc>() as size_t
	);
	(*a).x0 = x0;
	(*a).x1 = x1;
	if x1 >= n {
		x1 = n - 1 as ::core::ffi::c_int;
	}
	i = x0 as ::core::ffi::c_long;
	while i <= x1 as ::core::ffi::c_long {
		let mut quantized: ::core::ffi::c_int = vorbis_dBquant(flr.offset(i as isize));
		if quantized != 0 {
			if *mdct.offset(i as isize) + (*info).twofitatten >= *flr.offset(i as isize) {
				xa = (xa as ::core::ffi::c_long + i) as ::core::ffi::c_int;
				ya += quantized;
				x2a = (x2a as ::core::ffi::c_long + i * i) as ::core::ffi::c_int;
				y2a += quantized * quantized;
				xya = (xya as ::core::ffi::c_long + i * quantized as ::core::ffi::c_long)
					as ::core::ffi::c_int;
				na += 1;
			} else {
				xb = (xb as ::core::ffi::c_long + i) as ::core::ffi::c_int;
				yb += quantized;
				x2b = (x2b as ::core::ffi::c_long + i * i) as ::core::ffi::c_int;
				y2b += quantized * quantized;
				xyb = (xyb as ::core::ffi::c_long + i * quantized as ::core::ffi::c_long)
					as ::core::ffi::c_int;
				nb += 1;
			}
		}
		i += 1;
	}
	(*a).xa = xa;
	(*a).ya = ya;
	(*a).x2a = x2a;
	(*a).y2a = y2a;
	(*a).xya = xya;
	(*a).an = na;
	(*a).xb = xb;
	(*a).yb = yb;
	(*a).x2b = x2b;
	(*a).y2b = y2b;
	(*a).xyb = xyb;
	(*a).bn = nb;
	return na;
}
unsafe extern "C" fn fit_line(
	mut a: *mut lsfit_acc,
	mut fits: ::core::ffi::c_int,
	mut y0: *mut ::core::ffi::c_int,
	mut y1: *mut ::core::ffi::c_int,
	mut info: *mut vorbis_info_floor1
) -> ::core::ffi::c_int {
	let mut xb: ::core::ffi::c_double = 0 as ::core::ffi::c_int as ::core::ffi::c_double;
	let mut yb: ::core::ffi::c_double = 0 as ::core::ffi::c_int as ::core::ffi::c_double;
	let mut x2b: ::core::ffi::c_double = 0 as ::core::ffi::c_int as ::core::ffi::c_double;
	let mut y2b: ::core::ffi::c_double = 0 as ::core::ffi::c_int as ::core::ffi::c_double;
	let mut xyb: ::core::ffi::c_double = 0 as ::core::ffi::c_int as ::core::ffi::c_double;
	let mut bn: ::core::ffi::c_double = 0 as ::core::ffi::c_int as ::core::ffi::c_double;
	let mut i: ::core::ffi::c_int = 0;
	let mut x0: ::core::ffi::c_int = (*a.offset(0 as ::core::ffi::c_int as isize)).x0;
	let mut x1: ::core::ffi::c_int = (*a.offset((fits - 1 as ::core::ffi::c_int) as isize)).x1;
	i = 0 as ::core::ffi::c_int;
	while i < fits {
		let mut weight: ::core::ffi::c_double =
			(((*a.offset(i as isize)).bn + (*a.offset(i as isize)).an) as ::core::ffi::c_float
				* (*info).twofitweight
				/ ((*a.offset(i as isize)).an + 1 as ::core::ffi::c_int) as ::core::ffi::c_float)
				as ::core::ffi::c_double
				+ 1.0f64;
		xb += (*a.offset(i as isize)).xb as ::core::ffi::c_double
			+ (*a.offset(i as isize)).xa as ::core::ffi::c_double * weight;
		yb += (*a.offset(i as isize)).yb as ::core::ffi::c_double
			+ (*a.offset(i as isize)).ya as ::core::ffi::c_double * weight;
		x2b += (*a.offset(i as isize)).x2b as ::core::ffi::c_double
			+ (*a.offset(i as isize)).x2a as ::core::ffi::c_double * weight;
		y2b += (*a.offset(i as isize)).y2b as ::core::ffi::c_double
			+ (*a.offset(i as isize)).y2a as ::core::ffi::c_double * weight;
		xyb += (*a.offset(i as isize)).xyb as ::core::ffi::c_double
			+ (*a.offset(i as isize)).xya as ::core::ffi::c_double * weight;
		bn += (*a.offset(i as isize)).bn as ::core::ffi::c_double
			+ (*a.offset(i as isize)).an as ::core::ffi::c_double * weight;
		i += 1;
	}
	if *y0 >= 0 as ::core::ffi::c_int {
		xb += x0 as ::core::ffi::c_double;
		yb += *y0 as ::core::ffi::c_double;
		x2b += (x0 * x0) as ::core::ffi::c_double;
		y2b += (*y0 * *y0) as ::core::ffi::c_double;
		xyb += (*y0 * x0) as ::core::ffi::c_double;
		bn += 1.;
	}
	if *y1 >= 0 as ::core::ffi::c_int {
		xb += x1 as ::core::ffi::c_double;
		yb += *y1 as ::core::ffi::c_double;
		x2b += (x1 * x1) as ::core::ffi::c_double;
		y2b += (*y1 * *y1) as ::core::ffi::c_double;
		xyb += (*y1 * x1) as ::core::ffi::c_double;
		bn += 1.;
	}
	let mut denom: ::core::ffi::c_double = bn * x2b - xb * xb;
	if denom > 0.0f64 {
		let mut a_0: ::core::ffi::c_double = (yb * x2b - xyb * xb) / denom;
		let mut b: ::core::ffi::c_double = (bn * xyb - xb * yb) / denom;
		*y0 = rint(a_0 + b * x0 as ::core::ffi::c_double) as ::core::ffi::c_int;
		*y1 = rint(a_0 + b * x1 as ::core::ffi::c_double) as ::core::ffi::c_int;
		if *y0 > 1023 as ::core::ffi::c_int {
			*y0 = 1023 as ::core::ffi::c_int;
		}
		if *y1 > 1023 as ::core::ffi::c_int {
			*y1 = 1023 as ::core::ffi::c_int;
		}
		if *y0 < 0 as ::core::ffi::c_int {
			*y0 = 0 as ::core::ffi::c_int;
		}
		if *y1 < 0 as ::core::ffi::c_int {
			*y1 = 0 as ::core::ffi::c_int;
		}
		return 0 as ::core::ffi::c_int;
	} else {
		*y0 = 0 as ::core::ffi::c_int;
		*y1 = 0 as ::core::ffi::c_int;
		return 1 as ::core::ffi::c_int;
	};
}
unsafe extern "C" fn inspect_error(
	mut x0: ::core::ffi::c_int,
	mut x1: ::core::ffi::c_int,
	mut y0: ::core::ffi::c_int,
	mut y1: ::core::ffi::c_int,
	mut mask: *const ::core::ffi::c_float,
	mut mdct: *const ::core::ffi::c_float,
	mut info: *mut vorbis_info_floor1
) -> ::core::ffi::c_int {
	let mut dy: ::core::ffi::c_int = y1 - y0;
	let mut adx: ::core::ffi::c_int = x1 - x0;
	let mut ady: ::core::ffi::c_int = dy.abs();
	let mut base: ::core::ffi::c_int = dy / adx;
	let mut sy: ::core::ffi::c_int = if dy < 0 as ::core::ffi::c_int {
		base - 1 as ::core::ffi::c_int
	} else {
		base + 1 as ::core::ffi::c_int
	};
	let mut x: ::core::ffi::c_int = x0;
	let mut y: ::core::ffi::c_int = y0;
	let mut err: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
	let mut val: ::core::ffi::c_int = vorbis_dBquant(mask.offset(x as isize));
	let mut mse: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
	let mut n: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
	ady -= (base * adx).abs();
	mse = y - val;
	mse *= mse;
	n += 1;
	if *mdct.offset(x as isize) + (*info).twofitatten >= *mask.offset(x as isize) {
		if y as ::core::ffi::c_float + (*info).maxover < val as ::core::ffi::c_float {
			return 1 as ::core::ffi::c_int;
		}
		if y as ::core::ffi::c_float - (*info).maxunder > val as ::core::ffi::c_float {
			return 1 as ::core::ffi::c_int;
		}
	}
	loop {
		x += 1;
		if !(x < x1) {
			break;
		}
		err = err + ady;
		if err >= adx {
			err -= adx;
			y += sy;
		} else {
			y += base;
		}
		val = vorbis_dBquant(mask.offset(x as isize));
		mse += (y - val) * (y - val);
		n += 1;
		if *mdct.offset(x as isize) + (*info).twofitatten >= *mask.offset(x as isize) {
			if val != 0 {
				if y as ::core::ffi::c_float + (*info).maxover < val as ::core::ffi::c_float {
					return 1 as ::core::ffi::c_int;
				}
				if y as ::core::ffi::c_float - (*info).maxunder > val as ::core::ffi::c_float {
					return 1 as ::core::ffi::c_int;
				}
			}
		}
	}
	if (*info).maxover * (*info).maxover / n as ::core::ffi::c_float > (*info).maxerr {
		return 0 as ::core::ffi::c_int;
	}
	if (*info).maxunder * (*info).maxunder / n as ::core::ffi::c_float > (*info).maxerr {
		return 0 as ::core::ffi::c_int;
	}
	if (mse / n) as ::core::ffi::c_float > (*info).maxerr {
		return 1 as ::core::ffi::c_int;
	}
	return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn post_Y(
	mut A: *mut ::core::ffi::c_int,
	mut B: *mut ::core::ffi::c_int,
	mut pos: ::core::ffi::c_int
) -> ::core::ffi::c_int {
	if *A.offset(pos as isize) < 0 as ::core::ffi::c_int {
		return *B.offset(pos as isize);
	}
	if *B.offset(pos as isize) < 0 as ::core::ffi::c_int {
		return *A.offset(pos as isize);
	}
	return *A.offset(pos as isize) + *B.offset(pos as isize) >> 1 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn floor1_fit(
	mut vb: *mut vorbis_block,
	mut look: *mut vorbis_look_floor1,
	mut logmdct: *const ::core::ffi::c_float,
	mut logmask: *const ::core::ffi::c_float
) -> *mut ::core::ffi::c_int {
	let mut i: ::core::ffi::c_long = 0;
	let mut j: ::core::ffi::c_long = 0;
	let mut info: *mut vorbis_info_floor1 = (*look).vi;
	let mut n: ::core::ffi::c_long = (*look).n as ::core::ffi::c_long;
	let mut posts: ::core::ffi::c_long = (*look).posts as ::core::ffi::c_long;
	let mut nonzero: ::core::ffi::c_long = 0 as ::core::ffi::c_long;
	let mut fits: [lsfit_acc; 64] = [lsfit_acc {
		x0: 0,
		x1: 0,
		xa: 0,
		ya: 0,
		x2a: 0,
		y2a: 0,
		xya: 0,
		an: 0,
		xb: 0,
		yb: 0,
		x2b: 0,
		y2b: 0,
		xyb: 0,
		bn: 0
	}; 64];
	let mut fit_valueA: [::core::ffi::c_int; 65] = [0; 65];
	let mut fit_valueB: [::core::ffi::c_int; 65] = [0; 65];
	let mut loneighbor: [::core::ffi::c_int; 65] = [0; 65];
	let mut hineighbor: [::core::ffi::c_int; 65] = [0; 65];
	let mut output: *mut ::core::ffi::c_int = ::core::ptr::null_mut::<::core::ffi::c_int>();
	let mut memo: [::core::ffi::c_int; 65] = [0; 65];
	i = 0 as ::core::ffi::c_long;
	while i < posts {
		fit_valueA[i as usize] = -(200 as ::core::ffi::c_int);
		i += 1;
	}
	i = 0 as ::core::ffi::c_long;
	while i < posts {
		fit_valueB[i as usize] = -(200 as ::core::ffi::c_int);
		i += 1;
	}
	i = 0 as ::core::ffi::c_long;
	while i < posts {
		loneighbor[i as usize] = 0 as ::core::ffi::c_int;
		i += 1;
	}
	i = 0 as ::core::ffi::c_long;
	while i < posts {
		hineighbor[i as usize] = 1 as ::core::ffi::c_int;
		i += 1;
	}
	i = 0 as ::core::ffi::c_long;
	while i < posts {
		memo[i as usize] = -(1 as ::core::ffi::c_int);
		i += 1;
	}
	if posts == 0 as ::core::ffi::c_long {
		nonzero += accumulate_fit(
			logmask,
			logmdct,
			0 as ::core::ffi::c_int,
			n as ::core::ffi::c_int,
			&raw mut fits as *mut lsfit_acc,
			n as ::core::ffi::c_int,
			info
		) as ::core::ffi::c_long;
	} else {
		i = 0 as ::core::ffi::c_long;
		while i < posts - 1 as ::core::ffi::c_long {
			nonzero += accumulate_fit(
				logmask,
				logmdct,
				(*look).sorted_index[i as usize],
				(*look).sorted_index[(i + 1 as ::core::ffi::c_long) as usize],
				(&raw mut fits as *mut lsfit_acc).offset(i as isize),
				n as ::core::ffi::c_int,
				info
			) as ::core::ffi::c_long;
			i += 1;
		}
	}
	if nonzero != 0 {
		let mut y0: ::core::ffi::c_int = -(200 as ::core::ffi::c_int);
		let mut y1: ::core::ffi::c_int = -(200 as ::core::ffi::c_int);
		fit_line(
			&raw mut fits as *mut lsfit_acc,
			(posts - 1 as ::core::ffi::c_long) as ::core::ffi::c_int,
			&raw mut y0,
			&raw mut y1,
			info
		);
		fit_valueA[0 as ::core::ffi::c_int as usize] = y0;
		fit_valueB[0 as ::core::ffi::c_int as usize] = y0;
		fit_valueB[1 as ::core::ffi::c_int as usize] = y1;
		fit_valueA[1 as ::core::ffi::c_int as usize] = y1;
		i = 2 as ::core::ffi::c_long;
		while i < posts {
			let mut sortpos: ::core::ffi::c_int = (*look).reverse_index[i as usize];
			let mut ln: ::core::ffi::c_int = loneighbor[sortpos as usize];
			let mut hn: ::core::ffi::c_int = hineighbor[sortpos as usize];
			if memo[ln as usize] != hn {
				let mut lsortpos: ::core::ffi::c_int = (*look).reverse_index[ln as usize];
				let mut hsortpos: ::core::ffi::c_int = (*look).reverse_index[hn as usize];
				memo[ln as usize] = hn;
				let mut lx: ::core::ffi::c_int = (*info).postlist[ln as usize];
				let mut hx: ::core::ffi::c_int = (*info).postlist[hn as usize];
				let mut ly: ::core::ffi::c_int = post_Y(
					&raw mut fit_valueA as *mut ::core::ffi::c_int,
					&raw mut fit_valueB as *mut ::core::ffi::c_int,
					ln
				);
				let mut hy: ::core::ffi::c_int = post_Y(
					&raw mut fit_valueA as *mut ::core::ffi::c_int,
					&raw mut fit_valueB as *mut ::core::ffi::c_int,
					hn
				);
				if ly == -(1 as ::core::ffi::c_int) || hy == -(1 as ::core::ffi::c_int) {
					exit(1 as ::core::ffi::c_int);
				}
				if inspect_error(lx, hx, ly, hy, logmask, logmdct, info) != 0 {
					let mut ly0: ::core::ffi::c_int = -(200 as ::core::ffi::c_int);
					let mut ly1: ::core::ffi::c_int = -(200 as ::core::ffi::c_int);
					let mut hy0: ::core::ffi::c_int = -(200 as ::core::ffi::c_int);
					let mut hy1: ::core::ffi::c_int = -(200 as ::core::ffi::c_int);
					let mut ret0: ::core::ffi::c_int = fit_line(
						(&raw mut fits as *mut lsfit_acc).offset(lsortpos as isize),
						sortpos - lsortpos,
						&raw mut ly0,
						&raw mut ly1,
						info
					);
					let mut ret1: ::core::ffi::c_int = fit_line(
						(&raw mut fits as *mut lsfit_acc).offset(sortpos as isize),
						hsortpos - sortpos,
						&raw mut hy0,
						&raw mut hy1,
						info
					);
					if ret0 != 0 {
						ly0 = ly;
						ly1 = hy0;
					}
					if ret1 != 0 {
						hy0 = ly1;
						hy1 = hy;
					}
					if ret0 != 0 && ret1 != 0 {
						fit_valueA[i as usize] = -(200 as ::core::ffi::c_int);
						fit_valueB[i as usize] = -(200 as ::core::ffi::c_int);
					} else {
						fit_valueB[ln as usize] = ly0;
						if ln == 0 as ::core::ffi::c_int {
							fit_valueA[ln as usize] = ly0;
						}
						fit_valueA[i as usize] = ly1;
						fit_valueB[i as usize] = hy0;
						fit_valueA[hn as usize] = hy1;
						if hn == 1 as ::core::ffi::c_int {
							fit_valueB[hn as usize] = hy1;
						}
						if ly1 >= 0 as ::core::ffi::c_int || hy0 >= 0 as ::core::ffi::c_int {
							j = (sortpos - 1 as ::core::ffi::c_int) as ::core::ffi::c_long;
							while j >= 0 as ::core::ffi::c_long {
								if !(hineighbor[j as usize] == hn) {
									break;
								}
								hineighbor[j as usize] = i as ::core::ffi::c_int;
								j -= 1;
							}
							j = (sortpos + 1 as ::core::ffi::c_int) as ::core::ffi::c_long;
							while j < posts {
								if !(loneighbor[j as usize] == ln) {
									break;
								}
								loneighbor[j as usize] = i as ::core::ffi::c_int;
								j += 1;
							}
						}
					}
				} else {
					fit_valueA[i as usize] = -(200 as ::core::ffi::c_int);
					fit_valueB[i as usize] = -(200 as ::core::ffi::c_int);
				}
			}
			i += 1;
		}
		output = _vorbis_block_alloc(
			vb,
			(::core::mem::size_of::<::core::ffi::c_int>() as usize).wrapping_mul(posts as usize)
				as ::core::ffi::c_long
		) as *mut ::core::ffi::c_int;
		*output.offset(0 as ::core::ffi::c_int as isize) = post_Y(
			&raw mut fit_valueA as *mut ::core::ffi::c_int,
			&raw mut fit_valueB as *mut ::core::ffi::c_int,
			0 as ::core::ffi::c_int
		);
		*output.offset(1 as ::core::ffi::c_int as isize) = post_Y(
			&raw mut fit_valueA as *mut ::core::ffi::c_int,
			&raw mut fit_valueB as *mut ::core::ffi::c_int,
			1 as ::core::ffi::c_int
		);
		i = 2 as ::core::ffi::c_long;
		while i < posts {
			let mut ln_0: ::core::ffi::c_int =
				(*look).loneighbor[(i - 2 as ::core::ffi::c_long) as usize];
			let mut hn_0: ::core::ffi::c_int =
				(*look).hineighbor[(i - 2 as ::core::ffi::c_long) as usize];
			let mut x0: ::core::ffi::c_int = (*info).postlist[ln_0 as usize];
			let mut x1: ::core::ffi::c_int = (*info).postlist[hn_0 as usize];
			let mut y0_0: ::core::ffi::c_int = *output.offset(ln_0 as isize);
			let mut y1_0: ::core::ffi::c_int = *output.offset(hn_0 as isize);
			let mut predicted: ::core::ffi::c_int =
				render_point(x0, x1, y0_0, y1_0, (*info).postlist[i as usize]);
			let mut vx: ::core::ffi::c_int = post_Y(
				&raw mut fit_valueA as *mut ::core::ffi::c_int,
				&raw mut fit_valueB as *mut ::core::ffi::c_int,
				i as ::core::ffi::c_int
			);
			if vx >= 0 as ::core::ffi::c_int && predicted != vx {
				*output.offset(i as isize) = vx;
			} else {
				*output.offset(i as isize) = predicted | 0x8000 as ::core::ffi::c_int;
			}
			i += 1;
		}
	}
	return output;
}
#[no_mangle]
pub unsafe extern "C" fn floor1_interpolate_fit(
	mut vb: *mut vorbis_block,
	mut look: *mut vorbis_look_floor1,
	mut A: *mut ::core::ffi::c_int,
	mut B: *mut ::core::ffi::c_int,
	mut del: ::core::ffi::c_int
) -> *mut ::core::ffi::c_int {
	let mut i: ::core::ffi::c_long = 0;
	let mut posts: ::core::ffi::c_long = (*look).posts as ::core::ffi::c_long;
	let mut output: *mut ::core::ffi::c_int = ::core::ptr::null_mut::<::core::ffi::c_int>();
	if !A.is_null() && !B.is_null() {
		output = _vorbis_block_alloc(
			vb,
			(::core::mem::size_of::<::core::ffi::c_int>() as usize).wrapping_mul(posts as usize)
				as ::core::ffi::c_long
		) as *mut ::core::ffi::c_int;
		i = 0 as ::core::ffi::c_long;
		while i < posts {
			*output.offset(i as isize) = (65536 as ::core::ffi::c_int - del)
				* (*A.offset(i as isize) & 0x7fff as ::core::ffi::c_int)
				+ del * (*B.offset(i as isize) & 0x7fff as ::core::ffi::c_int)
				+ 32768 as ::core::ffi::c_int
				>> 16 as ::core::ffi::c_int;
			if *A.offset(i as isize) & 0x8000 as ::core::ffi::c_int != 0
				&& *B.offset(i as isize) & 0x8000 as ::core::ffi::c_int != 0
			{
				*output.offset(i as isize) |= 0x8000 as ::core::ffi::c_int;
			}
			i += 1;
		}
	}
	return output;
}
#[no_mangle]
pub unsafe extern "C" fn floor1_encode(
	mut opb: *mut oggpack_buffer,
	mut vb: *mut vorbis_block,
	mut look: *mut vorbis_look_floor1,
	mut post: *mut ::core::ffi::c_int,
	mut ilogmask: *mut ::core::ffi::c_int
) -> ::core::ffi::c_int {
	let mut i: ::core::ffi::c_long = 0;
	let mut j: ::core::ffi::c_long = 0;
	let mut info: *mut vorbis_info_floor1 = (*look).vi;
	let mut posts: ::core::ffi::c_long = (*look).posts as ::core::ffi::c_long;
	let mut ci: *mut codec_setup_info = (*(*(*vb).vd).vi).codec_setup as *mut codec_setup_info;
	let mut out: [::core::ffi::c_int; 65] = [0; 65];
	let mut sbooks: *mut *mut static_codebook =
		&raw mut (*ci).book_param as *mut *mut static_codebook;
	let mut books: *mut codebook = (*ci).fullbooks;
	if !post.is_null() {
		i = 0 as ::core::ffi::c_long;
		while i < posts {
			let mut val: ::core::ffi::c_int =
				*post.offset(i as isize) & 0x7fff as ::core::ffi::c_int;
			match (*info).mult {
				1 => {
					val >>= 2 as ::core::ffi::c_int;
				}
				2 => {
					val >>= 3 as ::core::ffi::c_int;
				}
				3 => {
					val /= 12 as ::core::ffi::c_int;
				}
				4 => {
					val >>= 4 as ::core::ffi::c_int;
				}
				_ => {}
			}
			*post.offset(i as isize) =
				val | *post.offset(i as isize) & 0x8000 as ::core::ffi::c_int;
			i += 1;
		}
		out[0 as ::core::ffi::c_int as usize] = *post.offset(0 as ::core::ffi::c_int as isize);
		out[1 as ::core::ffi::c_int as usize] = *post.offset(1 as ::core::ffi::c_int as isize);
		i = 2 as ::core::ffi::c_long;
		while i < posts {
			let mut ln: ::core::ffi::c_int =
				(*look).loneighbor[(i - 2 as ::core::ffi::c_long) as usize];
			let mut hn: ::core::ffi::c_int =
				(*look).hineighbor[(i - 2 as ::core::ffi::c_long) as usize];
			let mut x0: ::core::ffi::c_int = (*info).postlist[ln as usize];
			let mut x1: ::core::ffi::c_int = (*info).postlist[hn as usize];
			let mut y0: ::core::ffi::c_int = *post.offset(ln as isize);
			let mut y1: ::core::ffi::c_int = *post.offset(hn as isize);
			let mut predicted: ::core::ffi::c_int =
				render_point(x0, x1, y0, y1, (*info).postlist[i as usize]);
			if *post.offset(i as isize) & 0x8000 as ::core::ffi::c_int != 0
				|| predicted == *post.offset(i as isize)
			{
				*post.offset(i as isize) = predicted | 0x8000 as ::core::ffi::c_int;
				out[i as usize] = 0 as ::core::ffi::c_int;
			} else {
				let mut headroom: ::core::ffi::c_int = if (*look).quant_q - predicted < predicted {
					(*look).quant_q - predicted
				} else {
					predicted
				};
				let mut val_0: ::core::ffi::c_int = *post.offset(i as isize) - predicted;
				if val_0 < 0 as ::core::ffi::c_int {
					if val_0 < -headroom {
						val_0 = headroom - val_0 - 1 as ::core::ffi::c_int;
					} else {
						val_0 = -(1 as ::core::ffi::c_int) - val_0 * 2 as ::core::ffi::c_int;
					}
				} else if val_0 >= headroom {
					val_0 = val_0 + headroom;
				} else {
					val_0 <<= 1 as ::core::ffi::c_int;
				}
				out[i as usize] = val_0;
				*post.offset(ln as isize) &= 0x7fff as ::core::ffi::c_int;
				*post.offset(hn as isize) &= 0x7fff as ::core::ffi::c_int;
			}
			i += 1;
		}
		oggpack_write(opb, 1 as ::core::ffi::c_ulong, 1 as ::core::ffi::c_int);
		(*look).frames += 1;
		(*look).postbits += (ov_ilog(((*look).quant_q - 1 as ::core::ffi::c_int) as ogg_uint32_t)
			* 2 as ::core::ffi::c_int) as ::core::ffi::c_long;
		oggpack_write(
			opb,
			out[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_ulong,
			ov_ilog(((*look).quant_q - 1 as ::core::ffi::c_int) as ogg_uint32_t)
		);
		oggpack_write(
			opb,
			out[1 as ::core::ffi::c_int as usize] as ::core::ffi::c_ulong,
			ov_ilog(((*look).quant_q - 1 as ::core::ffi::c_int) as ogg_uint32_t)
		);
		i = 0 as ::core::ffi::c_long;
		j = 2 as ::core::ffi::c_long;
		while i < (*info).partitions as ::core::ffi::c_long {
			let mut class: ::core::ffi::c_int = (*info).partitionclass[i as usize];
			let mut cdim: ::core::ffi::c_int = (*info).class_dim[class as usize];
			let mut csubbits: ::core::ffi::c_int = (*info).class_subs[class as usize];
			let mut csub: ::core::ffi::c_int = (1 as ::core::ffi::c_int) << csubbits;
			let mut bookas: [::core::ffi::c_int; 8] = [
				0 as ::core::ffi::c_int,
				0 as ::core::ffi::c_int,
				0 as ::core::ffi::c_int,
				0 as ::core::ffi::c_int,
				0 as ::core::ffi::c_int,
				0 as ::core::ffi::c_int,
				0 as ::core::ffi::c_int,
				0 as ::core::ffi::c_int
			];
			let mut cval: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
			let mut cshift: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
			let mut k: ::core::ffi::c_int = 0;
			let mut l: ::core::ffi::c_int = 0;
			if csubbits != 0 {
				let mut maxval: [::core::ffi::c_int; 8] = [
					0 as ::core::ffi::c_int,
					0 as ::core::ffi::c_int,
					0 as ::core::ffi::c_int,
					0 as ::core::ffi::c_int,
					0 as ::core::ffi::c_int,
					0 as ::core::ffi::c_int,
					0 as ::core::ffi::c_int,
					0 as ::core::ffi::c_int
				];
				k = 0 as ::core::ffi::c_int;
				while k < csub {
					let mut booknum: ::core::ffi::c_int =
						(*info).class_subbook[class as usize][k as usize];
					if booknum < 0 as ::core::ffi::c_int {
						maxval[k as usize] = 1 as ::core::ffi::c_int;
					} else {
						maxval[k as usize] = (**sbooks
							.offset((*info).class_subbook[class as usize][k as usize] as isize))
						.entries as ::core::ffi::c_int;
					}
					k += 1;
				}
				k = 0 as ::core::ffi::c_int;
				while k < cdim {
					l = 0 as ::core::ffi::c_int;
					while l < csub {
						let mut val_1: ::core::ffi::c_int =
							out[(j + k as ::core::ffi::c_long) as usize];
						if val_1 < maxval[l as usize] {
							bookas[k as usize] = l;
							break;
						} else {
							l += 1;
						}
					}
					cval |= bookas[k as usize] << cshift;
					cshift += csubbits;
					k += 1;
				}
				(*look).phrasebits += vorbis_book_encode(
					books.offset((*info).class_book[class as usize] as isize),
					cval,
					opb
				) as ::core::ffi::c_long;
			}
			k = 0 as ::core::ffi::c_int;
			while k < cdim {
				let mut book: ::core::ffi::c_int =
					(*info).class_subbook[class as usize][bookas[k as usize] as usize];
				if book >= 0 as ::core::ffi::c_int {
					if (out[(j + k as ::core::ffi::c_long) as usize] as ::core::ffi::c_long)
						< (*books.offset(book as isize)).entries
					{
						(*look).postbits += vorbis_book_encode(
							books.offset(book as isize),
							out[(j + k as ::core::ffi::c_long) as usize],
							opb
						) as ::core::ffi::c_long;
					}
				}
				k += 1;
			}
			j += cdim as ::core::ffi::c_long;
			i += 1;
		}
		let mut hx: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
		let mut lx: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
		let mut ly: ::core::ffi::c_int =
			*post.offset(0 as ::core::ffi::c_int as isize) * (*info).mult;
		let mut n: ::core::ffi::c_int =
			((*ci).blocksizes[(*vb).W as usize] / 2 as ::core::ffi::c_long) as ::core::ffi::c_int;
		j = 1 as ::core::ffi::c_long;
		while j < (*look).posts as ::core::ffi::c_long {
			let mut current: ::core::ffi::c_int = (*look).forward_index[j as usize];
			let mut hy: ::core::ffi::c_int =
				*post.offset(current as isize) & 0x7fff as ::core::ffi::c_int;
			if hy == *post.offset(current as isize) {
				hy *= (*info).mult;
				hx = (*info).postlist[current as usize];
				render_line0(n, lx, hx, ly, hy, ilogmask);
				lx = hx;
				ly = hy;
			}
			j += 1;
		}
		j = hx as ::core::ffi::c_long;
		while j < ((*vb).pcmend / 2 as ::core::ffi::c_int) as ::core::ffi::c_long {
			*ilogmask.offset(j as isize) = ly;
			j += 1;
		}
		return 1 as ::core::ffi::c_int;
	} else {
		oggpack_write(opb, 0 as ::core::ffi::c_ulong, 1 as ::core::ffi::c_int);
		memset(
			ilogmask as *mut ::core::ffi::c_void,
			0 as ::core::ffi::c_int,
			(((*vb).pcmend / 2 as ::core::ffi::c_int) as size_t)
				.wrapping_mul(::core::mem::size_of::<::core::ffi::c_int>() as size_t)
		);
		return 0 as ::core::ffi::c_int;
	};
}
unsafe extern "C" fn floor1_inverse1(
	mut vb: *mut vorbis_block,
	mut in_0: *mut ::core::ffi::c_void
) -> *mut ::core::ffi::c_void {
	let mut current_block: u64;
	let mut look: *mut vorbis_look_floor1 = in_0 as *mut vorbis_look_floor1;
	let mut info: *mut vorbis_info_floor1 = (*look).vi;
	let mut ci: *mut codec_setup_info = (*(*(*vb).vd).vi).codec_setup as *mut codec_setup_info;
	let mut i: ::core::ffi::c_int = 0;
	let mut j: ::core::ffi::c_int = 0;
	let mut k: ::core::ffi::c_int = 0;
	let mut books: *mut dec_codebook = (*ci).decbooks;
	if oggpack_read(&raw mut (*vb).opb, 1 as ::core::ffi::c_int) == 1 as ::core::ffi::c_long {
		let mut fit_value: *mut ::core::ffi::c_int = _vorbis_block_alloc(
			vb,
			((*look).posts as usize)
				.wrapping_mul(::core::mem::size_of::<::core::ffi::c_int>() as usize)
				as ::core::ffi::c_long
		) as *mut ::core::ffi::c_int;
		*fit_value.offset(0 as ::core::ffi::c_int as isize) = oggpack_read(
			&raw mut (*vb).opb,
			ov_ilog(((*look).quant_q - 1 as ::core::ffi::c_int) as ogg_uint32_t)
		) as ::core::ffi::c_int;
		*fit_value.offset(1 as ::core::ffi::c_int as isize) = oggpack_read(
			&raw mut (*vb).opb,
			ov_ilog(((*look).quant_q - 1 as ::core::ffi::c_int) as ogg_uint32_t)
		) as ::core::ffi::c_int;
		i = 0 as ::core::ffi::c_int;
		j = 2 as ::core::ffi::c_int;
		's_23: loop {
			if !(i < (*info).partitions) {
				current_block = 2838571290723028321;
				break;
			}
			let mut class: ::core::ffi::c_int = (*info).partitionclass[i as usize];
			let mut cdim: ::core::ffi::c_int = (*info).class_dim[class as usize];
			let mut csubbits: ::core::ffi::c_int = (*info).class_subs[class as usize];
			let mut csub: ::core::ffi::c_int = (1 as ::core::ffi::c_int) << csubbits;
			let mut cval: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
			if csubbits != 0 {
				cval = vorbis_book_decode(
					books.offset((*info).class_book[class as usize] as isize),
					&raw mut (*vb).opb
				) as ::core::ffi::c_int;
				if cval == -(1 as ::core::ffi::c_int) {
					current_block = 610334525548354778;
					break;
				}
			}
			k = 0 as ::core::ffi::c_int;
			while k < cdim {
				let mut book: ::core::ffi::c_int = (*info).class_subbook[class as usize]
					[(cval & csub - 1 as ::core::ffi::c_int) as usize];
				cval >>= csubbits;
				if book >= 0 as ::core::ffi::c_int {
					let ref mut fresh0 = *fit_value.offset((j + k) as isize);
					*fresh0 = vorbis_book_decode(books.offset(book as isize), &raw mut (*vb).opb)
						as ::core::ffi::c_int;
					if *fresh0 == -(1 as ::core::ffi::c_int) {
						current_block = 610334525548354778;
						break 's_23;
					}
				} else {
					*fit_value.offset((j + k) as isize) = 0 as ::core::ffi::c_int;
				}
				k += 1;
			}
			j += cdim;
			i += 1;
		}
		match current_block {
			610334525548354778 => {}
			_ => {
				i = 2 as ::core::ffi::c_int;
				while i < (*look).posts {
					let mut predicted: ::core::ffi::c_int = render_point(
						(*info).postlist
							[(*look).loneighbor[(i - 2 as ::core::ffi::c_int) as usize] as usize],
						(*info).postlist
							[(*look).hineighbor[(i - 2 as ::core::ffi::c_int) as usize] as usize],
						*fit_value.offset(
							(*look).loneighbor[(i - 2 as ::core::ffi::c_int) as usize] as isize
						),
						*fit_value.offset(
							(*look).hineighbor[(i - 2 as ::core::ffi::c_int) as usize] as isize
						),
						(*info).postlist[i as usize]
					);
					let mut hiroom: ::core::ffi::c_int = (*look).quant_q - predicted;
					let mut loroom: ::core::ffi::c_int = predicted;
					let mut room: ::core::ffi::c_int =
						(if hiroom < loroom { hiroom } else { loroom }) << 1 as ::core::ffi::c_int;
					let mut val: ::core::ffi::c_int = *fit_value.offset(i as isize);
					if val != 0 {
						if val >= room {
							if hiroom > loroom {
								val = val - loroom;
							} else {
								val = -(1 as ::core::ffi::c_int) - (val - hiroom);
							}
						} else if val & 1 as ::core::ffi::c_int != 0 {
							val = -(val + 1 as ::core::ffi::c_int >> 1 as ::core::ffi::c_int);
						} else {
							val >>= 1 as ::core::ffi::c_int;
						}
						*fit_value.offset(i as isize) =
							val + predicted & 0x7fff as ::core::ffi::c_int;
						*fit_value.offset(
							(*look).loneighbor[(i - 2 as ::core::ffi::c_int) as usize] as isize
						) &= 0x7fff as ::core::ffi::c_int;
						*fit_value.offset(
							(*look).hineighbor[(i - 2 as ::core::ffi::c_int) as usize] as isize
						) &= 0x7fff as ::core::ffi::c_int;
					} else {
						*fit_value.offset(i as isize) = predicted | 0x8000 as ::core::ffi::c_int;
					}
					i += 1;
				}
				return fit_value as *mut ::core::ffi::c_void;
			}
		}
	}
	return ::core::ptr::null_mut::<::core::ffi::c_void>();
}
unsafe extern "C" fn floor1_inverse2(
	mut vb: *mut vorbis_block,
	mut in_0: *mut ::core::ffi::c_void,
	mut memo: *mut ::core::ffi::c_void,
	mut out: *mut ::core::ffi::c_float
) -> ::core::ffi::c_int {
	let mut look: *mut vorbis_look_floor1 = in_0 as *mut vorbis_look_floor1;
	let mut info: *mut vorbis_info_floor1 = (*look).vi;
	let mut ci: *mut codec_setup_info = (*(*(*vb).vd).vi).codec_setup as *mut codec_setup_info;
	let mut n: ::core::ffi::c_int =
		((*ci).blocksizes[(*vb).W as usize] / 2 as ::core::ffi::c_long) as ::core::ffi::c_int;
	let mut j: ::core::ffi::c_int = 0;
	if !memo.is_null() {
		let mut fit_value: *mut ::core::ffi::c_int = memo as *mut ::core::ffi::c_int;
		let mut hx: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
		let mut lx: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
		let mut ly: ::core::ffi::c_int =
			*fit_value.offset(0 as ::core::ffi::c_int as isize) * (*info).mult;
		ly = if ly < 0 as ::core::ffi::c_int {
			0 as ::core::ffi::c_int
		} else if ly > 255 as ::core::ffi::c_int {
			255 as ::core::ffi::c_int
		} else {
			ly
		};
		j = 1 as ::core::ffi::c_int;
		while j < (*look).posts {
			let mut current: ::core::ffi::c_int = (*look).forward_index[j as usize];
			let mut hy: ::core::ffi::c_int =
				*fit_value.offset(current as isize) & 0x7fff as ::core::ffi::c_int;
			if hy == *fit_value.offset(current as isize) {
				hx = (*info).postlist[current as usize];
				hy *= (*info).mult;
				hy = if hy < 0 as ::core::ffi::c_int {
					0 as ::core::ffi::c_int
				} else if hy > 255 as ::core::ffi::c_int {
					255 as ::core::ffi::c_int
				} else {
					hy
				};
				render_line(n, lx, hx, ly, hy, out);
				lx = hx;
				ly = hy;
			}
			j += 1;
		}
		j = hx;
		while j < n {
			*out.offset(j as isize) *= FLOOR1_fromdB_LOOKUP[ly as usize];
			j += 1;
		}
		return 1 as ::core::ffi::c_int;
	}
	memset(
		out as *mut ::core::ffi::c_void,
		0 as ::core::ffi::c_int,
		(::core::mem::size_of::<::core::ffi::c_float>() as size_t).wrapping_mul(n as size_t)
	);
	return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub static mut floor1_exportbundle: vorbis_func_floor = vorbis_func_floor {
	pack: Some(
		floor1_pack as unsafe extern "C" fn(*mut ::core::ffi::c_void, *mut oggpack_buffer) -> ()
	),
	unpack: Some(
		floor1_unpack
			as unsafe extern "C" fn(
				*mut vorbis_info,
				*mut oggpack_buffer
			) -> *mut ::core::ffi::c_void
	),
	look: Some(
		floor1_look
			as unsafe extern "C" fn(
				*mut vorbis_dsp_state,
				*mut ::core::ffi::c_void
			) -> *mut ::core::ffi::c_void
	),
	free_info: Some(floor1_free_info as unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()),
	free_look: Some(floor1_free_look as unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()),
	inverse1: Some(
		floor1_inverse1
			as unsafe extern "C" fn(
				*mut vorbis_block,
				*mut ::core::ffi::c_void
			) -> *mut ::core::ffi::c_void
	),
	inverse2: Some(
		floor1_inverse2
			as unsafe extern "C" fn(
				*mut vorbis_block,
				*mut ::core::ffi::c_void,
				*mut ::core::ffi::c_void,
				*mut ::core::ffi::c_float
			) -> ::core::ffi::c_int
	)
};
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
