extern "C" {
	fn calloc(__nmemb: size_t, __size: size_t) -> *mut ::core::ffi::c_void;
	fn free(__ptr: *mut ::core::ffi::c_void);
	fn memset(
		__s: *mut ::core::ffi::c_void,
		__c: ::core::ffi::c_int,
		__n: size_t
	) -> *mut ::core::ffi::c_void;
	fn oggpack_write(b: *mut oggpack_buffer, value: ::core::ffi::c_ulong, bits: ::core::ffi::c_int);
	fn oggpack_read(b: *mut oggpack_buffer, bits: ::core::ffi::c_int) -> ::core::ffi::c_long;
	fn mdct_forward(
		init: *mut mdct_lookup,
		in_0: *mut ::core::ffi::c_float,
		out: *mut ::core::ffi::c_float
	);
	fn mdct_backward(
		init: *mut mdct_lookup,
		in_0: *mut ::core::ffi::c_float,
		out: *mut ::core::ffi::c_float
	);
	fn floor1_fit(
		vb: *mut vorbis_block,
		look: *mut vorbis_look_floor1,
		logmdct: *const ::core::ffi::c_float,
		logmask: *const ::core::ffi::c_float
	) -> *mut ::core::ffi::c_int;
	fn floor1_interpolate_fit(
		vb: *mut vorbis_block,
		look: *mut vorbis_look_floor1,
		A: *mut ::core::ffi::c_int,
		B: *mut ::core::ffi::c_int,
		del: ::core::ffi::c_int
	) -> *mut ::core::ffi::c_int;
	fn floor1_encode(
		opb: *mut oggpack_buffer,
		vb: *mut vorbis_block,
		look: *mut vorbis_look_floor1,
		post: *mut ::core::ffi::c_int,
		ilogmask: *mut ::core::ffi::c_int
	) -> ::core::ffi::c_int;
	fn drft_forward(l: *mut drft_lookup, data: *mut ::core::ffi::c_float);
	fn _vp_noisemask(
		p: *mut vorbis_look_psy,
		logmdct: *mut ::core::ffi::c_float,
		logmask: *mut ::core::ffi::c_float
	);
	fn _vp_tonemask(
		p: *mut vorbis_look_psy,
		logfft: *mut ::core::ffi::c_float,
		logmask: *mut ::core::ffi::c_float,
		global_specmax: ::core::ffi::c_float,
		local_specmax: ::core::ffi::c_float
	);
	fn _vp_offset_and_mix(
		p: *mut vorbis_look_psy,
		noise: *mut ::core::ffi::c_float,
		tone: *mut ::core::ffi::c_float,
		offset_select: ::core::ffi::c_int,
		logmask: *mut ::core::ffi::c_float,
		mdct: *mut ::core::ffi::c_float,
		logmdct: *mut ::core::ffi::c_float
	);
	fn _vp_couple_quantize_normalize(
		blobno: ::core::ffi::c_int,
		g: *mut vorbis_info_psy_global,
		p: *mut vorbis_look_psy,
		vi: *mut vorbis_info_mapping0,
		mdct: *mut *mut ::core::ffi::c_float,
		iwork: *mut *mut ::core::ffi::c_int,
		nonzero: *mut ::core::ffi::c_int,
		sliding_lowpass: ::core::ffi::c_int,
		ch: ::core::ffi::c_int
	);
	fn _vorbis_block_alloc(
		vb: *mut vorbis_block,
		bytes: ::core::ffi::c_long
	) -> *mut ::core::ffi::c_void;
	fn ov_ilog(v: ogg_uint32_t) -> ::core::ffi::c_int;
	fn _vorbis_apply_window(
		d: *mut ::core::ffi::c_float,
		winno: *mut ::core::ffi::c_int,
		blocksizes: *mut ::core::ffi::c_long,
		lW: ::core::ffi::c_int,
		W: ::core::ffi::c_int,
		nW: ::core::ffi::c_int
	);
	fn vorbis_bitrate_managed(vb: *mut vorbis_block) -> ::core::ffi::c_int;
}
use ogg_c2rust_sys::oggpack_buffer;

use super::{
	block::{alloc_chain, vorbis_block, vorbis_dsp_state},
	info::vorbis_info,
	registry::{_floor_P, _residue_P, vorbis_func_mapping}
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
pub struct vorbis_block_internal {
	pub pcmdelay: *mut *mut ::core::ffi::c_float,
	pub ampmax: ::core::ffi::c_float,
	pub blocktype: ::core::ffi::c_int,
	pub packetblob: [*mut oggpack_buffer; 15]
}
pub type vorbis_look_floor = ();
pub type vorbis_look_residue = ();
pub type vorbis_look_transform = ();
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
pub struct drft_lookup {
	pub n: ::core::ffi::c_int,
	pub trigcache: *mut ::core::ffi::c_float,
	pub splitcache: *mut ::core::ffi::c_int
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
pub union C2RustUnnamed {
	pub i: ogg_uint32_t,
	pub f: ::core::ffi::c_float
}
pub const PACKETBLOBS: ::core::ffi::c_int = 15 as ::core::ffi::c_int;
unsafe extern "C" fn mapping0_free_info(mut i: *mut ::core::ffi::c_void) {
	let mut info: *mut vorbis_info_mapping0 = i as *mut vorbis_info_mapping0;
	if !info.is_null() {
		memset(
			info as *mut ::core::ffi::c_void,
			0 as ::core::ffi::c_int,
			::core::mem::size_of::<vorbis_info_mapping0>() as size_t
		);
		free(info as *mut ::core::ffi::c_void);
	}
}
unsafe extern "C" fn mapping0_pack(
	mut vi: *mut vorbis_info,
	mut vm: *mut ::core::ffi::c_void,
	mut opb: *mut oggpack_buffer
) {
	let mut i: ::core::ffi::c_int = 0;
	let mut info: *mut vorbis_info_mapping0 = vm as *mut vorbis_info_mapping0;
	if (*info).submaps > 1 as ::core::ffi::c_int {
		oggpack_write(opb, 1 as ::core::ffi::c_ulong, 1 as ::core::ffi::c_int);
		oggpack_write(
			opb,
			((*info).submaps - 1 as ::core::ffi::c_int) as ::core::ffi::c_ulong,
			4 as ::core::ffi::c_int
		);
	} else {
		oggpack_write(opb, 0 as ::core::ffi::c_ulong, 1 as ::core::ffi::c_int);
	}
	if (*info).coupling_steps > 0 as ::core::ffi::c_int {
		oggpack_write(opb, 1 as ::core::ffi::c_ulong, 1 as ::core::ffi::c_int);
		oggpack_write(
			opb,
			((*info).coupling_steps - 1 as ::core::ffi::c_int) as ::core::ffi::c_ulong,
			8 as ::core::ffi::c_int
		);
		i = 0 as ::core::ffi::c_int;
		while i < (*info).coupling_steps {
			oggpack_write(
				opb,
				(*info).coupling_mag[i as usize] as ::core::ffi::c_ulong,
				ov_ilog(((*vi).channels - 1 as ::core::ffi::c_int) as ogg_uint32_t)
			);
			oggpack_write(
				opb,
				(*info).coupling_ang[i as usize] as ::core::ffi::c_ulong,
				ov_ilog(((*vi).channels - 1 as ::core::ffi::c_int) as ogg_uint32_t)
			);
			i += 1;
		}
	} else {
		oggpack_write(opb, 0 as ::core::ffi::c_ulong, 1 as ::core::ffi::c_int);
	}
	oggpack_write(opb, 0 as ::core::ffi::c_ulong, 2 as ::core::ffi::c_int);
	if (*info).submaps > 1 as ::core::ffi::c_int {
		i = 0 as ::core::ffi::c_int;
		while i < (*vi).channels {
			oggpack_write(
				opb,
				(*info).chmuxlist[i as usize] as ::core::ffi::c_ulong,
				4 as ::core::ffi::c_int
			);
			i += 1;
		}
	}
	i = 0 as ::core::ffi::c_int;
	while i < (*info).submaps {
		oggpack_write(opb, 0 as ::core::ffi::c_ulong, 8 as ::core::ffi::c_int);
		oggpack_write(
			opb,
			(*info).floorsubmap[i as usize] as ::core::ffi::c_ulong,
			8 as ::core::ffi::c_int
		);
		oggpack_write(
			opb,
			(*info).residuesubmap[i as usize] as ::core::ffi::c_ulong,
			8 as ::core::ffi::c_int
		);
		i += 1;
	}
}
unsafe extern "C" fn mapping0_unpack(
	mut vi: *mut vorbis_info,
	mut opb: *mut oggpack_buffer
) -> *mut ::core::ffi::c_void {
	let mut current_block: u64;
	let mut i: ::core::ffi::c_int = 0;
	let mut b: ::core::ffi::c_int = 0;
	let mut info: *mut vorbis_info_mapping0 = calloc(
		1 as size_t,
		::core::mem::size_of::<vorbis_info_mapping0>() as size_t
	) as *mut vorbis_info_mapping0;
	let mut ci: *mut codec_setup_info = (*vi).codec_setup as *mut codec_setup_info;
	if !((*vi).channels <= 0 as ::core::ffi::c_int) {
		b = oggpack_read(opb, 1 as ::core::ffi::c_int) as ::core::ffi::c_int;
		if !(b < 0 as ::core::ffi::c_int) {
			if b != 0 {
				(*info).submaps = (oggpack_read(opb, 4 as ::core::ffi::c_int)
					+ 1 as ::core::ffi::c_long) as ::core::ffi::c_int;
				if (*info).submaps <= 0 as ::core::ffi::c_int {
					current_block = 13804471779124623223;
				} else {
					current_block = 7351195479953500246;
				}
			} else {
				(*info).submaps = 1 as ::core::ffi::c_int;
				current_block = 7351195479953500246;
			}
			match current_block {
				13804471779124623223 => {}
				_ => {
					b = oggpack_read(opb, 1 as ::core::ffi::c_int) as ::core::ffi::c_int;
					if !(b < 0 as ::core::ffi::c_int) {
						if b != 0 {
							(*info).coupling_steps = (oggpack_read(opb, 8 as ::core::ffi::c_int)
								+ 1 as ::core::ffi::c_long)
								as ::core::ffi::c_int;
							if (*info).coupling_steps <= 0 as ::core::ffi::c_int {
								current_block = 13804471779124623223;
							} else {
								i = 0 as ::core::ffi::c_int;
								loop {
									if !(i < (*info).coupling_steps) {
										current_block = 12039483399334584727;
										break;
									}
									(*info).coupling_mag[i as usize] = oggpack_read(
										opb,
										ov_ilog(
											((*vi).channels - 1 as ::core::ffi::c_int)
												as ogg_uint32_t
										)
									)
										as ::core::ffi::c_int;
									let mut testM: ::core::ffi::c_int =
										(*info).coupling_mag[i as usize];
									(*info).coupling_ang[i as usize] = oggpack_read(
										opb,
										ov_ilog(
											((*vi).channels - 1 as ::core::ffi::c_int)
												as ogg_uint32_t
										)
									)
										as ::core::ffi::c_int;
									let mut testA: ::core::ffi::c_int =
										(*info).coupling_ang[i as usize];
									if testM < 0 as ::core::ffi::c_int
										|| testA < 0 as ::core::ffi::c_int
										|| testM == testA || testM >= (*vi).channels
										|| testA >= (*vi).channels
									{
										current_block = 13804471779124623223;
										break;
									}
									i += 1;
								}
							}
						} else {
							current_block = 12039483399334584727;
						}
						match current_block {
							13804471779124623223 => {}
							_ => {
								if !(oggpack_read(opb, 2 as ::core::ffi::c_int)
									!= 0 as ::core::ffi::c_long)
								{
									if (*info).submaps > 1 as ::core::ffi::c_int {
										i = 0 as ::core::ffi::c_int;
										loop {
											if !(i < (*vi).channels) {
												current_block = 5634871135123216486;
												break;
											}
											(*info).chmuxlist[i as usize] =
												oggpack_read(opb, 4 as ::core::ffi::c_int)
													as ::core::ffi::c_int;
											if (*info).chmuxlist[i as usize] >= (*info).submaps
												|| (*info).chmuxlist[i as usize]
													< 0 as ::core::ffi::c_int
											{
												current_block = 13804471779124623223;
												break;
											}
											i += 1;
										}
									} else {
										current_block = 5634871135123216486;
									}
									match current_block {
										13804471779124623223 => {}
										_ => {
											i = 0 as ::core::ffi::c_int;
											loop {
												if !(i < (*info).submaps) {
													current_block = 4761528863920922185;
													break;
												}
												oggpack_read(opb, 8 as ::core::ffi::c_int);
												(*info).floorsubmap[i as usize] =
													oggpack_read(opb, 8 as ::core::ffi::c_int)
														as ::core::ffi::c_int;
												if (*info).floorsubmap[i as usize] >= (*ci).floors
													|| (*info).floorsubmap[i as usize]
														< 0 as ::core::ffi::c_int
												{
													current_block = 13804471779124623223;
													break;
												}
												(*info).residuesubmap[i as usize] =
													oggpack_read(opb, 8 as ::core::ffi::c_int)
														as ::core::ffi::c_int;
												if (*info).residuesubmap[i as usize]
													>= (*ci).residues || (*info).residuesubmap
													[i as usize] < 0
													as ::core::ffi::c_int
												{
													current_block = 13804471779124623223;
													break;
												}
												i += 1;
											}
											match current_block {
												13804471779124623223 => {}
												_ => return info as *mut ::core::ffi::c_void
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
	mapping0_free_info(info as *mut ::core::ffi::c_void);
	return ::core::ptr::null_mut::<::core::ffi::c_void>();
}
unsafe extern "C" fn mapping0_forward(mut vb: *mut vorbis_block) -> ::core::ffi::c_int {
	let mut alloca_allocations: Vec<Vec<u8>> = Vec::new();
	let mut vd: *mut vorbis_dsp_state = (*vb).vd;
	let mut vi: *mut vorbis_info = (*vd).vi;
	let mut ci: *mut codec_setup_info = (*vi).codec_setup as *mut codec_setup_info;
	let mut b: *mut private_state = (*(*vb).vd).backend_state as *mut private_state;
	let mut vbi: *mut vorbis_block_internal = (*vb).internal as *mut vorbis_block_internal;
	let mut n: ::core::ffi::c_int = (*vb).pcmend;
	let mut i: ::core::ffi::c_int = 0;
	let mut j: ::core::ffi::c_int = 0;
	let mut k: ::core::ffi::c_int = 0;
	alloca_allocations.push(::std::vec::from_elem(
		0,
		(::core::mem::size_of::<::core::ffi::c_int>() as usize)
			.wrapping_mul((*vi).channels as usize) as usize
	));
	let mut nonzero: *mut ::core::ffi::c_int =
		alloca_allocations.last_mut().unwrap().as_mut_ptr() as *mut ::core::ffi::c_int;
	let mut gmdct: *mut *mut ::core::ffi::c_float = _vorbis_block_alloc(
		vb,
		((*vi).channels as usize)
			.wrapping_mul(::core::mem::size_of::<*mut ::core::ffi::c_float>() as usize)
			as ::core::ffi::c_long
	) as *mut *mut ::core::ffi::c_float;
	let mut iwork: *mut *mut ::core::ffi::c_int = _vorbis_block_alloc(
		vb,
		((*vi).channels as usize)
			.wrapping_mul(::core::mem::size_of::<*mut ::core::ffi::c_int>() as usize)
			as ::core::ffi::c_long
	) as *mut *mut ::core::ffi::c_int;
	let mut floor_posts: *mut *mut *mut ::core::ffi::c_int = _vorbis_block_alloc(
		vb,
		((*vi).channels as usize)
			.wrapping_mul(::core::mem::size_of::<*mut *mut ::core::ffi::c_int>() as usize)
			as ::core::ffi::c_long
	)
		as *mut *mut *mut ::core::ffi::c_int;
	let mut global_ampmax: ::core::ffi::c_float = (*vbi).ampmax;
	alloca_allocations.push(::std::vec::from_elem(
		0,
		(::core::mem::size_of::<::core::ffi::c_float>() as usize)
			.wrapping_mul((*vi).channels as usize) as usize
	));
	let mut local_ampmax: *mut ::core::ffi::c_float =
		alloca_allocations.last_mut().unwrap().as_mut_ptr() as *mut ::core::ffi::c_float;
	let mut blocktype: ::core::ffi::c_int = (*vbi).blocktype;
	let mut modenumber: ::core::ffi::c_int = (*vb).W as ::core::ffi::c_int;
	let mut info: *mut vorbis_info_mapping0 =
		(*ci).map_param[modenumber as usize] as *mut vorbis_info_mapping0;
	let mut psy_look: *mut vorbis_look_psy = (*b).psy.offset(blocktype as isize).offset(
		(if (*vb).W != 0 {
			2 as ::core::ffi::c_int
		} else {
			0 as ::core::ffi::c_int
		}) as isize
	);
	(*vb).mode = modenumber;
	i = 0 as ::core::ffi::c_int;
	while i < (*vi).channels {
		let mut scale: ::core::ffi::c_float = 4.0f32 / n as ::core::ffi::c_float;
		let mut scale_dB: ::core::ffi::c_float = 0.;
		let mut pcm: *mut ::core::ffi::c_float = *(*vb).pcm.offset(i as isize);
		let mut logfft: *mut ::core::ffi::c_float = pcm;
		let ref mut fresh0 = *iwork.offset(i as isize);
		*fresh0 = _vorbis_block_alloc(
			vb,
			((n / 2 as ::core::ffi::c_int) as usize)
				.wrapping_mul(::core::mem::size_of::<::core::ffi::c_int>() as usize)
				as ::core::ffi::c_long
		) as *mut ::core::ffi::c_int;
		let ref mut fresh1 = *gmdct.offset(i as isize);
		*fresh1 = _vorbis_block_alloc(
			vb,
			((n / 2 as ::core::ffi::c_int) as usize)
				.wrapping_mul(::core::mem::size_of::<::core::ffi::c_float>() as usize)
				as ::core::ffi::c_long
		) as *mut ::core::ffi::c_float;
		scale_dB =
			(todB(&raw mut scale) as ::core::ffi::c_double + 0.345f64) as ::core::ffi::c_float;
		_vorbis_apply_window(
			pcm,
			&raw mut (*b).window as *mut ::core::ffi::c_int,
			&raw mut (*ci).blocksizes as *mut ::core::ffi::c_long,
			(*vb).lW as ::core::ffi::c_int,
			(*vb).W as ::core::ffi::c_int,
			(*vb).nW as ::core::ffi::c_int
		);
		mdct_forward(
			*(*b).transform[(*vb).W as usize].offset(0 as ::core::ffi::c_int as isize)
				as *mut mdct_lookup,
			pcm,
			*gmdct.offset(i as isize)
		);
		drft_forward(
			(&raw mut (*b).fft_look as *mut drft_lookup).offset((*vb).W as isize)
				as *mut drft_lookup,
			pcm
		);
		*logfft.offset(0 as ::core::ffi::c_int as isize) =
			((scale_dB + todB(pcm)) as ::core::ffi::c_double + 0.345f64) as ::core::ffi::c_float;
		*local_ampmax.offset(i as isize) = *logfft.offset(0 as ::core::ffi::c_int as isize);
		j = 1 as ::core::ffi::c_int;
		while j < n - 1 as ::core::ffi::c_int {
			let mut temp: ::core::ffi::c_float = *pcm.offset(j as isize) * *pcm.offset(j as isize)
				+ *pcm.offset((j + 1 as ::core::ffi::c_int) as isize)
					* *pcm.offset((j + 1 as ::core::ffi::c_int) as isize);
			let ref mut fresh2 =
				*logfft.offset((j + 1 as ::core::ffi::c_int >> 1 as ::core::ffi::c_int) as isize);
			*fresh2 = ((scale_dB + 0.5f32 * todB(&raw mut temp)) as ::core::ffi::c_double
				+ 0.345f64) as ::core::ffi::c_float;
			temp = *fresh2;
			if temp > *local_ampmax.offset(i as isize) {
				*local_ampmax.offset(i as isize) = temp;
			}
			j += 2 as ::core::ffi::c_int;
		}
		if *local_ampmax.offset(i as isize) > 0.0f32 {
			*local_ampmax.offset(i as isize) = 0.0f32;
		}
		if *local_ampmax.offset(i as isize) > global_ampmax {
			global_ampmax = *local_ampmax.offset(i as isize);
		}
		i += 1;
	}
	let mut noise: *mut ::core::ffi::c_float = _vorbis_block_alloc(
		vb,
		((n / 2 as ::core::ffi::c_int) as usize)
			.wrapping_mul(::core::mem::size_of::<::core::ffi::c_float>() as usize)
			as ::core::ffi::c_long
	) as *mut ::core::ffi::c_float;
	let mut tone: *mut ::core::ffi::c_float = _vorbis_block_alloc(
		vb,
		((n / 2 as ::core::ffi::c_int) as usize)
			.wrapping_mul(::core::mem::size_of::<::core::ffi::c_float>() as usize)
			as ::core::ffi::c_long
	) as *mut ::core::ffi::c_float;
	i = 0 as ::core::ffi::c_int;
	while i < (*vi).channels {
		let mut submap: ::core::ffi::c_int = (*info).chmuxlist[i as usize];
		let mut mdct: *mut ::core::ffi::c_float = *gmdct.offset(i as isize);
		let mut logfft_0: *mut ::core::ffi::c_float = *(*vb).pcm.offset(i as isize);
		let mut logmdct: *mut ::core::ffi::c_float =
			logfft_0.offset((n / 2 as ::core::ffi::c_int) as isize);
		let mut logmask: *mut ::core::ffi::c_float = logfft_0;
		(*vb).mode = modenumber;
		let ref mut fresh3 = *floor_posts.offset(i as isize);
		*fresh3 = _vorbis_block_alloc(
			vb,
			(PACKETBLOBS as usize)
				.wrapping_mul(::core::mem::size_of::<*mut ::core::ffi::c_int>() as usize)
				as ::core::ffi::c_long
		) as *mut *mut ::core::ffi::c_int;
		memset(
			*floor_posts.offset(i as isize) as *mut ::core::ffi::c_void,
			0 as ::core::ffi::c_int,
			(::core::mem::size_of::<*mut ::core::ffi::c_int>() as size_t)
				.wrapping_mul(PACKETBLOBS as size_t)
		);
		j = 0 as ::core::ffi::c_int;
		while j < n / 2 as ::core::ffi::c_int {
			*logmdct.offset(j as isize) = (todB(mdct.offset(j as isize)) as ::core::ffi::c_double
				+ 0.345f64) as ::core::ffi::c_float;
			j += 1;
		}
		_vp_noisemask(psy_look, logmdct, noise);
		_vp_tonemask(
			psy_look,
			logfft_0,
			tone,
			global_ampmax,
			*local_ampmax.offset(i as isize)
		);
		_vp_offset_and_mix(
			psy_look,
			noise,
			tone,
			1 as ::core::ffi::c_int,
			logmask,
			mdct,
			logmdct
		);
		if (*ci).floor_type[(*info).floorsubmap[submap as usize] as usize]
			!= 1 as ::core::ffi::c_int
		{
			return -(1 as ::core::ffi::c_int);
		}
		let ref mut fresh4 = *(*floor_posts.offset(i as isize))
			.offset((PACKETBLOBS / 2 as ::core::ffi::c_int) as isize);
		*fresh4 = floor1_fit(
			vb,
			*(*b)
				.flr
				.offset((*info).floorsubmap[submap as usize] as isize) as *mut vorbis_look_floor1,
			logmdct,
			logmask
		);
		if vorbis_bitrate_managed(vb) != 0
			&& !(*(*floor_posts.offset(i as isize))
				.offset((PACKETBLOBS / 2 as ::core::ffi::c_int) as isize))
			.is_null()
		{
			_vp_offset_and_mix(
				psy_look,
				noise,
				tone,
				2 as ::core::ffi::c_int,
				logmask,
				mdct,
				logmdct
			);
			let ref mut fresh5 = *(*floor_posts.offset(i as isize))
				.offset((PACKETBLOBS - 1 as ::core::ffi::c_int) as isize);
			*fresh5 = floor1_fit(
				vb,
				*(*b)
					.flr
					.offset((*info).floorsubmap[submap as usize] as isize)
					as *mut vorbis_look_floor1,
				logmdct,
				logmask
			);
			_vp_offset_and_mix(
				psy_look,
				noise,
				tone,
				0 as ::core::ffi::c_int,
				logmask,
				mdct,
				logmdct
			);
			let ref mut fresh6 =
				*(*floor_posts.offset(i as isize)).offset(0 as ::core::ffi::c_int as isize);
			*fresh6 = floor1_fit(
				vb,
				*(*b)
					.flr
					.offset((*info).floorsubmap[submap as usize] as isize)
					as *mut vorbis_look_floor1,
				logmdct,
				logmask
			);
			k = 1 as ::core::ffi::c_int;
			while k < PACKETBLOBS / 2 as ::core::ffi::c_int {
				let ref mut fresh7 = *(*floor_posts.offset(i as isize)).offset(k as isize);
				*fresh7 = floor1_interpolate_fit(
					vb,
					*(*b)
						.flr
						.offset((*info).floorsubmap[submap as usize] as isize)
						as *mut vorbis_look_floor1,
					*(*floor_posts.offset(i as isize)).offset(0 as ::core::ffi::c_int as isize),
					*(*floor_posts.offset(i as isize))
						.offset((PACKETBLOBS / 2 as ::core::ffi::c_int) as isize),
					k * 65536 as ::core::ffi::c_int / (PACKETBLOBS / 2 as ::core::ffi::c_int)
				);
				k += 1;
			}
			k = PACKETBLOBS / 2 as ::core::ffi::c_int + 1 as ::core::ffi::c_int;
			while k < PACKETBLOBS - 1 as ::core::ffi::c_int {
				let ref mut fresh8 = *(*floor_posts.offset(i as isize)).offset(k as isize);
				*fresh8 = floor1_interpolate_fit(
					vb,
					*(*b)
						.flr
						.offset((*info).floorsubmap[submap as usize] as isize)
						as *mut vorbis_look_floor1,
					*(*floor_posts.offset(i as isize))
						.offset((PACKETBLOBS / 2 as ::core::ffi::c_int) as isize),
					*(*floor_posts.offset(i as isize))
						.offset((PACKETBLOBS - 1 as ::core::ffi::c_int) as isize),
					(k - PACKETBLOBS / 2 as ::core::ffi::c_int) * 65536 as ::core::ffi::c_int
						/ (PACKETBLOBS / 2 as ::core::ffi::c_int)
				);
				k += 1;
			}
		}
		i += 1;
	}
	(*vbi).ampmax = global_ampmax;
	alloca_allocations.push(::std::vec::from_elem(
		0,
		(::core::mem::size_of::<*mut ::core::ffi::c_int>() as usize)
			.wrapping_mul((*vi).channels as usize) as usize
	));
	let mut couple_bundle: *mut *mut ::core::ffi::c_int =
		alloca_allocations.last_mut().unwrap().as_mut_ptr() as *mut *mut ::core::ffi::c_int;
	alloca_allocations.push(::std::vec::from_elem(
		0,
		(::core::mem::size_of::<::core::ffi::c_int>() as usize)
			.wrapping_mul((*vi).channels as usize) as usize
	));
	let mut zerobundle: *mut ::core::ffi::c_int =
		alloca_allocations.last_mut().unwrap().as_mut_ptr() as *mut ::core::ffi::c_int;
	k = if vorbis_bitrate_managed(vb) != 0 {
		0 as ::core::ffi::c_int
	} else {
		PACKETBLOBS / 2 as ::core::ffi::c_int
	};
	while k
		<= (if vorbis_bitrate_managed(vb) != 0 {
			PACKETBLOBS - 1 as ::core::ffi::c_int
		} else {
			PACKETBLOBS / 2 as ::core::ffi::c_int
		}) {
		let mut opb: *mut oggpack_buffer = (*vbi).packetblob[k as usize];
		oggpack_write(opb, 0 as ::core::ffi::c_ulong, 1 as ::core::ffi::c_int);
		oggpack_write(opb, modenumber as ::core::ffi::c_ulong, (*b).modebits);
		if (*vb).W != 0 {
			oggpack_write(
				opb,
				(*vb).lW as ::core::ffi::c_ulong,
				1 as ::core::ffi::c_int
			);
			oggpack_write(
				opb,
				(*vb).nW as ::core::ffi::c_ulong,
				1 as ::core::ffi::c_int
			);
		}
		i = 0 as ::core::ffi::c_int;
		while i < (*vi).channels {
			let mut submap_0: ::core::ffi::c_int = (*info).chmuxlist[i as usize];
			let mut ilogmask: *mut ::core::ffi::c_int = *iwork.offset(i as isize);
			*nonzero.offset(i as isize) = floor1_encode(
				opb,
				vb,
				*(*b)
					.flr
					.offset((*info).floorsubmap[submap_0 as usize] as isize)
					as *mut vorbis_look_floor1,
				*(*floor_posts.offset(i as isize)).offset(k as isize),
				ilogmask
			);
			i += 1;
		}
		_vp_couple_quantize_normalize(
			k,
			&raw mut (*ci).psy_g_param,
			psy_look,
			info,
			gmdct,
			iwork,
			nonzero,
			(*ci).psy_g_param.sliding_lowpass[(*vb).W as usize][k as usize],
			(*vi).channels
		);
		i = 0 as ::core::ffi::c_int;
		while i < (*info).submaps {
			let mut ch_in_bundle: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
			let mut classifications: *mut *mut ::core::ffi::c_long =
				::core::ptr::null_mut::<*mut ::core::ffi::c_long>();
			let mut resnum: ::core::ffi::c_int = (*info).residuesubmap[i as usize];
			j = 0 as ::core::ffi::c_int;
			while j < (*vi).channels {
				if (*info).chmuxlist[j as usize] == i {
					*zerobundle.offset(ch_in_bundle as isize) = 0 as ::core::ffi::c_int;
					if *nonzero.offset(j as isize) != 0 {
						*zerobundle.offset(ch_in_bundle as isize) = 1 as ::core::ffi::c_int;
					}
					let fresh9 = ch_in_bundle;
					ch_in_bundle = ch_in_bundle + 1;
					let ref mut fresh10 = *couple_bundle.offset(fresh9 as isize);
					*fresh10 = *iwork.offset(j as isize);
				}
				j += 1;
			}
			classifications = (**(&raw const _residue_P as *const *const vorbis_func_residue)
				.offset((*ci).residue_type[resnum as usize] as isize))
			.class
			.expect("non-null function pointer")(
				vb as *mut vorbis_block,
				*(*b).residue.offset(resnum as isize),
				couple_bundle,
				zerobundle,
				ch_in_bundle
			);
			ch_in_bundle = 0 as ::core::ffi::c_int;
			j = 0 as ::core::ffi::c_int;
			while j < (*vi).channels {
				if (*info).chmuxlist[j as usize] == i {
					let fresh11 = ch_in_bundle;
					ch_in_bundle = ch_in_bundle + 1;
					let ref mut fresh12 = *couple_bundle.offset(fresh11 as isize);
					*fresh12 = *iwork.offset(j as isize);
				}
				j += 1;
			}
			(**(&raw const _residue_P as *const *const vorbis_func_residue)
				.offset((*ci).residue_type[resnum as usize] as isize))
			.forward
			.expect("non-null function pointer")(
				opb,
				vb as *mut vorbis_block,
				*(*b).residue.offset(resnum as isize),
				couple_bundle,
				zerobundle,
				ch_in_bundle,
				classifications,
				i
			);
			i += 1;
		}
		k += 1;
	}
	return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn mapping0_inverse(
	mut vb: *mut vorbis_block,
	mut l: *mut ::core::ffi::c_void
) -> ::core::ffi::c_int {
	let mut alloca_allocations: Vec<Vec<u8>> = Vec::new();
	let mut vd: *mut vorbis_dsp_state = (*vb).vd;
	let mut vi: *mut vorbis_info = (*vd).vi;
	let mut ci: *mut codec_setup_info = (*vi).codec_setup as *mut codec_setup_info;
	let mut b: *mut private_state = (*vd).backend_state as *mut private_state;
	let mut info: *mut vorbis_info_mapping0 = l as *mut vorbis_info_mapping0;
	let mut i: ::core::ffi::c_int = 0;
	let mut j: ::core::ffi::c_int = 0;
	(*vb).pcmend = (*ci).blocksizes[(*vb).W as usize] as ::core::ffi::c_int;
	let mut n: ::core::ffi::c_long = (*vb).pcmend as ::core::ffi::c_long;
	alloca_allocations.push(::std::vec::from_elem(
		0,
		(::core::mem::size_of::<*mut ::core::ffi::c_float>() as usize)
			.wrapping_mul((*vi).channels as usize) as usize
	));
	let mut pcmbundle: *mut *mut ::core::ffi::c_float =
		alloca_allocations.last_mut().unwrap().as_mut_ptr() as *mut *mut ::core::ffi::c_float;
	alloca_allocations.push(::std::vec::from_elem(
		0,
		(::core::mem::size_of::<::core::ffi::c_int>() as usize)
			.wrapping_mul((*vi).channels as usize) as usize
	));
	let mut zerobundle: *mut ::core::ffi::c_int =
		alloca_allocations.last_mut().unwrap().as_mut_ptr() as *mut ::core::ffi::c_int;
	alloca_allocations.push(::std::vec::from_elem(
		0,
		(::core::mem::size_of::<::core::ffi::c_int>() as usize)
			.wrapping_mul((*vi).channels as usize) as usize
	));
	let mut nonzero: *mut ::core::ffi::c_int =
		alloca_allocations.last_mut().unwrap().as_mut_ptr() as *mut ::core::ffi::c_int;
	alloca_allocations.push(::std::vec::from_elem(
		0,
		(::core::mem::size_of::<*mut ::core::ffi::c_void>() as usize)
			.wrapping_mul((*vi).channels as usize) as usize
	));
	let mut floormemo: *mut *mut ::core::ffi::c_void =
		alloca_allocations.last_mut().unwrap().as_mut_ptr() as *mut *mut ::core::ffi::c_void;
	i = 0 as ::core::ffi::c_int;
	while i < (*vi).channels {
		let mut submap: ::core::ffi::c_int = (*info).chmuxlist[i as usize];
		let ref mut fresh13 = *floormemo.offset(i as isize);
		*fresh13 = (**(&raw const _floor_P as *const *const vorbis_func_floor)
			.offset((*ci).floor_type[(*info).floorsubmap[submap as usize] as usize] as isize))
		.inverse1
		.expect("non-null function pointer")(
			vb as *mut vorbis_block,
			*(*b)
				.flr
				.offset((*info).floorsubmap[submap as usize] as isize)
		);
		if !(*floormemo.offset(i as isize)).is_null() {
			*nonzero.offset(i as isize) = 1 as ::core::ffi::c_int;
		} else {
			*nonzero.offset(i as isize) = 0 as ::core::ffi::c_int;
		}
		memset(
			*(*vb).pcm.offset(i as isize) as *mut ::core::ffi::c_void,
			0 as ::core::ffi::c_int,
			(::core::mem::size_of::<::core::ffi::c_float>() as size_t)
				.wrapping_mul(n as size_t)
				.wrapping_div(2 as size_t)
		);
		i += 1;
	}
	i = 0 as ::core::ffi::c_int;
	while i < (*info).coupling_steps {
		if *nonzero.offset((*info).coupling_mag[i as usize] as isize) != 0
			|| *nonzero.offset((*info).coupling_ang[i as usize] as isize) != 0
		{
			*nonzero.offset((*info).coupling_mag[i as usize] as isize) = 1 as ::core::ffi::c_int;
			*nonzero.offset((*info).coupling_ang[i as usize] as isize) = 1 as ::core::ffi::c_int;
		}
		i += 1;
	}
	i = 0 as ::core::ffi::c_int;
	while i < (*info).submaps {
		let mut ch_in_bundle: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
		j = 0 as ::core::ffi::c_int;
		while j < (*vi).channels {
			if (*info).chmuxlist[j as usize] == i {
				if *nonzero.offset(j as isize) != 0 {
					*zerobundle.offset(ch_in_bundle as isize) = 1 as ::core::ffi::c_int;
				} else {
					*zerobundle.offset(ch_in_bundle as isize) = 0 as ::core::ffi::c_int;
				}
				let fresh14 = ch_in_bundle;
				ch_in_bundle = ch_in_bundle + 1;
				let ref mut fresh15 = *pcmbundle.offset(fresh14 as isize);
				*fresh15 = *(*vb).pcm.offset(j as isize);
			}
			j += 1;
		}
		(**(&raw const _residue_P as *const *const vorbis_func_residue)
			.offset((*ci).residue_type[(*info).residuesubmap[i as usize] as usize] as isize))
		.inverse
		.expect("non-null function pointer")(
			vb as *mut vorbis_block,
			*(*b)
				.residue
				.offset((*info).residuesubmap[i as usize] as isize),
			pcmbundle,
			zerobundle,
			ch_in_bundle
		);
		i += 1;
	}
	i = (*info).coupling_steps - 1 as ::core::ffi::c_int;
	while i >= 0 as ::core::ffi::c_int {
		let mut pcmM: *mut ::core::ffi::c_float =
			*(*vb).pcm.offset((*info).coupling_mag[i as usize] as isize);
		let mut pcmA: *mut ::core::ffi::c_float =
			*(*vb).pcm.offset((*info).coupling_ang[i as usize] as isize);
		j = 0 as ::core::ffi::c_int;
		while (j as ::core::ffi::c_long) < n / 2 as ::core::ffi::c_long {
			let mut mag: ::core::ffi::c_float = *pcmM.offset(j as isize);
			let mut ang: ::core::ffi::c_float = *pcmA.offset(j as isize);
			if mag > 0 as ::core::ffi::c_int as ::core::ffi::c_float {
				if ang > 0 as ::core::ffi::c_int as ::core::ffi::c_float {
					*pcmM.offset(j as isize) = mag;
					*pcmA.offset(j as isize) = mag - ang;
				} else {
					*pcmA.offset(j as isize) = mag;
					*pcmM.offset(j as isize) = mag + ang;
				}
			} else if ang > 0 as ::core::ffi::c_int as ::core::ffi::c_float {
				*pcmM.offset(j as isize) = mag;
				*pcmA.offset(j as isize) = mag + ang;
			} else {
				*pcmA.offset(j as isize) = mag;
				*pcmM.offset(j as isize) = mag - ang;
			}
			j += 1;
		}
		i -= 1;
	}
	i = 0 as ::core::ffi::c_int;
	while i < (*vi).channels {
		let mut pcm: *mut ::core::ffi::c_float = *(*vb).pcm.offset(i as isize);
		let mut submap_0: ::core::ffi::c_int = (*info).chmuxlist[i as usize];
		(**(&raw const _floor_P as *const *const vorbis_func_floor)
			.offset((*ci).floor_type[(*info).floorsubmap[submap_0 as usize] as usize] as isize))
		.inverse2
		.expect("non-null function pointer")(
			vb as *mut vorbis_block,
			*(*b)
				.flr
				.offset((*info).floorsubmap[submap_0 as usize] as isize),
			*floormemo.offset(i as isize),
			pcm
		);
		i += 1;
	}
	i = 0 as ::core::ffi::c_int;
	while i < (*vi).channels {
		let mut pcm_0: *mut ::core::ffi::c_float = *(*vb).pcm.offset(i as isize);
		mdct_backward(
			*(*b).transform[(*vb).W as usize].offset(0 as ::core::ffi::c_int as isize)
				as *mut mdct_lookup,
			pcm_0,
			pcm_0
		);
		i += 1;
	}
	return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub static mut mapping0_exportbundle: vorbis_func_mapping = vorbis_func_mapping {
	pack: Some(
		mapping0_pack
			as unsafe extern "C" fn(
				*mut vorbis_info,
				*mut ::core::ffi::c_void,
				*mut oggpack_buffer
			) -> ()
	),
	unpack: Some(
		mapping0_unpack
			as unsafe extern "C" fn(
				*mut vorbis_info,
				*mut oggpack_buffer
			) -> *mut ::core::ffi::c_void
	),
	free_info: Some(mapping0_free_info as unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()),
	forward: Some(
		mapping0_forward as unsafe extern "C" fn(*mut vorbis_block) -> ::core::ffi::c_int
	),
	inverse: Some(
		mapping0_inverse
			as unsafe extern "C" fn(
				*mut vorbis_block,
				*mut ::core::ffi::c_void
			) -> ::core::ffi::c_int
	)
};
#[inline]
unsafe extern "C" fn todB(mut x: *const ::core::ffi::c_float) -> ::core::ffi::c_float {
	let mut ix: C2RustUnnamed = C2RustUnnamed { i: 0 };
	ix.f = *x;
	ix.i = ix.i & 0x7fffffff as ogg_uint32_t;
	return ix.i as ::core::ffi::c_float * 7.17711438e-7f32 - 764.6161886f32;
}
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
