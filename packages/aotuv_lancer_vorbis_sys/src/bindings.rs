pub const OV_FALSE: i32 = -1;
pub const OV_EOF: i32 = -2;
pub const OV_HOLE: i32 = -3;
pub const OV_EREAD: i32 = -128;
pub const OV_EFAULT: i32 = -129;
pub const OV_EIMPL: i32 = -130;
pub const OV_EINVAL: i32 = -131;
pub const OV_ENOTVORBIS: i32 = -132;
pub const OV_EBADHEADER: i32 = -133;
pub const OV_EVERSION: i32 = -134;
pub const OV_ENOTAUDIO: i32 = -135;
pub const OV_EBADPACKET: i32 = -136;
pub const OV_EBADLINK: i32 = -137;
pub const OV_ENOSEEK: i32 = -138;
pub const OV_ECTL_RATEMANAGE2_GET: u32 = 20;
pub const OV_ECTL_RATEMANAGE2_SET: u32 = 21;
pub const OV_ECTL_LOWPASS_GET: u32 = 32;
pub const OV_ECTL_LOWPASS_SET: u32 = 33;
pub const OV_ECTL_IBLOCK_GET: u32 = 48;
pub const OV_ECTL_IBLOCK_SET: u32 = 49;
pub const OV_ECTL_COUPLING_GET: u32 = 64;
pub const OV_ECTL_COUPLING_SET: u32 = 65;
pub const OV_ECTL_RATEMANAGE_GET: u32 = 16;
pub const OV_ECTL_RATEMANAGE_SET: u32 = 17;
pub const OV_ECTL_RATEMANAGE_AVG: u32 = 18;
pub const OV_ECTL_RATEMANAGE_HARD: u32 = 19;
pub const NOTOPEN: u32 = 0;
pub const PARTOPEN: u32 = 1;
pub const OPENED: u32 = 2;
pub const STREAMSET: u32 = 3;
pub const INITSET: u32 = 4;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct oggpack_buffer {
	pub endbyte: ::std::os::raw::c_long,
	pub endbit: ::std::os::raw::c_int,
	pub buffer: *mut ::std::os::raw::c_uchar,
	pub ptr: *mut ::std::os::raw::c_uchar,
	pub storage: ::std::os::raw::c_long
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct vorbis_info {
	pub version: ::std::os::raw::c_int,
	pub channels: ::std::os::raw::c_int,
	pub rate: ::std::os::raw::c_long,
	pub bitrate_upper: ::std::os::raw::c_long,
	pub bitrate_nominal: ::std::os::raw::c_long,
	pub bitrate_lower: ::std::os::raw::c_long,
	pub bitrate_window: ::std::os::raw::c_long,
	pub codec_setup: *mut ::std::os::raw::c_void
}
#[repr(C)]
pub struct vorbis_dsp_state {
	pub analysisp: ::std::os::raw::c_int,
	pub vi: *mut vorbis_info,
	pub pcm: *mut *mut f32,
	pub pcmret: *mut *mut f32,
	pub preextrapolate_work: *mut f32,
	pub pcm_storage: ::std::os::raw::c_int,
	pub pcm_current: ::std::os::raw::c_int,
	pub pcm_returned: ::std::os::raw::c_int,
	pub preextrapolate: ::std::os::raw::c_int,
	pub eofflag: ::std::os::raw::c_int,
	pub lW: ::std::os::raw::c_long,
	pub W: ::std::os::raw::c_long,
	pub nW: ::std::os::raw::c_long,
	pub centerW: ::std::os::raw::c_long,
	pub granulepos: ogg_int64_t,
	pub sequence: ogg_int64_t,
	pub glue_bits: ogg_int64_t,
	pub time_bits: ogg_int64_t,
	pub floor_bits: ogg_int64_t,
	pub res_bits: ogg_int64_t,
	pub backend_state: *mut ::std::os::raw::c_void
}
#[repr(C)]
pub struct vorbis_block {
	pub pcm: *mut *mut f32,
	pub opb: oggpack_buffer,
	pub lW: ::std::os::raw::c_long,
	pub W: ::std::os::raw::c_long,
	pub nW: ::std::os::raw::c_long,
	pub pcmend: ::std::os::raw::c_int,
	pub mode: ::std::os::raw::c_int,
	pub eofflag: ::std::os::raw::c_int,
	pub granulepos: ogg_int64_t,
	pub sequence: ogg_int64_t,
	pub vd: *mut vorbis_dsp_state,
	pub localstore: *mut ::std::os::raw::c_void,
	pub localtop: ::std::os::raw::c_long,
	pub localalloc: ::std::os::raw::c_long,
	pub totaluse: ::std::os::raw::c_long,
	pub reap: *mut alloc_chain,
	pub glue_bits: ::std::os::raw::c_long,
	pub time_bits: ::std::os::raw::c_long,
	pub floor_bits: ::std::os::raw::c_long,
	pub res_bits: ::std::os::raw::c_long,
	pub internal: *mut ::std::os::raw::c_void
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct alloc_chain {
	pub ptr: *mut ::std::os::raw::c_void,
	pub next: *mut alloc_chain
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct vorbis_comment {
	pub user_comments: *mut *mut ::std::os::raw::c_char,
	pub comment_lengths: *mut ::std::os::raw::c_int,
	pub comments: ::std::os::raw::c_int,
	pub vendor: *mut ::std::os::raw::c_char
}
unsafe extern "C" {
	pub fn vorbis_info_init(vi: *mut vorbis_info);
}
unsafe extern "C" {
	pub fn vorbis_info_clear(vi: *mut vorbis_info);
}
unsafe extern "C" {
	pub fn vorbis_info_blocksize(
		vi: *mut vorbis_info,
		zo: ::std::os::raw::c_int
	) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
	pub fn vorbis_comment_init(vc: *mut vorbis_comment);
}
unsafe extern "C" {
	pub fn vorbis_comment_add(vc: *mut vorbis_comment, comment: *const ::std::os::raw::c_char);
}
unsafe extern "C" {
	pub fn vorbis_comment_add_tag(
		vc: *mut vorbis_comment,
		tag: *const ::std::os::raw::c_char,
		contents: *const ::std::os::raw::c_char
	);
}
unsafe extern "C" {
	pub fn vorbis_comment_query(
		vc: *mut vorbis_comment,
		tag: *const ::std::os::raw::c_char,
		count: ::std::os::raw::c_int
	) -> *mut ::std::os::raw::c_char;
}
unsafe extern "C" {
	pub fn vorbis_comment_query_count(
		vc: *mut vorbis_comment,
		tag: *const ::std::os::raw::c_char
	) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
	pub fn vorbis_comment_clear(vc: *mut vorbis_comment);
}
unsafe extern "C" {
	pub fn vorbis_block_init(
		v: *mut vorbis_dsp_state,
		vb: *mut vorbis_block
	) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
	pub fn vorbis_block_clear(vb: *mut vorbis_block) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
	pub fn vorbis_dsp_clear(v: *mut vorbis_dsp_state);
}
unsafe extern "C" {
	pub fn vorbis_granule_time(v: *mut vorbis_dsp_state, granulepos: ogg_int64_t) -> f64;
}
unsafe extern "C" {
	pub fn vorbis_version_string() -> *const ::std::os::raw::c_char;
}
unsafe extern "C" {
	pub fn vorbis_analysis_init(
		v: *mut vorbis_dsp_state,
		vi: *mut vorbis_info
	) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
	pub fn vorbis_commentheader_out(
		vc: *mut vorbis_comment,
		op: *mut ogg_packet
	) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
	pub fn vorbis_analysis_headerout(
		v: *mut vorbis_dsp_state,
		vc: *mut vorbis_comment,
		op: *mut ogg_packet,
		op_comm: *mut ogg_packet,
		op_code: *mut ogg_packet
	) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
	pub fn vorbis_analysis_buffer(
		v: *mut vorbis_dsp_state,
		vals: ::std::os::raw::c_int
	) -> *mut *mut f32;
}
unsafe extern "C" {
	pub fn vorbis_analysis_wrote(
		v: *mut vorbis_dsp_state,
		vals: ::std::os::raw::c_int
	) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
	pub fn vorbis_analysis_blockout(
		v: *mut vorbis_dsp_state,
		vb: *mut vorbis_block
	) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
	pub fn vorbis_analysis(vb: *mut vorbis_block, op: *mut ogg_packet) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
	pub fn vorbis_bitrate_addblock(vb: *mut vorbis_block) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
	pub fn vorbis_bitrate_flushpacket(
		vd: *mut vorbis_dsp_state,
		op: *mut ogg_packet
	) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
	pub fn vorbis_synthesis_idheader(op: *mut ogg_packet) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
	pub fn vorbis_synthesis_headerin(
		vi: *mut vorbis_info,
		vc: *mut vorbis_comment,
		op: *mut ogg_packet
	) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
	pub fn vorbis_synthesis_init(
		v: *mut vorbis_dsp_state,
		vi: *mut vorbis_info
	) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
	pub fn vorbis_synthesis_restart(v: *mut vorbis_dsp_state) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
	pub fn vorbis_synthesis(vb: *mut vorbis_block, op: *mut ogg_packet) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
	pub fn vorbis_synthesis_trackonly(
		vb: *mut vorbis_block,
		op: *mut ogg_packet
	) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
	pub fn vorbis_synthesis_blockin(
		v: *mut vorbis_dsp_state,
		vb: *mut vorbis_block
	) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
	pub fn vorbis_synthesis_pcmout(
		v: *mut vorbis_dsp_state,
		pcm: *mut *mut *mut f32
	) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
	pub fn vorbis_synthesis_lapout(
		v: *mut vorbis_dsp_state,
		pcm: *mut *mut *mut f32
	) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
	pub fn vorbis_synthesis_read(
		v: *mut vorbis_dsp_state,
		samples: ::std::os::raw::c_int
	) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
	pub fn vorbis_packet_blocksize(
		vi: *mut vorbis_info,
		op: *mut ogg_packet
	) -> ::std::os::raw::c_long;
}
unsafe extern "C" {
	pub fn vorbis_synthesis_halfrate(
		v: *mut vorbis_info,
		flag: ::std::os::raw::c_int
	) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
	pub fn vorbis_synthesis_halfrate_p(v: *mut vorbis_info) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
	#[doc = " This is the primary function within libvorbisenc for setting up managed\n bitrate modes.\n\n Before this function is called, the \\ref vorbis_info\n struct should be initialized by using vorbis_info_init() from the libvorbis\n API.  After encoding, vorbis_info_clear() should be called.\n\n The max_bitrate, nominal_bitrate, and min_bitrate settings are used to set\n constraints for the encoded file.  This function uses these settings to\n select the appropriate encoding mode and set it up.\n\n \\param vi               Pointer to an initialized \\ref vorbis_info struct.\n \\param channels         The number of channels to be encoded.\n \\param rate             The sampling rate of the source audio.\n \\param max_bitrate      Desired maximum bitrate (limit). -1 indicates unset.\n \\param nominal_bitrate  Desired average, or central, bitrate. -1 indicates unset.\n \\param min_bitrate      Desired minimum bitrate. -1 indicates unset.\n\n \\return Zero for success, and negative values for failure.\n\n \\retval 0          Success.\n \\retval OV_EFAULT  Internal logic fault; indicates a bug or heap/stack corruption.\n \\retval OV_EINVAL  Invalid setup request, eg, out of range argument.\n \\retval OV_EIMPL   Unimplemented mode; unable to comply with bitrate request."]
	pub fn vorbis_encode_init(
		vi: *mut vorbis_info,
		channels: ::std::os::raw::c_long,
		rate: ::std::os::raw::c_long,
		max_bitrate: ::std::os::raw::c_long,
		nominal_bitrate: ::std::os::raw::c_long,
		min_bitrate: ::std::os::raw::c_long
	) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
	#[doc = " This function performs step-one of a three-step bitrate-managed encode\n setup.  It functions similarly to the one-step setup performed by \\ref\n vorbis_encode_init but allows an application to make further encode setup\n tweaks using \\ref vorbis_encode_ctl before finally calling \\ref\n vorbis_encode_setup_init to complete the setup process.\n\n Before this function is called, the \\ref vorbis_info struct should be\n initialized by using vorbis_info_init() from the libvorbis API.  After\n encoding, vorbis_info_clear() should be called.\n\n The max_bitrate, nominal_bitrate, and min_bitrate settings are used to set\n constraints for the encoded file.  This function uses these settings to\n select the appropriate encoding mode and set it up.\n\n \\param vi                Pointer to an initialized vorbis_info struct.\n \\param channels          The number of channels to be encoded.\n \\param rate              The sampling rate of the source audio.\n \\param max_bitrate       Desired maximum bitrate (limit). -1 indicates unset.\n \\param nominal_bitrate   Desired average, or central, bitrate. -1 indicates unset.\n \\param min_bitrate       Desired minimum bitrate. -1 indicates unset.\n\n \\return Zero for success, and negative for failure.\n\n \\retval 0           Success\n \\retval OV_EFAULT   Internal logic fault; indicates a bug or heap/stack corruption.\n \\retval OV_EINVAL   Invalid setup request, eg, out of range argument.\n \\retval OV_EIMPL    Unimplemented mode; unable to comply with bitrate request."]
	pub fn vorbis_encode_setup_managed(
		vi: *mut vorbis_info,
		channels: ::std::os::raw::c_long,
		rate: ::std::os::raw::c_long,
		max_bitrate: ::std::os::raw::c_long,
		nominal_bitrate: ::std::os::raw::c_long,
		min_bitrate: ::std::os::raw::c_long
	) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
	#[doc = " This function performs step-one of a three-step variable bitrate\n (quality-based) encode setup.  It functions similarly to the one-step setup\n performed by \\ref vorbis_encode_init_vbr() but allows an application to\n make further encode setup tweaks using \\ref vorbis_encode_ctl() before\n finally calling \\ref vorbis_encode_setup_init to complete the setup\n process.\n\n Before this function is called, the \\ref vorbis_info struct should be\n initialized by using \\ref vorbis_info_init() from the libvorbis API.  After\n encoding, vorbis_info_clear() should be called.\n\n \\param vi        Pointer to an initialized vorbis_info struct.\n \\param channels  The number of channels to be encoded.\n \\param rate      The sampling rate of the source audio.\n \\param quality   Desired quality level, currently from -0.1 to 1.0 (lo to hi).\n\n \\return Zero for success, and negative values for failure.\n\n \\retval  0          Success\n \\retval  OV_EFAULT  Internal logic fault; indicates a bug or heap/stack corruption.\n \\retval  OV_EINVAL  Invalid setup request, eg, out of range argument.\n \\retval  OV_EIMPL   Unimplemented mode; unable to comply with quality level request."]
	pub fn vorbis_encode_setup_vbr(
		vi: *mut vorbis_info,
		channels: ::std::os::raw::c_long,
		rate: ::std::os::raw::c_long,
		quality: f32
	) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
	#[doc = " This is the primary function within libvorbisenc for setting up variable\n bitrate (\"quality\" based) modes.\n\n\n Before this function is called, the vorbis_info struct should be\n initialized by using vorbis_info_init() from the libvorbis API. After\n encoding, vorbis_info_clear() should be called.\n\n \\param vi           Pointer to an initialized vorbis_info struct.\n \\param channels     The number of channels to be encoded.\n \\param rate         The sampling rate of the source audio.\n \\param base_quality Desired quality level, currently from -0.1 to 1.0 (lo to hi).\n\n\n \\return Zero for success, or a negative number for failure.\n\n \\retval 0           Success\n \\retval OV_EFAULT   Internal logic fault; indicates a bug or heap/stack corruption.\n \\retval OV_EINVAL   Invalid setup request, eg, out of range argument.\n \\retval OV_EIMPL    Unimplemented mode; unable to comply with quality level request."]
	pub fn vorbis_encode_init_vbr(
		vi: *mut vorbis_info,
		channels: ::std::os::raw::c_long,
		rate: ::std::os::raw::c_long,
		base_quality: f32
	) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
	#[doc = " This function performs the last stage of three-step encoding setup, as\n described in the API overview under managed bitrate modes.\n\n Before this function is called, the \\ref vorbis_info struct should be\n initialized by using vorbis_info_init() from the libvorbis API, one of\n \\ref vorbis_encode_setup_managed() or \\ref vorbis_encode_setup_vbr() called to\n initialize the high-level encoding setup, and \\ref vorbis_encode_ctl()\n called if necessary to make encoding setup changes.\n vorbis_encode_setup_init() finalizes the highlevel encoding structure into\n a complete encoding setup after which the application may make no further\n setup changes.\n\n After encoding, vorbis_info_clear() should be called.\n\n \\param vi Pointer to an initialized \\ref vorbis_info struct.\n\n \\return Zero for success, and negative values for failure.\n\n \\retval  0           Success.\n \\retval  OV_EFAULT  Internal logic fault; indicates a bug or heap/stack corruption.\n\n \\retval OV_EINVAL   Attempt to use vorbis_encode_setup_init() without first\n calling one of vorbis_encode_setup_managed() or vorbis_encode_setup_vbr() to\n initialize the high-level encoding setup\n"]
	pub fn vorbis_encode_setup_init(vi: *mut vorbis_info) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
	#[doc = " This function implements a generic interface to miscellaneous encoder\n settings similar to the classic UNIX 'ioctl()' system call.  Applications\n may use vorbis_encode_ctl() to query or set bitrate management or quality\n mode details by using one of several \\e request arguments detailed below.\n vorbis_encode_ctl() must be called after one of\n vorbis_encode_setup_managed() or vorbis_encode_setup_vbr().  When used\n to modify settings, \\ref vorbis_encode_ctl() must be called before \\ref\n vorbis_encode_setup_init().\n\n \\param vi      Pointer to an initialized vorbis_info struct.\n\n \\param number Specifies the desired action; See \\ref encctlcodes \"the list\n of available requests\".\n\n \\param arg void * pointing to a data structure matching the request\n argument.\n\n \\retval 0          Success. Any further return information (such as the result of a\n query) is placed into the storage pointed to by *arg.\n\n \\retval OV_EINVAL  Invalid argument, or an attempt to modify a setting after\n calling vorbis_encode_setup_init().\n\n \\retval OV_EIMPL   Unimplemented or unknown request"]
	pub fn vorbis_encode_ctl(
		vi: *mut vorbis_info,
		number: ::std::os::raw::c_int,
		arg: *mut ::std::os::raw::c_void
	) -> ::std::os::raw::c_int;
}
#[doc = " \\deprecated This is a deprecated interface. Please use vorbis_encode_ctl()\n with the \\ref ovectl_ratemanage2_arg struct and \\ref\n OV_ECTL_RATEMANAGE2_GET and \\ref OV_ECTL_RATEMANAGE2_SET calls in new code.\n\n The \\ref ovectl_ratemanage_arg structure is used with vorbis_encode_ctl()\n and the \\ref OV_ECTL_RATEMANAGE_GET, \\ref OV_ECTL_RATEMANAGE_SET, \\ref\n OV_ECTL_RATEMANAGE_AVG, \\ref OV_ECTL_RATEMANAGE_HARD calls in order to\n query and modify specifics of the encoder's bitrate management\n configuration."]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ovectl_ratemanage_arg {
	#[doc = "< nonzero if bitrate management is active"]
	pub management_active: ::std::os::raw::c_int,
	#[doc = " hard lower limit (in kilobits per second) below which the stream bitrate\nwill never be allowed for any given bitrate_hard_window seconds of time."]
	pub bitrate_hard_min: ::std::os::raw::c_long,
	#[doc = " hard upper limit (in kilobits per second) above which the stream bitrate\nwill never be allowed for any given bitrate_hard_window seconds of time."]
	pub bitrate_hard_max: ::std::os::raw::c_long,
	#[doc = " the window period (in seconds) used to regulate the hard bitrate minimum\nand maximum"]
	pub bitrate_hard_window: f64,
	#[doc = " soft lower limit (in kilobits per second) below which the average bitrate\ntracker will start nudging the bitrate higher."]
	pub bitrate_av_lo: ::std::os::raw::c_long,
	#[doc = " soft upper limit (in kilobits per second) above which the average bitrate\ntracker will start nudging the bitrate lower."]
	pub bitrate_av_hi: ::std::os::raw::c_long,
	#[doc = " the window period (in seconds) used to regulate the average bitrate\nminimum and maximum."]
	pub bitrate_av_window: f64,
	#[doc = " Regulates the relative centering of the average and hard windows; in\nlibvorbis 1.0 and 1.0.1, the hard window regulation overlapped but\nfollowed the average window regulation. In libvorbis 1.1 a bit-reservoir\ninterface replaces the old windowing interface; the older windowing\ninterface is simulated and this field has no effect."]
	pub bitrate_av_window_center: f64
}
#[doc = " \\name struct ovectl_ratemanage2_arg\n\n The ovectl_ratemanage2_arg structure is used with vorbis_encode_ctl() and\n the OV_ECTL_RATEMANAGE2_GET and OV_ECTL_RATEMANAGE2_SET calls in order to\n query and modify specifics of the encoder's bitrate management\n configuration."]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ovectl_ratemanage2_arg {
	#[doc = "< nonzero if bitrate management is active"]
	pub management_active: ::std::os::raw::c_int,
	#[doc = " Lower allowed bitrate limit in kilobits per second"]
	pub bitrate_limit_min_kbps: ::std::os::raw::c_long,
	#[doc = " Upper allowed bitrate limit in kilobits per second"]
	pub bitrate_limit_max_kbps: ::std::os::raw::c_long,
	#[doc = "<Size of the bitrate reservoir in bits"]
	pub bitrate_limit_reservoir_bits: ::std::os::raw::c_long,
	#[doc = " Regulates the bitrate reservoir's preferred fill level in a range from 0.0\n to 1.0; 0.0 tries to bank bits to buffer against future bitrate spikes, 1.0\n buffers against future sudden drops in instantaneous bitrate. Default is\n 0.1"]
	pub bitrate_limit_reservoir_bias: f64,
	#[doc = " Average bitrate setting in kilobits per second"]
	pub bitrate_average_kbps: ::std::os::raw::c_long,
	#[doc = " Slew rate limit setting for average bitrate adjustment; sets the minimum\n  time in seconds the bitrate tracker may swing from one extreme to the\n  other when boosting or damping average bitrate."]
	pub bitrate_average_damping: f64
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ov_callbacks {
	pub read_func: ::std::option::Option<
		unsafe extern "C" fn(
			ptr: *mut ::std::os::raw::c_void,
			size: usize,
			nmemb: usize,
			datasource: *mut ::std::os::raw::c_void
		) -> usize
	>,
	pub seek_func: ::std::option::Option<
		unsafe extern "C" fn(
			datasource: *mut ::std::os::raw::c_void,
			offset: ogg_int64_t,
			whence: ::std::os::raw::c_int
		) -> ::std::os::raw::c_int
	>,
	pub close_func: ::std::option::Option<
		unsafe extern "C" fn(datasource: *mut ::std::os::raw::c_void) -> ::std::os::raw::c_int
	>,
	pub tell_func: ::std::option::Option<
		unsafe extern "C" fn(datasource: *mut ::std::os::raw::c_void) -> ::std::os::raw::c_long
	>
}
#[repr(C)]
pub struct OggVorbis_File {
	pub datasource: *mut ::std::os::raw::c_void,
	pub seekable: ::std::os::raw::c_int,
	pub offset: ogg_int64_t,
	pub end: ogg_int64_t,
	pub oy: ogg_sync_state,
	pub links: ::std::os::raw::c_int,
	pub offsets: *mut ogg_int64_t,
	pub dataoffsets: *mut ogg_int64_t,
	pub serialnos: *mut ::std::os::raw::c_long,
	pub pcmlengths: *mut ogg_int64_t,
	pub vi: *mut vorbis_info,
	pub vc: *mut vorbis_comment,
	pub pcm_offset: ogg_int64_t,
	pub ready_state: ::std::os::raw::c_int,
	pub current_serialno: ::std::os::raw::c_long,
	pub current_link: ::std::os::raw::c_int,
	pub bittrack: f64,
	pub samptrack: f64,
	pub os: ogg_stream_state,
	pub vd: vorbis_dsp_state,
	pub vb: vorbis_block,
	pub callbacks: ov_callbacks
}
unsafe extern "C" {
	pub fn ov_clear(vf: *mut OggVorbis_File) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
	pub fn ov_open_callbacks(
		datasource: *mut ::std::os::raw::c_void,
		vf: *mut OggVorbis_File,
		initial: *const ::std::os::raw::c_char,
		ibytes: ::std::os::raw::c_long,
		callbacks: ov_callbacks
	) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
	pub fn ov_test_callbacks(
		datasource: *mut ::std::os::raw::c_void,
		vf: *mut OggVorbis_File,
		initial: *const ::std::os::raw::c_char,
		ibytes: ::std::os::raw::c_long,
		callbacks: ov_callbacks
	) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
	pub fn ov_test_open(vf: *mut OggVorbis_File) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
	pub fn ov_bitrate(vf: *mut OggVorbis_File, i: ::std::os::raw::c_int) -> ::std::os::raw::c_long;
}
unsafe extern "C" {
	pub fn ov_bitrate_instant(vf: *mut OggVorbis_File) -> ::std::os::raw::c_long;
}
unsafe extern "C" {
	pub fn ov_streams(vf: *mut OggVorbis_File) -> ::std::os::raw::c_long;
}
unsafe extern "C" {
	pub fn ov_seekable(vf: *mut OggVorbis_File) -> ::std::os::raw::c_long;
}
unsafe extern "C" {
	pub fn ov_serialnumber(
		vf: *mut OggVorbis_File,
		i: ::std::os::raw::c_int
	) -> ::std::os::raw::c_long;
}
unsafe extern "C" {
	pub fn ov_raw_total(vf: *mut OggVorbis_File, i: ::std::os::raw::c_int) -> ogg_int64_t;
}
unsafe extern "C" {
	pub fn ov_pcm_total(vf: *mut OggVorbis_File, i: ::std::os::raw::c_int) -> ogg_int64_t;
}
unsafe extern "C" {
	pub fn ov_time_total(vf: *mut OggVorbis_File, i: ::std::os::raw::c_int) -> f64;
}
unsafe extern "C" {
	pub fn ov_raw_seek(vf: *mut OggVorbis_File, pos: ogg_int64_t) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
	pub fn ov_pcm_seek(vf: *mut OggVorbis_File, pos: ogg_int64_t) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
	pub fn ov_pcm_seek_page(vf: *mut OggVorbis_File, pos: ogg_int64_t) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
	pub fn ov_time_seek(vf: *mut OggVorbis_File, pos: f64) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
	pub fn ov_time_seek_page(vf: *mut OggVorbis_File, pos: f64) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
	pub fn ov_raw_seek_lap(vf: *mut OggVorbis_File, pos: ogg_int64_t) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
	pub fn ov_pcm_seek_lap(vf: *mut OggVorbis_File, pos: ogg_int64_t) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
	pub fn ov_pcm_seek_page_lap(vf: *mut OggVorbis_File, pos: ogg_int64_t)
		-> ::std::os::raw::c_int;
}
unsafe extern "C" {
	pub fn ov_time_seek_lap(vf: *mut OggVorbis_File, pos: f64) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
	pub fn ov_time_seek_page_lap(vf: *mut OggVorbis_File, pos: f64) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
	pub fn ov_raw_tell(vf: *mut OggVorbis_File) -> ogg_int64_t;
}
unsafe extern "C" {
	pub fn ov_pcm_tell(vf: *mut OggVorbis_File) -> ogg_int64_t;
}
unsafe extern "C" {
	pub fn ov_time_tell(vf: *mut OggVorbis_File) -> f64;
}
unsafe extern "C" {
	pub fn ov_info(vf: *mut OggVorbis_File, link: ::std::os::raw::c_int) -> *mut vorbis_info;
}
unsafe extern "C" {
	pub fn ov_comment(vf: *mut OggVorbis_File, link: ::std::os::raw::c_int) -> *mut vorbis_comment;
}
unsafe extern "C" {
	pub fn ov_read_float(
		vf: *mut OggVorbis_File,
		pcm_channels: *mut *mut *mut f32,
		samples: ::std::os::raw::c_int,
		bitstream: *mut ::std::os::raw::c_int
	) -> ::std::os::raw::c_long;
}
unsafe extern "C" {
	pub fn ov_read_filter(
		vf: *mut OggVorbis_File,
		buffer: *mut ::std::os::raw::c_char,
		length: ::std::os::raw::c_int,
		bigendianp: ::std::os::raw::c_int,
		word: ::std::os::raw::c_int,
		sgned: ::std::os::raw::c_int,
		bitstream: *mut ::std::os::raw::c_int,
		filter: ::std::option::Option<
			unsafe extern "C" fn(
				pcm: *mut *mut f32,
				channels: ::std::os::raw::c_long,
				samples: ::std::os::raw::c_long,
				filter_param: *mut ::std::os::raw::c_void
			)
		>,
		filter_param: *mut ::std::os::raw::c_void
	) -> ::std::os::raw::c_long;
}
unsafe extern "C" {
	pub fn ov_read(
		vf: *mut OggVorbis_File,
		buffer: *mut ::std::os::raw::c_char,
		length: ::std::os::raw::c_int,
		bigendianp: ::std::os::raw::c_int,
		word: ::std::os::raw::c_int,
		sgned: ::std::os::raw::c_int,
		bitstream: *mut ::std::os::raw::c_int
	) -> ::std::os::raw::c_long;
}
unsafe extern "C" {
	pub fn ov_crosslap(vf1: *mut OggVorbis_File, vf2: *mut OggVorbis_File)
		-> ::std::os::raw::c_int;
}
unsafe extern "C" {
	pub fn ov_halfrate(
		vf: *mut OggVorbis_File,
		flag: ::std::os::raw::c_int
	) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
	pub fn ov_halfrate_p(vf: *mut OggVorbis_File) -> ::std::os::raw::c_int;
}
