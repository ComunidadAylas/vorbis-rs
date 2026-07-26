extern "C" {
	fn memset(
		__s: *mut ::core::ffi::c_void,
		__c: ::core::ffi::c_int,
		__n: size_t
	) -> *mut ::core::ffi::c_void;
	fn rint(__x: ::core::ffi::c_double) -> ::core::ffi::c_double;
	fn oggpack_writetrunc(b: *mut oggpack_buffer, bits: ::core::ffi::c_long);
	fn oggpack_write(b: *mut oggpack_buffer, value: ::core::ffi::c_ulong, bits: ::core::ffi::c_int);
	fn oggpack_bytes(b: *mut oggpack_buffer) -> ::core::ffi::c_long;
	fn oggpack_get_buffer(b: *mut oggpack_buffer) -> *mut ::core::ffi::c_uchar;
}
use ogg_c2rust_sys::{ogg_packet, oggpack_buffer};

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
pub struct vorbis_block_internal {
	pub pcmdelay: *mut *mut ::core::ffi::c_float,
	pub ampmax: ::core::ffi::c_float,
	pub blocktype: ::core::ffi::c_int,
	pub packetblob: [*mut oggpack_buffer; 15]
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
#[no_mangle]
pub unsafe extern "C" fn vorbis_bitrate_init(
	mut vi: *mut vorbis_info,
	mut bm: *mut bitrate_manager_state
) {
	let mut ci: *mut codec_setup_info = (*vi).codec_setup as *mut codec_setup_info;
	let mut bi: *mut bitrate_manager_info = &raw mut (*ci).bi;
	memset(
		bm as *mut ::core::ffi::c_void,
		0 as ::core::ffi::c_int,
		::core::mem::size_of::<bitrate_manager_state>() as size_t
	);
	if !bi.is_null() && (*bi).reservoir_bits > 0 as ::core::ffi::c_long {
		let mut ratesamples: ::core::ffi::c_long = (*vi).rate;
		let mut halfsamples: ::core::ffi::c_int =
			((*ci).blocksizes[0 as ::core::ffi::c_int as usize] >> 1 as ::core::ffi::c_int)
				as ::core::ffi::c_int;
		(*bm).short_per_long = (*ci).blocksizes[1 as ::core::ffi::c_int as usize]
			/ (*ci).blocksizes[0 as ::core::ffi::c_int as usize];
		(*bm).managed = 1 as ::core::ffi::c_int;
		(*bm).avg_bitsper = rint(
			1.0f64 * (*bi).avg_rate as ::core::ffi::c_double * halfsamples as ::core::ffi::c_double
				/ ratesamples as ::core::ffi::c_double
		) as ::core::ffi::c_long;
		(*bm).min_bitsper = rint(
			1.0f64 * (*bi).min_rate as ::core::ffi::c_double * halfsamples as ::core::ffi::c_double
				/ ratesamples as ::core::ffi::c_double
		) as ::core::ffi::c_long;
		(*bm).max_bitsper = rint(
			1.0f64 * (*bi).max_rate as ::core::ffi::c_double * halfsamples as ::core::ffi::c_double
				/ ratesamples as ::core::ffi::c_double
		) as ::core::ffi::c_long;
		(*bm).avgfloat = (PACKETBLOBS / 2 as ::core::ffi::c_int) as ::core::ffi::c_double;
		let mut desired_fill: ::core::ffi::c_long = ((*bi).reservoir_bits as ::core::ffi::c_double
			* (*bi).reservoir_bias)
			as ::core::ffi::c_long;
		(*bm).minmax_reservoir = desired_fill;
		(*bm).avg_reservoir = desired_fill;
	}
}
#[no_mangle]
pub unsafe extern "C" fn vorbis_bitrate_clear(mut bm: *mut bitrate_manager_state) {
	memset(
		bm as *mut ::core::ffi::c_void,
		0 as ::core::ffi::c_int,
		::core::mem::size_of::<bitrate_manager_state>() as size_t
	);
}
#[no_mangle]
pub unsafe extern "C" fn vorbis_bitrate_managed(mut vb: *mut vorbis_block) -> ::core::ffi::c_int {
	let mut vd: *mut vorbis_dsp_state = (*vb).vd;
	let mut b: *mut private_state = (*vd).backend_state as *mut private_state;
	let mut bm: *mut bitrate_manager_state = &raw mut (*b).bms;
	if !bm.is_null() && (*bm).managed != 0 {
		return 1 as ::core::ffi::c_int;
	}
	return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn vorbis_bitrate_addblock(mut vb: *mut vorbis_block) -> ::core::ffi::c_int {
	let mut vbi: *mut vorbis_block_internal = (*vb).internal as *mut vorbis_block_internal;
	let mut vd: *mut vorbis_dsp_state = (*vb).vd;
	let mut b: *mut private_state = (*vd).backend_state as *mut private_state;
	let mut bm: *mut bitrate_manager_state = &raw mut (*b).bms;
	let mut vi: *mut vorbis_info = (*vd).vi;
	let mut ci: *mut codec_setup_info = (*vi).codec_setup as *mut codec_setup_info;
	let mut bi: *mut bitrate_manager_info = &raw mut (*ci).bi;
	let mut choice: ::core::ffi::c_int = rint((*bm).avgfloat) as ::core::ffi::c_int;
	let mut this_bits: ::core::ffi::c_long =
		oggpack_bytes((*vbi).packetblob[choice as usize]) * 8 as ::core::ffi::c_long;
	let mut min_target_bits: ::core::ffi::c_long = if (*vb).W != 0 {
		(*bm).min_bitsper * (*bm).short_per_long
	} else {
		(*bm).min_bitsper
	};
	let mut max_target_bits: ::core::ffi::c_long = if (*vb).W != 0 {
		(*bm).max_bitsper * (*bm).short_per_long
	} else {
		(*bm).max_bitsper
	};
	let mut samples: ::core::ffi::c_int =
		((*ci).blocksizes[(*vb).W as usize] >> 1 as ::core::ffi::c_int) as ::core::ffi::c_int;
	let mut desired_fill: ::core::ffi::c_long = ((*bi).reservoir_bits as ::core::ffi::c_double
		* (*bi).reservoir_bias) as ::core::ffi::c_long;
	if (*bm).managed == 0 {
		if !(*bm).vb.is_null() {
			return -(1 as ::core::ffi::c_int);
		}
		(*bm).vb = vb;
		return 0 as ::core::ffi::c_int;
	}
	(*bm).vb = vb;
	if (*bm).avg_bitsper > 0 as ::core::ffi::c_long {
		let mut slew: ::core::ffi::c_double = 0.0f64;
		let mut avg_target_bits: ::core::ffi::c_long = if (*vb).W != 0 {
			(*bm).avg_bitsper * (*bm).short_per_long
		} else {
			(*bm).avg_bitsper
		};
		let mut slewlimit: ::core::ffi::c_double = 15.0f64 / (*bi).slew_damp;
		if (*bm).avg_reservoir + (this_bits - avg_target_bits) > desired_fill {
			while choice > 0 as ::core::ffi::c_int
				&& this_bits > avg_target_bits
				&& (*bm).avg_reservoir + (this_bits - avg_target_bits) > desired_fill
			{
				choice -= 1;
				this_bits =
					oggpack_bytes((*vbi).packetblob[choice as usize]) * 8 as ::core::ffi::c_long;
			}
		} else if (*bm).avg_reservoir + (this_bits - avg_target_bits) < desired_fill {
			while (choice + 1 as ::core::ffi::c_int) < PACKETBLOBS
				&& this_bits < avg_target_bits
				&& (*bm).avg_reservoir + (this_bits - avg_target_bits) < desired_fill
			{
				choice += 1;
				this_bits =
					oggpack_bytes((*vbi).packetblob[choice as usize]) * 8 as ::core::ffi::c_long;
			}
		}
		slew = rint(choice as ::core::ffi::c_double - (*bm).avgfloat)
			/ samples as ::core::ffi::c_double
			* (*vi).rate as ::core::ffi::c_double;
		if slew < -slewlimit {
			slew = -slewlimit;
		}
		if slew > slewlimit {
			slew = slewlimit;
		}
		(*bm).avgfloat +=
			slew / (*vi).rate as ::core::ffi::c_double * samples as ::core::ffi::c_double;
		choice = rint((*bm).avgfloat) as ::core::ffi::c_int;
		this_bits = oggpack_bytes((*vbi).packetblob[choice as usize]) * 8 as ::core::ffi::c_long;
	}
	if (*bm).min_bitsper > 0 as ::core::ffi::c_long {
		if this_bits < min_target_bits {
			while (*bm).minmax_reservoir - (min_target_bits - this_bits) < 0 as ::core::ffi::c_long
			{
				choice += 1;
				if choice >= PACKETBLOBS {
					break;
				}
				this_bits =
					oggpack_bytes((*vbi).packetblob[choice as usize]) * 8 as ::core::ffi::c_long;
			}
		}
	}
	if (*bm).max_bitsper > 0 as ::core::ffi::c_long {
		if this_bits > max_target_bits {
			while (*bm).minmax_reservoir + (this_bits - max_target_bits) > (*bi).reservoir_bits {
				choice -= 1;
				if choice < 0 as ::core::ffi::c_int {
					break;
				}
				this_bits =
					oggpack_bytes((*vbi).packetblob[choice as usize]) * 8 as ::core::ffi::c_long;
			}
		}
	}
	if choice < 0 as ::core::ffi::c_int {
		let mut maxsize: ::core::ffi::c_long = (max_target_bits
			+ ((*bi).reservoir_bits - (*bm).minmax_reservoir))
			/ 8 as ::core::ffi::c_long;
		choice = 0 as ::core::ffi::c_int;
		(*bm).choice = choice;
		if oggpack_bytes((*vbi).packetblob[choice as usize]) > maxsize {
			oggpack_writetrunc(
				(*vbi).packetblob[choice as usize],
				maxsize * 8 as ::core::ffi::c_long
			);
			this_bits =
				oggpack_bytes((*vbi).packetblob[choice as usize]) * 8 as ::core::ffi::c_long;
		}
	} else {
		let mut minsize: ::core::ffi::c_long = (min_target_bits - (*bm).minmax_reservoir
			+ 7 as ::core::ffi::c_long)
			/ 8 as ::core::ffi::c_long;
		if choice >= PACKETBLOBS {
			choice = PACKETBLOBS - 1 as ::core::ffi::c_int;
		}
		(*bm).choice = choice;
		minsize -= oggpack_bytes((*vbi).packetblob[choice as usize]);
		loop {
			let fresh0 = minsize;
			minsize = minsize - 1;
			if !(fresh0 > 0 as ::core::ffi::c_long) {
				break;
			}
			oggpack_write(
				(*vbi).packetblob[choice as usize],
				0 as ::core::ffi::c_ulong,
				8 as ::core::ffi::c_int
			);
		}
		this_bits = oggpack_bytes((*vbi).packetblob[choice as usize]) * 8 as ::core::ffi::c_long;
	}
	if (*bm).min_bitsper > 0 as ::core::ffi::c_long || (*bm).max_bitsper > 0 as ::core::ffi::c_long
	{
		if max_target_bits > 0 as ::core::ffi::c_long && this_bits > max_target_bits {
			(*bm).minmax_reservoir += this_bits - max_target_bits;
		} else if min_target_bits > 0 as ::core::ffi::c_long && this_bits < min_target_bits {
			(*bm).minmax_reservoir += this_bits - min_target_bits;
		} else if (*bm).minmax_reservoir > desired_fill {
			if max_target_bits > 0 as ::core::ffi::c_long {
				(*bm).minmax_reservoir += this_bits - max_target_bits;
				if (*bm).minmax_reservoir < desired_fill {
					(*bm).minmax_reservoir = desired_fill;
				}
			} else {
				(*bm).minmax_reservoir = desired_fill;
			}
		} else if min_target_bits > 0 as ::core::ffi::c_long {
			(*bm).minmax_reservoir += this_bits - min_target_bits;
			if (*bm).minmax_reservoir > desired_fill {
				(*bm).minmax_reservoir = desired_fill;
			}
		} else {
			(*bm).minmax_reservoir = desired_fill;
		}
	}
	if (*bm).avg_bitsper > 0 as ::core::ffi::c_long {
		let mut avg_target_bits_0: ::core::ffi::c_long = if (*vb).W != 0 {
			(*bm).avg_bitsper * (*bm).short_per_long
		} else {
			(*bm).avg_bitsper
		};
		(*bm).avg_reservoir += this_bits - avg_target_bits_0;
	}
	return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn vorbis_bitrate_flushpacket(
	mut vd: *mut vorbis_dsp_state,
	mut op: *mut ogg_packet
) -> ::core::ffi::c_int {
	let mut b: *mut private_state = (*vd).backend_state as *mut private_state;
	let mut bm: *mut bitrate_manager_state = &raw mut (*b).bms;
	let mut vb: *mut vorbis_block = (*bm).vb;
	let mut choice: ::core::ffi::c_int = PACKETBLOBS / 2 as ::core::ffi::c_int;
	if vb.is_null() {
		return 0 as ::core::ffi::c_int;
	}
	if !op.is_null() {
		let mut vbi: *mut vorbis_block_internal = (*vb).internal as *mut vorbis_block_internal;
		if vorbis_bitrate_managed(vb) != 0 {
			choice = (*bm).choice;
		}
		(*op).packet = oggpack_get_buffer((*vbi).packetblob[choice as usize]);
		(*op).bytes = oggpack_bytes((*vbi).packetblob[choice as usize]);
		(*op).b_o_s = 0 as ::core::ffi::c_long;
		(*op).e_o_s = (*vb).eofflag as ::core::ffi::c_long;
		(*op).granulepos = (*vb).granulepos;
		(*op).packetno = (*vb).sequence;
	}
	(*bm).vb = ::core::ptr::null_mut::<vorbis_block>();
	return 1 as ::core::ffi::c_int;
}
pub const PACKETBLOBS: ::core::ffi::c_int = 15 as ::core::ffi::c_int;
