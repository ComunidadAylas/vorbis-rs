extern "C" {
	fn oggpack_readinit(
		b: *mut oggpack_buffer,
		buf: *mut ::core::ffi::c_uchar,
		bytes: ::core::ffi::c_int
	);
	fn oggpack_read(b: *mut oggpack_buffer, bits: ::core::ffi::c_int) -> ::core::ffi::c_long;
	fn _vorbis_block_alloc(
		vb: *mut vorbis_block,
		bytes: ::core::ffi::c_long
	) -> *mut ::core::ffi::c_void;
	fn _vorbis_block_ripcord(vb: *mut vorbis_block);
	fn ov_ilog(v: ogg_uint32_t) -> ::core::ffi::c_int;
}
use ogg_c2rust_sys::{ogg_packet, oggpack_buffer};

use super::{
	block::{alloc_chain, vorbis_block, vorbis_dsp_state},
	info::vorbis_info,
	registry::_mapping_P
};
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
pub const OV_EFAULT: ::core::ffi::c_int = -(129 as ::core::ffi::c_int);
pub const OV_ENOTAUDIO: ::core::ffi::c_int = -(135 as ::core::ffi::c_int);
pub const OV_EBADPACKET: ::core::ffi::c_int = -(136 as ::core::ffi::c_int);
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
#[no_mangle]
pub unsafe extern "C" fn vorbis_synthesis(
	mut vb: *mut vorbis_block,
	mut op: *mut ogg_packet
) -> ::core::ffi::c_int {
	let mut vd: *mut vorbis_dsp_state = if !vb.is_null() {
		(*vb).vd
	} else {
		::core::ptr::null_mut::<vorbis_dsp_state>()
	};
	let mut b: *mut private_state = (if !vd.is_null() {
		(*vd).backend_state
	} else {
		::core::ptr::null_mut::<::core::ffi::c_void>()
	}) as *mut private_state;
	let mut vi: *mut vorbis_info = if !vd.is_null() {
		(*vd).vi
	} else {
		::core::ptr::null_mut::<vorbis_info>()
	};
	let mut ci: *mut codec_setup_info = (if !vi.is_null() {
		(*vi).codec_setup
	} else {
		::core::ptr::null_mut::<::core::ffi::c_void>()
	}) as *mut codec_setup_info;
	let mut opb: *mut oggpack_buffer = if !vb.is_null() {
		&raw mut (*vb).opb
	} else {
		::core::ptr::null_mut::<oggpack_buffer>()
	};
	let mut type_0: ::core::ffi::c_int = 0;
	let mut mode: ::core::ffi::c_int = 0;
	let mut i: ::core::ffi::c_int = 0;
	if vd.is_null() || b.is_null() || vi.is_null() || ci.is_null() || opb.is_null() {
		return OV_EBADPACKET;
	}
	_vorbis_block_ripcord(vb);
	oggpack_readinit(opb, (*op).packet, (*op).bytes as ::core::ffi::c_int);
	if oggpack_read(opb, 1 as ::core::ffi::c_int) != 0 as ::core::ffi::c_long {
		return -(135 as ::core::ffi::c_int);
	}
	mode = oggpack_read(opb, (*b).modebits) as ::core::ffi::c_int;
	if mode == -(1 as ::core::ffi::c_int) {
		return -(136 as ::core::ffi::c_int);
	}
	(*vb).mode = mode;
	if (*ci).mode_param[mode as usize].is_null() {
		return -(136 as ::core::ffi::c_int);
	}
	(*vb).W = (*(*ci).mode_param[mode as usize]).blockflag as ::core::ffi::c_long;
	if (*vb).W != 0 {
		(*vb).lW = oggpack_read(opb, 1 as ::core::ffi::c_int);
		(*vb).nW = oggpack_read(opb, 1 as ::core::ffi::c_int);
		if (*vb).nW == -(1 as ::core::ffi::c_int) as ::core::ffi::c_long {
			return -(136 as ::core::ffi::c_int);
		}
	} else {
		(*vb).lW = 0 as ::core::ffi::c_long;
		(*vb).nW = 0 as ::core::ffi::c_long;
	}
	(*vb).granulepos = (*op).granulepos;
	(*vb).sequence = (*op).packetno;
	(*vb).eofflag = (*op).e_o_s as ::core::ffi::c_int;
	(*vb).pcmend = (*ci).blocksizes[(*vb).W as usize] as ::core::ffi::c_int;
	(*vb).pcm = _vorbis_block_alloc(
		vb,
		(::core::mem::size_of::<*mut ::core::ffi::c_float>() as usize)
			.wrapping_mul((*vi).channels as usize) as ::core::ffi::c_long
	) as *mut *mut ::core::ffi::c_float;
	i = 0 as ::core::ffi::c_int;
	while i < (*vi).channels {
		let ref mut fresh0 = *(*vb).pcm.offset(i as isize);
		*fresh0 = _vorbis_block_alloc(
			vb,
			((*vb).pcmend as usize)
				.wrapping_mul(::core::mem::size_of::<::core::ffi::c_float>() as usize)
				as ::core::ffi::c_long
		) as *mut ::core::ffi::c_float;
		i += 1;
	}
	type_0 = (*ci).map_type[(*(*ci).mode_param[mode as usize]).mapping as usize];
	return (**(&raw const _mapping_P as *const *const vorbis_func_mapping)
		.offset(type_0 as isize))
	.inverse
	.expect("non-null function pointer")(
		vb as *mut vorbis_block,
		(*ci).map_param[(*(*ci).mode_param[mode as usize]).mapping as usize]
	);
}
#[no_mangle]
pub unsafe extern "C" fn vorbis_synthesis_trackonly(
	mut vb: *mut vorbis_block,
	mut op: *mut ogg_packet
) -> ::core::ffi::c_int {
	let mut vd: *mut vorbis_dsp_state = (*vb).vd;
	let mut b: *mut private_state = (*vd).backend_state as *mut private_state;
	let mut vi: *mut vorbis_info = (*vd).vi;
	let mut ci: *mut codec_setup_info = (*vi).codec_setup as *mut codec_setup_info;
	let mut opb: *mut oggpack_buffer = &raw mut (*vb).opb;
	let mut mode: ::core::ffi::c_int = 0;
	_vorbis_block_ripcord(vb);
	oggpack_readinit(opb, (*op).packet, (*op).bytes as ::core::ffi::c_int);
	if oggpack_read(opb, 1 as ::core::ffi::c_int) != 0 as ::core::ffi::c_long {
		return -(135 as ::core::ffi::c_int);
	}
	mode = oggpack_read(opb, (*b).modebits) as ::core::ffi::c_int;
	if mode == -(1 as ::core::ffi::c_int) {
		return -(136 as ::core::ffi::c_int);
	}
	(*vb).mode = mode;
	if (*ci).mode_param[mode as usize].is_null() {
		return -(136 as ::core::ffi::c_int);
	}
	(*vb).W = (*(*ci).mode_param[mode as usize]).blockflag as ::core::ffi::c_long;
	if (*vb).W != 0 {
		(*vb).lW = oggpack_read(opb, 1 as ::core::ffi::c_int);
		(*vb).nW = oggpack_read(opb, 1 as ::core::ffi::c_int);
		if (*vb).nW == -(1 as ::core::ffi::c_int) as ::core::ffi::c_long {
			return -(136 as ::core::ffi::c_int);
		}
	} else {
		(*vb).lW = 0 as ::core::ffi::c_long;
		(*vb).nW = 0 as ::core::ffi::c_long;
	}
	(*vb).granulepos = (*op).granulepos;
	(*vb).sequence = (*op).packetno;
	(*vb).eofflag = (*op).e_o_s as ::core::ffi::c_int;
	(*vb).pcmend = 0 as ::core::ffi::c_int;
	(*vb).pcm = ::core::ptr::null_mut::<*mut ::core::ffi::c_float>();
	return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn vorbis_packet_blocksize(
	mut vi: *mut vorbis_info,
	mut op: *mut ogg_packet
) -> ::core::ffi::c_long {
	let mut ci: *mut codec_setup_info = (*vi).codec_setup as *mut codec_setup_info;
	let mut opb: oggpack_buffer = oggpack_buffer {
		endbyte: 0,
		endbit: 0,
		buffer: ::core::ptr::null_mut::<::core::ffi::c_uchar>(),
		ptr: ::core::ptr::null_mut::<::core::ffi::c_uchar>(),
		storage: 0
	};
	let mut mode: ::core::ffi::c_int = 0;
	if ci.is_null() || (*ci).modes <= 0 as ::core::ffi::c_int {
		return -(129 as ::core::ffi::c_int) as ::core::ffi::c_long;
	}
	oggpack_readinit(
		&raw mut opb,
		(*op).packet,
		(*op).bytes as ::core::ffi::c_int
	);
	if oggpack_read(&raw mut opb, 1 as ::core::ffi::c_int) != 0 as ::core::ffi::c_long {
		return -(135 as ::core::ffi::c_int) as ::core::ffi::c_long;
	}
	mode = oggpack_read(
		&raw mut opb,
		ov_ilog(((*ci).modes - 1 as ::core::ffi::c_int) as ogg_uint32_t)
	) as ::core::ffi::c_int;
	if mode == -(1 as ::core::ffi::c_int) || (*ci).mode_param[mode as usize].is_null() {
		return -(136 as ::core::ffi::c_int) as ::core::ffi::c_long;
	}
	return (*ci).blocksizes[(*(*ci).mode_param[mode as usize]).blockflag as usize];
}
#[no_mangle]
pub unsafe extern "C" fn vorbis_synthesis_halfrate(
	mut vi: *mut vorbis_info,
	mut flag: ::core::ffi::c_int
) -> ::core::ffi::c_int {
	let mut ci: *mut codec_setup_info = (*vi).codec_setup as *mut codec_setup_info;
	if (*ci).blocksizes[0 as ::core::ffi::c_int as usize] <= 64 as ::core::ffi::c_long && flag != 0
	{
		return -(1 as ::core::ffi::c_int);
	}
	(*ci).halfrate_flag = if flag != 0 {
		1 as ::core::ffi::c_int
	} else {
		0 as ::core::ffi::c_int
	};
	return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn vorbis_synthesis_halfrate_p(
	mut vi: *mut vorbis_info
) -> ::core::ffi::c_int {
	let mut ci: *mut codec_setup_info = (*vi).codec_setup as *mut codec_setup_info;
	return (*ci).halfrate_flag;
}
