use ::c2rust_bitfields;
#[cfg(target_arch = "x86")]
pub use ::core::arch::x86::{__m128d, _mm_cvtsd_si32, _mm_load_sd};
#[cfg(target_arch = "x86_64")]
pub use ::core::arch::x86_64::{__m128d, _mm_cvtsd_si32, _mm_load_sd};
use ogg_c2rust_sys::{ogg_packet, ogg_page, ogg_stream_state, ogg_sync_state, oggpack_buffer};

use super::{
	block::{alloc_chain, vorbis_block, vorbis_dsp_state},
	info::{vorbis_comment, vorbis_info}
};
extern "C" {
	pub type _IO_wide_data;
	pub type _IO_codecvt;
	pub type _IO_marker;
	fn malloc(__size: size_t) -> *mut ::core::ffi::c_void;
	fn calloc(__nmemb: size_t, __size: size_t) -> *mut ::core::ffi::c_void;
	fn realloc(__ptr: *mut ::core::ffi::c_void, __size: size_t) -> *mut ::core::ffi::c_void;
	fn free(__ptr: *mut ::core::ffi::c_void);
	fn fclose(__stream: *mut FILE) -> ::core::ffi::c_int;
	fn fopen(
		__filename: *const ::core::ffi::c_char,
		__modes: *const ::core::ffi::c_char
	) -> *mut FILE;
	fn fread(
		__ptr: *mut ::core::ffi::c_void,
		__size: size_t,
		__n: size_t,
		__stream: *mut FILE
	) -> ::core::ffi::c_ulong;
	fn fseek(
		__stream: *mut FILE,
		__off: ::core::ffi::c_long,
		__whence: ::core::ffi::c_int
	) -> ::core::ffi::c_int;
	fn ftell(__stream: *mut FILE) -> ::core::ffi::c_long;
	fn __errno_location() -> *mut ::core::ffi::c_int;
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
	fn rint(__x: ::core::ffi::c_double) -> ::core::ffi::c_double;
	fn ogg_sync_init(oy: *mut ogg_sync_state) -> ::core::ffi::c_int;
	fn ogg_sync_clear(oy: *mut ogg_sync_state) -> ::core::ffi::c_int;
	fn ogg_sync_reset(oy: *mut ogg_sync_state) -> ::core::ffi::c_int;
	fn ogg_sync_buffer(
		oy: *mut ogg_sync_state,
		size: ::core::ffi::c_long
	) -> *mut ::core::ffi::c_char;
	fn ogg_sync_wrote(oy: *mut ogg_sync_state, bytes: ::core::ffi::c_long) -> ::core::ffi::c_int;
	fn ogg_sync_pageseek(oy: *mut ogg_sync_state, og: *mut ogg_page) -> ::core::ffi::c_long;
	fn ogg_stream_pagein(os: *mut ogg_stream_state, og: *mut ogg_page) -> ::core::ffi::c_int;
	fn ogg_stream_packetout(os: *mut ogg_stream_state, op: *mut ogg_packet) -> ::core::ffi::c_int;
	fn ogg_stream_packetpeek(os: *mut ogg_stream_state, op: *mut ogg_packet) -> ::core::ffi::c_int;
	fn ogg_stream_init(
		os: *mut ogg_stream_state,
		serialno: ::core::ffi::c_int
	) -> ::core::ffi::c_int;
	fn ogg_stream_clear(os: *mut ogg_stream_state) -> ::core::ffi::c_int;
	fn ogg_stream_reset(os: *mut ogg_stream_state) -> ::core::ffi::c_int;
	fn ogg_stream_reset_serialno(
		os: *mut ogg_stream_state,
		serialno: ::core::ffi::c_int
	) -> ::core::ffi::c_int;
	fn ogg_page_continued(og: *const ogg_page) -> ::core::ffi::c_int;
	fn ogg_page_bos(og: *const ogg_page) -> ::core::ffi::c_int;
	fn ogg_page_eos(og: *const ogg_page) -> ::core::ffi::c_int;
	fn ogg_page_granulepos(og: *const ogg_page) -> ogg_int64_t;
	fn ogg_page_serialno(og: *const ogg_page) -> ::core::ffi::c_int;
	fn vorbis_info_init(vi: *mut vorbis_info);
	fn vorbis_info_clear(vi: *mut vorbis_info);
	fn vorbis_info_blocksize(vi: *mut vorbis_info, zo: ::core::ffi::c_int) -> ::core::ffi::c_int;
	fn vorbis_comment_init(vc: *mut vorbis_comment);
	fn vorbis_comment_clear(vc: *mut vorbis_comment);
	fn vorbis_block_init(v: *mut vorbis_dsp_state, vb: *mut vorbis_block) -> ::core::ffi::c_int;
	fn vorbis_block_clear(vb: *mut vorbis_block) -> ::core::ffi::c_int;
	fn vorbis_dsp_clear(v: *mut vorbis_dsp_state);
	fn vorbis_synthesis_idheader(op: *mut ogg_packet) -> ::core::ffi::c_int;
	fn vorbis_synthesis_headerin(
		vi: *mut vorbis_info,
		vc: *mut vorbis_comment,
		op: *mut ogg_packet
	) -> ::core::ffi::c_int;
	fn vorbis_synthesis_init(v: *mut vorbis_dsp_state, vi: *mut vorbis_info) -> ::core::ffi::c_int;
	fn vorbis_synthesis_restart(v: *mut vorbis_dsp_state) -> ::core::ffi::c_int;
	fn vorbis_synthesis(vb: *mut vorbis_block, op: *mut ogg_packet) -> ::core::ffi::c_int;
	fn vorbis_synthesis_trackonly(vb: *mut vorbis_block, op: *mut ogg_packet)
		-> ::core::ffi::c_int;
	fn vorbis_synthesis_blockin(
		v: *mut vorbis_dsp_state,
		vb: *mut vorbis_block
	) -> ::core::ffi::c_int;
	fn vorbis_synthesis_pcmout(
		v: *mut vorbis_dsp_state,
		pcm: *mut *mut *mut ::core::ffi::c_float
	) -> ::core::ffi::c_int;
	fn vorbis_synthesis_lapout(
		v: *mut vorbis_dsp_state,
		pcm: *mut *mut *mut ::core::ffi::c_float
	) -> ::core::ffi::c_int;
	fn vorbis_synthesis_read(
		v: *mut vorbis_dsp_state,
		samples: ::core::ffi::c_int
	) -> ::core::ffi::c_int;
	fn vorbis_packet_blocksize(vi: *mut vorbis_info, op: *mut ogg_packet) -> ::core::ffi::c_long;
	fn vorbis_synthesis_halfrate(
		v: *mut vorbis_info,
		flag: ::core::ffi::c_int
	) -> ::core::ffi::c_int;
	fn vorbis_synthesis_halfrate_p(v: *mut vorbis_info) -> ::core::ffi::c_int;
	fn vorbis_window(
		v: *mut vorbis_dsp_state,
		W: ::core::ffi::c_int
	) -> *const ::core::ffi::c_float;
}
pub type size_t = usize;
pub type __int16_t = i16;
pub type __int32_t = i32;
pub type __int64_t = i64;
pub type __uint64_t = u64;
pub type __off_t = ::core::ffi::c_long;
pub type __off64_t = ::core::ffi::c_long;
pub type int16_t = __int16_t;
pub type int32_t = __int32_t;
pub type int64_t = __int64_t;
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct _IO_FILE {
	pub _flags: ::core::ffi::c_int,
	pub _IO_read_ptr: *mut ::core::ffi::c_char,
	pub _IO_read_end: *mut ::core::ffi::c_char,
	pub _IO_read_base: *mut ::core::ffi::c_char,
	pub _IO_write_base: *mut ::core::ffi::c_char,
	pub _IO_write_ptr: *mut ::core::ffi::c_char,
	pub _IO_write_end: *mut ::core::ffi::c_char,
	pub _IO_buf_base: *mut ::core::ffi::c_char,
	pub _IO_buf_end: *mut ::core::ffi::c_char,
	pub _IO_save_base: *mut ::core::ffi::c_char,
	pub _IO_backup_base: *mut ::core::ffi::c_char,
	pub _IO_save_end: *mut ::core::ffi::c_char,
	pub _markers: *mut _IO_marker,
	pub _chain: *mut _IO_FILE,
	pub _fileno: ::core::ffi::c_int,
	#[bitfield(name = "_flags2", ty = "::core::ffi::c_int", bits = "0..=23")]
	pub _flags2: [u8; 3],
	pub _short_backupbuf: [::core::ffi::c_char; 1],
	pub _old_offset: __off_t,
	pub _cur_column: ::core::ffi::c_ushort,
	pub _vtable_offset: ::core::ffi::c_schar,
	pub _shortbuf: [::core::ffi::c_char; 1],
	pub _lock: *mut ::core::ffi::c_void,
	pub _offset: __off64_t,
	pub _codecvt: *mut _IO_codecvt,
	pub _wide_data: *mut _IO_wide_data,
	pub _freeres_list: *mut _IO_FILE,
	pub _freeres_buf: *mut ::core::ffi::c_void,
	pub _prevchain: *mut *mut _IO_FILE,
	pub _mode: ::core::ffi::c_int,
	pub _unused3: ::core::ffi::c_int,
	pub _total_written: __uint64_t,
	pub _unused2: [::core::ffi::c_char; 8]
}
pub type _IO_lock_t = ();
pub type FILE = _IO_FILE;
pub type ogg_int16_t = int16_t;
pub type ogg_int32_t = int32_t;
pub type ogg_int64_t = int64_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ov_callbacks {
	pub read_func: Option<
		unsafe extern "C" fn(
			*mut ::core::ffi::c_void,
			size_t,
			size_t,
			*mut ::core::ffi::c_void
		) -> size_t
	>,
	pub seek_func: Option<
		unsafe extern "C" fn(
			*mut ::core::ffi::c_void,
			ogg_int64_t,
			::core::ffi::c_int
		) -> ::core::ffi::c_int
	>,
	pub close_func: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ::core::ffi::c_int>,
	pub tell_func: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ::core::ffi::c_long>
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct OggVorbis_File {
	pub datasource: *mut ::core::ffi::c_void,
	pub seekable: ::core::ffi::c_int,
	pub offset: ogg_int64_t,
	pub end: ogg_int64_t,
	pub oy: ogg_sync_state,
	pub links: ::core::ffi::c_int,
	pub offsets: *mut ogg_int64_t,
	pub dataoffsets: *mut ogg_int64_t,
	pub serialnos: *mut ::core::ffi::c_long,
	pub pcmlengths: *mut ogg_int64_t,
	pub vi: *mut vorbis_info,
	pub vc: *mut vorbis_comment,
	pub pcm_offset: ogg_int64_t,
	pub ready_state: ::core::ffi::c_int,
	pub current_serialno: ::core::ffi::c_long,
	pub current_link: ::core::ffi::c_int,
	pub bittrack: ::core::ffi::c_double,
	pub samptrack: ::core::ffi::c_double,
	pub os: ogg_stream_state,
	pub vd: vorbis_dsp_state,
	pub vb: vorbis_block,
	pub callbacks: ov_callbacks
}
pub type vorbis_fpu_control = ogg_int16_t;
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct __mm_load_sd_struct {
	pub __u: ::core::ffi::c_double
}
pub const SEEK_SET: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const SEEK_CUR: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const SEEK_END: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const OV_FALSE: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
pub const OV_EOF: ::core::ffi::c_int = -(2 as ::core::ffi::c_int);
pub const OV_HOLE: ::core::ffi::c_int = -(3 as ::core::ffi::c_int);
pub const OV_EREAD: ::core::ffi::c_int = -(128 as ::core::ffi::c_int);
pub const OV_EFAULT: ::core::ffi::c_int = -(129 as ::core::ffi::c_int);
pub const OV_EINVAL: ::core::ffi::c_int = -(131 as ::core::ffi::c_int);
pub const OV_ENOTVORBIS: ::core::ffi::c_int = -(132 as ::core::ffi::c_int);
pub const OV_EBADHEADER: ::core::ffi::c_int = -(133 as ::core::ffi::c_int);
pub const OV_EBADPACKET: ::core::ffi::c_int = -(136 as ::core::ffi::c_int);
pub const OV_EBADLINK: ::core::ffi::c_int = -(137 as ::core::ffi::c_int);
pub const OV_ENOSEEK: ::core::ffi::c_int = -(138 as ::core::ffi::c_int);
pub const PARTOPEN: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const OPENED: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const STREAMSET: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
pub const INITSET: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const CHUNKSIZE: ::core::ffi::c_int = 65536 as ::core::ffi::c_int;
pub const READSIZE: ::core::ffi::c_int = 2048 as ::core::ffi::c_int;
unsafe extern "C" fn _get_data(mut vf: *mut OggVorbis_File) -> ::core::ffi::c_long {
	*__errno_location() = 0 as ::core::ffi::c_int;
	if (*vf).callbacks.read_func.is_none() {
		return -(1 as ::core::ffi::c_int) as ::core::ffi::c_long;
	}
	if !(*vf).datasource.is_null() {
		let mut buffer: *mut ::core::ffi::c_char =
			ogg_sync_buffer(&raw mut (*vf).oy, READSIZE as ::core::ffi::c_long);
		let mut bytes: ::core::ffi::c_long = (*vf)
			.callbacks
			.read_func
			.expect("non-null function pointer")(
			buffer as *mut ::core::ffi::c_void,
			1 as size_t,
			READSIZE as size_t,
			(*vf).datasource
		) as ::core::ffi::c_long;
		if bytes > 0 as ::core::ffi::c_long {
			ogg_sync_wrote(&raw mut (*vf).oy, bytes);
		}
		if bytes == 0 as ::core::ffi::c_long && *__errno_location() != 0 {
			return -(1 as ::core::ffi::c_int) as ::core::ffi::c_long;
		}
		return bytes;
	} else {
		return 0 as ::core::ffi::c_long;
	};
}
unsafe extern "C" fn _seek_helper(
	mut vf: *mut OggVorbis_File,
	mut offset: ogg_int64_t
) -> ::core::ffi::c_int {
	if !(*vf).datasource.is_null() {
		if (*vf).offset != offset {
			if (*vf).callbacks.seek_func.is_none()
				|| (*vf)
					.callbacks
					.seek_func
					.expect("non-null function pointer")((*vf).datasource, offset, SEEK_SET)
					== -(1 as ::core::ffi::c_int)
			{
				return OV_EREAD;
			}
			(*vf).offset = offset;
			ogg_sync_reset(&raw mut (*vf).oy);
		}
	} else {
		return OV_EFAULT;
	}
	return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn _get_next_page(
	mut vf: *mut OggVorbis_File,
	mut og: *mut ogg_page,
	mut boundary: ogg_int64_t
) -> ogg_int64_t {
	if boundary > 0 as ogg_int64_t {
		boundary += (*vf).offset;
	}
	loop {
		let mut more: ::core::ffi::c_long = 0;
		if boundary > 0 as ogg_int64_t && (*vf).offset >= boundary {
			return -(1 as ::core::ffi::c_int) as ogg_int64_t;
		}
		more = ogg_sync_pageseek(&raw mut (*vf).oy, og);
		if more < 0 as ::core::ffi::c_long {
			(*vf).offset = ((*vf).offset as ::core::ffi::c_long - more) as ogg_int64_t;
		} else if more == 0 as ::core::ffi::c_long {
			if boundary == 0 {
				return -(1 as ::core::ffi::c_int) as ogg_int64_t;
			}
			let mut ret: ::core::ffi::c_long = _get_data(vf);
			if ret == 0 as ::core::ffi::c_long {
				return -(2 as ::core::ffi::c_int) as ogg_int64_t;
			}
			if ret < 0 as ::core::ffi::c_long {
				return -(128 as ::core::ffi::c_int) as ogg_int64_t;
			}
		} else {
			let mut ret_0: ogg_int64_t = (*vf).offset;
			(*vf).offset = ((*vf).offset as ::core::ffi::c_long + more) as ogg_int64_t;
			return ret_0;
		}
	}
}
unsafe extern "C" fn _get_prev_page(
	mut vf: *mut OggVorbis_File,
	mut begin: ogg_int64_t,
	mut og: *mut ogg_page
) -> ogg_int64_t {
	let mut end: ogg_int64_t = begin;
	let mut ret: ogg_int64_t = 0;
	let mut offset: ogg_int64_t = -(1 as ::core::ffi::c_int) as ogg_int64_t;
	while offset == -(1 as ::core::ffi::c_int) as ogg_int64_t {
		begin -= CHUNKSIZE as ogg_int64_t;
		if begin < 0 as ogg_int64_t {
			begin = 0 as ogg_int64_t;
		}
		ret = _seek_helper(vf, begin) as ogg_int64_t;
		if ret != 0 {
			return ret;
		}
		while (*vf).offset < end {
			memset(
				og as *mut ::core::ffi::c_void,
				0 as ::core::ffi::c_int,
				::core::mem::size_of::<ogg_page>() as size_t
			);
			ret = _get_next_page(vf, og, end - (*vf).offset);
			if ret == OV_EREAD as ogg_int64_t {
				return -(128 as ::core::ffi::c_int) as ogg_int64_t;
			}
			if ret < 0 as ogg_int64_t {
				break;
			}
			offset = ret;
		}
	}
	if (*og).header_len == 0 as ::core::ffi::c_long {
		ret = _seek_helper(vf, offset) as ogg_int64_t;
		if ret != 0 {
			return ret;
		}
		ret = _get_next_page(vf, og, CHUNKSIZE as ogg_int64_t);
		if ret < 0 as ogg_int64_t {
			return -(129 as ::core::ffi::c_int) as ogg_int64_t;
		}
	}
	return offset;
}
unsafe extern "C" fn _add_serialno(
	mut og: *mut ogg_page,
	mut serialno_list: *mut *mut ::core::ffi::c_long,
	mut n: *mut ::core::ffi::c_int
) {
	let mut s: ::core::ffi::c_long = ogg_page_serialno(og) as ::core::ffi::c_long;
	*n += 1;
	if !(*serialno_list).is_null() {
		*serialno_list = realloc(
			*serialno_list as *mut ::core::ffi::c_void,
			(::core::mem::size_of::<::core::ffi::c_long>() as size_t).wrapping_mul(*n as size_t)
		) as *mut ::core::ffi::c_long;
	} else {
		*serialno_list = malloc(::core::mem::size_of::<::core::ffi::c_long>() as size_t)
			as *mut ::core::ffi::c_long;
	}
	*(*serialno_list).offset((*n - 1 as ::core::ffi::c_int) as isize) = s;
}
unsafe extern "C" fn _lookup_serialno(
	mut s: ::core::ffi::c_long,
	mut serialno_list: *mut ::core::ffi::c_long,
	mut n: ::core::ffi::c_int
) -> ::core::ffi::c_int {
	if !serialno_list.is_null() {
		loop {
			let fresh0 = n;
			n = n - 1;
			if !(fresh0 != 0) {
				break;
			}
			if *serialno_list == s {
				return 1 as ::core::ffi::c_int;
			}
			serialno_list = serialno_list.offset(1);
		}
	}
	return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn _lookup_page_serialno(
	mut og: *mut ogg_page,
	mut serialno_list: *mut ::core::ffi::c_long,
	mut n: ::core::ffi::c_int
) -> ::core::ffi::c_int {
	let mut s: ::core::ffi::c_long = ogg_page_serialno(og) as ::core::ffi::c_long;
	return _lookup_serialno(s, serialno_list, n);
}
unsafe extern "C" fn _get_prev_page_serial(
	mut vf: *mut OggVorbis_File,
	mut begin: ogg_int64_t,
	mut serial_list: *mut ::core::ffi::c_long,
	mut serial_n: ::core::ffi::c_int,
	mut serialno: *mut ::core::ffi::c_int,
	mut granpos: *mut ogg_int64_t
) -> ogg_int64_t {
	let mut og: ogg_page = ogg_page {
		header: ::core::ptr::null_mut::<::core::ffi::c_uchar>(),
		header_len: 0,
		body: ::core::ptr::null_mut::<::core::ffi::c_uchar>(),
		body_len: 0
	};
	let mut end: ogg_int64_t = begin;
	let mut ret: ogg_int64_t = 0;
	let mut prefoffset: ogg_int64_t = -(1 as ::core::ffi::c_int) as ogg_int64_t;
	let mut offset: ogg_int64_t = -(1 as ::core::ffi::c_int) as ogg_int64_t;
	let mut ret_serialno: ogg_int64_t = -(1 as ::core::ffi::c_int) as ogg_int64_t;
	let mut ret_gran: ogg_int64_t = -(1 as ::core::ffi::c_int) as ogg_int64_t;
	while offset == -(1 as ::core::ffi::c_int) as ogg_int64_t {
		begin -= CHUNKSIZE as ogg_int64_t;
		if begin < 0 as ogg_int64_t {
			begin = 0 as ogg_int64_t;
		}
		ret = _seek_helper(vf, begin) as ogg_int64_t;
		if ret != 0 {
			return ret;
		}
		while (*vf).offset < end {
			ret = _get_next_page(vf, &raw mut og, end - (*vf).offset);
			if ret == OV_EREAD as ogg_int64_t {
				return -(128 as ::core::ffi::c_int) as ogg_int64_t;
			}
			if ret < 0 as ogg_int64_t {
				break;
			}
			ret_serialno = ogg_page_serialno(&raw mut og) as ogg_int64_t;
			ret_gran = ogg_page_granulepos(&raw mut og);
			offset = ret;
			if ret_serialno == *serialno as ogg_int64_t {
				prefoffset = ret;
				*granpos = ret_gran;
			}
			if _lookup_serialno(ret_serialno as ::core::ffi::c_long, serial_list, serial_n) == 0 {
				prefoffset = -(1 as ::core::ffi::c_int) as ogg_int64_t;
			}
		}
		if begin == 0 && (*vf).offset < 0 as ogg_int64_t {
			return OV_EBADLINK as ogg_int64_t;
		}
	}
	if prefoffset >= 0 as ogg_int64_t {
		return prefoffset;
	}
	*serialno = ret_serialno as ::core::ffi::c_int;
	*granpos = ret_gran;
	return offset;
}
unsafe extern "C" fn _fetch_headers(
	mut vf: *mut OggVorbis_File,
	mut vi: *mut vorbis_info,
	mut vc: *mut vorbis_comment,
	mut serialno_list: *mut *mut ::core::ffi::c_long,
	mut serialno_n: *mut ::core::ffi::c_int,
	mut og_ptr: *mut ogg_page
) -> ::core::ffi::c_int {
	let mut current_block: u64;
	let mut og: ogg_page = ogg_page {
		header: ::core::ptr::null_mut::<::core::ffi::c_uchar>(),
		header_len: 0,
		body: ::core::ptr::null_mut::<::core::ffi::c_uchar>(),
		body_len: 0
	};
	let mut op: ogg_packet = ogg_packet {
		packet: ::core::ptr::null_mut::<::core::ffi::c_uchar>(),
		bytes: 0,
		b_o_s: 0,
		e_o_s: 0,
		granulepos: 0,
		packetno: 0
	};
	let mut i: ::core::ffi::c_int = 0;
	let mut ret: ::core::ffi::c_int = 0;
	let mut allbos: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
	if og_ptr.is_null() {
		let mut llret: ogg_int64_t = _get_next_page(vf, &raw mut og, CHUNKSIZE as ogg_int64_t);
		if llret == OV_EREAD as ogg_int64_t {
			return -(128 as ::core::ffi::c_int);
		}
		if llret < 0 as ogg_int64_t {
			return -(132 as ::core::ffi::c_int);
		}
		og_ptr = &raw mut og;
	}
	vorbis_info_init(vi);
	vorbis_comment_init(vc);
	(*vf).ready_state = OPENED;
	loop {
		if !(ogg_page_bos(og_ptr) != 0) {
			current_block = 11459959175219260272;
			break;
		}
		if !serialno_list.is_null() {
			if _lookup_page_serialno(og_ptr, *serialno_list, *serialno_n) != 0 {
				if !(*serialno_list).is_null() {
					free(*serialno_list as *mut ::core::ffi::c_void);
				}
				*serialno_list = ::core::ptr::null_mut::<::core::ffi::c_long>();
				*serialno_n = 0 as ::core::ffi::c_int;
				ret = OV_EBADHEADER;
				current_block = 11265694546655345872;
				break;
			} else {
				_add_serialno(og_ptr, serialno_list, serialno_n);
			}
		}
		if (*vf).ready_state < STREAMSET {
			ogg_stream_reset_serialno(&raw mut (*vf).os, ogg_page_serialno(og_ptr));
			ogg_stream_pagein(&raw mut (*vf).os, og_ptr);
			if ogg_stream_packetout(&raw mut (*vf).os, &raw mut op) > 0 as ::core::ffi::c_int
				&& vorbis_synthesis_idheader(&raw mut op) != 0
			{
				(*vf).ready_state = STREAMSET;
				ret = vorbis_synthesis_headerin(vi, vc, &raw mut op);
				if ret != 0 {
					ret = OV_EBADHEADER;
					current_block = 11265694546655345872;
					break;
				}
			}
		}
		let mut llret_0: ogg_int64_t = _get_next_page(vf, og_ptr, CHUNKSIZE as ogg_int64_t);
		if llret_0 == OV_EREAD as ogg_int64_t {
			ret = OV_EREAD;
			current_block = 11265694546655345872;
			break;
		} else if llret_0 < 0 as ogg_int64_t {
			ret = OV_ENOTVORBIS;
			current_block = 11265694546655345872;
			break;
		} else {
			if !((*vf).ready_state == STREAMSET
				&& (*vf).os.serialno == ogg_page_serialno(og_ptr) as ::core::ffi::c_long)
			{
				continue;
			}
			ogg_stream_pagein(&raw mut (*vf).os, og_ptr);
			current_block = 11459959175219260272;
			break;
		}
	}
	match current_block {
		11459959175219260272 => {
			if (*vf).ready_state != STREAMSET {
				ret = OV_ENOTVORBIS;
			} else {
				i = 0 as ::core::ffi::c_int;
				's_173: loop {
					if !(i < 2 as ::core::ffi::c_int) {
						current_block = 7420279277351916581;
						break;
					}
					while i < 2 as ::core::ffi::c_int {
						let mut result: ::core::ffi::c_int =
							ogg_stream_packetout(&raw mut (*vf).os, &raw mut op);
						if result == 0 as ::core::ffi::c_int {
							break;
						}
						if result == -(1 as ::core::ffi::c_int) {
							ret = OV_EBADHEADER;
							current_block = 11265694546655345872;
							break 's_173;
						} else {
							ret = vorbis_synthesis_headerin(vi, vc, &raw mut op);
							if ret != 0 {
								current_block = 11265694546655345872;
								break 's_173;
							}
							i += 1;
						}
					}
					while i < 2 as ::core::ffi::c_int {
						if _get_next_page(vf, og_ptr, CHUNKSIZE as ogg_int64_t) < 0 as ogg_int64_t {
							ret = OV_EBADHEADER;
							current_block = 11265694546655345872;
							break 's_173;
						} else if (*vf).os.serialno
							== ogg_page_serialno(og_ptr) as ::core::ffi::c_long
						{
							ogg_stream_pagein(&raw mut (*vf).os, og_ptr);
							break;
						} else {
							if !(ogg_page_bos(og_ptr) != 0) {
								continue;
							}
							if allbos != 0 {
								ret = OV_EBADHEADER;
								current_block = 11265694546655345872;
								break 's_173;
							} else {
								allbos = 1 as ::core::ffi::c_int;
							}
						}
					}
				}
				match current_block {
					11265694546655345872 => {}
					_ => return 0 as ::core::ffi::c_int
				}
			}
		}
		_ => {}
	}
	vorbis_info_clear(vi);
	vorbis_comment_clear(vc);
	(*vf).ready_state = OPENED;
	return ret;
}
unsafe extern "C" fn _initial_pcmoffset(
	mut vf: *mut OggVorbis_File,
	mut vi: *mut vorbis_info
) -> ogg_int64_t {
	let mut og: ogg_page = ogg_page {
		header: ::core::ptr::null_mut::<::core::ffi::c_uchar>(),
		header_len: 0,
		body: ::core::ptr::null_mut::<::core::ffi::c_uchar>(),
		body_len: 0
	};
	let mut accumulated: ogg_int64_t = 0 as ogg_int64_t;
	let mut lastblock: ::core::ffi::c_long = -(1 as ::core::ffi::c_int) as ::core::ffi::c_long;
	let mut result: ::core::ffi::c_int = 0;
	let mut serialno: ::core::ffi::c_int = (*vf).os.serialno as ::core::ffi::c_int;
	loop {
		let mut op: ogg_packet = ogg_packet {
			packet: ::core::ptr::null_mut::<::core::ffi::c_uchar>(),
			bytes: 0,
			b_o_s: 0,
			e_o_s: 0,
			granulepos: 0,
			packetno: 0
		};
		if _get_next_page(vf, &raw mut og, -(1 as ::core::ffi::c_int) as ogg_int64_t)
			< 0 as ogg_int64_t
		{
			break;
		}
		if ogg_page_bos(&raw mut og) != 0 {
			break;
		}
		if ogg_page_serialno(&raw mut og) != serialno {
			continue;
		}
		ogg_stream_pagein(&raw mut (*vf).os, &raw mut og);
		loop {
			result = ogg_stream_packetout(&raw mut (*vf).os, &raw mut op);
			if !(result != 0) {
				break;
			}
			if result > 0 as ::core::ffi::c_int {
				let mut thisblock: ::core::ffi::c_long = vorbis_packet_blocksize(vi, &raw mut op);
				if thisblock >= 0 as ::core::ffi::c_long {
					if lastblock != -(1 as ::core::ffi::c_int) as ::core::ffi::c_long {
						accumulated = (accumulated as ::core::ffi::c_long
							+ (lastblock + thisblock >> 2 as ::core::ffi::c_int))
							as ogg_int64_t;
					}
					lastblock = thisblock;
				}
			}
		}
		if !(ogg_page_granulepos(&raw mut og) != -(1 as ::core::ffi::c_int) as ogg_int64_t) {
			continue;
		}
		accumulated = ogg_page_granulepos(&raw mut og) - accumulated;
		break;
	}
	if accumulated < 0 as ogg_int64_t {
		accumulated = 0 as ogg_int64_t;
	}
	return accumulated;
}
unsafe extern "C" fn _bisect_forward_serialno(
	mut vf: *mut OggVorbis_File,
	mut begin: ogg_int64_t,
	mut searched: ogg_int64_t,
	mut end: ogg_int64_t,
	mut endgran: ogg_int64_t,
	mut endserial: ::core::ffi::c_int,
	mut currentno_list: *mut ::core::ffi::c_long,
	mut currentnos: ::core::ffi::c_int,
	mut m: ::core::ffi::c_long
) -> ::core::ffi::c_int {
	let mut pcmoffset: ogg_int64_t = 0;
	let mut dataoffset: ogg_int64_t = searched;
	let mut endsearched: ogg_int64_t = end;
	let mut next: ogg_int64_t = end;
	let mut searchgran: ogg_int64_t = -(1 as ::core::ffi::c_int) as ogg_int64_t;
	let mut og: ogg_page = ogg_page {
		header: ::core::ptr::null_mut::<::core::ffi::c_uchar>(),
		header_len: 0,
		body: ::core::ptr::null_mut::<::core::ffi::c_uchar>(),
		body_len: 0
	};
	let mut ret: ogg_int64_t = 0;
	let mut last: ogg_int64_t = 0;
	let mut serialno: ::core::ffi::c_int = (*vf).os.serialno as ::core::ffi::c_int;
	if _lookup_serialno(endserial as ::core::ffi::c_long, currentno_list, currentnos) != 0 {
		searched = end;
		while endserial != serialno {
			endserial = serialno;
			searched = _get_prev_page_serial(
				vf,
				searched,
				currentno_list,
				currentnos,
				&raw mut endserial,
				&raw mut endgran
			);
		}
		(*vf).links = (m + 1 as ::core::ffi::c_long) as ::core::ffi::c_int;
		if !(*vf).offsets.is_null() {
			free((*vf).offsets as *mut ::core::ffi::c_void);
		}
		if !(*vf).serialnos.is_null() {
			free((*vf).serialnos as *mut ::core::ffi::c_void);
		}
		if !(*vf).dataoffsets.is_null() {
			free((*vf).dataoffsets as *mut ::core::ffi::c_void);
		}
		(*vf).offsets = malloc(
			(((*vf).links + 1 as ::core::ffi::c_int) as size_t)
				.wrapping_mul(::core::mem::size_of::<ogg_int64_t>() as size_t)
		) as *mut ogg_int64_t;
		(*vf).vi = realloc(
			(*vf).vi as *mut ::core::ffi::c_void,
			((*vf).links as size_t).wrapping_mul(::core::mem::size_of::<vorbis_info>() as size_t)
		) as *mut vorbis_info;
		(*vf).vc = realloc(
			(*vf).vc as *mut ::core::ffi::c_void,
			((*vf).links as size_t)
				.wrapping_mul(::core::mem::size_of::<vorbis_comment>() as size_t)
		) as *mut vorbis_comment;
		(*vf).serialnos = malloc(
			((*vf).links as size_t)
				.wrapping_mul(::core::mem::size_of::<::core::ffi::c_long>() as size_t)
		) as *mut ::core::ffi::c_long;
		(*vf).dataoffsets = malloc(
			((*vf).links as size_t).wrapping_mul(::core::mem::size_of::<ogg_int64_t>() as size_t)
		) as *mut ogg_int64_t;
		(*vf).pcmlengths = malloc(
			(((*vf).links * 2 as ::core::ffi::c_int) as size_t)
				.wrapping_mul(::core::mem::size_of::<ogg_int64_t>() as size_t)
		) as *mut ogg_int64_t;
		*(*vf)
			.offsets
			.offset((m + 1 as ::core::ffi::c_long) as isize) = end;
		*(*vf).offsets.offset(m as isize) = begin;
		*(*vf)
			.pcmlengths
			.offset((m * 2 as ::core::ffi::c_long + 1 as ::core::ffi::c_long) as isize) =
			if endgran < 0 as ogg_int64_t {
				0 as ogg_int64_t
			} else {
				endgran
			};
	} else {
		let mut next_serialno_list: *mut ::core::ffi::c_long =
			::core::ptr::null_mut::<::core::ffi::c_long>();
		let mut next_serialnos: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
		let mut vi: vorbis_info = vorbis_info {
			version: 0,
			channels: 0,
			rate: 0,
			bitrate_upper: 0,
			bitrate_nominal: 0,
			bitrate_lower: 0,
			bitrate_window: 0,
			codec_setup: ::core::ptr::null_mut::<::core::ffi::c_void>()
		};
		let mut vc: vorbis_comment = vorbis_comment {
			user_comments: ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(),
			comment_lengths: ::core::ptr::null_mut::<::core::ffi::c_int>(),
			comments: 0,
			vendor: ::core::ptr::null_mut::<::core::ffi::c_char>()
		};
		let mut testserial: ::core::ffi::c_int = serialno + 1 as ::core::ffi::c_int;
		while searched < endsearched {
			let mut bisect: ogg_int64_t = 0;
			if endsearched - searched < CHUNKSIZE as ogg_int64_t {
				bisect = searched;
			} else {
				bisect = (searched + endsearched) / 2 as ogg_int64_t;
			}
			ret = _seek_helper(vf, bisect) as ogg_int64_t;
			if ret != 0 {
				return ret as ::core::ffi::c_int;
			}
			last = _get_next_page(vf, &raw mut og, -(1 as ::core::ffi::c_int) as ogg_int64_t);
			if last == OV_EREAD as ogg_int64_t {
				return -(128 as ::core::ffi::c_int);
			}
			if last < 0 as ogg_int64_t
				|| _lookup_page_serialno(&raw mut og, currentno_list, currentnos) == 0
			{
				endsearched = bisect;
				if last >= 0 as ogg_int64_t {
					next = last;
				}
			} else {
				searched = (*vf).offset;
			}
		}
		searched = next;
		while testserial != serialno {
			testserial = serialno;
			searched = _get_prev_page_serial(
				vf,
				searched,
				currentno_list,
				currentnos,
				&raw mut testserial,
				&raw mut searchgran
			);
		}
		ret = _seek_helper(vf, next) as ogg_int64_t;
		if ret != 0 {
			return ret as ::core::ffi::c_int;
		}
		ret = _fetch_headers(
			vf,
			&raw mut vi,
			&raw mut vc,
			&raw mut next_serialno_list,
			&raw mut next_serialnos,
			::core::ptr::null_mut::<ogg_page>()
		) as ogg_int64_t;
		if ret != 0 {
			return ret as ::core::ffi::c_int;
		}
		serialno = (*vf).os.serialno as ::core::ffi::c_int;
		dataoffset = (*vf).offset;
		pcmoffset = _initial_pcmoffset(vf, &raw mut vi);
		ret = _bisect_forward_serialno(
			vf,
			next,
			(*vf).offset,
			end,
			endgran,
			endserial,
			next_serialno_list,
			next_serialnos,
			m + 1 as ::core::ffi::c_long
		) as ogg_int64_t;
		if ret != 0 {
			return ret as ::core::ffi::c_int;
		}
		if !next_serialno_list.is_null() {
			free(next_serialno_list as *mut ::core::ffi::c_void);
		}
		*(*vf)
			.offsets
			.offset((m + 1 as ::core::ffi::c_long) as isize) = next;
		*(*vf)
			.serialnos
			.offset((m + 1 as ::core::ffi::c_long) as isize) = serialno as ::core::ffi::c_long;
		*(*vf)
			.dataoffsets
			.offset((m + 1 as ::core::ffi::c_long) as isize) = dataoffset;
		*(*vf).vi.offset((m + 1 as ::core::ffi::c_long) as isize) = vi;
		*(*vf).vc.offset((m + 1 as ::core::ffi::c_long) as isize) = vc;
		*(*vf)
			.pcmlengths
			.offset((m * 2 as ::core::ffi::c_long + 1 as ::core::ffi::c_long) as isize) = searchgran;
		*(*vf)
			.pcmlengths
			.offset((m * 2 as ::core::ffi::c_long + 2 as ::core::ffi::c_long) as isize) = pcmoffset;
		*(*vf)
			.pcmlengths
			.offset((m * 2 as ::core::ffi::c_long + 3 as ::core::ffi::c_long) as isize) -= pcmoffset;
		if *(*vf)
			.pcmlengths
			.offset((m * 2 as ::core::ffi::c_long + 3 as ::core::ffi::c_long) as isize)
			< 0 as ogg_int64_t
		{
			*(*vf)
				.pcmlengths
				.offset((m * 2 as ::core::ffi::c_long + 3 as ::core::ffi::c_long) as isize) = 0 as ogg_int64_t;
		}
	}
	return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn _make_decode_ready(mut vf: *mut OggVorbis_File) -> ::core::ffi::c_int {
	if (*vf).ready_state > STREAMSET {
		return 0 as ::core::ffi::c_int;
	}
	if (*vf).ready_state < STREAMSET {
		return OV_EFAULT;
	}
	if (*vf).seekable != 0 {
		if vorbis_synthesis_init(
			&raw mut (*vf).vd,
			(*vf).vi.offset((*vf).current_link as isize)
		) != 0
		{
			return OV_EBADLINK;
		}
	} else if vorbis_synthesis_init(&raw mut (*vf).vd, (*vf).vi) != 0 {
		return OV_EBADLINK;
	}
	vorbis_block_init(&raw mut (*vf).vd, &raw mut (*vf).vb);
	(*vf).ready_state = INITSET;
	(*vf).bittrack = 0.0f64;
	(*vf).samptrack = 0.0f64;
	return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn _open_seekable2(mut vf: *mut OggVorbis_File) -> ::core::ffi::c_int {
	let mut dataoffset: ogg_int64_t = *(*vf).dataoffsets.offset(0 as ::core::ffi::c_int as isize);
	let mut end: ogg_int64_t = 0;
	let mut endgran: ogg_int64_t = -(1 as ::core::ffi::c_int) as ogg_int64_t;
	let mut endserial: ::core::ffi::c_int = (*vf).os.serialno as ::core::ffi::c_int;
	let mut serialno: ::core::ffi::c_int = (*vf).os.serialno as ::core::ffi::c_int;
	let mut pcmoffset: ogg_int64_t = _initial_pcmoffset(vf, (*vf).vi);
	if (*vf).callbacks.seek_func.is_some() && (*vf).callbacks.tell_func.is_some() {
		(*vf)
			.callbacks
			.seek_func
			.expect("non-null function pointer")((*vf).datasource, 0 as ogg_int64_t, SEEK_END);
		(*vf).end = (*vf)
			.callbacks
			.tell_func
			.expect("non-null function pointer")((*vf).datasource) as ogg_int64_t;
		(*vf).offset = (*vf).end;
	} else {
		(*vf).end = -(1 as ::core::ffi::c_int) as ogg_int64_t;
		(*vf).offset = (*vf).end;
	}
	if (*vf).end == -(1 as ::core::ffi::c_int) as ogg_int64_t {
		return -(131 as ::core::ffi::c_int);
	}
	end = _get_prev_page_serial(
		vf,
		(*vf).end,
		(*vf).serialnos.offset(2 as ::core::ffi::c_int as isize),
		*(*vf).serialnos.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int,
		&raw mut endserial,
		&raw mut endgran
	);
	if end < 0 as ogg_int64_t {
		return end as ::core::ffi::c_int;
	}
	if _bisect_forward_serialno(
		vf,
		0 as ogg_int64_t,
		dataoffset,
		end,
		endgran,
		endserial,
		(*vf).serialnos.offset(2 as ::core::ffi::c_int as isize),
		*(*vf).serialnos.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int,
		0 as ::core::ffi::c_long
	) < 0 as ::core::ffi::c_int
	{
		return -(128 as ::core::ffi::c_int);
	}
	*(*vf).offsets.offset(0 as ::core::ffi::c_int as isize) = 0 as ogg_int64_t;
	*(*vf).serialnos.offset(0 as ::core::ffi::c_int as isize) = serialno as ::core::ffi::c_long;
	*(*vf).dataoffsets.offset(0 as ::core::ffi::c_int as isize) = dataoffset;
	*(*vf).pcmlengths.offset(0 as ::core::ffi::c_int as isize) = pcmoffset;
	*(*vf).pcmlengths.offset(1 as ::core::ffi::c_int as isize) -= pcmoffset;
	if *(*vf).pcmlengths.offset(1 as ::core::ffi::c_int as isize) < 0 as ogg_int64_t {
		*(*vf).pcmlengths.offset(1 as ::core::ffi::c_int as isize) = 0 as ogg_int64_t;
	}
	return ov_raw_seek(vf, dataoffset);
}
unsafe extern "C" fn _decode_clear(mut vf: *mut OggVorbis_File) {
	vorbis_dsp_clear(&raw mut (*vf).vd);
	vorbis_block_clear(&raw mut (*vf).vb);
	(*vf).ready_state = OPENED;
}
unsafe extern "C" fn _fetch_and_process_packet(
	mut vf: *mut OggVorbis_File,
	mut op_in: *mut ogg_packet,
	mut readp: ::core::ffi::c_int,
	mut spanp: ::core::ffi::c_int
) -> ::core::ffi::c_int {
	let mut og: ogg_page = ogg_page {
		header: ::core::ptr::null_mut::<::core::ffi::c_uchar>(),
		header_len: 0,
		body: ::core::ptr::null_mut::<::core::ffi::c_uchar>(),
		body_len: 0
	};
	loop {
		if (*vf).ready_state == STREAMSET {
			let mut ret: ::core::ffi::c_int = _make_decode_ready(vf);
			if ret < 0 as ::core::ffi::c_int {
				return ret;
			}
		}
		if (*vf).ready_state == INITSET {
			let mut hs: ::core::ffi::c_int = vorbis_synthesis_halfrate_p((*vf).vi);
			loop {
				let mut op: ogg_packet = ogg_packet {
					packet: ::core::ptr::null_mut::<::core::ffi::c_uchar>(),
					bytes: 0,
					b_o_s: 0,
					e_o_s: 0,
					granulepos: 0,
					packetno: 0
				};
				let mut op_ptr: *mut ogg_packet =
					if !op_in.is_null() { op_in } else { &raw mut op };
				let mut result: ::core::ffi::c_int =
					ogg_stream_packetout(&raw mut (*vf).os, op_ptr);
				let mut granulepos: ogg_int64_t = 0;
				op_in = ::core::ptr::null_mut::<ogg_packet>();
				if result == -(1 as ::core::ffi::c_int) {
					return -(3 as ::core::ffi::c_int);
				}
				if !(result > 0 as ::core::ffi::c_int) {
					break;
				}
				granulepos = (*op_ptr).granulepos;
				if vorbis_synthesis(&raw mut (*vf).vb, op_ptr) == 0 {
					let mut oldsamples: ::core::ffi::c_int = vorbis_synthesis_pcmout(
						&raw mut (*vf).vd,
						::core::ptr::null_mut::<*mut *mut ::core::ffi::c_float>()
					);
					if oldsamples != 0 {
						return -(129 as ::core::ffi::c_int);
					}
					vorbis_synthesis_blockin(&raw mut (*vf).vd, &raw mut (*vf).vb);
					(*vf).samptrack += (vorbis_synthesis_pcmout(
						&raw mut (*vf).vd,
						::core::ptr::null_mut::<*mut *mut ::core::ffi::c_float>()
					) << hs) as ::core::ffi::c_double;
					(*vf).bittrack +=
						((*op_ptr).bytes * 8 as ::core::ffi::c_long) as ::core::ffi::c_double;
					if granulepos != -(1 as ::core::ffi::c_int) as ogg_int64_t
						&& (*op_ptr).e_o_s == 0
					{
						let mut link: ::core::ffi::c_int = if (*vf).seekable != 0 {
							(*vf).current_link
						} else {
							0 as ::core::ffi::c_int
						};
						let mut i: ::core::ffi::c_int = 0;
						let mut samples: ::core::ffi::c_int = 0;
						if (*vf).seekable != 0 && link > 0 as ::core::ffi::c_int {
							granulepos -= *(*vf)
								.pcmlengths
								.offset((link * 2 as ::core::ffi::c_int) as isize);
						}
						if granulepos < 0 as ogg_int64_t {
							granulepos = 0 as ogg_int64_t;
						}
						samples = vorbis_synthesis_pcmout(
							&raw mut (*vf).vd,
							::core::ptr::null_mut::<*mut *mut ::core::ffi::c_float>()
						) << hs;
						granulepos -= samples as ogg_int64_t;
						i = 0 as ::core::ffi::c_int;
						while i < link {
							granulepos += *(*vf).pcmlengths.offset(
								(i * 2 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize
							);
							i += 1;
						}
						(*vf).pcm_offset = granulepos;
					}
					return 1 as ::core::ffi::c_int;
				}
			}
		}
		if (*vf).ready_state >= OPENED {
			let mut ret_0: ogg_int64_t = 0;
			loop {
				if readp == 0 {
					return 0 as ::core::ffi::c_int;
				}
				ret_0 = _get_next_page(vf, &raw mut og, -(1 as ::core::ffi::c_int) as ogg_int64_t);
				if ret_0 < 0 as ogg_int64_t {
					return -(2 as ::core::ffi::c_int);
				}
				(*vf).bittrack +=
					(og.header_len * 8 as ::core::ffi::c_long) as ::core::ffi::c_double;
				if !((*vf).ready_state == INITSET) {
					break;
				}
				if !((*vf).current_serialno
					!= ogg_page_serialno(&raw mut og) as ::core::ffi::c_long)
				{
					break;
				}
				if !(ogg_page_bos(&raw mut og) != 0) {
					continue;
				}
				if spanp == 0 {
					return -(2 as ::core::ffi::c_int);
				}
				_decode_clear(vf);
				if (*vf).seekable == 0 {
					vorbis_info_clear((*vf).vi);
					vorbis_comment_clear((*vf).vc);
				}
				break;
			}
		}
		if (*vf).ready_state != INITSET {
			let mut link_0: ::core::ffi::c_int = 0;
			if (*vf).ready_state < STREAMSET {
				if (*vf).seekable != 0 {
					let mut serialno: ::core::ffi::c_long =
						ogg_page_serialno(&raw mut og) as ::core::ffi::c_long;
					link_0 = 0 as ::core::ffi::c_int;
					while link_0 < (*vf).links {
						if *(*vf).serialnos.offset(link_0 as isize) == serialno {
							break;
						}
						link_0 += 1;
					}
					if link_0 == (*vf).links {
						continue;
					}
					(*vf).current_serialno = serialno;
					(*vf).current_link = link_0;
					ogg_stream_reset_serialno(
						&raw mut (*vf).os,
						(*vf).current_serialno as ::core::ffi::c_int
					);
					(*vf).ready_state = STREAMSET;
				} else {
					let mut ret_1: ::core::ffi::c_int = _fetch_headers(
						vf,
						(*vf).vi,
						(*vf).vc,
						::core::ptr::null_mut::<*mut ::core::ffi::c_long>(),
						::core::ptr::null_mut::<::core::ffi::c_int>(),
						&raw mut og
					);
					if ret_1 != 0 {
						return ret_1;
					}
					(*vf).current_serialno = (*vf).os.serialno;
					(*vf).current_link += 1;
					link_0 = 0 as ::core::ffi::c_int;
				}
			}
		}
		ogg_stream_pagein(&raw mut (*vf).os, &raw mut og);
	}
}
unsafe extern "C" fn _fseek64_wrap(
	mut f: *mut FILE,
	mut off: ogg_int64_t,
	mut whence: ::core::ffi::c_int
) -> ::core::ffi::c_int {
	if f.is_null() {
		return -(1 as ::core::ffi::c_int);
	}
	return fseek(f, off as ::core::ffi::c_long, whence);
}
unsafe extern "C" fn _ov_open1(
	mut f: *mut ::core::ffi::c_void,
	mut vf: *mut OggVorbis_File,
	mut initial: *const ::core::ffi::c_char,
	mut ibytes: ::core::ffi::c_long,
	mut callbacks: ov_callbacks
) -> ::core::ffi::c_int {
	let mut offsettest: ::core::ffi::c_int = if !f.is_null() && callbacks.seek_func.is_some() {
		callbacks.seek_func.expect("non-null function pointer")(f, 0 as ogg_int64_t, SEEK_CUR)
	} else {
		-(1 as ::core::ffi::c_int)
	};
	let mut serialno_list: *mut ::core::ffi::c_long =
		::core::ptr::null_mut::<::core::ffi::c_long>();
	let mut serialno_list_size: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
	let mut ret: ::core::ffi::c_int = 0;
	memset(
		vf as *mut ::core::ffi::c_void,
		0 as ::core::ffi::c_int,
		::core::mem::size_of::<OggVorbis_File>() as size_t
	);
	(*vf).datasource = f;
	(*vf).callbacks = callbacks;
	ogg_sync_init(&raw mut (*vf).oy);
	if !initial.is_null() {
		let mut buffer: *mut ::core::ffi::c_char = ogg_sync_buffer(&raw mut (*vf).oy, ibytes);
		memcpy(
			buffer as *mut ::core::ffi::c_void,
			initial as *const ::core::ffi::c_void,
			ibytes as size_t
		);
		ogg_sync_wrote(&raw mut (*vf).oy, ibytes);
	}
	if offsettest != -(1 as ::core::ffi::c_int) {
		(*vf).seekable = 1 as ::core::ffi::c_int;
	}
	(*vf).links = 1 as ::core::ffi::c_int;
	(*vf).vi = calloc(
		(*vf).links as size_t,
		::core::mem::size_of::<vorbis_info>() as size_t
	) as *mut vorbis_info;
	(*vf).vc = calloc(
		(*vf).links as size_t,
		::core::mem::size_of::<vorbis_comment>() as size_t
	) as *mut vorbis_comment;
	ogg_stream_init(&raw mut (*vf).os, -(1 as ::core::ffi::c_int));
	ret = _fetch_headers(
		vf,
		(*vf).vi,
		(*vf).vc,
		&raw mut serialno_list,
		&raw mut serialno_list_size,
		::core::ptr::null_mut::<ogg_page>()
	);
	if ret < 0 as ::core::ffi::c_int {
		(*vf).datasource = NULL;
		ov_clear(vf);
	} else {
		(*vf).serialnos = calloc(
			(serialno_list_size + 2 as ::core::ffi::c_int) as size_t,
			::core::mem::size_of::<::core::ffi::c_long>() as size_t
		) as *mut ::core::ffi::c_long;
		(*vf).current_serialno = (*vf).os.serialno;
		*(*vf).serialnos.offset(0 as ::core::ffi::c_int as isize) = (*vf).current_serialno;
		*(*vf).serialnos.offset(1 as ::core::ffi::c_int as isize) =
			serialno_list_size as ::core::ffi::c_long;
		memcpy(
			(*vf).serialnos.offset(2 as ::core::ffi::c_int as isize) as *mut ::core::ffi::c_void,
			serialno_list as *const ::core::ffi::c_void,
			(serialno_list_size as size_t)
				.wrapping_mul(::core::mem::size_of::<::core::ffi::c_long>() as size_t)
		);
		(*vf).offsets = calloc(1 as size_t, ::core::mem::size_of::<ogg_int64_t>() as size_t)
			as *mut ogg_int64_t;
		(*vf).dataoffsets = calloc(1 as size_t, ::core::mem::size_of::<ogg_int64_t>() as size_t)
			as *mut ogg_int64_t;
		*(*vf).offsets.offset(0 as ::core::ffi::c_int as isize) = 0 as ogg_int64_t;
		*(*vf).dataoffsets.offset(0 as ::core::ffi::c_int as isize) = (*vf).offset;
		(*vf).ready_state = PARTOPEN;
	}
	if !serialno_list.is_null() {
		free(serialno_list as *mut ::core::ffi::c_void);
	}
	return ret;
}
unsafe extern "C" fn _ov_open2(mut vf: *mut OggVorbis_File) -> ::core::ffi::c_int {
	if (*vf).ready_state != PARTOPEN {
		return OV_EINVAL;
	}
	(*vf).ready_state = OPENED;
	if (*vf).seekable != 0 {
		let mut ret: ::core::ffi::c_int = _open_seekable2(vf);
		if ret != 0 {
			(*vf).datasource = NULL;
			ov_clear(vf);
		}
		return ret;
	} else {
		(*vf).ready_state = STREAMSET;
	}
	return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ov_clear(mut vf: *mut OggVorbis_File) -> ::core::ffi::c_int {
	if !vf.is_null() {
		vorbis_block_clear(&raw mut (*vf).vb);
		vorbis_dsp_clear(&raw mut (*vf).vd);
		ogg_stream_clear(&raw mut (*vf).os);
		if !(*vf).vi.is_null() && (*vf).links != 0 {
			let mut i: ::core::ffi::c_int = 0;
			i = 0 as ::core::ffi::c_int;
			while i < (*vf).links {
				vorbis_info_clear((*vf).vi.offset(i as isize));
				vorbis_comment_clear((*vf).vc.offset(i as isize));
				i += 1;
			}
			free((*vf).vi as *mut ::core::ffi::c_void);
			free((*vf).vc as *mut ::core::ffi::c_void);
		}
		if !(*vf).dataoffsets.is_null() {
			free((*vf).dataoffsets as *mut ::core::ffi::c_void);
		}
		if !(*vf).pcmlengths.is_null() {
			free((*vf).pcmlengths as *mut ::core::ffi::c_void);
		}
		if !(*vf).serialnos.is_null() {
			free((*vf).serialnos as *mut ::core::ffi::c_void);
		}
		if !(*vf).offsets.is_null() {
			free((*vf).offsets as *mut ::core::ffi::c_void);
		}
		ogg_sync_clear(&raw mut (*vf).oy);
		if !(*vf).datasource.is_null() && (*vf).callbacks.close_func.is_some() {
			(*vf)
				.callbacks
				.close_func
				.expect("non-null function pointer")((*vf).datasource);
		}
		memset(
			vf as *mut ::core::ffi::c_void,
			0 as ::core::ffi::c_int,
			::core::mem::size_of::<OggVorbis_File>() as size_t
		);
	}
	return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ov_open_callbacks(
	mut f: *mut ::core::ffi::c_void,
	mut vf: *mut OggVorbis_File,
	mut initial: *const ::core::ffi::c_char,
	mut ibytes: ::core::ffi::c_long,
	mut callbacks: ov_callbacks
) -> ::core::ffi::c_int {
	let mut ret: ::core::ffi::c_int = _ov_open1(f, vf, initial, ibytes, callbacks);
	if ret != 0 {
		return ret;
	}
	return _ov_open2(vf);
}
#[no_mangle]
pub unsafe extern "C" fn ov_open(
	mut f: *mut FILE,
	mut vf: *mut OggVorbis_File,
	mut initial: *const ::core::ffi::c_char,
	mut ibytes: ::core::ffi::c_long
) -> ::core::ffi::c_int {
	let mut callbacks: ov_callbacks = ov_callbacks {
		read_func: ::core::mem::transmute::<
			Option<
				unsafe extern "C" fn(
					*mut ::core::ffi::c_void,
					size_t,
					size_t,
					*mut FILE
				) -> ::core::ffi::c_ulong
			>,
			Option<
				unsafe extern "C" fn(
					*mut ::core::ffi::c_void,
					size_t,
					size_t,
					*mut ::core::ffi::c_void
				) -> size_t
			>
		>(Some(
			fread
				as unsafe extern "C" fn(
					*mut ::core::ffi::c_void,
					size_t,
					size_t,
					*mut FILE
				) -> ::core::ffi::c_ulong
		)),
		seek_func: ::core::mem::transmute::<
			Option<
				unsafe extern "C" fn(
					*mut FILE,
					ogg_int64_t,
					::core::ffi::c_int
				) -> ::core::ffi::c_int
			>,
			Option<
				unsafe extern "C" fn(
					*mut ::core::ffi::c_void,
					ogg_int64_t,
					::core::ffi::c_int
				) -> ::core::ffi::c_int
			>
		>(Some(
			_fseek64_wrap
				as unsafe extern "C" fn(
					*mut FILE,
					ogg_int64_t,
					::core::ffi::c_int
				) -> ::core::ffi::c_int
		)),
		close_func: ::core::mem::transmute::<
			Option<unsafe extern "C" fn(*mut FILE) -> ::core::ffi::c_int>,
			Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ::core::ffi::c_int>
		>(Some(
			fclose as unsafe extern "C" fn(*mut FILE) -> ::core::ffi::c_int
		)),
		tell_func: ::core::mem::transmute::<
			Option<unsafe extern "C" fn(*mut FILE) -> ::core::ffi::c_long>,
			Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ::core::ffi::c_long>
		>(Some(
			ftell as unsafe extern "C" fn(*mut FILE) -> ::core::ffi::c_long
		))
	};
	return ov_open_callbacks(
		f as *mut ::core::ffi::c_void,
		vf,
		initial,
		ibytes,
		callbacks
	);
}
#[no_mangle]
pub unsafe extern "C" fn ov_fopen(
	mut path: *const ::core::ffi::c_char,
	mut vf: *mut OggVorbis_File
) -> ::core::ffi::c_int {
	let mut ret: ::core::ffi::c_int = 0;
	let mut f: *mut FILE =
		fopen(path, b"rb\0" as *const u8 as *const ::core::ffi::c_char) as *mut FILE;
	if f.is_null() {
		return -(1 as ::core::ffi::c_int);
	}
	ret = ov_open(
		f,
		vf,
		::core::ptr::null::<::core::ffi::c_char>(),
		0 as ::core::ffi::c_long
	);
	if ret != 0 {
		fclose(f);
	}
	return ret;
}
#[no_mangle]
pub unsafe extern "C" fn ov_halfrate(
	mut vf: *mut OggVorbis_File,
	mut flag: ::core::ffi::c_int
) -> ::core::ffi::c_int {
	let mut i: ::core::ffi::c_int = 0;
	if (*vf).vi.is_null() {
		return OV_EINVAL;
	}
	if (*vf).ready_state > STREAMSET {
		vorbis_dsp_clear(&raw mut (*vf).vd);
		vorbis_block_clear(&raw mut (*vf).vb);
		(*vf).ready_state = STREAMSET;
		if (*vf).pcm_offset >= 0 as ogg_int64_t {
			let mut pos: ogg_int64_t = (*vf).pcm_offset;
			(*vf).pcm_offset = -(1 as ::core::ffi::c_int) as ogg_int64_t;
			ov_pcm_seek(vf, pos);
		}
	}
	i = 0 as ::core::ffi::c_int;
	while i < (*vf).links {
		if vorbis_synthesis_halfrate((*vf).vi.offset(i as isize), flag) != 0 {
			if flag != 0 {
				ov_halfrate(vf, 0 as ::core::ffi::c_int);
			}
			return OV_EINVAL;
		}
		i += 1;
	}
	return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ov_halfrate_p(mut vf: *mut OggVorbis_File) -> ::core::ffi::c_int {
	if (*vf).vi.is_null() {
		return OV_EINVAL;
	}
	return vorbis_synthesis_halfrate_p((*vf).vi);
}
#[no_mangle]
pub unsafe extern "C" fn ov_test_callbacks(
	mut f: *mut ::core::ffi::c_void,
	mut vf: *mut OggVorbis_File,
	mut initial: *const ::core::ffi::c_char,
	mut ibytes: ::core::ffi::c_long,
	mut callbacks: ov_callbacks
) -> ::core::ffi::c_int {
	return _ov_open1(f, vf, initial, ibytes, callbacks);
}
#[no_mangle]
pub unsafe extern "C" fn ov_test(
	mut f: *mut FILE,
	mut vf: *mut OggVorbis_File,
	mut initial: *const ::core::ffi::c_char,
	mut ibytes: ::core::ffi::c_long
) -> ::core::ffi::c_int {
	let mut callbacks: ov_callbacks = ov_callbacks {
		read_func: ::core::mem::transmute::<
			Option<
				unsafe extern "C" fn(
					*mut ::core::ffi::c_void,
					size_t,
					size_t,
					*mut FILE
				) -> ::core::ffi::c_ulong
			>,
			Option<
				unsafe extern "C" fn(
					*mut ::core::ffi::c_void,
					size_t,
					size_t,
					*mut ::core::ffi::c_void
				) -> size_t
			>
		>(Some(
			fread
				as unsafe extern "C" fn(
					*mut ::core::ffi::c_void,
					size_t,
					size_t,
					*mut FILE
				) -> ::core::ffi::c_ulong
		)),
		seek_func: ::core::mem::transmute::<
			Option<
				unsafe extern "C" fn(
					*mut FILE,
					ogg_int64_t,
					::core::ffi::c_int
				) -> ::core::ffi::c_int
			>,
			Option<
				unsafe extern "C" fn(
					*mut ::core::ffi::c_void,
					ogg_int64_t,
					::core::ffi::c_int
				) -> ::core::ffi::c_int
			>
		>(Some(
			_fseek64_wrap
				as unsafe extern "C" fn(
					*mut FILE,
					ogg_int64_t,
					::core::ffi::c_int
				) -> ::core::ffi::c_int
		)),
		close_func: ::core::mem::transmute::<
			Option<unsafe extern "C" fn(*mut FILE) -> ::core::ffi::c_int>,
			Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ::core::ffi::c_int>
		>(Some(
			fclose as unsafe extern "C" fn(*mut FILE) -> ::core::ffi::c_int
		)),
		tell_func: ::core::mem::transmute::<
			Option<unsafe extern "C" fn(*mut FILE) -> ::core::ffi::c_long>,
			Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ::core::ffi::c_long>
		>(Some(
			ftell as unsafe extern "C" fn(*mut FILE) -> ::core::ffi::c_long
		))
	};
	return ov_test_callbacks(
		f as *mut ::core::ffi::c_void,
		vf,
		initial,
		ibytes,
		callbacks
	);
}
#[no_mangle]
pub unsafe extern "C" fn ov_test_open(mut vf: *mut OggVorbis_File) -> ::core::ffi::c_int {
	if (*vf).ready_state != PARTOPEN {
		return -(131 as ::core::ffi::c_int);
	}
	return _ov_open2(vf);
}
#[no_mangle]
pub unsafe extern "C" fn ov_streams(mut vf: *mut OggVorbis_File) -> ::core::ffi::c_long {
	return (*vf).links as ::core::ffi::c_long;
}
#[no_mangle]
pub unsafe extern "C" fn ov_seekable(mut vf: *mut OggVorbis_File) -> ::core::ffi::c_long {
	return (*vf).seekable as ::core::ffi::c_long;
}
#[no_mangle]
pub unsafe extern "C" fn ov_bitrate(
	mut vf: *mut OggVorbis_File,
	mut i: ::core::ffi::c_int
) -> ::core::ffi::c_long {
	if (*vf).ready_state < OPENED {
		return -(131 as ::core::ffi::c_int) as ::core::ffi::c_long;
	}
	if i >= (*vf).links {
		return -(131 as ::core::ffi::c_int) as ::core::ffi::c_long;
	}
	if (*vf).seekable == 0 && i != 0 as ::core::ffi::c_int {
		return ov_bitrate(vf, 0 as ::core::ffi::c_int);
	}
	if i < 0 as ::core::ffi::c_int {
		let mut bits: ogg_int64_t = 0 as ogg_int64_t;
		let mut i_0: ::core::ffi::c_int = 0;
		let mut br: ::core::ffi::c_float = 0.;
		i_0 = 0 as ::core::ffi::c_int;
		while i_0 < (*vf).links {
			bits += (*(*vf)
				.offsets
				.offset((i_0 + 1 as ::core::ffi::c_int) as isize)
				- *(*vf).dataoffsets.offset(i_0 as isize))
				* 8 as ogg_int64_t;
			i_0 += 1;
		}
		br = (bits as ::core::ffi::c_double / ov_time_total(vf, -(1 as ::core::ffi::c_int)))
			as ::core::ffi::c_float;
		return rint(br as ::core::ffi::c_double) as ::core::ffi::c_long;
	} else if (*vf).seekable != 0 {
		return rint(
			((*(*vf).offsets.offset((i + 1 as ::core::ffi::c_int) as isize)
				- *(*vf).dataoffsets.offset(i as isize))
				* 8 as ogg_int64_t) as ::core::ffi::c_double
				/ ov_time_total(vf, i)
		) as ::core::ffi::c_long;
	} else if (*(*vf).vi.offset(i as isize)).bitrate_nominal > 0 as ::core::ffi::c_long {
		return (*(*vf).vi.offset(i as isize)).bitrate_nominal;
	} else {
		if (*(*vf).vi.offset(i as isize)).bitrate_upper > 0 as ::core::ffi::c_long {
			if (*(*vf).vi.offset(i as isize)).bitrate_lower > 0 as ::core::ffi::c_long {
				return ((*(*vf).vi.offset(i as isize)).bitrate_upper
					+ (*(*vf).vi.offset(i as isize)).bitrate_lower)
					/ 2 as ::core::ffi::c_long;
			} else {
				return (*(*vf).vi.offset(i as isize)).bitrate_upper;
			}
		}
		return -(1 as ::core::ffi::c_int) as ::core::ffi::c_long;
	};
}
#[no_mangle]
pub unsafe extern "C" fn ov_bitrate_instant(mut vf: *mut OggVorbis_File) -> ::core::ffi::c_long {
	let mut link: ::core::ffi::c_int = if (*vf).seekable != 0 {
		(*vf).current_link
	} else {
		0 as ::core::ffi::c_int
	};
	let mut ret: ::core::ffi::c_long = 0;
	if (*vf).ready_state < OPENED {
		return -(131 as ::core::ffi::c_int) as ::core::ffi::c_long;
	}
	if (*vf).samptrack == 0 as ::core::ffi::c_int as ::core::ffi::c_double {
		return -(1 as ::core::ffi::c_int) as ::core::ffi::c_long;
	}
	ret = ((*vf).bittrack / (*vf).samptrack
		* (*(*vf).vi.offset(link as isize)).rate as ::core::ffi::c_double
		+ 0.5f64) as ::core::ffi::c_long;
	(*vf).bittrack = 0.0f64;
	(*vf).samptrack = 0.0f64;
	return ret;
}
#[no_mangle]
pub unsafe extern "C" fn ov_serialnumber(
	mut vf: *mut OggVorbis_File,
	mut i: ::core::ffi::c_int
) -> ::core::ffi::c_long {
	if i >= (*vf).links {
		return ov_serialnumber(vf, (*vf).links - 1 as ::core::ffi::c_int);
	}
	if (*vf).seekable == 0 && i >= 0 as ::core::ffi::c_int {
		return ov_serialnumber(vf, -(1 as ::core::ffi::c_int));
	}
	if i < 0 as ::core::ffi::c_int {
		return (*vf).current_serialno;
	} else {
		return *(*vf).serialnos.offset(i as isize);
	};
}
#[no_mangle]
pub unsafe extern "C" fn ov_raw_total(
	mut vf: *mut OggVorbis_File,
	mut i: ::core::ffi::c_int
) -> ogg_int64_t {
	if (*vf).ready_state < OPENED {
		return -(131 as ::core::ffi::c_int) as ogg_int64_t;
	}
	if (*vf).seekable == 0 || i >= (*vf).links {
		return -(131 as ::core::ffi::c_int) as ogg_int64_t;
	}
	if i < 0 as ::core::ffi::c_int {
		let mut acc: ogg_int64_t = 0 as ogg_int64_t;
		let mut i_0: ::core::ffi::c_int = 0;
		i_0 = 0 as ::core::ffi::c_int;
		while i_0 < (*vf).links {
			acc += ov_raw_total(vf, i_0);
			i_0 += 1;
		}
		return acc;
	} else {
		return *(*vf).offsets.offset((i + 1 as ::core::ffi::c_int) as isize)
			- *(*vf).offsets.offset(i as isize);
	};
}
#[no_mangle]
pub unsafe extern "C" fn ov_pcm_total(
	mut vf: *mut OggVorbis_File,
	mut i: ::core::ffi::c_int
) -> ogg_int64_t {
	if (*vf).ready_state < OPENED {
		return -(131 as ::core::ffi::c_int) as ogg_int64_t;
	}
	if (*vf).seekable == 0 || i >= (*vf).links {
		return -(131 as ::core::ffi::c_int) as ogg_int64_t;
	}
	if i < 0 as ::core::ffi::c_int {
		let mut acc: ogg_int64_t = 0 as ogg_int64_t;
		let mut i_0: ::core::ffi::c_int = 0;
		i_0 = 0 as ::core::ffi::c_int;
		while i_0 < (*vf).links {
			acc += ov_pcm_total(vf, i_0);
			i_0 += 1;
		}
		return acc;
	} else {
		return *(*vf)
			.pcmlengths
			.offset((i * 2 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize);
	};
}
#[no_mangle]
pub unsafe extern "C" fn ov_time_total(
	mut vf: *mut OggVorbis_File,
	mut i: ::core::ffi::c_int
) -> ::core::ffi::c_double {
	if (*vf).ready_state < OPENED {
		return -(131 as ::core::ffi::c_int) as ::core::ffi::c_double;
	}
	if (*vf).seekable == 0 || i >= (*vf).links {
		return -(131 as ::core::ffi::c_int) as ::core::ffi::c_double;
	}
	if i < 0 as ::core::ffi::c_int {
		let mut acc: ::core::ffi::c_double = 0 as ::core::ffi::c_int as ::core::ffi::c_double;
		let mut i_0: ::core::ffi::c_int = 0;
		i_0 = 0 as ::core::ffi::c_int;
		while i_0 < (*vf).links {
			acc += ov_time_total(vf, i_0);
			i_0 += 1;
		}
		return acc;
	} else {
		return *(*vf)
			.pcmlengths
			.offset((i * 2 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize)
			as ::core::ffi::c_double
			/ (*(*vf).vi.offset(i as isize)).rate as ::core::ffi::c_double;
	};
}
#[no_mangle]
pub unsafe extern "C" fn ov_raw_seek(
	mut vf: *mut OggVorbis_File,
	mut pos: ogg_int64_t
) -> ::core::ffi::c_int {
	let mut work_os: ogg_stream_state = ogg_stream_state {
		body_data: ::core::ptr::null_mut::<::core::ffi::c_uchar>(),
		body_storage: 0,
		body_fill: 0,
		body_returned: 0,
		lacing_vals: ::core::ptr::null_mut::<::core::ffi::c_int>(),
		granule_vals: ::core::ptr::null_mut::<ogg_int64_t>(),
		lacing_storage: 0,
		lacing_fill: 0,
		lacing_packet: 0,
		lacing_returned: 0,
		header: [0; 282],
		header_fill: 0,
		e_o_s: 0,
		b_o_s: 0,
		serialno: 0,
		pageno: 0,
		packetno: 0,
		granulepos: 0
	};
	if (*vf).ready_state < OPENED {
		return -(131 as ::core::ffi::c_int);
	}
	if (*vf).seekable == 0 {
		return -(138 as ::core::ffi::c_int);
	}
	if pos < 0 as ogg_int64_t || pos > (*vf).end {
		return -(131 as ::core::ffi::c_int);
	}
	if (*vf).ready_state >= STREAMSET {
		if pos < *(*vf).offsets.offset((*vf).current_link as isize)
			|| pos
				>= *(*vf)
					.offsets
					.offset(((*vf).current_link + 1 as ::core::ffi::c_int) as isize)
		{
			_decode_clear(vf);
		}
	}
	(*vf).pcm_offset = -(1 as ::core::ffi::c_int) as ogg_int64_t;
	ogg_stream_reset_serialno(
		&raw mut (*vf).os,
		(*vf).current_serialno as ::core::ffi::c_int
	);
	vorbis_synthesis_restart(&raw mut (*vf).vd);
	if _seek_helper(vf, pos) != 0 {
		(*vf).pcm_offset = -(1 as ::core::ffi::c_int) as ogg_int64_t;
		_decode_clear(vf);
		return OV_EBADLINK;
	}
	let mut og: ogg_page = ogg_page {
		header: ::core::ptr::null_mut::<::core::ffi::c_uchar>(),
		header_len: 0,
		body: ::core::ptr::null_mut::<::core::ffi::c_uchar>(),
		body_len: 0
	};
	let mut op: ogg_packet = ogg_packet {
		packet: ::core::ptr::null_mut::<::core::ffi::c_uchar>(),
		bytes: 0,
		b_o_s: 0,
		e_o_s: 0,
		granulepos: 0,
		packetno: 0
	};
	let mut lastblock: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
	let mut accblock: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
	let mut thisblock: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
	let mut lastflag: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
	let mut firstflag: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
	let mut pagepos: ogg_int64_t = -(1 as ::core::ffi::c_int) as ogg_int64_t;
	ogg_stream_init(
		&raw mut work_os,
		(*vf).current_serialno as ::core::ffi::c_int
	);
	ogg_stream_reset(&raw mut work_os);
	loop {
		if (*vf).ready_state >= STREAMSET {
			let mut result: ::core::ffi::c_int =
				ogg_stream_packetout(&raw mut work_os, &raw mut op);
			if result > 0 as ::core::ffi::c_int {
				if !(*(*vf).vi.offset((*vf).current_link as isize))
					.codec_setup
					.is_null()
				{
					thisblock = vorbis_packet_blocksize(
						(*vf).vi.offset((*vf).current_link as isize),
						&raw mut op
					) as ::core::ffi::c_int;
					if thisblock < 0 as ::core::ffi::c_int {
						ogg_stream_packetout(
							&raw mut (*vf).os,
							::core::ptr::null_mut::<ogg_packet>()
						);
						thisblock = 0 as ::core::ffi::c_int;
					} else if lastflag != 0 && firstflag == 0 {
						ogg_stream_packetout(
							&raw mut (*vf).os,
							::core::ptr::null_mut::<ogg_packet>()
						);
					} else if lastblock != 0 {
						accblock += lastblock + thisblock >> 2 as ::core::ffi::c_int;
					}
					if op.granulepos != -(1 as ::core::ffi::c_int) as ogg_int64_t {
						let mut i: ::core::ffi::c_int = 0;
						let mut link: ::core::ffi::c_int = (*vf).current_link;
						let mut granulepos: ogg_int64_t = op.granulepos
							- *(*vf)
								.pcmlengths
								.offset((link * 2 as ::core::ffi::c_int) as isize);
						if granulepos < 0 as ogg_int64_t {
							granulepos = 0 as ogg_int64_t;
						}
						i = 0 as ::core::ffi::c_int;
						while i < link {
							granulepos += *(*vf).pcmlengths.offset(
								(i * 2 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize
							);
							i += 1;
						}
						(*vf).pcm_offset = granulepos - accblock as ogg_int64_t;
						if (*vf).pcm_offset < 0 as ogg_int64_t {
							(*vf).pcm_offset = 0 as ogg_int64_t;
						}
						break;
					} else {
						lastblock = thisblock;
						continue;
					}
				} else {
					ogg_stream_packetout(&raw mut (*vf).os, ::core::ptr::null_mut::<ogg_packet>());
				}
			}
		}
		if lastblock == 0 {
			pagepos = _get_next_page(vf, &raw mut og, -(1 as ::core::ffi::c_int) as ogg_int64_t);
			if pagepos < 0 as ogg_int64_t {
				(*vf).pcm_offset = ov_pcm_total(vf, -(1 as ::core::ffi::c_int));
				break;
			} else {
				if (*vf).ready_state >= STREAMSET {
					if (*vf).current_serialno
						!= ogg_page_serialno(&raw mut og) as ::core::ffi::c_long
					{
						if ogg_page_bos(&raw mut og) != 0 {
							_decode_clear(vf);
							ogg_stream_clear(&raw mut work_os);
						}
					}
				}
				if (*vf).ready_state < STREAMSET {
					let mut link_0: ::core::ffi::c_int = 0;
					let mut serialno: ::core::ffi::c_long =
						ogg_page_serialno(&raw mut og) as ::core::ffi::c_long;
					link_0 = 0 as ::core::ffi::c_int;
					while link_0 < (*vf).links {
						if *(*vf).serialnos.offset(link_0 as isize) == serialno {
							break;
						}
						link_0 += 1;
					}
					if link_0 == (*vf).links {
						continue;
					}
					(*vf).current_link = link_0;
					(*vf).current_serialno = serialno;
					ogg_stream_reset_serialno(&raw mut (*vf).os, serialno as ::core::ffi::c_int);
					ogg_stream_reset_serialno(&raw mut work_os, serialno as ::core::ffi::c_int);
					(*vf).ready_state = STREAMSET;
					firstflag = (pagepos <= *(*vf).dataoffsets.offset(link_0 as isize))
						as ::core::ffi::c_int;
				}
				ogg_stream_pagein(&raw mut (*vf).os, &raw mut og);
				ogg_stream_pagein(&raw mut work_os, &raw mut og);
				lastflag = ogg_page_eos(&raw mut og);
			}
		} else {
			(*vf).pcm_offset = -(1 as ::core::ffi::c_int) as ogg_int64_t;
			break;
		}
	}
	ogg_stream_clear(&raw mut work_os);
	(*vf).bittrack = 0.0f64;
	(*vf).samptrack = 0.0f64;
	return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ov_pcm_seek_page(
	mut vf: *mut OggVorbis_File,
	mut pos: ogg_int64_t
) -> ::core::ffi::c_int {
	let mut current_block: u64;
	let mut link: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
	let mut result: ogg_int64_t = 0 as ogg_int64_t;
	let mut total: ogg_int64_t = ov_pcm_total(vf, -(1 as ::core::ffi::c_int));
	if (*vf).ready_state < OPENED {
		return -(131 as ::core::ffi::c_int);
	}
	if (*vf).seekable == 0 {
		return -(138 as ::core::ffi::c_int);
	}
	if pos < 0 as ogg_int64_t || pos > total {
		return -(131 as ::core::ffi::c_int);
	}
	link = (*vf).links - 1 as ::core::ffi::c_int;
	while link >= 0 as ::core::ffi::c_int {
		total -= *(*vf)
			.pcmlengths
			.offset((link * 2 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize);
		if pos >= total {
			break;
		}
		link -= 1;
	}
	let mut end: ogg_int64_t = *(*vf)
		.offsets
		.offset((link + 1 as ::core::ffi::c_int) as isize);
	let mut begin: ogg_int64_t = *(*vf).dataoffsets.offset(link as isize);
	let mut begintime: ogg_int64_t = *(*vf)
		.pcmlengths
		.offset((link * 2 as ::core::ffi::c_int) as isize);
	let mut endtime: ogg_int64_t = *(*vf)
		.pcmlengths
		.offset((link * 2 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize)
		+ begintime;
	let mut target: ogg_int64_t = pos - total + begintime;
	let mut best: ogg_int64_t = -(1 as ::core::ffi::c_int) as ogg_int64_t;
	let mut got_page: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
	let mut og: ogg_page = ogg_page {
		header: ::core::ptr::null_mut::<::core::ffi::c_uchar>(),
		header_len: 0,
		body: ::core::ptr::null_mut::<::core::ffi::c_uchar>(),
		body_len: 0
	};
	if begin == end {
		result = _seek_helper(vf, begin) as ogg_int64_t;
		if result != 0 {
			current_block = 11506424456341058227;
		} else {
			result = _get_next_page(vf, &raw mut og, 1 as ogg_int64_t);
			if result < 0 as ogg_int64_t {
				current_block = 11506424456341058227;
			} else {
				got_page = 1 as ::core::ffi::c_int;
				current_block = 5948590327928692120;
			}
		}
	} else {
		current_block = 5948590327928692120;
	}
	's_93: loop {
		match current_block {
			11506424456341058227 => {
				(*vf).pcm_offset = -(1 as ::core::ffi::c_int) as ogg_int64_t;
				_decode_clear(vf);
				return result as ::core::ffi::c_int;
			}
			_ => {
				if begin < end {
					let mut bisect: ogg_int64_t = 0;
					if end - begin < CHUNKSIZE as ogg_int64_t {
						bisect = begin;
					} else {
						bisect = begin
							+ ((target - begintime) as ::core::ffi::c_double
								* (end - begin) as ::core::ffi::c_double
								/ (endtime - begintime) as ::core::ffi::c_double)
								as ogg_int64_t - CHUNKSIZE as ogg_int64_t;
						if bisect < begin + CHUNKSIZE as ogg_int64_t {
							bisect = begin;
						}
					}
					result = _seek_helper(vf, bisect) as ogg_int64_t;
					if result != 0 {
						current_block = 11506424456341058227;
						continue;
					}
					loop {
						if !(begin < end) {
							current_block = 5948590327928692120;
							break;
						}
						result = _get_next_page(vf, &raw mut og, end - (*vf).offset);
						if result == OV_EREAD as ogg_int64_t {
							current_block = 11506424456341058227;
							break;
						}
						if result < 0 as ogg_int64_t {
							if bisect <= begin + 1 as ogg_int64_t {
								end = begin;
							} else {
								if bisect == 0 as ogg_int64_t {
									current_block = 11506424456341058227;
									break;
								}
								bisect -= CHUNKSIZE as ogg_int64_t;
								if bisect <= begin {
									bisect = begin + 1 as ogg_int64_t;
								}
								result = _seek_helper(vf, bisect) as ogg_int64_t;
								if result != 0 {
									current_block = 11506424456341058227;
									break;
								}
							}
						} else {
							let mut granulepos: ogg_int64_t = 0;
							got_page = 1 as ::core::ffi::c_int;
							if ogg_page_serialno(&raw mut og) as ::core::ffi::c_long
								!= *(*vf).serialnos.offset(link as isize)
							{
								continue;
							}
							granulepos = ogg_page_granulepos(&raw mut og);
							if granulepos == -(1 as ::core::ffi::c_int) as ogg_int64_t {
								continue;
							}
							if granulepos < target {
								best = result;
								begin = (*vf).offset;
								begintime = granulepos;
								if target - begintime > 44100 as ogg_int64_t {
									current_block = 5948590327928692120;
									break;
								}
								bisect = begin;
							} else if bisect <= begin + 1 as ogg_int64_t {
								end = begin;
							} else if end == (*vf).offset {
								end = result;
								bisect -= CHUNKSIZE as ogg_int64_t;
								if bisect <= begin {
									bisect = begin + 1 as ogg_int64_t;
								}
								result = _seek_helper(vf, bisect) as ogg_int64_t;
								if result != 0 {
									current_block = 11506424456341058227;
									break;
								}
							} else {
								end = bisect;
								endtime = granulepos;
								current_block = 5948590327928692120;
								break;
							}
						}
					}
				} else {
					if best == -(1 as ::core::ffi::c_int) as ogg_int64_t {
						if !(got_page != 0
							&& begin == *(*vf).dataoffsets.offset(link as isize)
							&& ogg_page_serialno(&raw mut og) as ::core::ffi::c_long
								== *(*vf).serialnos.offset(link as isize))
						{
							current_block = 11506424456341058227;
							continue;
						}
						(*vf).pcm_offset = total;
						if link != (*vf).current_link {
							_decode_clear(vf);
							(*vf).current_link = link;
							(*vf).current_serialno = *(*vf).serialnos.offset(link as isize);
							(*vf).ready_state = STREAMSET;
						} else {
							vorbis_synthesis_restart(&raw mut (*vf).vd);
						}
						ogg_stream_reset_serialno(
							&raw mut (*vf).os,
							(*vf).current_serialno as ::core::ffi::c_int
						);
						ogg_stream_pagein(&raw mut (*vf).os, &raw mut og);
					} else {
						let mut og_0: ogg_page = ogg_page {
							header: ::core::ptr::null_mut::<::core::ffi::c_uchar>(),
							header_len: 0,
							body: ::core::ptr::null_mut::<::core::ffi::c_uchar>(),
							body_len: 0
						};
						let mut op: ogg_packet = ogg_packet {
							packet: ::core::ptr::null_mut::<::core::ffi::c_uchar>(),
							bytes: 0,
							b_o_s: 0,
							e_o_s: 0,
							granulepos: 0,
							packetno: 0
						};
						result = _seek_helper(vf, best) as ogg_int64_t;
						(*vf).pcm_offset = -(1 as ::core::ffi::c_int) as ogg_int64_t;
						if result != 0 {
							current_block = 11506424456341058227;
							continue;
						}
						result = _get_next_page(
							vf,
							&raw mut og_0,
							-(1 as ::core::ffi::c_int) as ogg_int64_t
						);
						if result < 0 as ogg_int64_t {
							current_block = 11506424456341058227;
							continue;
						}
						if link != (*vf).current_link {
							_decode_clear(vf);
							(*vf).current_link = link;
							(*vf).current_serialno = *(*vf).serialnos.offset(link as isize);
							(*vf).ready_state = STREAMSET;
						} else {
							vorbis_synthesis_restart(&raw mut (*vf).vd);
						}
						ogg_stream_reset_serialno(
							&raw mut (*vf).os,
							(*vf).current_serialno as ::core::ffi::c_int
						);
						ogg_stream_pagein(&raw mut (*vf).os, &raw mut og_0);
						loop {
							result = ogg_stream_packetpeek(&raw mut (*vf).os, &raw mut op)
								as ogg_int64_t;
							if result == 0 as ogg_int64_t {
								result = best;
								while result > *(*vf).dataoffsets.offset(link as isize) {
									result = _get_prev_page(vf, result, &raw mut og_0);
									if result < 0 as ogg_int64_t {
										current_block = 11506424456341058227;
										continue 's_93;
									}
									if ogg_page_serialno(&raw mut og_0) as ::core::ffi::c_long
										== (*vf).current_serialno && (ogg_page_granulepos(
										&raw mut og_0
									) > -(1 as ::core::ffi::c_int)
										as ogg_int64_t
										|| ogg_page_continued(&raw mut og_0) == 0)
									{
										return ov_raw_seek(vf, result);
									}
								}
							}
							if result < 0 as ogg_int64_t {
								result = OV_EBADPACKET as ogg_int64_t;
								current_block = 11506424456341058227;
								continue 's_93;
							} else if op.granulepos != -(1 as ::core::ffi::c_int) as ogg_int64_t {
								(*vf).pcm_offset = op.granulepos
									- *(*vf).pcmlengths.offset(
										((*vf).current_link * 2 as ::core::ffi::c_int) as isize
									);
								if (*vf).pcm_offset < 0 as ogg_int64_t {
									(*vf).pcm_offset = 0 as ogg_int64_t;
								}
								(*vf).pcm_offset += total;
								break;
							} else {
								result = ogg_stream_packetout(
									&raw mut (*vf).os,
									::core::ptr::null_mut::<ogg_packet>()
								) as ogg_int64_t;
							}
						}
					}
					if (*vf).pcm_offset > pos || pos > ov_pcm_total(vf, -(1 as ::core::ffi::c_int))
					{
						result = OV_EFAULT as ogg_int64_t;
						current_block = 11506424456341058227;
					} else {
						(*vf).bittrack = 0.0f64;
						(*vf).samptrack = 0.0f64;
						return 0 as ::core::ffi::c_int;
					}
				}
			}
		}
	}
}
#[no_mangle]
pub unsafe extern "C" fn ov_pcm_seek(
	mut vf: *mut OggVorbis_File,
	mut pos: ogg_int64_t
) -> ::core::ffi::c_int {
	let mut thisblock: ::core::ffi::c_int = 0;
	let mut lastblock: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
	let mut ret: ::core::ffi::c_int = ov_pcm_seek_page(vf, pos);
	if ret < 0 as ::core::ffi::c_int {
		return ret;
	}
	ret = _make_decode_ready(vf);
	if ret != 0 {
		return ret;
	}
	loop {
		let mut op: ogg_packet = ogg_packet {
			packet: ::core::ptr::null_mut::<::core::ffi::c_uchar>(),
			bytes: 0,
			b_o_s: 0,
			e_o_s: 0,
			granulepos: 0,
			packetno: 0
		};
		let mut og: ogg_page = ogg_page {
			header: ::core::ptr::null_mut::<::core::ffi::c_uchar>(),
			header_len: 0,
			body: ::core::ptr::null_mut::<::core::ffi::c_uchar>(),
			body_len: 0
		};
		let mut ret_0: ::core::ffi::c_int = ogg_stream_packetpeek(&raw mut (*vf).os, &raw mut op);
		if ret_0 > 0 as ::core::ffi::c_int {
			thisblock =
				vorbis_packet_blocksize((*vf).vi.offset((*vf).current_link as isize), &raw mut op)
					as ::core::ffi::c_int;
			if thisblock < 0 as ::core::ffi::c_int {
				ogg_stream_packetout(&raw mut (*vf).os, ::core::ptr::null_mut::<ogg_packet>());
			} else {
				if lastblock != 0 {
					(*vf).pcm_offset +=
						(lastblock + thisblock >> 2 as ::core::ffi::c_int) as ogg_int64_t;
				}
				if (*vf).pcm_offset
					+ (thisblock + vorbis_info_blocksize((*vf).vi, 1 as ::core::ffi::c_int)
						>> 2 as ::core::ffi::c_int) as ogg_int64_t
					>= pos
				{
					break;
				}
				ogg_stream_packetout(&raw mut (*vf).os, ::core::ptr::null_mut::<ogg_packet>());
				vorbis_synthesis_trackonly(&raw mut (*vf).vb, &raw mut op);
				vorbis_synthesis_blockin(&raw mut (*vf).vd, &raw mut (*vf).vb);
				if op.granulepos > -(1 as ::core::ffi::c_int) as ogg_int64_t {
					let mut i: ::core::ffi::c_int = 0;
					(*vf).pcm_offset = op.granulepos
						- *(*vf)
							.pcmlengths
							.offset(((*vf).current_link * 2 as ::core::ffi::c_int) as isize);
					if (*vf).pcm_offset < 0 as ogg_int64_t {
						(*vf).pcm_offset = 0 as ogg_int64_t;
					}
					i = 0 as ::core::ffi::c_int;
					while i < (*vf).current_link {
						(*vf).pcm_offset += *(*vf).pcmlengths.offset(
							(i * 2 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize
						);
						i += 1;
					}
				}
				lastblock = thisblock;
			}
		} else {
			if ret_0 < 0 as ::core::ffi::c_int && ret_0 != OV_HOLE {
				break;
			}
			if _get_next_page(vf, &raw mut og, -(1 as ::core::ffi::c_int) as ogg_int64_t)
				< 0 as ogg_int64_t
			{
				break;
			}
			if ogg_page_bos(&raw mut og) != 0 {
				_decode_clear(vf);
			}
			if (*vf).ready_state < STREAMSET {
				let mut serialno: ::core::ffi::c_long =
					ogg_page_serialno(&raw mut og) as ::core::ffi::c_long;
				let mut link: ::core::ffi::c_int = 0;
				link = 0 as ::core::ffi::c_int;
				while link < (*vf).links {
					if *(*vf).serialnos.offset(link as isize) == serialno {
						break;
					}
					link += 1;
				}
				if link == (*vf).links {
					continue;
				}
				(*vf).current_link = link;
				(*vf).ready_state = STREAMSET;
				(*vf).current_serialno = ogg_page_serialno(&raw mut og) as ::core::ffi::c_long;
				ogg_stream_reset_serialno(&raw mut (*vf).os, serialno as ::core::ffi::c_int);
				ret_0 = _make_decode_ready(vf);
				if ret_0 != 0 {
					return ret_0;
				}
				lastblock = 0 as ::core::ffi::c_int;
			}
			ogg_stream_pagein(&raw mut (*vf).os, &raw mut og);
		}
	}
	(*vf).bittrack = 0.0f64;
	(*vf).samptrack = 0.0f64;
	let mut hs: ::core::ffi::c_int = vorbis_synthesis_halfrate_p((*vf).vi);
	while (*vf).pcm_offset < pos >> hs << hs {
		let mut target: ogg_int64_t = pos - (*vf).pcm_offset >> hs;
		let mut samples: ::core::ffi::c_long = vorbis_synthesis_pcmout(
			&raw mut (*vf).vd,
			::core::ptr::null_mut::<*mut *mut ::core::ffi::c_float>()
		) as ::core::ffi::c_long;
		if samples as ogg_int64_t > target {
			samples = target as ::core::ffi::c_long;
		}
		vorbis_synthesis_read(&raw mut (*vf).vd, samples as ::core::ffi::c_int);
		(*vf).pcm_offset =
			((*vf).pcm_offset as ::core::ffi::c_long + (samples << hs)) as ogg_int64_t;
		if (samples as ogg_int64_t) < target {
			if _fetch_and_process_packet(
				vf,
				::core::ptr::null_mut::<ogg_packet>(),
				1 as ::core::ffi::c_int,
				1 as ::core::ffi::c_int
			) <= 0 as ::core::ffi::c_int
			{
				(*vf).pcm_offset = ov_pcm_total(vf, -(1 as ::core::ffi::c_int));
			}
		}
	}
	return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ov_time_seek(
	mut vf: *mut OggVorbis_File,
	mut seconds: ::core::ffi::c_double
) -> ::core::ffi::c_int {
	let mut link: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
	let mut pcm_total: ogg_int64_t = 0 as ogg_int64_t;
	let mut time_total: ::core::ffi::c_double = 0.0f64;
	if (*vf).ready_state < OPENED {
		return -(131 as ::core::ffi::c_int);
	}
	if (*vf).seekable == 0 {
		return -(138 as ::core::ffi::c_int);
	}
	if seconds < 0 as ::core::ffi::c_int as ::core::ffi::c_double {
		return -(131 as ::core::ffi::c_int);
	}
	link = 0 as ::core::ffi::c_int;
	while link < (*vf).links {
		let mut addsec: ::core::ffi::c_double = ov_time_total(vf, link);
		if seconds < time_total + addsec {
			break;
		}
		time_total += addsec;
		pcm_total += *(*vf)
			.pcmlengths
			.offset((link * 2 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize);
		link += 1;
	}
	if link == (*vf).links {
		return -(131 as ::core::ffi::c_int);
	}
	let mut target: ogg_int64_t = (pcm_total as ::core::ffi::c_double
		+ (seconds - time_total) * (*(*vf).vi.offset(link as isize)).rate as ::core::ffi::c_double)
		as ogg_int64_t;
	return ov_pcm_seek(vf, target);
}
#[no_mangle]
pub unsafe extern "C" fn ov_time_seek_page(
	mut vf: *mut OggVorbis_File,
	mut seconds: ::core::ffi::c_double
) -> ::core::ffi::c_int {
	let mut link: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
	let mut pcm_total: ogg_int64_t = 0 as ogg_int64_t;
	let mut time_total: ::core::ffi::c_double = 0.0f64;
	if (*vf).ready_state < OPENED {
		return -(131 as ::core::ffi::c_int);
	}
	if (*vf).seekable == 0 {
		return -(138 as ::core::ffi::c_int);
	}
	if seconds < 0 as ::core::ffi::c_int as ::core::ffi::c_double {
		return -(131 as ::core::ffi::c_int);
	}
	link = 0 as ::core::ffi::c_int;
	while link < (*vf).links {
		let mut addsec: ::core::ffi::c_double = ov_time_total(vf, link);
		if seconds < time_total + addsec {
			break;
		}
		time_total += addsec;
		pcm_total += *(*vf)
			.pcmlengths
			.offset((link * 2 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize);
		link += 1;
	}
	if link == (*vf).links {
		return -(131 as ::core::ffi::c_int);
	}
	let mut target: ogg_int64_t = (pcm_total as ::core::ffi::c_double
		+ (seconds - time_total) * (*(*vf).vi.offset(link as isize)).rate as ::core::ffi::c_double)
		as ogg_int64_t;
	return ov_pcm_seek_page(vf, target);
}
#[no_mangle]
pub unsafe extern "C" fn ov_raw_tell(mut vf: *mut OggVorbis_File) -> ogg_int64_t {
	if (*vf).ready_state < OPENED {
		return -(131 as ::core::ffi::c_int) as ogg_int64_t;
	}
	return (*vf).offset;
}
#[no_mangle]
pub unsafe extern "C" fn ov_pcm_tell(mut vf: *mut OggVorbis_File) -> ogg_int64_t {
	if (*vf).ready_state < OPENED {
		return -(131 as ::core::ffi::c_int) as ogg_int64_t;
	}
	return (*vf).pcm_offset;
}
#[no_mangle]
pub unsafe extern "C" fn ov_time_tell(mut vf: *mut OggVorbis_File) -> ::core::ffi::c_double {
	let mut link: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
	let mut pcm_total: ogg_int64_t = 0 as ogg_int64_t;
	let mut time_total: ::core::ffi::c_double = 0.0f64;
	if (*vf).ready_state < OPENED {
		return -(131 as ::core::ffi::c_int) as ::core::ffi::c_double;
	}
	if (*vf).seekable != 0 {
		pcm_total = ov_pcm_total(vf, -(1 as ::core::ffi::c_int));
		time_total = ov_time_total(vf, -(1 as ::core::ffi::c_int));
		link = (*vf).links - 1 as ::core::ffi::c_int;
		while link >= 0 as ::core::ffi::c_int {
			pcm_total -= *(*vf)
				.pcmlengths
				.offset((link * 2 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize);
			time_total -= ov_time_total(vf, link);
			if (*vf).pcm_offset >= pcm_total {
				break;
			}
			link -= 1;
		}
	}
	return time_total
		+ ((*vf).pcm_offset - pcm_total) as ::core::ffi::c_double
			/ (*(*vf).vi.offset(link as isize)).rate as ::core::ffi::c_double;
}
#[no_mangle]
pub unsafe extern "C" fn ov_info(
	mut vf: *mut OggVorbis_File,
	mut link: ::core::ffi::c_int
) -> *mut vorbis_info {
	if (*vf).seekable != 0 {
		if link < 0 as ::core::ffi::c_int {
			if (*vf).ready_state >= STREAMSET {
				return (*vf).vi.offset((*vf).current_link as isize);
			} else {
				return (*vf).vi;
			}
		} else if link >= (*vf).links {
			return ::core::ptr::null_mut::<vorbis_info>();
		} else {
			return (*vf).vi.offset(link as isize);
		}
	} else {
		return (*vf).vi;
	};
}
#[no_mangle]
pub unsafe extern "C" fn ov_comment(
	mut vf: *mut OggVorbis_File,
	mut link: ::core::ffi::c_int
) -> *mut vorbis_comment {
	if (*vf).seekable != 0 {
		if link < 0 as ::core::ffi::c_int {
			if (*vf).ready_state >= STREAMSET {
				return (*vf).vc.offset((*vf).current_link as isize);
			} else {
				return (*vf).vc;
			}
		} else if link >= (*vf).links {
			return ::core::ptr::null_mut::<vorbis_comment>();
		} else {
			return (*vf).vc.offset(link as isize);
		}
	} else {
		return (*vf).vc;
	};
}
unsafe extern "C" fn host_is_big_endian() -> ::core::ffi::c_int {
	let mut pattern: ogg_int32_t = 0xfeedface as ::core::ffi::c_uint as ogg_int32_t;
	let mut bytewise: *mut ::core::ffi::c_uchar = &raw mut pattern as *mut ::core::ffi::c_uchar;
	if *bytewise.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
		== 0xfe as ::core::ffi::c_int
	{
		return 1 as ::core::ffi::c_int;
	}
	return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ov_read_filter(
	mut vf: *mut OggVorbis_File,
	mut buffer: *mut ::core::ffi::c_char,
	mut length: ::core::ffi::c_int,
	mut bigendianp: ::core::ffi::c_int,
	mut word: ::core::ffi::c_int,
	mut sgned: ::core::ffi::c_int,
	mut bitstream: *mut ::core::ffi::c_int,
	mut filter: Option<
		unsafe extern "C" fn(
			*mut *mut ::core::ffi::c_float,
			::core::ffi::c_long,
			::core::ffi::c_long,
			*mut ::core::ffi::c_void
		) -> ()
	>,
	mut filter_param: *mut ::core::ffi::c_void
) -> ::core::ffi::c_long {
	let mut i: ::core::ffi::c_int = 0;
	let mut j: ::core::ffi::c_int = 0;
	let mut host_endian: ::core::ffi::c_int = host_is_big_endian();
	let mut hs: ::core::ffi::c_int = 0;
	let mut pcm: *mut *mut ::core::ffi::c_float =
		::core::ptr::null_mut::<*mut ::core::ffi::c_float>();
	let mut samples: ::core::ffi::c_long = 0;
	if (*vf).ready_state < OPENED {
		return -(131 as ::core::ffi::c_int) as ::core::ffi::c_long;
	}
	if word <= 0 as ::core::ffi::c_int {
		return -(131 as ::core::ffi::c_int) as ::core::ffi::c_long;
	}
	loop {
		if (*vf).ready_state == INITSET {
			samples =
				vorbis_synthesis_pcmout(&raw mut (*vf).vd, &raw mut pcm) as ::core::ffi::c_long;
			if samples != 0 {
				break;
			}
		}
		let mut ret: ::core::ffi::c_int = _fetch_and_process_packet(
			vf,
			::core::ptr::null_mut::<ogg_packet>(),
			1 as ::core::ffi::c_int,
			1 as ::core::ffi::c_int
		);
		if ret == OV_EOF {
			return 0 as ::core::ffi::c_long;
		}
		if ret <= 0 as ::core::ffi::c_int {
			return ret as ::core::ffi::c_long;
		}
	}
	if samples > 0 as ::core::ffi::c_long {
		let mut channels: ::core::ffi::c_long =
			(*ov_info(vf, -(1 as ::core::ffi::c_int))).channels as ::core::ffi::c_long;
		let mut bytespersample: ::core::ffi::c_long = word as ::core::ffi::c_long * channels;
		let mut fpu: vorbis_fpu_control = 0;
		if channels < 1 as ::core::ffi::c_long || channels > 255 as ::core::ffi::c_long {
			return -(131 as ::core::ffi::c_int) as ::core::ffi::c_long;
		}
		if samples > length as ::core::ffi::c_long / bytespersample {
			samples = length as ::core::ffi::c_long / bytespersample;
		}
		if samples <= 0 as ::core::ffi::c_long {
			return OV_EINVAL as ::core::ffi::c_long;
		}
		if filter.is_some() {
			filter.expect("non-null function pointer")(pcm, channels, samples, filter_param);
		}
		let mut val: ::core::ffi::c_int = 0;
		if word == 1 as ::core::ffi::c_int {
			let mut off: ::core::ffi::c_int = if sgned != 0 {
				0 as ::core::ffi::c_int
			} else {
				128 as ::core::ffi::c_int
			};
			vorbis_fpu_setround(&raw mut fpu);
			j = 0 as ::core::ffi::c_int;
			while (j as ::core::ffi::c_long) < samples {
				i = 0 as ::core::ffi::c_int;
				while (i as ::core::ffi::c_long) < channels {
					val = vorbis_ftoi(
						(*(*pcm.offset(i as isize)).offset(j as isize) * 128.0f32)
							as ::core::ffi::c_double
					);
					if val > 127 as ::core::ffi::c_int {
						val = 127 as ::core::ffi::c_int;
					} else if val < -(128 as ::core::ffi::c_int) {
						val = -(128 as ::core::ffi::c_int);
					}
					let fresh3 = buffer;
					buffer = buffer.offset(1);
					*fresh3 = (val + off) as ::core::ffi::c_char;
					i += 1;
				}
				j += 1;
			}
			vorbis_fpu_restore(fpu);
		} else {
			let mut off_0: ::core::ffi::c_int = if sgned != 0 {
				0 as ::core::ffi::c_int
			} else {
				32768 as ::core::ffi::c_int
			};
			if host_endian == bigendianp {
				if sgned != 0 {
					vorbis_fpu_setround(&raw mut fpu);
					i = 0 as ::core::ffi::c_int;
					while (i as ::core::ffi::c_long) < channels {
						let mut src: *mut ::core::ffi::c_float = *pcm.offset(i as isize);
						let mut dest: *mut ::core::ffi::c_short =
							(buffer as *mut ::core::ffi::c_short).offset(i as isize);
						j = 0 as ::core::ffi::c_int;
						while (j as ::core::ffi::c_long) < samples {
							val = vorbis_ftoi(
								(*src.offset(j as isize) * 32768.0f32) as ::core::ffi::c_double
							);
							if val > 32767 as ::core::ffi::c_int {
								val = 32767 as ::core::ffi::c_int;
							} else if val < -(32768 as ::core::ffi::c_int) {
								val = -(32768 as ::core::ffi::c_int);
							}
							*dest = val as ::core::ffi::c_short;
							dest = dest.offset(channels as isize);
							j += 1;
						}
						i += 1;
					}
					vorbis_fpu_restore(fpu);
				} else {
					vorbis_fpu_setround(&raw mut fpu);
					i = 0 as ::core::ffi::c_int;
					while (i as ::core::ffi::c_long) < channels {
						let mut src_0: *mut ::core::ffi::c_float = *pcm.offset(i as isize);
						let mut dest_0: *mut ::core::ffi::c_short =
							(buffer as *mut ::core::ffi::c_short).offset(i as isize);
						j = 0 as ::core::ffi::c_int;
						while (j as ::core::ffi::c_long) < samples {
							val = vorbis_ftoi(
								(*src_0.offset(j as isize) * 32768.0f32) as ::core::ffi::c_double
							);
							if val > 32767 as ::core::ffi::c_int {
								val = 32767 as ::core::ffi::c_int;
							} else if val < -(32768 as ::core::ffi::c_int) {
								val = -(32768 as ::core::ffi::c_int);
							}
							*dest_0 = (val + off_0) as ::core::ffi::c_short;
							dest_0 = dest_0.offset(channels as isize);
							j += 1;
						}
						i += 1;
					}
					vorbis_fpu_restore(fpu);
				}
			} else if bigendianp != 0 {
				vorbis_fpu_setround(&raw mut fpu);
				j = 0 as ::core::ffi::c_int;
				while (j as ::core::ffi::c_long) < samples {
					i = 0 as ::core::ffi::c_int;
					while (i as ::core::ffi::c_long) < channels {
						val = vorbis_ftoi(
							(*(*pcm.offset(i as isize)).offset(j as isize) * 32768.0f32)
								as ::core::ffi::c_double
						);
						if val > 32767 as ::core::ffi::c_int {
							val = 32767 as ::core::ffi::c_int;
						} else if val < -(32768 as ::core::ffi::c_int) {
							val = -(32768 as ::core::ffi::c_int);
						}
						val += off_0;
						let fresh4 = buffer;
						buffer = buffer.offset(1);
						*fresh4 = (val >> 8 as ::core::ffi::c_int) as ::core::ffi::c_char;
						let fresh5 = buffer;
						buffer = buffer.offset(1);
						*fresh5 = (val & 0xff as ::core::ffi::c_int) as ::core::ffi::c_char;
						i += 1;
					}
					j += 1;
				}
				vorbis_fpu_restore(fpu);
			} else {
				let mut val_0: ::core::ffi::c_int = 0;
				vorbis_fpu_setround(&raw mut fpu);
				j = 0 as ::core::ffi::c_int;
				while (j as ::core::ffi::c_long) < samples {
					i = 0 as ::core::ffi::c_int;
					while (i as ::core::ffi::c_long) < channels {
						val_0 = vorbis_ftoi(
							(*(*pcm.offset(i as isize)).offset(j as isize) * 32768.0f32)
								as ::core::ffi::c_double
						);
						if val_0 > 32767 as ::core::ffi::c_int {
							val_0 = 32767 as ::core::ffi::c_int;
						} else if val_0 < -(32768 as ::core::ffi::c_int) {
							val_0 = -(32768 as ::core::ffi::c_int);
						}
						val_0 += off_0;
						let fresh6 = buffer;
						buffer = buffer.offset(1);
						*fresh6 = (val_0 & 0xff as ::core::ffi::c_int) as ::core::ffi::c_char;
						let fresh7 = buffer;
						buffer = buffer.offset(1);
						*fresh7 = (val_0 >> 8 as ::core::ffi::c_int) as ::core::ffi::c_char;
						i += 1;
					}
					j += 1;
				}
				vorbis_fpu_restore(fpu);
			}
		}
		vorbis_synthesis_read(&raw mut (*vf).vd, samples as ::core::ffi::c_int);
		hs = vorbis_synthesis_halfrate_p((*vf).vi);
		(*vf).pcm_offset =
			((*vf).pcm_offset as ::core::ffi::c_long + (samples << hs)) as ogg_int64_t;
		if !bitstream.is_null() {
			*bitstream = (*vf).current_link;
		}
		return samples * bytespersample;
	} else {
		return samples;
	};
}
#[no_mangle]
pub unsafe extern "C" fn ov_read(
	mut vf: *mut OggVorbis_File,
	mut buffer: *mut ::core::ffi::c_char,
	mut length: ::core::ffi::c_int,
	mut bigendianp: ::core::ffi::c_int,
	mut word: ::core::ffi::c_int,
	mut sgned: ::core::ffi::c_int,
	mut bitstream: *mut ::core::ffi::c_int
) -> ::core::ffi::c_long {
	return ov_read_filter(
		vf, buffer, length, bigendianp, word, sgned, bitstream, None, NULL
	);
}
#[no_mangle]
pub unsafe extern "C" fn ov_read_float(
	mut vf: *mut OggVorbis_File,
	mut pcm_channels: *mut *mut *mut ::core::ffi::c_float,
	mut length: ::core::ffi::c_int,
	mut bitstream: *mut ::core::ffi::c_int
) -> ::core::ffi::c_long {
	if (*vf).ready_state < OPENED {
		return -(131 as ::core::ffi::c_int) as ::core::ffi::c_long;
	}
	loop {
		if (*vf).ready_state == INITSET {
			let mut pcm: *mut *mut ::core::ffi::c_float =
				::core::ptr::null_mut::<*mut ::core::ffi::c_float>();
			let mut samples: ::core::ffi::c_long =
				vorbis_synthesis_pcmout(&raw mut (*vf).vd, &raw mut pcm) as ::core::ffi::c_long;
			if samples != 0 {
				let mut hs: ::core::ffi::c_int = vorbis_synthesis_halfrate_p((*vf).vi);
				if !pcm_channels.is_null() {
					*pcm_channels = pcm;
				}
				if samples > length as ::core::ffi::c_long {
					samples = length as ::core::ffi::c_long;
				}
				vorbis_synthesis_read(&raw mut (*vf).vd, samples as ::core::ffi::c_int);
				(*vf).pcm_offset =
					((*vf).pcm_offset as ::core::ffi::c_long + (samples << hs)) as ogg_int64_t;
				if !bitstream.is_null() {
					*bitstream = (*vf).current_link;
				}
				return samples;
			}
		}
		let mut ret: ::core::ffi::c_int = _fetch_and_process_packet(
			vf,
			::core::ptr::null_mut::<ogg_packet>(),
			1 as ::core::ffi::c_int,
			1 as ::core::ffi::c_int
		);
		if ret == OV_EOF {
			return 0 as ::core::ffi::c_long;
		}
		if ret <= 0 as ::core::ffi::c_int {
			return ret as ::core::ffi::c_long;
		}
	}
}
unsafe extern "C" fn _ov_splice(
	mut pcm: *mut *mut ::core::ffi::c_float,
	mut lappcm: *mut *mut ::core::ffi::c_float,
	mut n1: ::core::ffi::c_int,
	mut n2: ::core::ffi::c_int,
	mut ch1: ::core::ffi::c_int,
	mut ch2: ::core::ffi::c_int,
	mut w1: *const ::core::ffi::c_float,
	mut w2: *const ::core::ffi::c_float
) {
	let mut i: ::core::ffi::c_int = 0;
	let mut j: ::core::ffi::c_int = 0;
	let mut w: *const ::core::ffi::c_float = w1;
	let mut n: ::core::ffi::c_int = n1;
	if n1 > n2 {
		n = n2;
		w = w2;
	}
	j = 0 as ::core::ffi::c_int;
	while j < ch1 && j < ch2 {
		let mut s: *mut ::core::ffi::c_float = *lappcm.offset(j as isize);
		let mut d: *mut ::core::ffi::c_float = *pcm.offset(j as isize);
		i = 0 as ::core::ffi::c_int;
		while i < n {
			let mut wd: ::core::ffi::c_float = *w.offset(i as isize) * *w.offset(i as isize);
			let mut ws: ::core::ffi::c_float =
				(1.0f64 - wd as ::core::ffi::c_double) as ::core::ffi::c_float;
			*d.offset(i as isize) = *d.offset(i as isize) * wd + *s.offset(i as isize) * ws;
			i += 1;
		}
		j += 1;
	}
	while j < ch2 {
		let mut d_0: *mut ::core::ffi::c_float = *pcm.offset(j as isize);
		i = 0 as ::core::ffi::c_int;
		while i < n {
			let mut wd_0: ::core::ffi::c_float = *w.offset(i as isize) * *w.offset(i as isize);
			*d_0.offset(i as isize) = *d_0.offset(i as isize) * wd_0;
			i += 1;
		}
		j += 1;
	}
}
unsafe extern "C" fn _ov_initset(mut vf: *mut OggVorbis_File) -> ::core::ffi::c_int {
	while !((*vf).ready_state == INITSET) {
		let mut ret: ::core::ffi::c_int = _fetch_and_process_packet(
			vf,
			::core::ptr::null_mut::<ogg_packet>(),
			1 as ::core::ffi::c_int,
			0 as ::core::ffi::c_int
		);
		if ret < 0 as ::core::ffi::c_int && ret != OV_HOLE {
			return ret;
		}
	}
	return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn _ov_initprime(mut vf: *mut OggVorbis_File) -> ::core::ffi::c_int {
	let mut vd: *mut vorbis_dsp_state = &raw mut (*vf).vd;
	loop {
		if (*vf).ready_state == INITSET {
			if vorbis_synthesis_pcmout(
				vd,
				::core::ptr::null_mut::<*mut *mut ::core::ffi::c_float>()
			) != 0
			{
				break;
			}
		}
		let mut ret: ::core::ffi::c_int = _fetch_and_process_packet(
			vf,
			::core::ptr::null_mut::<ogg_packet>(),
			1 as ::core::ffi::c_int,
			0 as ::core::ffi::c_int
		);
		if ret < 0 as ::core::ffi::c_int && ret != OV_HOLE {
			return ret;
		}
	}
	return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn _ov_getlap(
	mut vf: *mut OggVorbis_File,
	mut vi: *mut vorbis_info,
	mut vd: *mut vorbis_dsp_state,
	mut lappcm: *mut *mut ::core::ffi::c_float,
	mut lapsize: ::core::ffi::c_int
) {
	let mut lapcount: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
	let mut i: ::core::ffi::c_int = 0;
	let mut pcm: *mut *mut ::core::ffi::c_float =
		::core::ptr::null_mut::<*mut ::core::ffi::c_float>();
	while lapcount < lapsize {
		let mut samples: ::core::ffi::c_int = vorbis_synthesis_pcmout(vd, &raw mut pcm);
		if samples != 0 {
			if samples > lapsize - lapcount {
				samples = lapsize - lapcount;
			}
			i = 0 as ::core::ffi::c_int;
			while i < (*vi).channels {
				memcpy(
					(*lappcm.offset(i as isize)).offset(lapcount as isize)
						as *mut ::core::ffi::c_void,
					*pcm.offset(i as isize) as *const ::core::ffi::c_void,
					(::core::mem::size_of::<::core::ffi::c_float>() as size_t)
						.wrapping_mul(samples as size_t)
				);
				i += 1;
			}
			lapcount += samples;
			vorbis_synthesis_read(vd, samples);
		} else {
			let mut ret: ::core::ffi::c_int = _fetch_and_process_packet(
				vf,
				::core::ptr::null_mut::<ogg_packet>(),
				1 as ::core::ffi::c_int,
				0 as ::core::ffi::c_int
			);
			if ret == OV_EOF {
				break;
			}
		}
	}
	if lapcount < lapsize {
		let mut samples_0: ::core::ffi::c_int =
			vorbis_synthesis_lapout(&raw mut (*vf).vd, &raw mut pcm);
		if samples_0 == 0 as ::core::ffi::c_int {
			i = 0 as ::core::ffi::c_int;
			while i < (*vi).channels {
				memset(
					(*lappcm.offset(i as isize)).offset(lapcount as isize)
						as *mut ::core::ffi::c_void,
					0 as ::core::ffi::c_int,
					(::core::mem::size_of::<::core::ffi::c_float>() as size_t)
						.wrapping_mul(lapsize as size_t)
						.wrapping_sub(lapcount as size_t)
				);
				i += 1;
			}
			lapcount = lapsize;
		} else {
			if samples_0 > lapsize - lapcount {
				samples_0 = lapsize - lapcount;
			}
			i = 0 as ::core::ffi::c_int;
			while i < (*vi).channels {
				memcpy(
					(*lappcm.offset(i as isize)).offset(lapcount as isize)
						as *mut ::core::ffi::c_void,
					*pcm.offset(i as isize) as *const ::core::ffi::c_void,
					(::core::mem::size_of::<::core::ffi::c_float>() as size_t)
						.wrapping_mul(samples_0 as size_t)
				);
				i += 1;
			}
			lapcount += samples_0;
		}
	}
}
pub const LAPBUF_CHANNELS_CUTOFF: ::core::ffi::c_int = 8 as ::core::ffi::c_int;
#[no_mangle]
pub unsafe extern "C" fn ov_crosslap(
	mut vf1: *mut OggVorbis_File,
	mut vf2: *mut OggVorbis_File
) -> ::core::ffi::c_int {
	let mut alloca_allocations: Vec<Vec<u8>> = Vec::new();
	let mut vi1: *mut vorbis_info = ::core::ptr::null_mut::<vorbis_info>();
	let mut vi2: *mut vorbis_info = ::core::ptr::null_mut::<vorbis_info>();
	let mut lapbuf: *mut ::core::ffi::c_float = ::core::ptr::null_mut::<::core::ffi::c_float>();
	let mut lappcm: *mut *mut ::core::ffi::c_float =
		::core::ptr::null_mut::<*mut ::core::ffi::c_float>();
	let mut pcm: *mut *mut ::core::ffi::c_float =
		::core::ptr::null_mut::<*mut ::core::ffi::c_float>();
	let mut w1: *const ::core::ffi::c_float = ::core::ptr::null::<::core::ffi::c_float>();
	let mut w2: *const ::core::ffi::c_float = ::core::ptr::null::<::core::ffi::c_float>();
	let mut n1: ::core::ffi::c_int = 0;
	let mut n2: ::core::ffi::c_int = 0;
	let mut ch1: ::core::ffi::c_int = 0;
	let mut i: ::core::ffi::c_int = 0;
	let mut ret: ::core::ffi::c_int = 0;
	let mut hs1: ::core::ffi::c_int = 0;
	let mut hs2: ::core::ffi::c_int = 0;
	if vf1 == vf2 {
		return 0 as ::core::ffi::c_int;
	}
	if (*vf1).ready_state < OPENED {
		return -(131 as ::core::ffi::c_int);
	}
	if (*vf2).ready_state < OPENED {
		return -(131 as ::core::ffi::c_int);
	}
	ret = _ov_initset(vf1);
	if ret != 0 {
		return ret;
	}
	ret = _ov_initprime(vf2);
	if ret != 0 {
		return ret;
	}
	vi1 = ov_info(vf1, -(1 as ::core::ffi::c_int));
	vi2 = ov_info(vf2, -(1 as ::core::ffi::c_int));
	hs1 = ov_halfrate_p(vf1);
	hs2 = ov_halfrate_p(vf2);
	ch1 = (*vi1).channels;
	alloca_allocations.push(::std::vec::from_elem(
		0,
		(::core::mem::size_of::<*mut ::core::ffi::c_float>() as usize).wrapping_mul(ch1 as usize)
			as usize
	));
	lappcm = alloca_allocations.last_mut().unwrap().as_mut_ptr() as *mut *mut ::core::ffi::c_float;
	n1 = vorbis_info_blocksize(vi1, 0 as ::core::ffi::c_int) >> 1 as ::core::ffi::c_int + hs1;
	n2 = vorbis_info_blocksize(vi2, 0 as ::core::ffi::c_int) >> 1 as ::core::ffi::c_int + hs2;
	w1 = vorbis_window(&raw mut (*vf1).vd, 0 as ::core::ffi::c_int);
	w2 = vorbis_window(&raw mut (*vf2).vd, 0 as ::core::ffi::c_int);
	if ch1 <= LAPBUF_CHANNELS_CUTOFF {
		alloca_allocations.push(::std::vec::from_elem(
			0,
			(::core::mem::size_of::<::core::ffi::c_float>() as usize)
				.wrapping_mul(n1 as usize)
				.wrapping_mul(ch1 as usize) as usize
		));
		lapbuf = alloca_allocations.last_mut().unwrap().as_mut_ptr() as *mut ::core::ffi::c_float;
	} else {
		lapbuf = malloc(
			(::core::mem::size_of::<::core::ffi::c_float>() as size_t)
				.wrapping_mul(n1 as size_t)
				.wrapping_mul(ch1 as size_t)
		) as *mut ::core::ffi::c_float;
		if lapbuf.is_null() {
			return OV_EFAULT;
		}
	}
	i = 0 as ::core::ffi::c_int;
	while i < ch1 {
		let ref mut fresh8 = *lappcm.offset(i as isize);
		*fresh8 = lapbuf.offset((n1 * i) as isize);
		i += 1;
	}
	_ov_getlap(vf1, vi1, &raw mut (*vf1).vd, lappcm, n1);
	vorbis_synthesis_lapout(&raw mut (*vf2).vd, &raw mut pcm);
	_ov_splice(pcm, lappcm, n1, n2, ch1, (*vi2).channels, w1, w2);
	if ch1 > LAPBUF_CHANNELS_CUTOFF {
		free(lapbuf as *mut ::core::ffi::c_void);
	}
	return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn _ov_64_seek_lap_finish(
	mut vf: *mut OggVorbis_File,
	mut pos: ogg_int64_t,
	mut localseek: Option<
		unsafe extern "C" fn(*mut OggVorbis_File, ogg_int64_t) -> ::core::ffi::c_int
	>,
	mut hs: ::core::ffi::c_int,
	mut n1: ::core::ffi::c_int,
	mut ch1: ::core::ffi::c_int,
	mut w1: *const ::core::ffi::c_float,
	mut lappcm: *mut *mut ::core::ffi::c_float
) -> ::core::ffi::c_int {
	let mut vi: *mut vorbis_info = ::core::ptr::null_mut::<vorbis_info>();
	let mut pcm: *mut *mut ::core::ffi::c_float =
		::core::ptr::null_mut::<*mut ::core::ffi::c_float>();
	let mut w2: *const ::core::ffi::c_float = ::core::ptr::null::<::core::ffi::c_float>();
	let mut n2: ::core::ffi::c_int = 0;
	let mut ch2: ::core::ffi::c_int = 0;
	let mut ret: ::core::ffi::c_int = 0;
	ret = localseek.expect("non-null function pointer")(vf, pos);
	if ret != 0 {
		return ret;
	}
	ret = _ov_initprime(vf);
	if ret != 0 {
		return ret;
	}
	vi = ov_info(vf, -(1 as ::core::ffi::c_int));
	ch2 = (*vi).channels;
	n2 = vorbis_info_blocksize(vi, 0 as ::core::ffi::c_int) >> 1 as ::core::ffi::c_int + hs;
	w2 = vorbis_window(&raw mut (*vf).vd, 0 as ::core::ffi::c_int);
	vorbis_synthesis_lapout(&raw mut (*vf).vd, &raw mut pcm);
	_ov_splice(pcm, lappcm, n1, n2, ch1, ch2, w1, w2);
	return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn _ov_64_seek_lap(
	mut vf: *mut OggVorbis_File,
	mut pos: ogg_int64_t,
	mut localseek: Option<
		unsafe extern "C" fn(*mut OggVorbis_File, ogg_int64_t) -> ::core::ffi::c_int
	>
) -> ::core::ffi::c_int {
	let mut alloca_allocations: Vec<Vec<u8>> = Vec::new();
	let mut vi: *mut vorbis_info = ::core::ptr::null_mut::<vorbis_info>();
	let mut lapbuf: *mut ::core::ffi::c_float = ::core::ptr::null_mut::<::core::ffi::c_float>();
	let mut lappcm: *mut *mut ::core::ffi::c_float =
		::core::ptr::null_mut::<*mut ::core::ffi::c_float>();
	let mut w1: *const ::core::ffi::c_float = ::core::ptr::null::<::core::ffi::c_float>();
	let mut n1: ::core::ffi::c_int = 0;
	let mut ch1: ::core::ffi::c_int = 0;
	let mut hs: ::core::ffi::c_int = 0;
	let mut i: ::core::ffi::c_int = 0;
	let mut ret: ::core::ffi::c_int = 0;
	if (*vf).ready_state < OPENED {
		return -(131 as ::core::ffi::c_int);
	}
	ret = _ov_initset(vf);
	if ret != 0 {
		return ret;
	}
	vi = ov_info(vf, -(1 as ::core::ffi::c_int));
	hs = ov_halfrate_p(vf);
	ch1 = (*vi).channels;
	n1 = vorbis_info_blocksize(vi, 0 as ::core::ffi::c_int) >> 1 as ::core::ffi::c_int + hs;
	w1 = vorbis_window(&raw mut (*vf).vd, 0 as ::core::ffi::c_int);
	alloca_allocations.push(::std::vec::from_elem(
		0,
		(::core::mem::size_of::<*mut ::core::ffi::c_float>() as usize).wrapping_mul(ch1 as usize)
			as usize
	));
	lappcm = alloca_allocations.last_mut().unwrap().as_mut_ptr() as *mut *mut ::core::ffi::c_float;
	if ch1 <= LAPBUF_CHANNELS_CUTOFF {
		alloca_allocations.push(::std::vec::from_elem(
			0,
			(::core::mem::size_of::<::core::ffi::c_float>() as usize)
				.wrapping_mul(n1 as usize)
				.wrapping_mul(ch1 as usize) as usize
		));
		lapbuf = alloca_allocations.last_mut().unwrap().as_mut_ptr() as *mut ::core::ffi::c_float;
	} else {
		lapbuf = malloc(
			(::core::mem::size_of::<::core::ffi::c_float>() as size_t)
				.wrapping_mul(n1 as size_t)
				.wrapping_mul(ch1 as size_t)
		) as *mut ::core::ffi::c_float;
		if lapbuf.is_null() {
			return OV_EFAULT;
		}
	}
	i = 0 as ::core::ffi::c_int;
	while i < ch1 {
		let ref mut fresh1 = *lappcm.offset(i as isize);
		*fresh1 = lapbuf.offset((n1 * i) as isize);
		i += 1;
	}
	_ov_getlap(vf, vi, &raw mut (*vf).vd, lappcm, n1);
	ret = _ov_64_seek_lap_finish(vf, pos, localseek, hs, n1, ch1, w1, lappcm);
	if ch1 > LAPBUF_CHANNELS_CUTOFF {
		free(lapbuf as *mut ::core::ffi::c_void);
	}
	return ret;
}
#[no_mangle]
pub unsafe extern "C" fn ov_raw_seek_lap(
	mut vf: *mut OggVorbis_File,
	mut pos: ogg_int64_t
) -> ::core::ffi::c_int {
	return _ov_64_seek_lap(
		vf,
		pos,
		Some(
			ov_raw_seek
				as unsafe extern "C" fn(*mut OggVorbis_File, ogg_int64_t) -> ::core::ffi::c_int
		)
	);
}
#[no_mangle]
pub unsafe extern "C" fn ov_pcm_seek_lap(
	mut vf: *mut OggVorbis_File,
	mut pos: ogg_int64_t
) -> ::core::ffi::c_int {
	return _ov_64_seek_lap(
		vf,
		pos,
		Some(
			ov_pcm_seek
				as unsafe extern "C" fn(*mut OggVorbis_File, ogg_int64_t) -> ::core::ffi::c_int
		)
	);
}
#[no_mangle]
pub unsafe extern "C" fn ov_pcm_seek_page_lap(
	mut vf: *mut OggVorbis_File,
	mut pos: ogg_int64_t
) -> ::core::ffi::c_int {
	return _ov_64_seek_lap(
		vf,
		pos,
		Some(
			ov_pcm_seek_page
				as unsafe extern "C" fn(*mut OggVorbis_File, ogg_int64_t) -> ::core::ffi::c_int
		)
	);
}
unsafe extern "C" fn _ov_d_seek_lap_finish(
	mut vf: *mut OggVorbis_File,
	mut pos: ::core::ffi::c_double,
	mut localseek: Option<
		unsafe extern "C" fn(*mut OggVorbis_File, ::core::ffi::c_double) -> ::core::ffi::c_int
	>,
	mut hs: ::core::ffi::c_int,
	mut n1: ::core::ffi::c_int,
	mut ch1: ::core::ffi::c_int,
	mut w1: *const ::core::ffi::c_float,
	mut lappcm: *mut *mut ::core::ffi::c_float
) -> ::core::ffi::c_int {
	let mut vi: *mut vorbis_info = ::core::ptr::null_mut::<vorbis_info>();
	let mut pcm: *mut *mut ::core::ffi::c_float =
		::core::ptr::null_mut::<*mut ::core::ffi::c_float>();
	let mut w2: *const ::core::ffi::c_float = ::core::ptr::null::<::core::ffi::c_float>();
	let mut n2: ::core::ffi::c_int = 0;
	let mut ch2: ::core::ffi::c_int = 0;
	let mut ret: ::core::ffi::c_int = 0;
	ret = localseek.expect("non-null function pointer")(vf, pos);
	if ret != 0 {
		return ret;
	}
	ret = _ov_initprime(vf);
	if ret != 0 {
		return ret;
	}
	vi = ov_info(vf, -(1 as ::core::ffi::c_int));
	ch2 = (*vi).channels;
	n2 = vorbis_info_blocksize(vi, 0 as ::core::ffi::c_int) >> 1 as ::core::ffi::c_int + hs;
	w2 = vorbis_window(&raw mut (*vf).vd, 0 as ::core::ffi::c_int);
	vorbis_synthesis_lapout(&raw mut (*vf).vd, &raw mut pcm);
	_ov_splice(pcm, lappcm, n1, n2, ch1, ch2, w1, w2);
	return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn _ov_d_seek_lap(
	mut vf: *mut OggVorbis_File,
	mut pos: ::core::ffi::c_double,
	mut localseek: Option<
		unsafe extern "C" fn(*mut OggVorbis_File, ::core::ffi::c_double) -> ::core::ffi::c_int
	>
) -> ::core::ffi::c_int {
	let mut alloca_allocations: Vec<Vec<u8>> = Vec::new();
	let mut vi: *mut vorbis_info = ::core::ptr::null_mut::<vorbis_info>();
	let mut lapbuf: *mut ::core::ffi::c_float = ::core::ptr::null_mut::<::core::ffi::c_float>();
	let mut lappcm: *mut *mut ::core::ffi::c_float =
		::core::ptr::null_mut::<*mut ::core::ffi::c_float>();
	let mut w1: *const ::core::ffi::c_float = ::core::ptr::null::<::core::ffi::c_float>();
	let mut n1: ::core::ffi::c_int = 0;
	let mut ch1: ::core::ffi::c_int = 0;
	let mut hs: ::core::ffi::c_int = 0;
	let mut i: ::core::ffi::c_int = 0;
	let mut ret: ::core::ffi::c_int = 0;
	if (*vf).ready_state < OPENED {
		return -(131 as ::core::ffi::c_int);
	}
	ret = _ov_initset(vf);
	if ret != 0 {
		return ret;
	}
	vi = ov_info(vf, -(1 as ::core::ffi::c_int));
	hs = ov_halfrate_p(vf);
	ch1 = (*vi).channels;
	n1 = vorbis_info_blocksize(vi, 0 as ::core::ffi::c_int) >> 1 as ::core::ffi::c_int + hs;
	w1 = vorbis_window(&raw mut (*vf).vd, 0 as ::core::ffi::c_int);
	alloca_allocations.push(::std::vec::from_elem(
		0,
		(::core::mem::size_of::<*mut ::core::ffi::c_float>() as usize).wrapping_mul(ch1 as usize)
			as usize
	));
	lappcm = alloca_allocations.last_mut().unwrap().as_mut_ptr() as *mut *mut ::core::ffi::c_float;
	if ch1 <= LAPBUF_CHANNELS_CUTOFF {
		alloca_allocations.push(::std::vec::from_elem(
			0,
			(::core::mem::size_of::<::core::ffi::c_float>() as usize)
				.wrapping_mul(n1 as usize)
				.wrapping_mul(ch1 as usize) as usize
		));
		lapbuf = alloca_allocations.last_mut().unwrap().as_mut_ptr() as *mut ::core::ffi::c_float;
	} else {
		lapbuf = malloc(
			(::core::mem::size_of::<::core::ffi::c_float>() as size_t)
				.wrapping_mul(n1 as size_t)
				.wrapping_mul(ch1 as size_t)
		) as *mut ::core::ffi::c_float;
		if lapbuf.is_null() {
			return OV_EFAULT;
		}
	}
	i = 0 as ::core::ffi::c_int;
	while i < ch1 {
		let ref mut fresh2 = *lappcm.offset(i as isize);
		*fresh2 = lapbuf.offset((n1 * i) as isize);
		i += 1;
	}
	_ov_getlap(vf, vi, &raw mut (*vf).vd, lappcm, n1);
	ret = _ov_d_seek_lap_finish(vf, pos, localseek, hs, n1, ch1, w1, lappcm);
	if ch1 > LAPBUF_CHANNELS_CUTOFF {
		free(lapbuf as *mut ::core::ffi::c_void);
	}
	return ret;
}
#[no_mangle]
pub unsafe extern "C" fn ov_time_seek_lap(
	mut vf: *mut OggVorbis_File,
	mut pos: ::core::ffi::c_double
) -> ::core::ffi::c_int {
	return _ov_d_seek_lap(
		vf,
		pos,
		Some(
			ov_time_seek
				as unsafe extern "C" fn(
					*mut OggVorbis_File,
					::core::ffi::c_double
				) -> ::core::ffi::c_int
		)
	);
}
#[no_mangle]
pub unsafe extern "C" fn ov_time_seek_page_lap(
	mut vf: *mut OggVorbis_File,
	mut pos: ::core::ffi::c_double
) -> ::core::ffi::c_int {
	return _ov_d_seek_lap(
		vf,
		pos,
		Some(
			ov_time_seek_page
				as unsafe extern "C" fn(
					*mut OggVorbis_File,
					::core::ffi::c_double
				) -> ::core::ffi::c_int
		)
	);
}
#[inline]
unsafe extern "C" fn vorbis_ftoi(mut f: ::core::ffi::c_double) -> ::core::ffi::c_int {
	return _mm_cvtsd_si32(_mm_load_sd(&raw mut f));
}
#[inline]
unsafe extern "C" fn vorbis_fpu_setround(mut fpu: *mut vorbis_fpu_control) {}
#[inline]
unsafe extern "C" fn vorbis_fpu_restore(mut fpu: vorbis_fpu_control) {}
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
