extern "C" {
	fn malloc(__size: size_t) -> *mut ::core::ffi::c_void;
	fn realloc(__ptr: *mut ::core::ffi::c_void, __size: size_t) -> *mut ::core::ffi::c_void;
	fn free(__ptr: *mut ::core::ffi::c_void);
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
	fn memcmp(
		__s1: *const ::core::ffi::c_void,
		__s2: *const ::core::ffi::c_void,
		__n: size_t
	) -> ::core::ffi::c_int;
	fn memchr(
		__s: *const ::core::ffi::c_void,
		__c: ::core::ffi::c_int,
		__n: size_t
	) -> *mut ::core::ffi::c_void;
}
pub type size_t = usize;
pub type __uint32_t = u32;
pub type __int64_t = i64;
pub type __uint64_t = u64;
pub type int64_t = __int64_t;
pub type uint32_t = __uint32_t;
pub type uint64_t = __uint64_t;
pub type ogg_uint32_t = uint32_t;
pub type ogg_int64_t = int64_t;
pub type ogg_uint64_t = uint64_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ogg_iovec_t {
	pub iov_base: *mut ::core::ffi::c_void,
	pub iov_len: size_t
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ogg_page {
	pub header: *mut ::core::ffi::c_uchar,
	pub header_len: ::core::ffi::c_long,
	pub body: *mut ::core::ffi::c_uchar,
	pub body_len: ::core::ffi::c_long
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ogg_stream_state {
	pub body_data: *mut ::core::ffi::c_uchar,
	pub body_storage: ::core::ffi::c_long,
	pub body_fill: ::core::ffi::c_long,
	pub body_returned: ::core::ffi::c_long,
	pub lacing_vals: *mut ::core::ffi::c_int,
	pub granule_vals: *mut ogg_int64_t,
	pub lacing_storage: ::core::ffi::c_long,
	pub lacing_fill: ::core::ffi::c_long,
	pub lacing_packet: ::core::ffi::c_long,
	pub lacing_returned: ::core::ffi::c_long,
	pub header: [::core::ffi::c_uchar; 282],
	pub header_fill: ::core::ffi::c_int,
	pub e_o_s: ::core::ffi::c_int,
	pub b_o_s: ::core::ffi::c_int,
	pub serialno: ::core::ffi::c_long,
	pub pageno: ::core::ffi::c_long,
	pub packetno: ogg_int64_t,
	pub granulepos: ogg_int64_t
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ogg_packet {
	pub packet: *mut ::core::ffi::c_uchar,
	pub bytes: ::core::ffi::c_long,
	pub b_o_s: ::core::ffi::c_long,
	pub e_o_s: ::core::ffi::c_long,
	pub granulepos: ogg_int64_t,
	pub packetno: ogg_int64_t
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ogg_sync_state {
	pub data: *mut ::core::ffi::c_uchar,
	pub storage: ::core::ffi::c_int,
	pub fill: ::core::ffi::c_int,
	pub returned: ::core::ffi::c_int,
	pub unsynced: ::core::ffi::c_int,
	pub headerbytes: ::core::ffi::c_int,
	pub bodybytes: ::core::ffi::c_int
}
pub const INT_MAX: ::core::ffi::c_int = __INT_MAX__;
pub const LONG_MAX: ::core::ffi::c_long = __LONG_MAX__;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
#[no_mangle]
pub unsafe extern "C" fn ogg_page_version(mut og: *const ogg_page) -> ::core::ffi::c_int {
	return *(*og).header.offset(4 as ::core::ffi::c_int as isize) as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ogg_page_continued(mut og: *const ogg_page) -> ::core::ffi::c_int {
	return *(*og).header.offset(5 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
		& 0x1 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ogg_page_bos(mut og: *const ogg_page) -> ::core::ffi::c_int {
	return *(*og).header.offset(5 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
		& 0x2 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ogg_page_eos(mut og: *const ogg_page) -> ::core::ffi::c_int {
	return *(*og).header.offset(5 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
		& 0x4 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ogg_page_granulepos(mut og: *const ogg_page) -> ogg_int64_t {
	let mut page: *mut ::core::ffi::c_uchar = (*og).header;
	let mut granulepos: ogg_uint64_t = (*page.offset(13 as ::core::ffi::c_int as isize)
		as ::core::ffi::c_int
		& 0xff as ::core::ffi::c_int) as ogg_uint64_t;
	granulepos = granulepos << 8 as ::core::ffi::c_int
		| (*page.offset(12 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
			& 0xff as ::core::ffi::c_int) as ogg_uint64_t;
	granulepos = granulepos << 8 as ::core::ffi::c_int
		| (*page.offset(11 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
			& 0xff as ::core::ffi::c_int) as ogg_uint64_t;
	granulepos = granulepos << 8 as ::core::ffi::c_int
		| (*page.offset(10 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
			& 0xff as ::core::ffi::c_int) as ogg_uint64_t;
	granulepos = granulepos << 8 as ::core::ffi::c_int
		| (*page.offset(9 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
			& 0xff as ::core::ffi::c_int) as ogg_uint64_t;
	granulepos = granulepos << 8 as ::core::ffi::c_int
		| (*page.offset(8 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
			& 0xff as ::core::ffi::c_int) as ogg_uint64_t;
	granulepos = granulepos << 8 as ::core::ffi::c_int
		| (*page.offset(7 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
			& 0xff as ::core::ffi::c_int) as ogg_uint64_t;
	granulepos = granulepos << 8 as ::core::ffi::c_int
		| (*page.offset(6 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
			& 0xff as ::core::ffi::c_int) as ogg_uint64_t;
	return granulepos as ogg_int64_t;
}
#[no_mangle]
pub unsafe extern "C" fn ogg_page_serialno(mut og: *const ogg_page) -> ::core::ffi::c_int {
	return (*(*og).header.offset(14 as ::core::ffi::c_int as isize) as ogg_uint32_t
		as ::core::ffi::c_int as ogg_uint32_t
		| (*(*og).header.offset(15 as ::core::ffi::c_int as isize) as ogg_uint32_t)
			<< 8 as ::core::ffi::c_int
		| (*(*og).header.offset(16 as ::core::ffi::c_int as isize) as ogg_uint32_t)
			<< 16 as ::core::ffi::c_int
		| (*(*og).header.offset(17 as ::core::ffi::c_int as isize) as ogg_uint32_t)
			<< 24 as ::core::ffi::c_int) as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ogg_page_pageno(mut og: *const ogg_page) -> ::core::ffi::c_long {
	return *(*og).header.offset(18 as ::core::ffi::c_int as isize) as ogg_uint32_t
		as ::core::ffi::c_long
		| ((*(*og).header.offset(19 as ::core::ffi::c_int as isize) as ogg_uint32_t)
			<< 8 as ::core::ffi::c_int) as ::core::ffi::c_long
		| ((*(*og).header.offset(20 as ::core::ffi::c_int as isize) as ogg_uint32_t)
			<< 16 as ::core::ffi::c_int) as ::core::ffi::c_long
		| ((*(*og).header.offset(21 as ::core::ffi::c_int as isize) as ogg_uint32_t)
			<< 24 as ::core::ffi::c_int) as ::core::ffi::c_long;
}
#[no_mangle]
pub unsafe extern "C" fn ogg_page_packets(mut og: *const ogg_page) -> ::core::ffi::c_int {
	let mut i: ::core::ffi::c_int = 0;
	let mut n: ::core::ffi::c_int =
		*(*og).header.offset(26 as ::core::ffi::c_int as isize) as ::core::ffi::c_int;
	let mut count: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
	i = 0 as ::core::ffi::c_int;
	while i < n {
		if (*(*og).header.offset((27 as ::core::ffi::c_int + i) as isize) as ::core::ffi::c_int)
			< 255 as ::core::ffi::c_int
		{
			count += 1;
		}
		i += 1;
	}
	return count;
}
static mut crc_lookup: [[ogg_uint32_t; 256]; 8] = [
	[
		0 as ::core::ffi::c_int as ogg_uint32_t,
		0x4c11db7 as ::core::ffi::c_int as ogg_uint32_t,
		0x9823b6e as ::core::ffi::c_int as ogg_uint32_t,
		0xd4326d9 as ::core::ffi::c_int as ogg_uint32_t,
		0x130476dc as ::core::ffi::c_int as ogg_uint32_t,
		0x17c56b6b as ::core::ffi::c_int as ogg_uint32_t,
		0x1a864db2 as ::core::ffi::c_int as ogg_uint32_t,
		0x1e475005 as ::core::ffi::c_int as ogg_uint32_t,
		0x2608edb8 as ::core::ffi::c_int as ogg_uint32_t,
		0x22c9f00f as ::core::ffi::c_int as ogg_uint32_t,
		0x2f8ad6d6 as ::core::ffi::c_int as ogg_uint32_t,
		0x2b4bcb61 as ::core::ffi::c_int as ogg_uint32_t,
		0x350c9b64 as ::core::ffi::c_int as ogg_uint32_t,
		0x31cd86d3 as ::core::ffi::c_int as ogg_uint32_t,
		0x3c8ea00a as ::core::ffi::c_int as ogg_uint32_t,
		0x384fbdbd as ::core::ffi::c_int as ogg_uint32_t,
		0x4c11db70 as ::core::ffi::c_int as ogg_uint32_t,
		0x48d0c6c7 as ::core::ffi::c_int as ogg_uint32_t,
		0x4593e01e as ::core::ffi::c_int as ogg_uint32_t,
		0x4152fda9 as ::core::ffi::c_int as ogg_uint32_t,
		0x5f15adac as ::core::ffi::c_int as ogg_uint32_t,
		0x5bd4b01b as ::core::ffi::c_int as ogg_uint32_t,
		0x569796c2 as ::core::ffi::c_int as ogg_uint32_t,
		0x52568b75 as ::core::ffi::c_int as ogg_uint32_t,
		0x6a1936c8 as ::core::ffi::c_int as ogg_uint32_t,
		0x6ed82b7f as ::core::ffi::c_int as ogg_uint32_t,
		0x639b0da6 as ::core::ffi::c_int as ogg_uint32_t,
		0x675a1011 as ::core::ffi::c_int as ogg_uint32_t,
		0x791d4014 as ::core::ffi::c_int as ogg_uint32_t,
		0x7ddc5da3 as ::core::ffi::c_int as ogg_uint32_t,
		0x709f7b7a as ::core::ffi::c_int as ogg_uint32_t,
		0x745e66cd as ::core::ffi::c_int as ogg_uint32_t,
		0x9823b6e0 as ::core::ffi::c_uint,
		0x9ce2ab57 as ::core::ffi::c_uint,
		0x91a18d8e as ::core::ffi::c_uint,
		0x95609039 as ::core::ffi::c_uint,
		0x8b27c03c as ::core::ffi::c_uint,
		0x8fe6dd8b as ::core::ffi::c_uint,
		0x82a5fb52 as ::core::ffi::c_uint,
		0x8664e6e5 as ::core::ffi::c_uint,
		0xbe2b5b58 as ::core::ffi::c_uint,
		0xbaea46ef as ::core::ffi::c_uint,
		0xb7a96036 as ::core::ffi::c_uint,
		0xb3687d81 as ::core::ffi::c_uint,
		0xad2f2d84 as ::core::ffi::c_uint,
		0xa9ee3033 as ::core::ffi::c_uint,
		0xa4ad16ea as ::core::ffi::c_uint,
		0xa06c0b5d as ::core::ffi::c_uint,
		0xd4326d90 as ::core::ffi::c_uint,
		0xd0f37027 as ::core::ffi::c_uint,
		0xddb056fe as ::core::ffi::c_uint,
		0xd9714b49 as ::core::ffi::c_uint,
		0xc7361b4c as ::core::ffi::c_uint,
		0xc3f706fb as ::core::ffi::c_uint,
		0xceb42022 as ::core::ffi::c_uint,
		0xca753d95 as ::core::ffi::c_uint,
		0xf23a8028 as ::core::ffi::c_uint,
		0xf6fb9d9f as ::core::ffi::c_uint,
		0xfbb8bb46 as ::core::ffi::c_uint,
		0xff79a6f1 as ::core::ffi::c_uint,
		0xe13ef6f4 as ::core::ffi::c_uint,
		0xe5ffeb43 as ::core::ffi::c_uint,
		0xe8bccd9a as ::core::ffi::c_uint,
		0xec7dd02d as ::core::ffi::c_uint,
		0x34867077 as ::core::ffi::c_int as ogg_uint32_t,
		0x30476dc0 as ::core::ffi::c_int as ogg_uint32_t,
		0x3d044b19 as ::core::ffi::c_int as ogg_uint32_t,
		0x39c556ae as ::core::ffi::c_int as ogg_uint32_t,
		0x278206ab as ::core::ffi::c_int as ogg_uint32_t,
		0x23431b1c as ::core::ffi::c_int as ogg_uint32_t,
		0x2e003dc5 as ::core::ffi::c_int as ogg_uint32_t,
		0x2ac12072 as ::core::ffi::c_int as ogg_uint32_t,
		0x128e9dcf as ::core::ffi::c_int as ogg_uint32_t,
		0x164f8078 as ::core::ffi::c_int as ogg_uint32_t,
		0x1b0ca6a1 as ::core::ffi::c_int as ogg_uint32_t,
		0x1fcdbb16 as ::core::ffi::c_int as ogg_uint32_t,
		0x18aeb13 as ::core::ffi::c_int as ogg_uint32_t,
		0x54bf6a4 as ::core::ffi::c_int as ogg_uint32_t,
		0x808d07d as ::core::ffi::c_int as ogg_uint32_t,
		0xcc9cdca as ::core::ffi::c_int as ogg_uint32_t,
		0x7897ab07 as ::core::ffi::c_int as ogg_uint32_t,
		0x7c56b6b0 as ::core::ffi::c_int as ogg_uint32_t,
		0x71159069 as ::core::ffi::c_int as ogg_uint32_t,
		0x75d48dde as ::core::ffi::c_int as ogg_uint32_t,
		0x6b93dddb as ::core::ffi::c_int as ogg_uint32_t,
		0x6f52c06c as ::core::ffi::c_int as ogg_uint32_t,
		0x6211e6b5 as ::core::ffi::c_int as ogg_uint32_t,
		0x66d0fb02 as ::core::ffi::c_int as ogg_uint32_t,
		0x5e9f46bf as ::core::ffi::c_int as ogg_uint32_t,
		0x5a5e5b08 as ::core::ffi::c_int as ogg_uint32_t,
		0x571d7dd1 as ::core::ffi::c_int as ogg_uint32_t,
		0x53dc6066 as ::core::ffi::c_int as ogg_uint32_t,
		0x4d9b3063 as ::core::ffi::c_int as ogg_uint32_t,
		0x495a2dd4 as ::core::ffi::c_int as ogg_uint32_t,
		0x44190b0d as ::core::ffi::c_int as ogg_uint32_t,
		0x40d816ba as ::core::ffi::c_int as ogg_uint32_t,
		0xaca5c697 as ::core::ffi::c_uint,
		0xa864db20 as ::core::ffi::c_uint,
		0xa527fdf9 as ::core::ffi::c_uint,
		0xa1e6e04e as ::core::ffi::c_uint,
		0xbfa1b04b as ::core::ffi::c_uint,
		0xbb60adfc as ::core::ffi::c_uint,
		0xb6238b25 as ::core::ffi::c_uint,
		0xb2e29692 as ::core::ffi::c_uint,
		0x8aad2b2f as ::core::ffi::c_uint,
		0x8e6c3698 as ::core::ffi::c_uint,
		0x832f1041 as ::core::ffi::c_uint,
		0x87ee0df6 as ::core::ffi::c_uint,
		0x99a95df3 as ::core::ffi::c_uint,
		0x9d684044 as ::core::ffi::c_uint,
		0x902b669d as ::core::ffi::c_uint,
		0x94ea7b2a as ::core::ffi::c_uint,
		0xe0b41de7 as ::core::ffi::c_uint,
		0xe4750050 as ::core::ffi::c_uint,
		0xe9362689 as ::core::ffi::c_uint,
		0xedf73b3e as ::core::ffi::c_uint,
		0xf3b06b3b as ::core::ffi::c_uint,
		0xf771768c as ::core::ffi::c_uint,
		0xfa325055 as ::core::ffi::c_uint,
		0xfef34de2 as ::core::ffi::c_uint,
		0xc6bcf05f as ::core::ffi::c_uint,
		0xc27dede8 as ::core::ffi::c_uint,
		0xcf3ecb31 as ::core::ffi::c_uint,
		0xcbffd686 as ::core::ffi::c_uint,
		0xd5b88683 as ::core::ffi::c_uint,
		0xd1799b34 as ::core::ffi::c_uint,
		0xdc3abded as ::core::ffi::c_uint,
		0xd8fba05a as ::core::ffi::c_uint,
		0x690ce0ee as ::core::ffi::c_int as ogg_uint32_t,
		0x6dcdfd59 as ::core::ffi::c_int as ogg_uint32_t,
		0x608edb80 as ::core::ffi::c_int as ogg_uint32_t,
		0x644fc637 as ::core::ffi::c_int as ogg_uint32_t,
		0x7a089632 as ::core::ffi::c_int as ogg_uint32_t,
		0x7ec98b85 as ::core::ffi::c_int as ogg_uint32_t,
		0x738aad5c as ::core::ffi::c_int as ogg_uint32_t,
		0x774bb0eb as ::core::ffi::c_int as ogg_uint32_t,
		0x4f040d56 as ::core::ffi::c_int as ogg_uint32_t,
		0x4bc510e1 as ::core::ffi::c_int as ogg_uint32_t,
		0x46863638 as ::core::ffi::c_int as ogg_uint32_t,
		0x42472b8f as ::core::ffi::c_int as ogg_uint32_t,
		0x5c007b8a as ::core::ffi::c_int as ogg_uint32_t,
		0x58c1663d as ::core::ffi::c_int as ogg_uint32_t,
		0x558240e4 as ::core::ffi::c_int as ogg_uint32_t,
		0x51435d53 as ::core::ffi::c_int as ogg_uint32_t,
		0x251d3b9e as ::core::ffi::c_int as ogg_uint32_t,
		0x21dc2629 as ::core::ffi::c_int as ogg_uint32_t,
		0x2c9f00f0 as ::core::ffi::c_int as ogg_uint32_t,
		0x285e1d47 as ::core::ffi::c_int as ogg_uint32_t,
		0x36194d42 as ::core::ffi::c_int as ogg_uint32_t,
		0x32d850f5 as ::core::ffi::c_int as ogg_uint32_t,
		0x3f9b762c as ::core::ffi::c_int as ogg_uint32_t,
		0x3b5a6b9b as ::core::ffi::c_int as ogg_uint32_t,
		0x315d626 as ::core::ffi::c_int as ogg_uint32_t,
		0x7d4cb91 as ::core::ffi::c_int as ogg_uint32_t,
		0xa97ed48 as ::core::ffi::c_int as ogg_uint32_t,
		0xe56f0ff as ::core::ffi::c_int as ogg_uint32_t,
		0x1011a0fa as ::core::ffi::c_int as ogg_uint32_t,
		0x14d0bd4d as ::core::ffi::c_int as ogg_uint32_t,
		0x19939b94 as ::core::ffi::c_int as ogg_uint32_t,
		0x1d528623 as ::core::ffi::c_int as ogg_uint32_t,
		0xf12f560e as ::core::ffi::c_uint,
		0xf5ee4bb9 as ::core::ffi::c_uint,
		0xf8ad6d60 as ::core::ffi::c_uint,
		0xfc6c70d7 as ::core::ffi::c_uint,
		0xe22b20d2 as ::core::ffi::c_uint,
		0xe6ea3d65 as ::core::ffi::c_uint,
		0xeba91bbc as ::core::ffi::c_uint,
		0xef68060b as ::core::ffi::c_uint,
		0xd727bbb6 as ::core::ffi::c_uint,
		0xd3e6a601 as ::core::ffi::c_uint,
		0xdea580d8 as ::core::ffi::c_uint,
		0xda649d6f as ::core::ffi::c_uint,
		0xc423cd6a as ::core::ffi::c_uint,
		0xc0e2d0dd as ::core::ffi::c_uint,
		0xcda1f604 as ::core::ffi::c_uint,
		0xc960ebb3 as ::core::ffi::c_uint,
		0xbd3e8d7e as ::core::ffi::c_uint,
		0xb9ff90c9 as ::core::ffi::c_uint,
		0xb4bcb610 as ::core::ffi::c_uint,
		0xb07daba7 as ::core::ffi::c_uint,
		0xae3afba2 as ::core::ffi::c_uint,
		0xaafbe615 as ::core::ffi::c_uint,
		0xa7b8c0cc as ::core::ffi::c_uint,
		0xa379dd7b as ::core::ffi::c_uint,
		0x9b3660c6 as ::core::ffi::c_uint,
		0x9ff77d71 as ::core::ffi::c_uint,
		0x92b45ba8 as ::core::ffi::c_uint,
		0x9675461f as ::core::ffi::c_uint,
		0x8832161a as ::core::ffi::c_uint,
		0x8cf30bad as ::core::ffi::c_uint,
		0x81b02d74 as ::core::ffi::c_uint,
		0x857130c3 as ::core::ffi::c_uint,
		0x5d8a9099 as ::core::ffi::c_int as ogg_uint32_t,
		0x594b8d2e as ::core::ffi::c_int as ogg_uint32_t,
		0x5408abf7 as ::core::ffi::c_int as ogg_uint32_t,
		0x50c9b640 as ::core::ffi::c_int as ogg_uint32_t,
		0x4e8ee645 as ::core::ffi::c_int as ogg_uint32_t,
		0x4a4ffbf2 as ::core::ffi::c_int as ogg_uint32_t,
		0x470cdd2b as ::core::ffi::c_int as ogg_uint32_t,
		0x43cdc09c as ::core::ffi::c_int as ogg_uint32_t,
		0x7b827d21 as ::core::ffi::c_int as ogg_uint32_t,
		0x7f436096 as ::core::ffi::c_int as ogg_uint32_t,
		0x7200464f as ::core::ffi::c_int as ogg_uint32_t,
		0x76c15bf8 as ::core::ffi::c_int as ogg_uint32_t,
		0x68860bfd as ::core::ffi::c_int as ogg_uint32_t,
		0x6c47164a as ::core::ffi::c_int as ogg_uint32_t,
		0x61043093 as ::core::ffi::c_int as ogg_uint32_t,
		0x65c52d24 as ::core::ffi::c_int as ogg_uint32_t,
		0x119b4be9 as ::core::ffi::c_int as ogg_uint32_t,
		0x155a565e as ::core::ffi::c_int as ogg_uint32_t,
		0x18197087 as ::core::ffi::c_int as ogg_uint32_t,
		0x1cd86d30 as ::core::ffi::c_int as ogg_uint32_t,
		0x29f3d35 as ::core::ffi::c_int as ogg_uint32_t,
		0x65e2082 as ::core::ffi::c_int as ogg_uint32_t,
		0xb1d065b as ::core::ffi::c_int as ogg_uint32_t,
		0xfdc1bec as ::core::ffi::c_int as ogg_uint32_t,
		0x3793a651 as ::core::ffi::c_int as ogg_uint32_t,
		0x3352bbe6 as ::core::ffi::c_int as ogg_uint32_t,
		0x3e119d3f as ::core::ffi::c_int as ogg_uint32_t,
		0x3ad08088 as ::core::ffi::c_int as ogg_uint32_t,
		0x2497d08d as ::core::ffi::c_int as ogg_uint32_t,
		0x2056cd3a as ::core::ffi::c_int as ogg_uint32_t,
		0x2d15ebe3 as ::core::ffi::c_int as ogg_uint32_t,
		0x29d4f654 as ::core::ffi::c_int as ogg_uint32_t,
		0xc5a92679 as ::core::ffi::c_uint,
		0xc1683bce as ::core::ffi::c_uint,
		0xcc2b1d17 as ::core::ffi::c_uint,
		0xc8ea00a0 as ::core::ffi::c_uint,
		0xd6ad50a5 as ::core::ffi::c_uint,
		0xd26c4d12 as ::core::ffi::c_uint,
		0xdf2f6bcb as ::core::ffi::c_uint,
		0xdbee767c as ::core::ffi::c_uint,
		0xe3a1cbc1 as ::core::ffi::c_uint,
		0xe760d676 as ::core::ffi::c_uint,
		0xea23f0af as ::core::ffi::c_uint,
		0xeee2ed18 as ::core::ffi::c_uint,
		0xf0a5bd1d as ::core::ffi::c_uint,
		0xf464a0aa as ::core::ffi::c_uint,
		0xf9278673 as ::core::ffi::c_uint,
		0xfde69bc4 as ::core::ffi::c_uint,
		0x89b8fd09 as ::core::ffi::c_uint,
		0x8d79e0be as ::core::ffi::c_uint,
		0x803ac667 as ::core::ffi::c_uint,
		0x84fbdbd0 as ::core::ffi::c_uint,
		0x9abc8bd5 as ::core::ffi::c_uint,
		0x9e7d9662 as ::core::ffi::c_uint,
		0x933eb0bb as ::core::ffi::c_uint,
		0x97ffad0c as ::core::ffi::c_uint,
		0xafb010b1 as ::core::ffi::c_uint,
		0xab710d06 as ::core::ffi::c_uint,
		0xa6322bdf as ::core::ffi::c_uint,
		0xa2f33668 as ::core::ffi::c_uint,
		0xbcb4666d as ::core::ffi::c_uint,
		0xb8757bda as ::core::ffi::c_uint,
		0xb5365d03 as ::core::ffi::c_uint,
		0xb1f740b4 as ::core::ffi::c_uint
	],
	[
		0 as ::core::ffi::c_int as ogg_uint32_t,
		0xd219c1dc as ::core::ffi::c_uint,
		0xa0f29e0f as ::core::ffi::c_uint,
		0x72eb5fd3 as ::core::ffi::c_int as ogg_uint32_t,
		0x452421a9 as ::core::ffi::c_int as ogg_uint32_t,
		0x973de075 as ::core::ffi::c_uint,
		0xe5d6bfa6 as ::core::ffi::c_uint,
		0x37cf7e7a as ::core::ffi::c_int as ogg_uint32_t,
		0x8a484352 as ::core::ffi::c_uint,
		0x5851828e as ::core::ffi::c_int as ogg_uint32_t,
		0x2abadd5d as ::core::ffi::c_int as ogg_uint32_t,
		0xf8a31c81 as ::core::ffi::c_uint,
		0xcf6c62fb as ::core::ffi::c_uint,
		0x1d75a327 as ::core::ffi::c_int as ogg_uint32_t,
		0x6f9efcf4 as ::core::ffi::c_int as ogg_uint32_t,
		0xbd873d28 as ::core::ffi::c_uint,
		0x10519b13 as ::core::ffi::c_int as ogg_uint32_t,
		0xc2485acf as ::core::ffi::c_uint,
		0xb0a3051c as ::core::ffi::c_uint,
		0x62bac4c0 as ::core::ffi::c_int as ogg_uint32_t,
		0x5575baba as ::core::ffi::c_int as ogg_uint32_t,
		0x876c7b66 as ::core::ffi::c_uint,
		0xf58724b5 as ::core::ffi::c_uint,
		0x279ee569 as ::core::ffi::c_int as ogg_uint32_t,
		0x9a19d841 as ::core::ffi::c_uint,
		0x4800199d as ::core::ffi::c_int as ogg_uint32_t,
		0x3aeb464e as ::core::ffi::c_int as ogg_uint32_t,
		0xe8f28792 as ::core::ffi::c_uint,
		0xdf3df9e8 as ::core::ffi::c_uint,
		0xd243834 as ::core::ffi::c_int as ogg_uint32_t,
		0x7fcf67e7 as ::core::ffi::c_int as ogg_uint32_t,
		0xadd6a63b as ::core::ffi::c_uint,
		0x20a33626 as ::core::ffi::c_int as ogg_uint32_t,
		0xf2baf7fa as ::core::ffi::c_uint,
		0x8051a829 as ::core::ffi::c_uint,
		0x524869f5 as ::core::ffi::c_int as ogg_uint32_t,
		0x6587178f as ::core::ffi::c_int as ogg_uint32_t,
		0xb79ed653 as ::core::ffi::c_uint,
		0xc5758980 as ::core::ffi::c_uint,
		0x176c485c as ::core::ffi::c_int as ogg_uint32_t,
		0xaaeb7574 as ::core::ffi::c_uint,
		0x78f2b4a8 as ::core::ffi::c_int as ogg_uint32_t,
		0xa19eb7b as ::core::ffi::c_int as ogg_uint32_t,
		0xd8002aa7 as ::core::ffi::c_uint,
		0xefcf54dd as ::core::ffi::c_uint,
		0x3dd69501 as ::core::ffi::c_int as ogg_uint32_t,
		0x4f3dcad2 as ::core::ffi::c_int as ogg_uint32_t,
		0x9d240b0e as ::core::ffi::c_uint,
		0x30f2ad35 as ::core::ffi::c_int as ogg_uint32_t,
		0xe2eb6ce9 as ::core::ffi::c_uint,
		0x9000333a as ::core::ffi::c_uint,
		0x4219f2e6 as ::core::ffi::c_int as ogg_uint32_t,
		0x75d68c9c as ::core::ffi::c_int as ogg_uint32_t,
		0xa7cf4d40 as ::core::ffi::c_uint,
		0xd5241293 as ::core::ffi::c_uint,
		0x73dd34f as ::core::ffi::c_int as ogg_uint32_t,
		0xbabaee67 as ::core::ffi::c_uint,
		0x68a32fbb as ::core::ffi::c_int as ogg_uint32_t,
		0x1a487068 as ::core::ffi::c_int as ogg_uint32_t,
		0xc851b1b4 as ::core::ffi::c_uint,
		0xff9ecfce as ::core::ffi::c_uint,
		0x2d870e12 as ::core::ffi::c_int as ogg_uint32_t,
		0x5f6c51c1 as ::core::ffi::c_int as ogg_uint32_t,
		0x8d75901d as ::core::ffi::c_uint,
		0x41466c4c as ::core::ffi::c_int as ogg_uint32_t,
		0x935fad90 as ::core::ffi::c_uint,
		0xe1b4f243 as ::core::ffi::c_uint,
		0x33ad339f as ::core::ffi::c_int as ogg_uint32_t,
		0x4624de5 as ::core::ffi::c_int as ogg_uint32_t,
		0xd67b8c39 as ::core::ffi::c_uint,
		0xa490d3ea as ::core::ffi::c_uint,
		0x76891236 as ::core::ffi::c_int as ogg_uint32_t,
		0xcb0e2f1e as ::core::ffi::c_uint,
		0x1917eec2 as ::core::ffi::c_int as ogg_uint32_t,
		0x6bfcb111 as ::core::ffi::c_int as ogg_uint32_t,
		0xb9e570cd as ::core::ffi::c_uint,
		0x8e2a0eb7 as ::core::ffi::c_uint,
		0x5c33cf6b as ::core::ffi::c_int as ogg_uint32_t,
		0x2ed890b8 as ::core::ffi::c_int as ogg_uint32_t,
		0xfcc15164 as ::core::ffi::c_uint,
		0x5117f75f as ::core::ffi::c_int as ogg_uint32_t,
		0x830e3683 as ::core::ffi::c_uint,
		0xf1e56950 as ::core::ffi::c_uint,
		0x23fca88c as ::core::ffi::c_int as ogg_uint32_t,
		0x1433d6f6 as ::core::ffi::c_int as ogg_uint32_t,
		0xc62a172a as ::core::ffi::c_uint,
		0xb4c148f9 as ::core::ffi::c_uint,
		0x66d88925 as ::core::ffi::c_int as ogg_uint32_t,
		0xdb5fb40d as ::core::ffi::c_uint,
		0x94675d1 as ::core::ffi::c_int as ogg_uint32_t,
		0x7bad2a02 as ::core::ffi::c_int as ogg_uint32_t,
		0xa9b4ebde as ::core::ffi::c_uint,
		0x9e7b95a4 as ::core::ffi::c_uint,
		0x4c625478 as ::core::ffi::c_int as ogg_uint32_t,
		0x3e890bab as ::core::ffi::c_int as ogg_uint32_t,
		0xec90ca77 as ::core::ffi::c_uint,
		0x61e55a6a as ::core::ffi::c_int as ogg_uint32_t,
		0xb3fc9bb6 as ::core::ffi::c_uint,
		0xc117c465 as ::core::ffi::c_uint,
		0x130e05b9 as ::core::ffi::c_int as ogg_uint32_t,
		0x24c17bc3 as ::core::ffi::c_int as ogg_uint32_t,
		0xf6d8ba1f as ::core::ffi::c_uint,
		0x8433e5cc as ::core::ffi::c_uint,
		0x562a2410 as ::core::ffi::c_int as ogg_uint32_t,
		0xebad1938 as ::core::ffi::c_uint,
		0x39b4d8e4 as ::core::ffi::c_int as ogg_uint32_t,
		0x4b5f8737 as ::core::ffi::c_int as ogg_uint32_t,
		0x994646eb as ::core::ffi::c_uint,
		0xae893891 as ::core::ffi::c_uint,
		0x7c90f94d as ::core::ffi::c_int as ogg_uint32_t,
		0xe7ba69e as ::core::ffi::c_int as ogg_uint32_t,
		0xdc626742 as ::core::ffi::c_uint,
		0x71b4c179 as ::core::ffi::c_int as ogg_uint32_t,
		0xa3ad00a5 as ::core::ffi::c_uint,
		0xd1465f76 as ::core::ffi::c_uint,
		0x35f9eaa as ::core::ffi::c_int as ogg_uint32_t,
		0x3490e0d0 as ::core::ffi::c_int as ogg_uint32_t,
		0xe689210c as ::core::ffi::c_uint,
		0x94627edf as ::core::ffi::c_uint,
		0x467bbf03 as ::core::ffi::c_int as ogg_uint32_t,
		0xfbfc822b as ::core::ffi::c_uint,
		0x29e543f7 as ::core::ffi::c_int as ogg_uint32_t,
		0x5b0e1c24 as ::core::ffi::c_int as ogg_uint32_t,
		0x8917ddf8 as ::core::ffi::c_uint,
		0xbed8a382 as ::core::ffi::c_uint,
		0x6cc1625e as ::core::ffi::c_int as ogg_uint32_t,
		0x1e2a3d8d as ::core::ffi::c_int as ogg_uint32_t,
		0xcc33fc51 as ::core::ffi::c_uint,
		0x828cd898 as ::core::ffi::c_uint,
		0x50951944 as ::core::ffi::c_int as ogg_uint32_t,
		0x227e4697 as ::core::ffi::c_int as ogg_uint32_t,
		0xf067874b as ::core::ffi::c_uint,
		0xc7a8f931 as ::core::ffi::c_uint,
		0x15b138ed as ::core::ffi::c_int as ogg_uint32_t,
		0x675a673e as ::core::ffi::c_int as ogg_uint32_t,
		0xb543a6e2 as ::core::ffi::c_uint,
		0x8c49bca as ::core::ffi::c_int as ogg_uint32_t,
		0xdadd5a16 as ::core::ffi::c_uint,
		0xa83605c5 as ::core::ffi::c_uint,
		0x7a2fc419 as ::core::ffi::c_int as ogg_uint32_t,
		0x4de0ba63 as ::core::ffi::c_int as ogg_uint32_t,
		0x9ff97bbf as ::core::ffi::c_uint,
		0xed12246c as ::core::ffi::c_uint,
		0x3f0be5b0 as ::core::ffi::c_int as ogg_uint32_t,
		0x92dd438b as ::core::ffi::c_uint,
		0x40c48257 as ::core::ffi::c_int as ogg_uint32_t,
		0x322fdd84 as ::core::ffi::c_int as ogg_uint32_t,
		0xe0361c58 as ::core::ffi::c_uint,
		0xd7f96222 as ::core::ffi::c_uint,
		0x5e0a3fe as ::core::ffi::c_int as ogg_uint32_t,
		0x770bfc2d as ::core::ffi::c_int as ogg_uint32_t,
		0xa5123df1 as ::core::ffi::c_uint,
		0x189500d9 as ::core::ffi::c_int as ogg_uint32_t,
		0xca8cc105 as ::core::ffi::c_uint,
		0xb8679ed6 as ::core::ffi::c_uint,
		0x6a7e5f0a as ::core::ffi::c_int as ogg_uint32_t,
		0x5db12170 as ::core::ffi::c_int as ogg_uint32_t,
		0x8fa8e0ac as ::core::ffi::c_uint,
		0xfd43bf7f as ::core::ffi::c_uint,
		0x2f5a7ea3 as ::core::ffi::c_int as ogg_uint32_t,
		0xa22feebe as ::core::ffi::c_uint,
		0x70362f62 as ::core::ffi::c_int as ogg_uint32_t,
		0x2dd70b1 as ::core::ffi::c_int as ogg_uint32_t,
		0xd0c4b16d as ::core::ffi::c_uint,
		0xe70bcf17 as ::core::ffi::c_uint,
		0x35120ecb as ::core::ffi::c_int as ogg_uint32_t,
		0x47f95118 as ::core::ffi::c_int as ogg_uint32_t,
		0x95e090c4 as ::core::ffi::c_uint,
		0x2867adec as ::core::ffi::c_int as ogg_uint32_t,
		0xfa7e6c30 as ::core::ffi::c_uint,
		0x889533e3 as ::core::ffi::c_uint,
		0x5a8cf23f as ::core::ffi::c_int as ogg_uint32_t,
		0x6d438c45 as ::core::ffi::c_int as ogg_uint32_t,
		0xbf5a4d99 as ::core::ffi::c_uint,
		0xcdb1124a as ::core::ffi::c_uint,
		0x1fa8d396 as ::core::ffi::c_int as ogg_uint32_t,
		0xb27e75ad as ::core::ffi::c_uint,
		0x6067b471 as ::core::ffi::c_int as ogg_uint32_t,
		0x128ceba2 as ::core::ffi::c_int as ogg_uint32_t,
		0xc0952a7e as ::core::ffi::c_uint,
		0xf75a5404 as ::core::ffi::c_uint,
		0x254395d8 as ::core::ffi::c_int as ogg_uint32_t,
		0x57a8ca0b as ::core::ffi::c_int as ogg_uint32_t,
		0x85b10bd7 as ::core::ffi::c_uint,
		0x383636ff as ::core::ffi::c_int as ogg_uint32_t,
		0xea2ff723 as ::core::ffi::c_uint,
		0x98c4a8f0 as ::core::ffi::c_uint,
		0x4add692c as ::core::ffi::c_int as ogg_uint32_t,
		0x7d121756 as ::core::ffi::c_int as ogg_uint32_t,
		0xaf0bd68a as ::core::ffi::c_uint,
		0xdde08959 as ::core::ffi::c_uint,
		0xff94885 as ::core::ffi::c_int as ogg_uint32_t,
		0xc3cab4d4 as ::core::ffi::c_uint,
		0x11d37508 as ::core::ffi::c_int as ogg_uint32_t,
		0x63382adb as ::core::ffi::c_int as ogg_uint32_t,
		0xb121eb07 as ::core::ffi::c_uint,
		0x86ee957d as ::core::ffi::c_uint,
		0x54f754a1 as ::core::ffi::c_int as ogg_uint32_t,
		0x261c0b72 as ::core::ffi::c_int as ogg_uint32_t,
		0xf405caae as ::core::ffi::c_uint,
		0x4982f786 as ::core::ffi::c_int as ogg_uint32_t,
		0x9b9b365a as ::core::ffi::c_uint,
		0xe9706989 as ::core::ffi::c_uint,
		0x3b69a855 as ::core::ffi::c_int as ogg_uint32_t,
		0xca6d62f as ::core::ffi::c_int as ogg_uint32_t,
		0xdebf17f3 as ::core::ffi::c_uint,
		0xac544820 as ::core::ffi::c_uint,
		0x7e4d89fc as ::core::ffi::c_int as ogg_uint32_t,
		0xd39b2fc7 as ::core::ffi::c_uint,
		0x182ee1b as ::core::ffi::c_int as ogg_uint32_t,
		0x7369b1c8 as ::core::ffi::c_int as ogg_uint32_t,
		0xa1707014 as ::core::ffi::c_uint,
		0x96bf0e6e as ::core::ffi::c_uint,
		0x44a6cfb2 as ::core::ffi::c_int as ogg_uint32_t,
		0x364d9061 as ::core::ffi::c_int as ogg_uint32_t,
		0xe45451bd as ::core::ffi::c_uint,
		0x59d36c95 as ::core::ffi::c_int as ogg_uint32_t,
		0x8bcaad49 as ::core::ffi::c_uint,
		0xf921f29a as ::core::ffi::c_uint,
		0x2b383346 as ::core::ffi::c_int as ogg_uint32_t,
		0x1cf74d3c as ::core::ffi::c_int as ogg_uint32_t,
		0xceee8ce0 as ::core::ffi::c_uint,
		0xbc05d333 as ::core::ffi::c_uint,
		0x6e1c12ef as ::core::ffi::c_int as ogg_uint32_t,
		0xe36982f2 as ::core::ffi::c_uint,
		0x3170432e as ::core::ffi::c_int as ogg_uint32_t,
		0x439b1cfd as ::core::ffi::c_int as ogg_uint32_t,
		0x9182dd21 as ::core::ffi::c_uint,
		0xa64da35b as ::core::ffi::c_uint,
		0x74546287 as ::core::ffi::c_int as ogg_uint32_t,
		0x6bf3d54 as ::core::ffi::c_int as ogg_uint32_t,
		0xd4a6fc88 as ::core::ffi::c_uint,
		0x6921c1a0 as ::core::ffi::c_int as ogg_uint32_t,
		0xbb38007c as ::core::ffi::c_uint,
		0xc9d35faf as ::core::ffi::c_uint,
		0x1bca9e73 as ::core::ffi::c_int as ogg_uint32_t,
		0x2c05e009 as ::core::ffi::c_int as ogg_uint32_t,
		0xfe1c21d5 as ::core::ffi::c_uint,
		0x8cf77e06 as ::core::ffi::c_uint,
		0x5eeebfda as ::core::ffi::c_int as ogg_uint32_t,
		0xf33819e1 as ::core::ffi::c_uint,
		0x2121d83d as ::core::ffi::c_int as ogg_uint32_t,
		0x53ca87ee as ::core::ffi::c_int as ogg_uint32_t,
		0x81d34632 as ::core::ffi::c_uint,
		0xb61c3848 as ::core::ffi::c_uint,
		0x6405f994 as ::core::ffi::c_int as ogg_uint32_t,
		0x16eea647 as ::core::ffi::c_int as ogg_uint32_t,
		0xc4f7679b as ::core::ffi::c_uint,
		0x79705ab3 as ::core::ffi::c_int as ogg_uint32_t,
		0xab699b6f as ::core::ffi::c_uint,
		0xd982c4bc as ::core::ffi::c_uint,
		0xb9b0560 as ::core::ffi::c_int as ogg_uint32_t,
		0x3c547b1a as ::core::ffi::c_int as ogg_uint32_t,
		0xee4dbac6 as ::core::ffi::c_uint,
		0x9ca6e515 as ::core::ffi::c_uint,
		0x4ebf24c9 as ::core::ffi::c_int as ogg_uint32_t
	],
	[
		0 as ::core::ffi::c_int as ogg_uint32_t,
		0x1d8ac87 as ::core::ffi::c_int as ogg_uint32_t,
		0x3b1590e as ::core::ffi::c_int as ogg_uint32_t,
		0x269f589 as ::core::ffi::c_int as ogg_uint32_t,
		0x762b21c as ::core::ffi::c_int as ogg_uint32_t,
		0x6ba1e9b as ::core::ffi::c_int as ogg_uint32_t,
		0x4d3eb12 as ::core::ffi::c_int as ogg_uint32_t,
		0x50b4795 as ::core::ffi::c_int as ogg_uint32_t,
		0xec56438 as ::core::ffi::c_int as ogg_uint32_t,
		0xf1dc8bf as ::core::ffi::c_int as ogg_uint32_t,
		0xd743d36 as ::core::ffi::c_int as ogg_uint32_t,
		0xcac91b1 as ::core::ffi::c_int as ogg_uint32_t,
		0x9a7d624 as ::core::ffi::c_int as ogg_uint32_t,
		0x87f7aa3 as ::core::ffi::c_int as ogg_uint32_t,
		0xa168f2a as ::core::ffi::c_int as ogg_uint32_t,
		0xbce23ad as ::core::ffi::c_int as ogg_uint32_t,
		0x1d8ac870 as ::core::ffi::c_int as ogg_uint32_t,
		0x1c5264f7 as ::core::ffi::c_int as ogg_uint32_t,
		0x1e3b917e as ::core::ffi::c_int as ogg_uint32_t,
		0x1fe33df9 as ::core::ffi::c_int as ogg_uint32_t,
		0x1ae87a6c as ::core::ffi::c_int as ogg_uint32_t,
		0x1b30d6eb as ::core::ffi::c_int as ogg_uint32_t,
		0x19592362 as ::core::ffi::c_int as ogg_uint32_t,
		0x18818fe5 as ::core::ffi::c_int as ogg_uint32_t,
		0x134fac48 as ::core::ffi::c_int as ogg_uint32_t,
		0x129700cf as ::core::ffi::c_int as ogg_uint32_t,
		0x10fef546 as ::core::ffi::c_int as ogg_uint32_t,
		0x112659c1 as ::core::ffi::c_int as ogg_uint32_t,
		0x142d1e54 as ::core::ffi::c_int as ogg_uint32_t,
		0x15f5b2d3 as ::core::ffi::c_int as ogg_uint32_t,
		0x179c475a as ::core::ffi::c_int as ogg_uint32_t,
		0x1644ebdd as ::core::ffi::c_int as ogg_uint32_t,
		0x3b1590e0 as ::core::ffi::c_int as ogg_uint32_t,
		0x3acd3c67 as ::core::ffi::c_int as ogg_uint32_t,
		0x38a4c9ee as ::core::ffi::c_int as ogg_uint32_t,
		0x397c6569 as ::core::ffi::c_int as ogg_uint32_t,
		0x3c7722fc as ::core::ffi::c_int as ogg_uint32_t,
		0x3daf8e7b as ::core::ffi::c_int as ogg_uint32_t,
		0x3fc67bf2 as ::core::ffi::c_int as ogg_uint32_t,
		0x3e1ed775 as ::core::ffi::c_int as ogg_uint32_t,
		0x35d0f4d8 as ::core::ffi::c_int as ogg_uint32_t,
		0x3408585f as ::core::ffi::c_int as ogg_uint32_t,
		0x3661add6 as ::core::ffi::c_int as ogg_uint32_t,
		0x37b90151 as ::core::ffi::c_int as ogg_uint32_t,
		0x32b246c4 as ::core::ffi::c_int as ogg_uint32_t,
		0x336aea43 as ::core::ffi::c_int as ogg_uint32_t,
		0x31031fca as ::core::ffi::c_int as ogg_uint32_t,
		0x30dbb34d as ::core::ffi::c_int as ogg_uint32_t,
		0x269f5890 as ::core::ffi::c_int as ogg_uint32_t,
		0x2747f417 as ::core::ffi::c_int as ogg_uint32_t,
		0x252e019e as ::core::ffi::c_int as ogg_uint32_t,
		0x24f6ad19 as ::core::ffi::c_int as ogg_uint32_t,
		0x21fdea8c as ::core::ffi::c_int as ogg_uint32_t,
		0x2025460b as ::core::ffi::c_int as ogg_uint32_t,
		0x224cb382 as ::core::ffi::c_int as ogg_uint32_t,
		0x23941f05 as ::core::ffi::c_int as ogg_uint32_t,
		0x285a3ca8 as ::core::ffi::c_int as ogg_uint32_t,
		0x2982902f as ::core::ffi::c_int as ogg_uint32_t,
		0x2beb65a6 as ::core::ffi::c_int as ogg_uint32_t,
		0x2a33c921 as ::core::ffi::c_int as ogg_uint32_t,
		0x2f388eb4 as ::core::ffi::c_int as ogg_uint32_t,
		0x2ee02233 as ::core::ffi::c_int as ogg_uint32_t,
		0x2c89d7ba as ::core::ffi::c_int as ogg_uint32_t,
		0x2d517b3d as ::core::ffi::c_int as ogg_uint32_t,
		0x762b21c0 as ::core::ffi::c_int as ogg_uint32_t,
		0x77f38d47 as ::core::ffi::c_int as ogg_uint32_t,
		0x759a78ce as ::core::ffi::c_int as ogg_uint32_t,
		0x7442d449 as ::core::ffi::c_int as ogg_uint32_t,
		0x714993dc as ::core::ffi::c_int as ogg_uint32_t,
		0x70913f5b as ::core::ffi::c_int as ogg_uint32_t,
		0x72f8cad2 as ::core::ffi::c_int as ogg_uint32_t,
		0x73206655 as ::core::ffi::c_int as ogg_uint32_t,
		0x78ee45f8 as ::core::ffi::c_int as ogg_uint32_t,
		0x7936e97f as ::core::ffi::c_int as ogg_uint32_t,
		0x7b5f1cf6 as ::core::ffi::c_int as ogg_uint32_t,
		0x7a87b071 as ::core::ffi::c_int as ogg_uint32_t,
		0x7f8cf7e4 as ::core::ffi::c_int as ogg_uint32_t,
		0x7e545b63 as ::core::ffi::c_int as ogg_uint32_t,
		0x7c3daeea as ::core::ffi::c_int as ogg_uint32_t,
		0x7de5026d as ::core::ffi::c_int as ogg_uint32_t,
		0x6ba1e9b0 as ::core::ffi::c_int as ogg_uint32_t,
		0x6a794537 as ::core::ffi::c_int as ogg_uint32_t,
		0x6810b0be as ::core::ffi::c_int as ogg_uint32_t,
		0x69c81c39 as ::core::ffi::c_int as ogg_uint32_t,
		0x6cc35bac as ::core::ffi::c_int as ogg_uint32_t,
		0x6d1bf72b as ::core::ffi::c_int as ogg_uint32_t,
		0x6f7202a2 as ::core::ffi::c_int as ogg_uint32_t,
		0x6eaaae25 as ::core::ffi::c_int as ogg_uint32_t,
		0x65648d88 as ::core::ffi::c_int as ogg_uint32_t,
		0x64bc210f as ::core::ffi::c_int as ogg_uint32_t,
		0x66d5d486 as ::core::ffi::c_int as ogg_uint32_t,
		0x670d7801 as ::core::ffi::c_int as ogg_uint32_t,
		0x62063f94 as ::core::ffi::c_int as ogg_uint32_t,
		0x63de9313 as ::core::ffi::c_int as ogg_uint32_t,
		0x61b7669a as ::core::ffi::c_int as ogg_uint32_t,
		0x606fca1d as ::core::ffi::c_int as ogg_uint32_t,
		0x4d3eb120 as ::core::ffi::c_int as ogg_uint32_t,
		0x4ce61da7 as ::core::ffi::c_int as ogg_uint32_t,
		0x4e8fe82e as ::core::ffi::c_int as ogg_uint32_t,
		0x4f5744a9 as ::core::ffi::c_int as ogg_uint32_t,
		0x4a5c033c as ::core::ffi::c_int as ogg_uint32_t,
		0x4b84afbb as ::core::ffi::c_int as ogg_uint32_t,
		0x49ed5a32 as ::core::ffi::c_int as ogg_uint32_t,
		0x4835f6b5 as ::core::ffi::c_int as ogg_uint32_t,
		0x43fbd518 as ::core::ffi::c_int as ogg_uint32_t,
		0x4223799f as ::core::ffi::c_int as ogg_uint32_t,
		0x404a8c16 as ::core::ffi::c_int as ogg_uint32_t,
		0x41922091 as ::core::ffi::c_int as ogg_uint32_t,
		0x44996704 as ::core::ffi::c_int as ogg_uint32_t,
		0x4541cb83 as ::core::ffi::c_int as ogg_uint32_t,
		0x47283e0a as ::core::ffi::c_int as ogg_uint32_t,
		0x46f0928d as ::core::ffi::c_int as ogg_uint32_t,
		0x50b47950 as ::core::ffi::c_int as ogg_uint32_t,
		0x516cd5d7 as ::core::ffi::c_int as ogg_uint32_t,
		0x5305205e as ::core::ffi::c_int as ogg_uint32_t,
		0x52dd8cd9 as ::core::ffi::c_int as ogg_uint32_t,
		0x57d6cb4c as ::core::ffi::c_int as ogg_uint32_t,
		0x560e67cb as ::core::ffi::c_int as ogg_uint32_t,
		0x54679242 as ::core::ffi::c_int as ogg_uint32_t,
		0x55bf3ec5 as ::core::ffi::c_int as ogg_uint32_t,
		0x5e711d68 as ::core::ffi::c_int as ogg_uint32_t,
		0x5fa9b1ef as ::core::ffi::c_int as ogg_uint32_t,
		0x5dc04466 as ::core::ffi::c_int as ogg_uint32_t,
		0x5c18e8e1 as ::core::ffi::c_int as ogg_uint32_t,
		0x5913af74 as ::core::ffi::c_int as ogg_uint32_t,
		0x58cb03f3 as ::core::ffi::c_int as ogg_uint32_t,
		0x5aa2f67a as ::core::ffi::c_int as ogg_uint32_t,
		0x5b7a5afd as ::core::ffi::c_int as ogg_uint32_t,
		0xec564380 as ::core::ffi::c_uint,
		0xed8eef07 as ::core::ffi::c_uint,
		0xefe71a8e as ::core::ffi::c_uint,
		0xee3fb609 as ::core::ffi::c_uint,
		0xeb34f19c as ::core::ffi::c_uint,
		0xeaec5d1b as ::core::ffi::c_uint,
		0xe885a892 as ::core::ffi::c_uint,
		0xe95d0415 as ::core::ffi::c_uint,
		0xe29327b8 as ::core::ffi::c_uint,
		0xe34b8b3f as ::core::ffi::c_uint,
		0xe1227eb6 as ::core::ffi::c_uint,
		0xe0fad231 as ::core::ffi::c_uint,
		0xe5f195a4 as ::core::ffi::c_uint,
		0xe4293923 as ::core::ffi::c_uint,
		0xe640ccaa as ::core::ffi::c_uint,
		0xe798602d as ::core::ffi::c_uint,
		0xf1dc8bf0 as ::core::ffi::c_uint,
		0xf0042777 as ::core::ffi::c_uint,
		0xf26dd2fe as ::core::ffi::c_uint,
		0xf3b57e79 as ::core::ffi::c_uint,
		0xf6be39ec as ::core::ffi::c_uint,
		0xf766956b as ::core::ffi::c_uint,
		0xf50f60e2 as ::core::ffi::c_uint,
		0xf4d7cc65 as ::core::ffi::c_uint,
		0xff19efc8 as ::core::ffi::c_uint,
		0xfec1434f as ::core::ffi::c_uint,
		0xfca8b6c6 as ::core::ffi::c_uint,
		0xfd701a41 as ::core::ffi::c_uint,
		0xf87b5dd4 as ::core::ffi::c_uint,
		0xf9a3f153 as ::core::ffi::c_uint,
		0xfbca04da as ::core::ffi::c_uint,
		0xfa12a85d as ::core::ffi::c_uint,
		0xd743d360 as ::core::ffi::c_uint,
		0xd69b7fe7 as ::core::ffi::c_uint,
		0xd4f28a6e as ::core::ffi::c_uint,
		0xd52a26e9 as ::core::ffi::c_uint,
		0xd021617c as ::core::ffi::c_uint,
		0xd1f9cdfb as ::core::ffi::c_uint,
		0xd3903872 as ::core::ffi::c_uint,
		0xd24894f5 as ::core::ffi::c_uint,
		0xd986b758 as ::core::ffi::c_uint,
		0xd85e1bdf as ::core::ffi::c_uint,
		0xda37ee56 as ::core::ffi::c_uint,
		0xdbef42d1 as ::core::ffi::c_uint,
		0xdee40544 as ::core::ffi::c_uint,
		0xdf3ca9c3 as ::core::ffi::c_uint,
		0xdd555c4a as ::core::ffi::c_uint,
		0xdc8df0cd as ::core::ffi::c_uint,
		0xcac91b10 as ::core::ffi::c_uint,
		0xcb11b797 as ::core::ffi::c_uint,
		0xc978421e as ::core::ffi::c_uint,
		0xc8a0ee99 as ::core::ffi::c_uint,
		0xcdaba90c as ::core::ffi::c_uint,
		0xcc73058b as ::core::ffi::c_uint,
		0xce1af002 as ::core::ffi::c_uint,
		0xcfc25c85 as ::core::ffi::c_uint,
		0xc40c7f28 as ::core::ffi::c_uint,
		0xc5d4d3af as ::core::ffi::c_uint,
		0xc7bd2626 as ::core::ffi::c_uint,
		0xc6658aa1 as ::core::ffi::c_uint,
		0xc36ecd34 as ::core::ffi::c_uint,
		0xc2b661b3 as ::core::ffi::c_uint,
		0xc0df943a as ::core::ffi::c_uint,
		0xc10738bd as ::core::ffi::c_uint,
		0x9a7d6240 as ::core::ffi::c_uint,
		0x9ba5cec7 as ::core::ffi::c_uint,
		0x99cc3b4e as ::core::ffi::c_uint,
		0x981497c9 as ::core::ffi::c_uint,
		0x9d1fd05c as ::core::ffi::c_uint,
		0x9cc77cdb as ::core::ffi::c_uint,
		0x9eae8952 as ::core::ffi::c_uint,
		0x9f7625d5 as ::core::ffi::c_uint,
		0x94b80678 as ::core::ffi::c_uint,
		0x9560aaff as ::core::ffi::c_uint,
		0x97095f76 as ::core::ffi::c_uint,
		0x96d1f3f1 as ::core::ffi::c_uint,
		0x93dab464 as ::core::ffi::c_uint,
		0x920218e3 as ::core::ffi::c_uint,
		0x906bed6a as ::core::ffi::c_uint,
		0x91b341ed as ::core::ffi::c_uint,
		0x87f7aa30 as ::core::ffi::c_uint,
		0x862f06b7 as ::core::ffi::c_uint,
		0x8446f33e as ::core::ffi::c_uint,
		0x859e5fb9 as ::core::ffi::c_uint,
		0x8095182c as ::core::ffi::c_uint,
		0x814db4ab as ::core::ffi::c_uint,
		0x83244122 as ::core::ffi::c_uint,
		0x82fceda5 as ::core::ffi::c_uint,
		0x8932ce08 as ::core::ffi::c_uint,
		0x88ea628f as ::core::ffi::c_uint,
		0x8a839706 as ::core::ffi::c_uint,
		0x8b5b3b81 as ::core::ffi::c_uint,
		0x8e507c14 as ::core::ffi::c_uint,
		0x8f88d093 as ::core::ffi::c_uint,
		0x8de1251a as ::core::ffi::c_uint,
		0x8c39899d as ::core::ffi::c_uint,
		0xa168f2a0 as ::core::ffi::c_uint,
		0xa0b05e27 as ::core::ffi::c_uint,
		0xa2d9abae as ::core::ffi::c_uint,
		0xa3010729 as ::core::ffi::c_uint,
		0xa60a40bc as ::core::ffi::c_uint,
		0xa7d2ec3b as ::core::ffi::c_uint,
		0xa5bb19b2 as ::core::ffi::c_uint,
		0xa463b535 as ::core::ffi::c_uint,
		0xafad9698 as ::core::ffi::c_uint,
		0xae753a1f as ::core::ffi::c_uint,
		0xac1ccf96 as ::core::ffi::c_uint,
		0xadc46311 as ::core::ffi::c_uint,
		0xa8cf2484 as ::core::ffi::c_uint,
		0xa9178803 as ::core::ffi::c_uint,
		0xab7e7d8a as ::core::ffi::c_uint,
		0xaaa6d10d as ::core::ffi::c_uint,
		0xbce23ad0 as ::core::ffi::c_uint,
		0xbd3a9657 as ::core::ffi::c_uint,
		0xbf5363de as ::core::ffi::c_uint,
		0xbe8bcf59 as ::core::ffi::c_uint,
		0xbb8088cc as ::core::ffi::c_uint,
		0xba58244b as ::core::ffi::c_uint,
		0xb831d1c2 as ::core::ffi::c_uint,
		0xb9e97d45 as ::core::ffi::c_uint,
		0xb2275ee8 as ::core::ffi::c_uint,
		0xb3fff26f as ::core::ffi::c_uint,
		0xb19607e6 as ::core::ffi::c_uint,
		0xb04eab61 as ::core::ffi::c_uint,
		0xb545ecf4 as ::core::ffi::c_uint,
		0xb49d4073 as ::core::ffi::c_uint,
		0xb6f4b5fa as ::core::ffi::c_uint,
		0xb72c197d as ::core::ffi::c_uint
	],
	[
		0 as ::core::ffi::c_int as ogg_uint32_t,
		0xdc6d9ab7 as ::core::ffi::c_uint,
		0xbc1a28d9 as ::core::ffi::c_uint,
		0x6077b26e as ::core::ffi::c_int as ogg_uint32_t,
		0x7cf54c05 as ::core::ffi::c_int as ogg_uint32_t,
		0xa098d6b2 as ::core::ffi::c_uint,
		0xc0ef64dc as ::core::ffi::c_uint,
		0x1c82fe6b as ::core::ffi::c_int as ogg_uint32_t,
		0xf9ea980a as ::core::ffi::c_uint,
		0x258702bd as ::core::ffi::c_int as ogg_uint32_t,
		0x45f0b0d3 as ::core::ffi::c_int as ogg_uint32_t,
		0x999d2a64 as ::core::ffi::c_uint,
		0x851fd40f as ::core::ffi::c_uint,
		0x59724eb8 as ::core::ffi::c_int as ogg_uint32_t,
		0x3905fcd6 as ::core::ffi::c_int as ogg_uint32_t,
		0xe5686661 as ::core::ffi::c_uint,
		0xf7142da3 as ::core::ffi::c_uint,
		0x2b79b714 as ::core::ffi::c_int as ogg_uint32_t,
		0x4b0e057a as ::core::ffi::c_int as ogg_uint32_t,
		0x97639fcd as ::core::ffi::c_uint,
		0x8be161a6 as ::core::ffi::c_uint,
		0x578cfb11 as ::core::ffi::c_int as ogg_uint32_t,
		0x37fb497f as ::core::ffi::c_int as ogg_uint32_t,
		0xeb96d3c8 as ::core::ffi::c_uint,
		0xefeb5a9 as ::core::ffi::c_int as ogg_uint32_t,
		0xd2932f1e as ::core::ffi::c_uint,
		0xb2e49d70 as ::core::ffi::c_uint,
		0x6e8907c7 as ::core::ffi::c_int as ogg_uint32_t,
		0x720bf9ac as ::core::ffi::c_int as ogg_uint32_t,
		0xae66631b as ::core::ffi::c_uint,
		0xce11d175 as ::core::ffi::c_uint,
		0x127c4bc2 as ::core::ffi::c_int as ogg_uint32_t,
		0xeae946f1 as ::core::ffi::c_uint,
		0x3684dc46 as ::core::ffi::c_int as ogg_uint32_t,
		0x56f36e28 as ::core::ffi::c_int as ogg_uint32_t,
		0x8a9ef49f as ::core::ffi::c_uint,
		0x961c0af4 as ::core::ffi::c_uint,
		0x4a719043 as ::core::ffi::c_int as ogg_uint32_t,
		0x2a06222d as ::core::ffi::c_int as ogg_uint32_t,
		0xf66bb89a as ::core::ffi::c_uint,
		0x1303defb as ::core::ffi::c_int as ogg_uint32_t,
		0xcf6e444c as ::core::ffi::c_uint,
		0xaf19f622 as ::core::ffi::c_uint,
		0x73746c95 as ::core::ffi::c_int as ogg_uint32_t,
		0x6ff692fe as ::core::ffi::c_int as ogg_uint32_t,
		0xb39b0849 as ::core::ffi::c_uint,
		0xd3ecba27 as ::core::ffi::c_uint,
		0xf812090 as ::core::ffi::c_int as ogg_uint32_t,
		0x1dfd6b52 as ::core::ffi::c_int as ogg_uint32_t,
		0xc190f1e5 as ::core::ffi::c_uint,
		0xa1e7438b as ::core::ffi::c_uint,
		0x7d8ad93c as ::core::ffi::c_int as ogg_uint32_t,
		0x61082757 as ::core::ffi::c_int as ogg_uint32_t,
		0xbd65bde0 as ::core::ffi::c_uint,
		0xdd120f8e as ::core::ffi::c_uint,
		0x17f9539 as ::core::ffi::c_int as ogg_uint32_t,
		0xe417f358 as ::core::ffi::c_uint,
		0x387a69ef as ::core::ffi::c_int as ogg_uint32_t,
		0x580ddb81 as ::core::ffi::c_int as ogg_uint32_t,
		0x84604136 as ::core::ffi::c_uint,
		0x98e2bf5d as ::core::ffi::c_uint,
		0x448f25ea as ::core::ffi::c_int as ogg_uint32_t,
		0x24f89784 as ::core::ffi::c_int as ogg_uint32_t,
		0xf8950d33 as ::core::ffi::c_uint,
		0xd1139055 as ::core::ffi::c_uint,
		0xd7e0ae2 as ::core::ffi::c_int as ogg_uint32_t,
		0x6d09b88c as ::core::ffi::c_int as ogg_uint32_t,
		0xb164223b as ::core::ffi::c_uint,
		0xade6dc50 as ::core::ffi::c_uint,
		0x718b46e7 as ::core::ffi::c_int as ogg_uint32_t,
		0x11fcf489 as ::core::ffi::c_int as ogg_uint32_t,
		0xcd916e3e as ::core::ffi::c_uint,
		0x28f9085f as ::core::ffi::c_int as ogg_uint32_t,
		0xf49492e8 as ::core::ffi::c_uint,
		0x94e32086 as ::core::ffi::c_uint,
		0x488eba31 as ::core::ffi::c_int as ogg_uint32_t,
		0x540c445a as ::core::ffi::c_int as ogg_uint32_t,
		0x8861deed as ::core::ffi::c_uint,
		0xe8166c83 as ::core::ffi::c_uint,
		0x347bf634 as ::core::ffi::c_int as ogg_uint32_t,
		0x2607bdf6 as ::core::ffi::c_int as ogg_uint32_t,
		0xfa6a2741 as ::core::ffi::c_uint,
		0x9a1d952f as ::core::ffi::c_uint,
		0x46700f98 as ::core::ffi::c_int as ogg_uint32_t,
		0x5af2f1f3 as ::core::ffi::c_int as ogg_uint32_t,
		0x869f6b44 as ::core::ffi::c_uint,
		0xe6e8d92a as ::core::ffi::c_uint,
		0x3a85439d as ::core::ffi::c_int as ogg_uint32_t,
		0xdfed25fc as ::core::ffi::c_uint,
		0x380bf4b as ::core::ffi::c_int as ogg_uint32_t,
		0x63f70d25 as ::core::ffi::c_int as ogg_uint32_t,
		0xbf9a9792 as ::core::ffi::c_uint,
		0xa31869f9 as ::core::ffi::c_uint,
		0x7f75f34e as ::core::ffi::c_int as ogg_uint32_t,
		0x1f024120 as ::core::ffi::c_int as ogg_uint32_t,
		0xc36fdb97 as ::core::ffi::c_uint,
		0x3bfad6a4 as ::core::ffi::c_int as ogg_uint32_t,
		0xe7974c13 as ::core::ffi::c_uint,
		0x87e0fe7d as ::core::ffi::c_uint,
		0x5b8d64ca as ::core::ffi::c_int as ogg_uint32_t,
		0x470f9aa1 as ::core::ffi::c_int as ogg_uint32_t,
		0x9b620016 as ::core::ffi::c_uint,
		0xfb15b278 as ::core::ffi::c_uint,
		0x277828cf as ::core::ffi::c_int as ogg_uint32_t,
		0xc2104eae as ::core::ffi::c_uint,
		0x1e7dd419 as ::core::ffi::c_int as ogg_uint32_t,
		0x7e0a6677 as ::core::ffi::c_int as ogg_uint32_t,
		0xa267fcc0 as ::core::ffi::c_uint,
		0xbee502ab as ::core::ffi::c_uint,
		0x6288981c as ::core::ffi::c_int as ogg_uint32_t,
		0x2ff2a72 as ::core::ffi::c_int as ogg_uint32_t,
		0xde92b0c5 as ::core::ffi::c_uint,
		0xcceefb07 as ::core::ffi::c_uint,
		0x108361b0 as ::core::ffi::c_int as ogg_uint32_t,
		0x70f4d3de as ::core::ffi::c_int as ogg_uint32_t,
		0xac994969 as ::core::ffi::c_uint,
		0xb01bb702 as ::core::ffi::c_uint,
		0x6c762db5 as ::core::ffi::c_int as ogg_uint32_t,
		0xc019fdb as ::core::ffi::c_int as ogg_uint32_t,
		0xd06c056c as ::core::ffi::c_uint,
		0x3504630d as ::core::ffi::c_int as ogg_uint32_t,
		0xe969f9ba as ::core::ffi::c_uint,
		0x891e4bd4 as ::core::ffi::c_uint,
		0x5573d163 as ::core::ffi::c_int as ogg_uint32_t,
		0x49f12f08 as ::core::ffi::c_int as ogg_uint32_t,
		0x959cb5bf as ::core::ffi::c_uint,
		0xf5eb07d1 as ::core::ffi::c_uint,
		0x29869d66 as ::core::ffi::c_int as ogg_uint32_t,
		0xa6e63d1d as ::core::ffi::c_uint,
		0x7a8ba7aa as ::core::ffi::c_int as ogg_uint32_t,
		0x1afc15c4 as ::core::ffi::c_int as ogg_uint32_t,
		0xc6918f73 as ::core::ffi::c_uint,
		0xda137118 as ::core::ffi::c_uint,
		0x67eebaf as ::core::ffi::c_int as ogg_uint32_t,
		0x660959c1 as ::core::ffi::c_int as ogg_uint32_t,
		0xba64c376 as ::core::ffi::c_uint,
		0x5f0ca517 as ::core::ffi::c_int as ogg_uint32_t,
		0x83613fa0 as ::core::ffi::c_uint,
		0xe3168dce as ::core::ffi::c_uint,
		0x3f7b1779 as ::core::ffi::c_int as ogg_uint32_t,
		0x23f9e912 as ::core::ffi::c_int as ogg_uint32_t,
		0xff9473a5 as ::core::ffi::c_uint,
		0x9fe3c1cb as ::core::ffi::c_uint,
		0x438e5b7c as ::core::ffi::c_int as ogg_uint32_t,
		0x51f210be as ::core::ffi::c_int as ogg_uint32_t,
		0x8d9f8a09 as ::core::ffi::c_uint,
		0xede83867 as ::core::ffi::c_uint,
		0x3185a2d0 as ::core::ffi::c_int as ogg_uint32_t,
		0x2d075cbb as ::core::ffi::c_int as ogg_uint32_t,
		0xf16ac60c as ::core::ffi::c_uint,
		0x911d7462 as ::core::ffi::c_uint,
		0x4d70eed5 as ::core::ffi::c_int as ogg_uint32_t,
		0xa81888b4 as ::core::ffi::c_uint,
		0x74751203 as ::core::ffi::c_int as ogg_uint32_t,
		0x1402a06d as ::core::ffi::c_int as ogg_uint32_t,
		0xc86f3ada as ::core::ffi::c_uint,
		0xd4edc4b1 as ::core::ffi::c_uint,
		0x8805e06 as ::core::ffi::c_int as ogg_uint32_t,
		0x68f7ec68 as ::core::ffi::c_int as ogg_uint32_t,
		0xb49a76df as ::core::ffi::c_uint,
		0x4c0f7bec as ::core::ffi::c_int as ogg_uint32_t,
		0x9062e15b as ::core::ffi::c_uint,
		0xf0155335 as ::core::ffi::c_uint,
		0x2c78c982 as ::core::ffi::c_int as ogg_uint32_t,
		0x30fa37e9 as ::core::ffi::c_int as ogg_uint32_t,
		0xec97ad5e as ::core::ffi::c_uint,
		0x8ce01f30 as ::core::ffi::c_uint,
		0x508d8587 as ::core::ffi::c_int as ogg_uint32_t,
		0xb5e5e3e6 as ::core::ffi::c_uint,
		0x69887951 as ::core::ffi::c_int as ogg_uint32_t,
		0x9ffcb3f as ::core::ffi::c_int as ogg_uint32_t,
		0xd5925188 as ::core::ffi::c_uint,
		0xc910afe3 as ::core::ffi::c_uint,
		0x157d3554 as ::core::ffi::c_int as ogg_uint32_t,
		0x750a873a as ::core::ffi::c_int as ogg_uint32_t,
		0xa9671d8d as ::core::ffi::c_uint,
		0xbb1b564f as ::core::ffi::c_uint,
		0x6776ccf8 as ::core::ffi::c_int as ogg_uint32_t,
		0x7017e96 as ::core::ffi::c_int as ogg_uint32_t,
		0xdb6ce421 as ::core::ffi::c_uint,
		0xc7ee1a4a as ::core::ffi::c_uint,
		0x1b8380fd as ::core::ffi::c_int as ogg_uint32_t,
		0x7bf43293 as ::core::ffi::c_int as ogg_uint32_t,
		0xa799a824 as ::core::ffi::c_uint,
		0x42f1ce45 as ::core::ffi::c_int as ogg_uint32_t,
		0x9e9c54f2 as ::core::ffi::c_uint,
		0xfeebe69c as ::core::ffi::c_uint,
		0x22867c2b as ::core::ffi::c_int as ogg_uint32_t,
		0x3e048240 as ::core::ffi::c_int as ogg_uint32_t,
		0xe26918f7 as ::core::ffi::c_uint,
		0x821eaa99 as ::core::ffi::c_uint,
		0x5e73302e as ::core::ffi::c_int as ogg_uint32_t,
		0x77f5ad48 as ::core::ffi::c_int as ogg_uint32_t,
		0xab9837ff as ::core::ffi::c_uint,
		0xcbef8591 as ::core::ffi::c_uint,
		0x17821f26 as ::core::ffi::c_int as ogg_uint32_t,
		0xb00e14d as ::core::ffi::c_int as ogg_uint32_t,
		0xd76d7bfa as ::core::ffi::c_uint,
		0xb71ac994 as ::core::ffi::c_uint,
		0x6b775323 as ::core::ffi::c_int as ogg_uint32_t,
		0x8e1f3542 as ::core::ffi::c_uint,
		0x5272aff5 as ::core::ffi::c_int as ogg_uint32_t,
		0x32051d9b as ::core::ffi::c_int as ogg_uint32_t,
		0xee68872c as ::core::ffi::c_uint,
		0xf2ea7947 as ::core::ffi::c_uint,
		0x2e87e3f0 as ::core::ffi::c_int as ogg_uint32_t,
		0x4ef0519e as ::core::ffi::c_int as ogg_uint32_t,
		0x929dcb29 as ::core::ffi::c_uint,
		0x80e180eb as ::core::ffi::c_uint,
		0x5c8c1a5c as ::core::ffi::c_int as ogg_uint32_t,
		0x3cfba832 as ::core::ffi::c_int as ogg_uint32_t,
		0xe0963285 as ::core::ffi::c_uint,
		0xfc14ccee as ::core::ffi::c_uint,
		0x20795659 as ::core::ffi::c_int as ogg_uint32_t,
		0x400ee437 as ::core::ffi::c_int as ogg_uint32_t,
		0x9c637e80 as ::core::ffi::c_uint,
		0x790b18e1 as ::core::ffi::c_int as ogg_uint32_t,
		0xa5668256 as ::core::ffi::c_uint,
		0xc5113038 as ::core::ffi::c_uint,
		0x197caa8f as ::core::ffi::c_int as ogg_uint32_t,
		0x5fe54e4 as ::core::ffi::c_int as ogg_uint32_t,
		0xd993ce53 as ::core::ffi::c_uint,
		0xb9e47c3d as ::core::ffi::c_uint,
		0x6589e68a as ::core::ffi::c_int as ogg_uint32_t,
		0x9d1cebb9 as ::core::ffi::c_uint,
		0x4171710e as ::core::ffi::c_int as ogg_uint32_t,
		0x2106c360 as ::core::ffi::c_int as ogg_uint32_t,
		0xfd6b59d7 as ::core::ffi::c_uint,
		0xe1e9a7bc as ::core::ffi::c_uint,
		0x3d843d0b as ::core::ffi::c_int as ogg_uint32_t,
		0x5df38f65 as ::core::ffi::c_int as ogg_uint32_t,
		0x819e15d2 as ::core::ffi::c_uint,
		0x64f673b3 as ::core::ffi::c_int as ogg_uint32_t,
		0xb89be904 as ::core::ffi::c_uint,
		0xd8ec5b6a as ::core::ffi::c_uint,
		0x481c1dd as ::core::ffi::c_int as ogg_uint32_t,
		0x18033fb6 as ::core::ffi::c_int as ogg_uint32_t,
		0xc46ea501 as ::core::ffi::c_uint,
		0xa419176f as ::core::ffi::c_uint,
		0x78748dd8 as ::core::ffi::c_int as ogg_uint32_t,
		0x6a08c61a as ::core::ffi::c_int as ogg_uint32_t,
		0xb6655cad as ::core::ffi::c_uint,
		0xd612eec3 as ::core::ffi::c_uint,
		0xa7f7474 as ::core::ffi::c_int as ogg_uint32_t,
		0x16fd8a1f as ::core::ffi::c_int as ogg_uint32_t,
		0xca9010a8 as ::core::ffi::c_uint,
		0xaae7a2c6 as ::core::ffi::c_uint,
		0x768a3871 as ::core::ffi::c_int as ogg_uint32_t,
		0x93e25e10 as ::core::ffi::c_uint,
		0x4f8fc4a7 as ::core::ffi::c_int as ogg_uint32_t,
		0x2ff876c9 as ::core::ffi::c_int as ogg_uint32_t,
		0xf395ec7e as ::core::ffi::c_uint,
		0xef171215 as ::core::ffi::c_uint,
		0x337a88a2 as ::core::ffi::c_int as ogg_uint32_t,
		0x530d3acc as ::core::ffi::c_int as ogg_uint32_t,
		0x8f60a07b as ::core::ffi::c_uint
	],
	[
		0 as ::core::ffi::c_int as ogg_uint32_t,
		0x490d678d as ::core::ffi::c_int as ogg_uint32_t,
		0x921acf1a as ::core::ffi::c_uint,
		0xdb17a897 as ::core::ffi::c_uint,
		0x20f48383 as ::core::ffi::c_int as ogg_uint32_t,
		0x69f9e40e as ::core::ffi::c_int as ogg_uint32_t,
		0xb2ee4c99 as ::core::ffi::c_uint,
		0xfbe32b14 as ::core::ffi::c_uint,
		0x41e90706 as ::core::ffi::c_int as ogg_uint32_t,
		0x8e4608b as ::core::ffi::c_int as ogg_uint32_t,
		0xd3f3c81c as ::core::ffi::c_uint,
		0x9afeaf91 as ::core::ffi::c_uint,
		0x611d8485 as ::core::ffi::c_int as ogg_uint32_t,
		0x2810e308 as ::core::ffi::c_int as ogg_uint32_t,
		0xf3074b9f as ::core::ffi::c_uint,
		0xba0a2c12 as ::core::ffi::c_uint,
		0x83d20e0c as ::core::ffi::c_uint,
		0xcadf6981 as ::core::ffi::c_uint,
		0x11c8c116 as ::core::ffi::c_int as ogg_uint32_t,
		0x58c5a69b as ::core::ffi::c_int as ogg_uint32_t,
		0xa3268d8f as ::core::ffi::c_uint,
		0xea2bea02 as ::core::ffi::c_uint,
		0x313c4295 as ::core::ffi::c_int as ogg_uint32_t,
		0x78312518 as ::core::ffi::c_int as ogg_uint32_t,
		0xc23b090a as ::core::ffi::c_uint,
		0x8b366e87 as ::core::ffi::c_uint,
		0x5021c610 as ::core::ffi::c_int as ogg_uint32_t,
		0x192ca19d as ::core::ffi::c_int as ogg_uint32_t,
		0xe2cf8a89 as ::core::ffi::c_uint,
		0xabc2ed04 as ::core::ffi::c_uint,
		0x70d54593 as ::core::ffi::c_int as ogg_uint32_t,
		0x39d8221e as ::core::ffi::c_int as ogg_uint32_t,
		0x36501af as ::core::ffi::c_int as ogg_uint32_t,
		0x4a686622 as ::core::ffi::c_int as ogg_uint32_t,
		0x917fceb5 as ::core::ffi::c_uint,
		0xd872a938 as ::core::ffi::c_uint,
		0x2391822c as ::core::ffi::c_int as ogg_uint32_t,
		0x6a9ce5a1 as ::core::ffi::c_int as ogg_uint32_t,
		0xb18b4d36 as ::core::ffi::c_uint,
		0xf8862abb as ::core::ffi::c_uint,
		0x428c06a9 as ::core::ffi::c_int as ogg_uint32_t,
		0xb816124 as ::core::ffi::c_int as ogg_uint32_t,
		0xd096c9b3 as ::core::ffi::c_uint,
		0x999bae3e as ::core::ffi::c_uint,
		0x6278852a as ::core::ffi::c_int as ogg_uint32_t,
		0x2b75e2a7 as ::core::ffi::c_int as ogg_uint32_t,
		0xf0624a30 as ::core::ffi::c_uint,
		0xb96f2dbd as ::core::ffi::c_uint,
		0x80b70fa3 as ::core::ffi::c_uint,
		0xc9ba682e as ::core::ffi::c_uint,
		0x12adc0b9 as ::core::ffi::c_int as ogg_uint32_t,
		0x5ba0a734 as ::core::ffi::c_int as ogg_uint32_t,
		0xa0438c20 as ::core::ffi::c_uint,
		0xe94eebad as ::core::ffi::c_uint,
		0x3259433a as ::core::ffi::c_int as ogg_uint32_t,
		0x7b5424b7 as ::core::ffi::c_int as ogg_uint32_t,
		0xc15e08a5 as ::core::ffi::c_uint,
		0x88536f28 as ::core::ffi::c_uint,
		0x5344c7bf as ::core::ffi::c_int as ogg_uint32_t,
		0x1a49a032 as ::core::ffi::c_int as ogg_uint32_t,
		0xe1aa8b26 as ::core::ffi::c_uint,
		0xa8a7ecab as ::core::ffi::c_uint,
		0x73b0443c as ::core::ffi::c_int as ogg_uint32_t,
		0x3abd23b1 as ::core::ffi::c_int as ogg_uint32_t,
		0x6ca035e as ::core::ffi::c_int as ogg_uint32_t,
		0x4fc764d3 as ::core::ffi::c_int as ogg_uint32_t,
		0x94d0cc44 as ::core::ffi::c_uint,
		0xddddabc9 as ::core::ffi::c_uint,
		0x263e80dd as ::core::ffi::c_int as ogg_uint32_t,
		0x6f33e750 as ::core::ffi::c_int as ogg_uint32_t,
		0xb4244fc7 as ::core::ffi::c_uint,
		0xfd29284a as ::core::ffi::c_uint,
		0x47230458 as ::core::ffi::c_int as ogg_uint32_t,
		0xe2e63d5 as ::core::ffi::c_int as ogg_uint32_t,
		0xd539cb42 as ::core::ffi::c_uint,
		0x9c34accf as ::core::ffi::c_uint,
		0x67d787db as ::core::ffi::c_int as ogg_uint32_t,
		0x2edae056 as ::core::ffi::c_int as ogg_uint32_t,
		0xf5cd48c1 as ::core::ffi::c_uint,
		0xbcc02f4c as ::core::ffi::c_uint,
		0x85180d52 as ::core::ffi::c_uint,
		0xcc156adf as ::core::ffi::c_uint,
		0x1702c248 as ::core::ffi::c_int as ogg_uint32_t,
		0x5e0fa5c5 as ::core::ffi::c_int as ogg_uint32_t,
		0xa5ec8ed1 as ::core::ffi::c_uint,
		0xece1e95c as ::core::ffi::c_uint,
		0x37f641cb as ::core::ffi::c_int as ogg_uint32_t,
		0x7efb2646 as ::core::ffi::c_int as ogg_uint32_t,
		0xc4f10a54 as ::core::ffi::c_uint,
		0x8dfc6dd9 as ::core::ffi::c_uint,
		0x56ebc54e as ::core::ffi::c_int as ogg_uint32_t,
		0x1fe6a2c3 as ::core::ffi::c_int as ogg_uint32_t,
		0xe40589d7 as ::core::ffi::c_uint,
		0xad08ee5a as ::core::ffi::c_uint,
		0x761f46cd as ::core::ffi::c_int as ogg_uint32_t,
		0x3f122140 as ::core::ffi::c_int as ogg_uint32_t,
		0x5af02f1 as ::core::ffi::c_int as ogg_uint32_t,
		0x4ca2657c as ::core::ffi::c_int as ogg_uint32_t,
		0x97b5cdeb as ::core::ffi::c_uint,
		0xdeb8aa66 as ::core::ffi::c_uint,
		0x255b8172 as ::core::ffi::c_int as ogg_uint32_t,
		0x6c56e6ff as ::core::ffi::c_int as ogg_uint32_t,
		0xb7414e68 as ::core::ffi::c_uint,
		0xfe4c29e5 as ::core::ffi::c_uint,
		0x444605f7 as ::core::ffi::c_int as ogg_uint32_t,
		0xd4b627a as ::core::ffi::c_int as ogg_uint32_t,
		0xd65ccaed as ::core::ffi::c_uint,
		0x9f51ad60 as ::core::ffi::c_uint,
		0x64b28674 as ::core::ffi::c_int as ogg_uint32_t,
		0x2dbfe1f9 as ::core::ffi::c_int as ogg_uint32_t,
		0xf6a8496e as ::core::ffi::c_uint,
		0xbfa52ee3 as ::core::ffi::c_uint,
		0x867d0cfd as ::core::ffi::c_uint,
		0xcf706b70 as ::core::ffi::c_uint,
		0x1467c3e7 as ::core::ffi::c_int as ogg_uint32_t,
		0x5d6aa46a as ::core::ffi::c_int as ogg_uint32_t,
		0xa6898f7e as ::core::ffi::c_uint,
		0xef84e8f3 as ::core::ffi::c_uint,
		0x34934064 as ::core::ffi::c_int as ogg_uint32_t,
		0x7d9e27e9 as ::core::ffi::c_int as ogg_uint32_t,
		0xc7940bfb as ::core::ffi::c_uint,
		0x8e996c76 as ::core::ffi::c_uint,
		0x558ec4e1 as ::core::ffi::c_int as ogg_uint32_t,
		0x1c83a36c as ::core::ffi::c_int as ogg_uint32_t,
		0xe7608878 as ::core::ffi::c_uint,
		0xae6deff5 as ::core::ffi::c_uint,
		0x757a4762 as ::core::ffi::c_int as ogg_uint32_t,
		0x3c7720ef as ::core::ffi::c_int as ogg_uint32_t,
		0xd9406bc as ::core::ffi::c_int as ogg_uint32_t,
		0x44996131 as ::core::ffi::c_int as ogg_uint32_t,
		0x9f8ec9a6 as ::core::ffi::c_uint,
		0xd683ae2b as ::core::ffi::c_uint,
		0x2d60853f as ::core::ffi::c_int as ogg_uint32_t,
		0x646de2b2 as ::core::ffi::c_int as ogg_uint32_t,
		0xbf7a4a25 as ::core::ffi::c_uint,
		0xf6772da8 as ::core::ffi::c_uint,
		0x4c7d01ba as ::core::ffi::c_int as ogg_uint32_t,
		0x5706637 as ::core::ffi::c_int as ogg_uint32_t,
		0xde67cea0 as ::core::ffi::c_uint,
		0x976aa92d as ::core::ffi::c_uint,
		0x6c898239 as ::core::ffi::c_int as ogg_uint32_t,
		0x2584e5b4 as ::core::ffi::c_int as ogg_uint32_t,
		0xfe934d23 as ::core::ffi::c_uint,
		0xb79e2aae as ::core::ffi::c_uint,
		0x8e4608b0 as ::core::ffi::c_uint,
		0xc74b6f3d as ::core::ffi::c_uint,
		0x1c5cc7aa as ::core::ffi::c_int as ogg_uint32_t,
		0x5551a027 as ::core::ffi::c_int as ogg_uint32_t,
		0xaeb28b33 as ::core::ffi::c_uint,
		0xe7bfecbe as ::core::ffi::c_uint,
		0x3ca84429 as ::core::ffi::c_int as ogg_uint32_t,
		0x75a523a4 as ::core::ffi::c_int as ogg_uint32_t,
		0xcfaf0fb6 as ::core::ffi::c_uint,
		0x86a2683b as ::core::ffi::c_uint,
		0x5db5c0ac as ::core::ffi::c_int as ogg_uint32_t,
		0x14b8a721 as ::core::ffi::c_int as ogg_uint32_t,
		0xef5b8c35 as ::core::ffi::c_uint,
		0xa656ebb8 as ::core::ffi::c_uint,
		0x7d41432f as ::core::ffi::c_int as ogg_uint32_t,
		0x344c24a2 as ::core::ffi::c_int as ogg_uint32_t,
		0xef10713 as ::core::ffi::c_int as ogg_uint32_t,
		0x47fc609e as ::core::ffi::c_int as ogg_uint32_t,
		0x9cebc809 as ::core::ffi::c_uint,
		0xd5e6af84 as ::core::ffi::c_uint,
		0x2e058490 as ::core::ffi::c_int as ogg_uint32_t,
		0x6708e31d as ::core::ffi::c_int as ogg_uint32_t,
		0xbc1f4b8a as ::core::ffi::c_uint,
		0xf5122c07 as ::core::ffi::c_uint,
		0x4f180015 as ::core::ffi::c_int as ogg_uint32_t,
		0x6156798 as ::core::ffi::c_int as ogg_uint32_t,
		0xdd02cf0f as ::core::ffi::c_uint,
		0x940fa882 as ::core::ffi::c_uint,
		0x6fec8396 as ::core::ffi::c_int as ogg_uint32_t,
		0x26e1e41b as ::core::ffi::c_int as ogg_uint32_t,
		0xfdf64c8c as ::core::ffi::c_uint,
		0xb4fb2b01 as ::core::ffi::c_uint,
		0x8d23091f as ::core::ffi::c_uint,
		0xc42e6e92 as ::core::ffi::c_uint,
		0x1f39c605 as ::core::ffi::c_int as ogg_uint32_t,
		0x5634a188 as ::core::ffi::c_int as ogg_uint32_t,
		0xadd78a9c as ::core::ffi::c_uint,
		0xe4daed11 as ::core::ffi::c_uint,
		0x3fcd4586 as ::core::ffi::c_int as ogg_uint32_t,
		0x76c0220b as ::core::ffi::c_int as ogg_uint32_t,
		0xccca0e19 as ::core::ffi::c_uint,
		0x85c76994 as ::core::ffi::c_uint,
		0x5ed0c103 as ::core::ffi::c_int as ogg_uint32_t,
		0x17dda68e as ::core::ffi::c_int as ogg_uint32_t,
		0xec3e8d9a as ::core::ffi::c_uint,
		0xa533ea17 as ::core::ffi::c_uint,
		0x7e244280 as ::core::ffi::c_int as ogg_uint32_t,
		0x3729250d as ::core::ffi::c_int as ogg_uint32_t,
		0xb5e05e2 as ::core::ffi::c_int as ogg_uint32_t,
		0x4253626f as ::core::ffi::c_int as ogg_uint32_t,
		0x9944caf8 as ::core::ffi::c_uint,
		0xd049ad75 as ::core::ffi::c_uint,
		0x2baa8661 as ::core::ffi::c_int as ogg_uint32_t,
		0x62a7e1ec as ::core::ffi::c_int as ogg_uint32_t,
		0xb9b0497b as ::core::ffi::c_uint,
		0xf0bd2ef6 as ::core::ffi::c_uint,
		0x4ab702e4 as ::core::ffi::c_int as ogg_uint32_t,
		0x3ba6569 as ::core::ffi::c_int as ogg_uint32_t,
		0xd8adcdfe as ::core::ffi::c_uint,
		0x91a0aa73 as ::core::ffi::c_uint,
		0x6a438167 as ::core::ffi::c_int as ogg_uint32_t,
		0x234ee6ea as ::core::ffi::c_int as ogg_uint32_t,
		0xf8594e7d as ::core::ffi::c_uint,
		0xb15429f0 as ::core::ffi::c_uint,
		0x888c0bee as ::core::ffi::c_uint,
		0xc1816c63 as ::core::ffi::c_uint,
		0x1a96c4f4 as ::core::ffi::c_int as ogg_uint32_t,
		0x539ba379 as ::core::ffi::c_int as ogg_uint32_t,
		0xa878886d as ::core::ffi::c_uint,
		0xe175efe0 as ::core::ffi::c_uint,
		0x3a624777 as ::core::ffi::c_int as ogg_uint32_t,
		0x736f20fa as ::core::ffi::c_int as ogg_uint32_t,
		0xc9650ce8 as ::core::ffi::c_uint,
		0x80686b65 as ::core::ffi::c_uint,
		0x5b7fc3f2 as ::core::ffi::c_int as ogg_uint32_t,
		0x1272a47f as ::core::ffi::c_int as ogg_uint32_t,
		0xe9918f6b as ::core::ffi::c_uint,
		0xa09ce8e6 as ::core::ffi::c_uint,
		0x7b8b4071 as ::core::ffi::c_int as ogg_uint32_t,
		0x328627fc as ::core::ffi::c_int as ogg_uint32_t,
		0x83b044d as ::core::ffi::c_int as ogg_uint32_t,
		0x413663c0 as ::core::ffi::c_int as ogg_uint32_t,
		0x9a21cb57 as ::core::ffi::c_uint,
		0xd32cacda as ::core::ffi::c_uint,
		0x28cf87ce as ::core::ffi::c_int as ogg_uint32_t,
		0x61c2e043 as ::core::ffi::c_int as ogg_uint32_t,
		0xbad548d4 as ::core::ffi::c_uint,
		0xf3d82f59 as ::core::ffi::c_uint,
		0x49d2034b as ::core::ffi::c_int as ogg_uint32_t,
		0xdf64c6 as ::core::ffi::c_int as ogg_uint32_t,
		0xdbc8cc51 as ::core::ffi::c_uint,
		0x92c5abdc as ::core::ffi::c_uint,
		0x692680c8 as ::core::ffi::c_int as ogg_uint32_t,
		0x202be745 as ::core::ffi::c_int as ogg_uint32_t,
		0xfb3c4fd2 as ::core::ffi::c_uint,
		0xb231285f as ::core::ffi::c_uint,
		0x8be90a41 as ::core::ffi::c_uint,
		0xc2e46dcc as ::core::ffi::c_uint,
		0x19f3c55b as ::core::ffi::c_int as ogg_uint32_t,
		0x50fea2d6 as ::core::ffi::c_int as ogg_uint32_t,
		0xab1d89c2 as ::core::ffi::c_uint,
		0xe210ee4f as ::core::ffi::c_uint,
		0x390746d8 as ::core::ffi::c_int as ogg_uint32_t,
		0x700a2155 as ::core::ffi::c_int as ogg_uint32_t,
		0xca000d47 as ::core::ffi::c_uint,
		0x830d6aca as ::core::ffi::c_uint,
		0x581ac25d as ::core::ffi::c_int as ogg_uint32_t,
		0x1117a5d0 as ::core::ffi::c_int as ogg_uint32_t,
		0xeaf48ec4 as ::core::ffi::c_uint,
		0xa3f9e949 as ::core::ffi::c_uint,
		0x78ee41de as ::core::ffi::c_int as ogg_uint32_t,
		0x31e32653 as ::core::ffi::c_int as ogg_uint32_t
	],
	[
		0 as ::core::ffi::c_int as ogg_uint32_t,
		0x1b280d78 as ::core::ffi::c_int as ogg_uint32_t,
		0x36501af0 as ::core::ffi::c_int as ogg_uint32_t,
		0x2d781788 as ::core::ffi::c_int as ogg_uint32_t,
		0x6ca035e0 as ::core::ffi::c_int as ogg_uint32_t,
		0x77883898 as ::core::ffi::c_int as ogg_uint32_t,
		0x5af02f10 as ::core::ffi::c_int as ogg_uint32_t,
		0x41d82268 as ::core::ffi::c_int as ogg_uint32_t,
		0xd9406bc0 as ::core::ffi::c_uint,
		0xc26866b8 as ::core::ffi::c_uint,
		0xef107130 as ::core::ffi::c_uint,
		0xf4387c48 as ::core::ffi::c_uint,
		0xb5e05e20 as ::core::ffi::c_uint,
		0xaec85358 as ::core::ffi::c_uint,
		0x83b044d0 as ::core::ffi::c_uint,
		0x989849a8 as ::core::ffi::c_uint,
		0xb641ca37 as ::core::ffi::c_uint,
		0xad69c74f as ::core::ffi::c_uint,
		0x8011d0c7 as ::core::ffi::c_uint,
		0x9b39ddbf as ::core::ffi::c_uint,
		0xdae1ffd7 as ::core::ffi::c_uint,
		0xc1c9f2af as ::core::ffi::c_uint,
		0xecb1e527 as ::core::ffi::c_uint,
		0xf799e85f as ::core::ffi::c_uint,
		0x6f01a1f7 as ::core::ffi::c_int as ogg_uint32_t,
		0x7429ac8f as ::core::ffi::c_int as ogg_uint32_t,
		0x5951bb07 as ::core::ffi::c_int as ogg_uint32_t,
		0x4279b67f as ::core::ffi::c_int as ogg_uint32_t,
		0x3a19417 as ::core::ffi::c_int as ogg_uint32_t,
		0x1889996f as ::core::ffi::c_int as ogg_uint32_t,
		0x35f18ee7 as ::core::ffi::c_int as ogg_uint32_t,
		0x2ed9839f as ::core::ffi::c_int as ogg_uint32_t,
		0x684289d9 as ::core::ffi::c_int as ogg_uint32_t,
		0x736a84a1 as ::core::ffi::c_int as ogg_uint32_t,
		0x5e129329 as ::core::ffi::c_int as ogg_uint32_t,
		0x453a9e51 as ::core::ffi::c_int as ogg_uint32_t,
		0x4e2bc39 as ::core::ffi::c_int as ogg_uint32_t,
		0x1fcab141 as ::core::ffi::c_int as ogg_uint32_t,
		0x32b2a6c9 as ::core::ffi::c_int as ogg_uint32_t,
		0x299aabb1 as ::core::ffi::c_int as ogg_uint32_t,
		0xb102e219 as ::core::ffi::c_uint,
		0xaa2aef61 as ::core::ffi::c_uint,
		0x8752f8e9 as ::core::ffi::c_uint,
		0x9c7af591 as ::core::ffi::c_uint,
		0xdda2d7f9 as ::core::ffi::c_uint,
		0xc68ada81 as ::core::ffi::c_uint,
		0xebf2cd09 as ::core::ffi::c_uint,
		0xf0dac071 as ::core::ffi::c_uint,
		0xde0343ee as ::core::ffi::c_uint,
		0xc52b4e96 as ::core::ffi::c_uint,
		0xe853591e as ::core::ffi::c_uint,
		0xf37b5466 as ::core::ffi::c_uint,
		0xb2a3760e as ::core::ffi::c_uint,
		0xa98b7b76 as ::core::ffi::c_uint,
		0x84f36cfe as ::core::ffi::c_uint,
		0x9fdb6186 as ::core::ffi::c_uint,
		0x743282e as ::core::ffi::c_int as ogg_uint32_t,
		0x1c6b2556 as ::core::ffi::c_int as ogg_uint32_t,
		0x311332de as ::core::ffi::c_int as ogg_uint32_t,
		0x2a3b3fa6 as ::core::ffi::c_int as ogg_uint32_t,
		0x6be31dce as ::core::ffi::c_int as ogg_uint32_t,
		0x70cb10b6 as ::core::ffi::c_int as ogg_uint32_t,
		0x5db3073e as ::core::ffi::c_int as ogg_uint32_t,
		0x469b0a46 as ::core::ffi::c_int as ogg_uint32_t,
		0xd08513b2 as ::core::ffi::c_uint,
		0xcbad1eca as ::core::ffi::c_uint,
		0xe6d50942 as ::core::ffi::c_uint,
		0xfdfd043a as ::core::ffi::c_uint,
		0xbc252652 as ::core::ffi::c_uint,
		0xa70d2b2a as ::core::ffi::c_uint,
		0x8a753ca2 as ::core::ffi::c_uint,
		0x915d31da as ::core::ffi::c_uint,
		0x9c57872 as ::core::ffi::c_int as ogg_uint32_t,
		0x12ed750a as ::core::ffi::c_int as ogg_uint32_t,
		0x3f956282 as ::core::ffi::c_int as ogg_uint32_t,
		0x24bd6ffa as ::core::ffi::c_int as ogg_uint32_t,
		0x65654d92 as ::core::ffi::c_int as ogg_uint32_t,
		0x7e4d40ea as ::core::ffi::c_int as ogg_uint32_t,
		0x53355762 as ::core::ffi::c_int as ogg_uint32_t,
		0x481d5a1a as ::core::ffi::c_int as ogg_uint32_t,
		0x66c4d985 as ::core::ffi::c_int as ogg_uint32_t,
		0x7decd4fd as ::core::ffi::c_int as ogg_uint32_t,
		0x5094c375 as ::core::ffi::c_int as ogg_uint32_t,
		0x4bbcce0d as ::core::ffi::c_int as ogg_uint32_t,
		0xa64ec65 as ::core::ffi::c_int as ogg_uint32_t,
		0x114ce11d as ::core::ffi::c_int as ogg_uint32_t,
		0x3c34f695 as ::core::ffi::c_int as ogg_uint32_t,
		0x271cfbed as ::core::ffi::c_int as ogg_uint32_t,
		0xbf84b245 as ::core::ffi::c_uint,
		0xa4acbf3d as ::core::ffi::c_uint,
		0x89d4a8b5 as ::core::ffi::c_uint,
		0x92fca5cd as ::core::ffi::c_uint,
		0xd32487a5 as ::core::ffi::c_uint,
		0xc80c8add as ::core::ffi::c_uint,
		0xe5749d55 as ::core::ffi::c_uint,
		0xfe5c902d as ::core::ffi::c_uint,
		0xb8c79a6b as ::core::ffi::c_uint,
		0xa3ef9713 as ::core::ffi::c_uint,
		0x8e97809b as ::core::ffi::c_uint,
		0x95bf8de3 as ::core::ffi::c_uint,
		0xd467af8b as ::core::ffi::c_uint,
		0xcf4fa2f3 as ::core::ffi::c_uint,
		0xe237b57b as ::core::ffi::c_uint,
		0xf91fb803 as ::core::ffi::c_uint,
		0x6187f1ab as ::core::ffi::c_int as ogg_uint32_t,
		0x7aaffcd3 as ::core::ffi::c_int as ogg_uint32_t,
		0x57d7eb5b as ::core::ffi::c_int as ogg_uint32_t,
		0x4cffe623 as ::core::ffi::c_int as ogg_uint32_t,
		0xd27c44b as ::core::ffi::c_int as ogg_uint32_t,
		0x160fc933 as ::core::ffi::c_int as ogg_uint32_t,
		0x3b77debb as ::core::ffi::c_int as ogg_uint32_t,
		0x205fd3c3 as ::core::ffi::c_int as ogg_uint32_t,
		0xe86505c as ::core::ffi::c_int as ogg_uint32_t,
		0x15ae5d24 as ::core::ffi::c_int as ogg_uint32_t,
		0x38d64aac as ::core::ffi::c_int as ogg_uint32_t,
		0x23fe47d4 as ::core::ffi::c_int as ogg_uint32_t,
		0x622665bc as ::core::ffi::c_int as ogg_uint32_t,
		0x790e68c4 as ::core::ffi::c_int as ogg_uint32_t,
		0x54767f4c as ::core::ffi::c_int as ogg_uint32_t,
		0x4f5e7234 as ::core::ffi::c_int as ogg_uint32_t,
		0xd7c63b9c as ::core::ffi::c_uint,
		0xccee36e4 as ::core::ffi::c_uint,
		0xe196216c as ::core::ffi::c_uint,
		0xfabe2c14 as ::core::ffi::c_uint,
		0xbb660e7c as ::core::ffi::c_uint,
		0xa04e0304 as ::core::ffi::c_uint,
		0x8d36148c as ::core::ffi::c_uint,
		0x961e19f4 as ::core::ffi::c_uint,
		0xa5cb3ad3 as ::core::ffi::c_uint,
		0xbee337ab as ::core::ffi::c_uint,
		0x939b2023 as ::core::ffi::c_uint,
		0x88b32d5b as ::core::ffi::c_uint,
		0xc96b0f33 as ::core::ffi::c_uint,
		0xd243024b as ::core::ffi::c_uint,
		0xff3b15c3 as ::core::ffi::c_uint,
		0xe41318bb as ::core::ffi::c_uint,
		0x7c8b5113 as ::core::ffi::c_int as ogg_uint32_t,
		0x67a35c6b as ::core::ffi::c_int as ogg_uint32_t,
		0x4adb4be3 as ::core::ffi::c_int as ogg_uint32_t,
		0x51f3469b as ::core::ffi::c_int as ogg_uint32_t,
		0x102b64f3 as ::core::ffi::c_int as ogg_uint32_t,
		0xb03698b as ::core::ffi::c_int as ogg_uint32_t,
		0x267b7e03 as ::core::ffi::c_int as ogg_uint32_t,
		0x3d53737b as ::core::ffi::c_int as ogg_uint32_t,
		0x138af0e4 as ::core::ffi::c_int as ogg_uint32_t,
		0x8a2fd9c as ::core::ffi::c_int as ogg_uint32_t,
		0x25daea14 as ::core::ffi::c_int as ogg_uint32_t,
		0x3ef2e76c as ::core::ffi::c_int as ogg_uint32_t,
		0x7f2ac504 as ::core::ffi::c_int as ogg_uint32_t,
		0x6402c87c as ::core::ffi::c_int as ogg_uint32_t,
		0x497adff4 as ::core::ffi::c_int as ogg_uint32_t,
		0x5252d28c as ::core::ffi::c_int as ogg_uint32_t,
		0xcaca9b24 as ::core::ffi::c_uint,
		0xd1e2965c as ::core::ffi::c_uint,
		0xfc9a81d4 as ::core::ffi::c_uint,
		0xe7b28cac as ::core::ffi::c_uint,
		0xa66aaec4 as ::core::ffi::c_uint,
		0xbd42a3bc as ::core::ffi::c_uint,
		0x903ab434 as ::core::ffi::c_uint,
		0x8b12b94c as ::core::ffi::c_uint,
		0xcd89b30a as ::core::ffi::c_uint,
		0xd6a1be72 as ::core::ffi::c_uint,
		0xfbd9a9fa as ::core::ffi::c_uint,
		0xe0f1a482 as ::core::ffi::c_uint,
		0xa12986ea as ::core::ffi::c_uint,
		0xba018b92 as ::core::ffi::c_uint,
		0x97799c1a as ::core::ffi::c_uint,
		0x8c519162 as ::core::ffi::c_uint,
		0x14c9d8ca as ::core::ffi::c_int as ogg_uint32_t,
		0xfe1d5b2 as ::core::ffi::c_int as ogg_uint32_t,
		0x2299c23a as ::core::ffi::c_int as ogg_uint32_t,
		0x39b1cf42 as ::core::ffi::c_int as ogg_uint32_t,
		0x7869ed2a as ::core::ffi::c_int as ogg_uint32_t,
		0x6341e052 as ::core::ffi::c_int as ogg_uint32_t,
		0x4e39f7da as ::core::ffi::c_int as ogg_uint32_t,
		0x5511faa2 as ::core::ffi::c_int as ogg_uint32_t,
		0x7bc8793d as ::core::ffi::c_int as ogg_uint32_t,
		0x60e07445 as ::core::ffi::c_int as ogg_uint32_t,
		0x4d9863cd as ::core::ffi::c_int as ogg_uint32_t,
		0x56b06eb5 as ::core::ffi::c_int as ogg_uint32_t,
		0x17684cdd as ::core::ffi::c_int as ogg_uint32_t,
		0xc4041a5 as ::core::ffi::c_int as ogg_uint32_t,
		0x2138562d as ::core::ffi::c_int as ogg_uint32_t,
		0x3a105b55 as ::core::ffi::c_int as ogg_uint32_t,
		0xa28812fd as ::core::ffi::c_uint,
		0xb9a01f85 as ::core::ffi::c_uint,
		0x94d8080d as ::core::ffi::c_uint,
		0x8ff00575 as ::core::ffi::c_uint,
		0xce28271d as ::core::ffi::c_uint,
		0xd5002a65 as ::core::ffi::c_uint,
		0xf8783ded as ::core::ffi::c_uint,
		0xe3503095 as ::core::ffi::c_uint,
		0x754e2961 as ::core::ffi::c_int as ogg_uint32_t,
		0x6e662419 as ::core::ffi::c_int as ogg_uint32_t,
		0x431e3391 as ::core::ffi::c_int as ogg_uint32_t,
		0x58363ee9 as ::core::ffi::c_int as ogg_uint32_t,
		0x19ee1c81 as ::core::ffi::c_int as ogg_uint32_t,
		0x2c611f9 as ::core::ffi::c_int as ogg_uint32_t,
		0x2fbe0671 as ::core::ffi::c_int as ogg_uint32_t,
		0x34960b09 as ::core::ffi::c_int as ogg_uint32_t,
		0xac0e42a1 as ::core::ffi::c_uint,
		0xb7264fd9 as ::core::ffi::c_uint,
		0x9a5e5851 as ::core::ffi::c_uint,
		0x81765529 as ::core::ffi::c_uint,
		0xc0ae7741 as ::core::ffi::c_uint,
		0xdb867a39 as ::core::ffi::c_uint,
		0xf6fe6db1 as ::core::ffi::c_uint,
		0xedd660c9 as ::core::ffi::c_uint,
		0xc30fe356 as ::core::ffi::c_uint,
		0xd827ee2e as ::core::ffi::c_uint,
		0xf55ff9a6 as ::core::ffi::c_uint,
		0xee77f4de as ::core::ffi::c_uint,
		0xafafd6b6 as ::core::ffi::c_uint,
		0xb487dbce as ::core::ffi::c_uint,
		0x99ffcc46 as ::core::ffi::c_uint,
		0x82d7c13e as ::core::ffi::c_uint,
		0x1a4f8896 as ::core::ffi::c_int as ogg_uint32_t,
		0x16785ee as ::core::ffi::c_int as ogg_uint32_t,
		0x2c1f9266 as ::core::ffi::c_int as ogg_uint32_t,
		0x37379f1e as ::core::ffi::c_int as ogg_uint32_t,
		0x76efbd76 as ::core::ffi::c_int as ogg_uint32_t,
		0x6dc7b00e as ::core::ffi::c_int as ogg_uint32_t,
		0x40bfa786 as ::core::ffi::c_int as ogg_uint32_t,
		0x5b97aafe as ::core::ffi::c_int as ogg_uint32_t,
		0x1d0ca0b8 as ::core::ffi::c_int as ogg_uint32_t,
		0x624adc0 as ::core::ffi::c_int as ogg_uint32_t,
		0x2b5cba48 as ::core::ffi::c_int as ogg_uint32_t,
		0x3074b730 as ::core::ffi::c_int as ogg_uint32_t,
		0x71ac9558 as ::core::ffi::c_int as ogg_uint32_t,
		0x6a849820 as ::core::ffi::c_int as ogg_uint32_t,
		0x47fc8fa8 as ::core::ffi::c_int as ogg_uint32_t,
		0x5cd482d0 as ::core::ffi::c_int as ogg_uint32_t,
		0xc44ccb78 as ::core::ffi::c_uint,
		0xdf64c600 as ::core::ffi::c_uint,
		0xf21cd188 as ::core::ffi::c_uint,
		0xe934dcf0 as ::core::ffi::c_uint,
		0xa8ecfe98 as ::core::ffi::c_uint,
		0xb3c4f3e0 as ::core::ffi::c_uint,
		0x9ebce468 as ::core::ffi::c_uint,
		0x8594e910 as ::core::ffi::c_uint,
		0xab4d6a8f as ::core::ffi::c_uint,
		0xb06567f7 as ::core::ffi::c_uint,
		0x9d1d707f as ::core::ffi::c_uint,
		0x86357d07 as ::core::ffi::c_uint,
		0xc7ed5f6f as ::core::ffi::c_uint,
		0xdcc55217 as ::core::ffi::c_uint,
		0xf1bd459f as ::core::ffi::c_uint,
		0xea9548e7 as ::core::ffi::c_uint,
		0x720d014f as ::core::ffi::c_int as ogg_uint32_t,
		0x69250c37 as ::core::ffi::c_int as ogg_uint32_t,
		0x445d1bbf as ::core::ffi::c_int as ogg_uint32_t,
		0x5f7516c7 as ::core::ffi::c_int as ogg_uint32_t,
		0x1ead34af as ::core::ffi::c_int as ogg_uint32_t,
		0x58539d7 as ::core::ffi::c_int as ogg_uint32_t,
		0x28fd2e5f as ::core::ffi::c_int as ogg_uint32_t,
		0x33d52327 as ::core::ffi::c_int as ogg_uint32_t
	],
	[
		0 as ::core::ffi::c_int as ogg_uint32_t,
		0x4f576811 as ::core::ffi::c_int as ogg_uint32_t,
		0x9eaed022 as ::core::ffi::c_uint,
		0xd1f9b833 as ::core::ffi::c_uint,
		0x399cbdf3 as ::core::ffi::c_int as ogg_uint32_t,
		0x76cbd5e2 as ::core::ffi::c_int as ogg_uint32_t,
		0xa7326dd1 as ::core::ffi::c_uint,
		0xe86505c0 as ::core::ffi::c_uint,
		0x73397be6 as ::core::ffi::c_int as ogg_uint32_t,
		0x3c6e13f7 as ::core::ffi::c_int as ogg_uint32_t,
		0xed97abc4 as ::core::ffi::c_uint,
		0xa2c0c3d5 as ::core::ffi::c_uint,
		0x4aa5c615 as ::core::ffi::c_int as ogg_uint32_t,
		0x5f2ae04 as ::core::ffi::c_int as ogg_uint32_t,
		0xd40b1637 as ::core::ffi::c_uint,
		0x9b5c7e26 as ::core::ffi::c_uint,
		0xe672f7cc as ::core::ffi::c_uint,
		0xa9259fdd as ::core::ffi::c_uint,
		0x78dc27ee as ::core::ffi::c_int as ogg_uint32_t,
		0x378b4fff as ::core::ffi::c_int as ogg_uint32_t,
		0xdfee4a3f as ::core::ffi::c_uint,
		0x90b9222e as ::core::ffi::c_uint,
		0x41409a1d as ::core::ffi::c_int as ogg_uint32_t,
		0xe17f20c as ::core::ffi::c_int as ogg_uint32_t,
		0x954b8c2a as ::core::ffi::c_uint,
		0xda1ce43b as ::core::ffi::c_uint,
		0xbe55c08 as ::core::ffi::c_int as ogg_uint32_t,
		0x44b23419 as ::core::ffi::c_int as ogg_uint32_t,
		0xacd731d9 as ::core::ffi::c_uint,
		0xe38059c8 as ::core::ffi::c_uint,
		0x3279e1fb as ::core::ffi::c_int as ogg_uint32_t,
		0x7d2e89ea as ::core::ffi::c_int as ogg_uint32_t,
		0xc824f22f as ::core::ffi::c_uint,
		0x87739a3e as ::core::ffi::c_uint,
		0x568a220d as ::core::ffi::c_int as ogg_uint32_t,
		0x19dd4a1c as ::core::ffi::c_int as ogg_uint32_t,
		0xf1b84fdc as ::core::ffi::c_uint,
		0xbeef27cd as ::core::ffi::c_uint,
		0x6f169ffe as ::core::ffi::c_int as ogg_uint32_t,
		0x2041f7ef as ::core::ffi::c_int as ogg_uint32_t,
		0xbb1d89c9 as ::core::ffi::c_uint,
		0xf44ae1d8 as ::core::ffi::c_uint,
		0x25b359eb as ::core::ffi::c_int as ogg_uint32_t,
		0x6ae431fa as ::core::ffi::c_int as ogg_uint32_t,
		0x8281343a as ::core::ffi::c_uint,
		0xcdd65c2b as ::core::ffi::c_uint,
		0x1c2fe418 as ::core::ffi::c_int as ogg_uint32_t,
		0x53788c09 as ::core::ffi::c_int as ogg_uint32_t,
		0x2e5605e3 as ::core::ffi::c_int as ogg_uint32_t,
		0x61016df2 as ::core::ffi::c_int as ogg_uint32_t,
		0xb0f8d5c1 as ::core::ffi::c_uint,
		0xffafbdd0 as ::core::ffi::c_uint,
		0x17cab810 as ::core::ffi::c_int as ogg_uint32_t,
		0x589dd001 as ::core::ffi::c_int as ogg_uint32_t,
		0x89646832 as ::core::ffi::c_uint,
		0xc6330023 as ::core::ffi::c_uint,
		0x5d6f7e05 as ::core::ffi::c_int as ogg_uint32_t,
		0x12381614 as ::core::ffi::c_int as ogg_uint32_t,
		0xc3c1ae27 as ::core::ffi::c_uint,
		0x8c96c636 as ::core::ffi::c_uint,
		0x64f3c3f6 as ::core::ffi::c_int as ogg_uint32_t,
		0x2ba4abe7 as ::core::ffi::c_int as ogg_uint32_t,
		0xfa5d13d4 as ::core::ffi::c_uint,
		0xb50a7bc5 as ::core::ffi::c_uint,
		0x9488f9e9 as ::core::ffi::c_uint,
		0xdbdf91f8 as ::core::ffi::c_uint,
		0xa2629cb as ::core::ffi::c_int as ogg_uint32_t,
		0x457141da as ::core::ffi::c_int as ogg_uint32_t,
		0xad14441a as ::core::ffi::c_uint,
		0xe2432c0b as ::core::ffi::c_uint,
		0x33ba9438 as ::core::ffi::c_int as ogg_uint32_t,
		0x7cedfc29 as ::core::ffi::c_int as ogg_uint32_t,
		0xe7b1820f as ::core::ffi::c_uint,
		0xa8e6ea1e as ::core::ffi::c_uint,
		0x791f522d as ::core::ffi::c_int as ogg_uint32_t,
		0x36483a3c as ::core::ffi::c_int as ogg_uint32_t,
		0xde2d3ffc as ::core::ffi::c_uint,
		0x917a57ed as ::core::ffi::c_uint,
		0x4083efde as ::core::ffi::c_int as ogg_uint32_t,
		0xfd487cf as ::core::ffi::c_int as ogg_uint32_t,
		0x72fa0e25 as ::core::ffi::c_int as ogg_uint32_t,
		0x3dad6634 as ::core::ffi::c_int as ogg_uint32_t,
		0xec54de07 as ::core::ffi::c_uint,
		0xa303b616 as ::core::ffi::c_uint,
		0x4b66b3d6 as ::core::ffi::c_int as ogg_uint32_t,
		0x431dbc7 as ::core::ffi::c_int as ogg_uint32_t,
		0xd5c863f4 as ::core::ffi::c_uint,
		0x9a9f0be5 as ::core::ffi::c_uint,
		0x1c375c3 as ::core::ffi::c_int as ogg_uint32_t,
		0x4e941dd2 as ::core::ffi::c_int as ogg_uint32_t,
		0x9f6da5e1 as ::core::ffi::c_uint,
		0xd03acdf0 as ::core::ffi::c_uint,
		0x385fc830 as ::core::ffi::c_int as ogg_uint32_t,
		0x7708a021 as ::core::ffi::c_int as ogg_uint32_t,
		0xa6f11812 as ::core::ffi::c_uint,
		0xe9a67003 as ::core::ffi::c_uint,
		0x5cac0bc6 as ::core::ffi::c_int as ogg_uint32_t,
		0x13fb63d7 as ::core::ffi::c_int as ogg_uint32_t,
		0xc202dbe4 as ::core::ffi::c_uint,
		0x8d55b3f5 as ::core::ffi::c_uint,
		0x6530b635 as ::core::ffi::c_int as ogg_uint32_t,
		0x2a67de24 as ::core::ffi::c_int as ogg_uint32_t,
		0xfb9e6617 as ::core::ffi::c_uint,
		0xb4c90e06 as ::core::ffi::c_uint,
		0x2f957020 as ::core::ffi::c_int as ogg_uint32_t,
		0x60c21831 as ::core::ffi::c_int as ogg_uint32_t,
		0xb13ba002 as ::core::ffi::c_uint,
		0xfe6cc813 as ::core::ffi::c_uint,
		0x1609cdd3 as ::core::ffi::c_int as ogg_uint32_t,
		0x595ea5c2 as ::core::ffi::c_int as ogg_uint32_t,
		0x88a71df1 as ::core::ffi::c_uint,
		0xc7f075e0 as ::core::ffi::c_uint,
		0xbadefc0a as ::core::ffi::c_uint,
		0xf589941b as ::core::ffi::c_uint,
		0x24702c28 as ::core::ffi::c_int as ogg_uint32_t,
		0x6b274439 as ::core::ffi::c_int as ogg_uint32_t,
		0x834241f9 as ::core::ffi::c_uint,
		0xcc1529e8 as ::core::ffi::c_uint,
		0x1dec91db as ::core::ffi::c_int as ogg_uint32_t,
		0x52bbf9ca as ::core::ffi::c_int as ogg_uint32_t,
		0xc9e787ec as ::core::ffi::c_uint,
		0x86b0effd as ::core::ffi::c_uint,
		0x574957ce as ::core::ffi::c_int as ogg_uint32_t,
		0x181e3fdf as ::core::ffi::c_int as ogg_uint32_t,
		0xf07b3a1f as ::core::ffi::c_uint,
		0xbf2c520e as ::core::ffi::c_uint,
		0x6ed5ea3d as ::core::ffi::c_int as ogg_uint32_t,
		0x2182822c as ::core::ffi::c_int as ogg_uint32_t,
		0x2dd0ee65 as ::core::ffi::c_int as ogg_uint32_t,
		0x62878674 as ::core::ffi::c_int as ogg_uint32_t,
		0xb37e3e47 as ::core::ffi::c_uint,
		0xfc295656 as ::core::ffi::c_uint,
		0x144c5396 as ::core::ffi::c_int as ogg_uint32_t,
		0x5b1b3b87 as ::core::ffi::c_int as ogg_uint32_t,
		0x8ae283b4 as ::core::ffi::c_uint,
		0xc5b5eba5 as ::core::ffi::c_uint,
		0x5ee99583 as ::core::ffi::c_int as ogg_uint32_t,
		0x11befd92 as ::core::ffi::c_int as ogg_uint32_t,
		0xc04745a1 as ::core::ffi::c_uint,
		0x8f102db0 as ::core::ffi::c_uint,
		0x67752870 as ::core::ffi::c_int as ogg_uint32_t,
		0x28224061 as ::core::ffi::c_int as ogg_uint32_t,
		0xf9dbf852 as ::core::ffi::c_uint,
		0xb68c9043 as ::core::ffi::c_uint,
		0xcba219a9 as ::core::ffi::c_uint,
		0x84f571b8 as ::core::ffi::c_uint,
		0x550cc98b as ::core::ffi::c_int as ogg_uint32_t,
		0x1a5ba19a as ::core::ffi::c_int as ogg_uint32_t,
		0xf23ea45a as ::core::ffi::c_uint,
		0xbd69cc4b as ::core::ffi::c_uint,
		0x6c907478 as ::core::ffi::c_int as ogg_uint32_t,
		0x23c71c69 as ::core::ffi::c_int as ogg_uint32_t,
		0xb89b624f as ::core::ffi::c_uint,
		0xf7cc0a5e as ::core::ffi::c_uint,
		0x2635b26d as ::core::ffi::c_int as ogg_uint32_t,
		0x6962da7c as ::core::ffi::c_int as ogg_uint32_t,
		0x8107dfbc as ::core::ffi::c_uint,
		0xce50b7ad as ::core::ffi::c_uint,
		0x1fa90f9e as ::core::ffi::c_int as ogg_uint32_t,
		0x50fe678f as ::core::ffi::c_int as ogg_uint32_t,
		0xe5f41c4a as ::core::ffi::c_uint,
		0xaaa3745b as ::core::ffi::c_uint,
		0x7b5acc68 as ::core::ffi::c_int as ogg_uint32_t,
		0x340da479 as ::core::ffi::c_int as ogg_uint32_t,
		0xdc68a1b9 as ::core::ffi::c_uint,
		0x933fc9a8 as ::core::ffi::c_uint,
		0x42c6719b as ::core::ffi::c_int as ogg_uint32_t,
		0xd91198a as ::core::ffi::c_int as ogg_uint32_t,
		0x96cd67ac as ::core::ffi::c_uint,
		0xd99a0fbd as ::core::ffi::c_uint,
		0x863b78e as ::core::ffi::c_int as ogg_uint32_t,
		0x4734df9f as ::core::ffi::c_int as ogg_uint32_t,
		0xaf51da5f as ::core::ffi::c_uint,
		0xe006b24e as ::core::ffi::c_uint,
		0x31ff0a7d as ::core::ffi::c_int as ogg_uint32_t,
		0x7ea8626c as ::core::ffi::c_int as ogg_uint32_t,
		0x386eb86 as ::core::ffi::c_int as ogg_uint32_t,
		0x4cd18397 as ::core::ffi::c_int as ogg_uint32_t,
		0x9d283ba4 as ::core::ffi::c_uint,
		0xd27f53b5 as ::core::ffi::c_uint,
		0x3a1a5675 as ::core::ffi::c_int as ogg_uint32_t,
		0x754d3e64 as ::core::ffi::c_int as ogg_uint32_t,
		0xa4b48657 as ::core::ffi::c_uint,
		0xebe3ee46 as ::core::ffi::c_uint,
		0x70bf9060 as ::core::ffi::c_int as ogg_uint32_t,
		0x3fe8f871 as ::core::ffi::c_int as ogg_uint32_t,
		0xee114042 as ::core::ffi::c_uint,
		0xa1462853 as ::core::ffi::c_uint,
		0x49232d93 as ::core::ffi::c_int as ogg_uint32_t,
		0x6744582 as ::core::ffi::c_int as ogg_uint32_t,
		0xd78dfdb1 as ::core::ffi::c_uint,
		0x98da95a0 as ::core::ffi::c_uint,
		0xb958178c as ::core::ffi::c_uint,
		0xf60f7f9d as ::core::ffi::c_uint,
		0x27f6c7ae as ::core::ffi::c_int as ogg_uint32_t,
		0x68a1afbf as ::core::ffi::c_int as ogg_uint32_t,
		0x80c4aa7f as ::core::ffi::c_uint,
		0xcf93c26e as ::core::ffi::c_uint,
		0x1e6a7a5d as ::core::ffi::c_int as ogg_uint32_t,
		0x513d124c as ::core::ffi::c_int as ogg_uint32_t,
		0xca616c6a as ::core::ffi::c_uint,
		0x8536047b as ::core::ffi::c_uint,
		0x54cfbc48 as ::core::ffi::c_int as ogg_uint32_t,
		0x1b98d459 as ::core::ffi::c_int as ogg_uint32_t,
		0xf3fdd199 as ::core::ffi::c_uint,
		0xbcaab988 as ::core::ffi::c_uint,
		0x6d5301bb as ::core::ffi::c_int as ogg_uint32_t,
		0x220469aa as ::core::ffi::c_int as ogg_uint32_t,
		0x5f2ae040 as ::core::ffi::c_int as ogg_uint32_t,
		0x107d8851 as ::core::ffi::c_int as ogg_uint32_t,
		0xc1843062 as ::core::ffi::c_uint,
		0x8ed35873 as ::core::ffi::c_uint,
		0x66b65db3 as ::core::ffi::c_int as ogg_uint32_t,
		0x29e135a2 as ::core::ffi::c_int as ogg_uint32_t,
		0xf8188d91 as ::core::ffi::c_uint,
		0xb74fe580 as ::core::ffi::c_uint,
		0x2c139ba6 as ::core::ffi::c_int as ogg_uint32_t,
		0x6344f3b7 as ::core::ffi::c_int as ogg_uint32_t,
		0xb2bd4b84 as ::core::ffi::c_uint,
		0xfdea2395 as ::core::ffi::c_uint,
		0x158f2655 as ::core::ffi::c_int as ogg_uint32_t,
		0x5ad84e44 as ::core::ffi::c_int as ogg_uint32_t,
		0x8b21f677 as ::core::ffi::c_uint,
		0xc4769e66 as ::core::ffi::c_uint,
		0x717ce5a3 as ::core::ffi::c_int as ogg_uint32_t,
		0x3e2b8db2 as ::core::ffi::c_int as ogg_uint32_t,
		0xefd23581 as ::core::ffi::c_uint,
		0xa0855d90 as ::core::ffi::c_uint,
		0x48e05850 as ::core::ffi::c_int as ogg_uint32_t,
		0x7b73041 as ::core::ffi::c_int as ogg_uint32_t,
		0xd64e8872 as ::core::ffi::c_uint,
		0x9919e063 as ::core::ffi::c_uint,
		0x2459e45 as ::core::ffi::c_int as ogg_uint32_t,
		0x4d12f654 as ::core::ffi::c_int as ogg_uint32_t,
		0x9ceb4e67 as ::core::ffi::c_uint,
		0xd3bc2676 as ::core::ffi::c_uint,
		0x3bd923b6 as ::core::ffi::c_int as ogg_uint32_t,
		0x748e4ba7 as ::core::ffi::c_int as ogg_uint32_t,
		0xa577f394 as ::core::ffi::c_uint,
		0xea209b85 as ::core::ffi::c_uint,
		0x970e126f as ::core::ffi::c_uint,
		0xd8597a7e as ::core::ffi::c_uint,
		0x9a0c24d as ::core::ffi::c_int as ogg_uint32_t,
		0x46f7aa5c as ::core::ffi::c_int as ogg_uint32_t,
		0xae92af9c as ::core::ffi::c_uint,
		0xe1c5c78d as ::core::ffi::c_uint,
		0x303c7fbe as ::core::ffi::c_int as ogg_uint32_t,
		0x7f6b17af as ::core::ffi::c_int as ogg_uint32_t,
		0xe4376989 as ::core::ffi::c_uint,
		0xab600198 as ::core::ffi::c_uint,
		0x7a99b9ab as ::core::ffi::c_int as ogg_uint32_t,
		0x35ced1ba as ::core::ffi::c_int as ogg_uint32_t,
		0xddabd47a as ::core::ffi::c_uint,
		0x92fcbc6b as ::core::ffi::c_uint,
		0x43050458 as ::core::ffi::c_int as ogg_uint32_t,
		0xc526c49 as ::core::ffi::c_int as ogg_uint32_t
	],
	[
		0 as ::core::ffi::c_int as ogg_uint32_t,
		0x5ba1dcca as ::core::ffi::c_int as ogg_uint32_t,
		0xb743b994 as ::core::ffi::c_uint,
		0xece2655e as ::core::ffi::c_uint,
		0x6a466e9f as ::core::ffi::c_int as ogg_uint32_t,
		0x31e7b255 as ::core::ffi::c_int as ogg_uint32_t,
		0xdd05d70b as ::core::ffi::c_uint,
		0x86a40bc1 as ::core::ffi::c_uint,
		0xd48cdd3e as ::core::ffi::c_uint,
		0x8f2d01f4 as ::core::ffi::c_uint,
		0x63cf64aa as ::core::ffi::c_int as ogg_uint32_t,
		0x386eb860 as ::core::ffi::c_int as ogg_uint32_t,
		0xbecab3a1 as ::core::ffi::c_uint,
		0xe56b6f6b as ::core::ffi::c_uint,
		0x9890a35 as ::core::ffi::c_int as ogg_uint32_t,
		0x5228d6ff as ::core::ffi::c_int as ogg_uint32_t,
		0xadd8a7cb as ::core::ffi::c_uint,
		0xf6797b01 as ::core::ffi::c_uint,
		0x1a9b1e5f as ::core::ffi::c_int as ogg_uint32_t,
		0x413ac295 as ::core::ffi::c_int as ogg_uint32_t,
		0xc79ec954 as ::core::ffi::c_uint,
		0x9c3f159e as ::core::ffi::c_uint,
		0x70dd70c0 as ::core::ffi::c_int as ogg_uint32_t,
		0x2b7cac0a as ::core::ffi::c_int as ogg_uint32_t,
		0x79547af5 as ::core::ffi::c_int as ogg_uint32_t,
		0x22f5a63f as ::core::ffi::c_int as ogg_uint32_t,
		0xce17c361 as ::core::ffi::c_uint,
		0x95b61fab as ::core::ffi::c_uint,
		0x1312146a as ::core::ffi::c_int as ogg_uint32_t,
		0x48b3c8a0 as ::core::ffi::c_int as ogg_uint32_t,
		0xa451adfe as ::core::ffi::c_uint,
		0xfff07134 as ::core::ffi::c_uint,
		0x5f705221 as ::core::ffi::c_int as ogg_uint32_t,
		0x4d18eeb as ::core::ffi::c_int as ogg_uint32_t,
		0xe833ebb5 as ::core::ffi::c_uint,
		0xb392377f as ::core::ffi::c_uint,
		0x35363cbe as ::core::ffi::c_int as ogg_uint32_t,
		0x6e97e074 as ::core::ffi::c_int as ogg_uint32_t,
		0x8275852a as ::core::ffi::c_uint,
		0xd9d459e0 as ::core::ffi::c_uint,
		0x8bfc8f1f as ::core::ffi::c_uint,
		0xd05d53d5 as ::core::ffi::c_uint,
		0x3cbf368b as ::core::ffi::c_int as ogg_uint32_t,
		0x671eea41 as ::core::ffi::c_int as ogg_uint32_t,
		0xe1bae180 as ::core::ffi::c_uint,
		0xba1b3d4a as ::core::ffi::c_uint,
		0x56f95814 as ::core::ffi::c_int as ogg_uint32_t,
		0xd5884de as ::core::ffi::c_int as ogg_uint32_t,
		0xf2a8f5ea as ::core::ffi::c_uint,
		0xa9092920 as ::core::ffi::c_uint,
		0x45eb4c7e as ::core::ffi::c_int as ogg_uint32_t,
		0x1e4a90b4 as ::core::ffi::c_int as ogg_uint32_t,
		0x98ee9b75 as ::core::ffi::c_uint,
		0xc34f47bf as ::core::ffi::c_uint,
		0x2fad22e1 as ::core::ffi::c_int as ogg_uint32_t,
		0x740cfe2b as ::core::ffi::c_int as ogg_uint32_t,
		0x262428d4 as ::core::ffi::c_int as ogg_uint32_t,
		0x7d85f41e as ::core::ffi::c_int as ogg_uint32_t,
		0x91679140 as ::core::ffi::c_uint,
		0xcac64d8a as ::core::ffi::c_uint,
		0x4c62464b as ::core::ffi::c_int as ogg_uint32_t,
		0x17c39a81 as ::core::ffi::c_int as ogg_uint32_t,
		0xfb21ffdf as ::core::ffi::c_uint,
		0xa0802315 as ::core::ffi::c_uint,
		0xbee0a442 as ::core::ffi::c_uint,
		0xe5417888 as ::core::ffi::c_uint,
		0x9a31dd6 as ::core::ffi::c_int as ogg_uint32_t,
		0x5202c11c as ::core::ffi::c_int as ogg_uint32_t,
		0xd4a6cadd as ::core::ffi::c_uint,
		0x8f071617 as ::core::ffi::c_uint,
		0x63e57349 as ::core::ffi::c_int as ogg_uint32_t,
		0x3844af83 as ::core::ffi::c_int as ogg_uint32_t,
		0x6a6c797c as ::core::ffi::c_int as ogg_uint32_t,
		0x31cda5b6 as ::core::ffi::c_int as ogg_uint32_t,
		0xdd2fc0e8 as ::core::ffi::c_uint,
		0x868e1c22 as ::core::ffi::c_uint,
		0x2a17e3 as ::core::ffi::c_int as ogg_uint32_t,
		0x5b8bcb29 as ::core::ffi::c_int as ogg_uint32_t,
		0xb769ae77 as ::core::ffi::c_uint,
		0xecc872bd as ::core::ffi::c_uint,
		0x13380389 as ::core::ffi::c_int as ogg_uint32_t,
		0x4899df43 as ::core::ffi::c_int as ogg_uint32_t,
		0xa47bba1d as ::core::ffi::c_uint,
		0xffda66d7 as ::core::ffi::c_uint,
		0x797e6d16 as ::core::ffi::c_int as ogg_uint32_t,
		0x22dfb1dc as ::core::ffi::c_int as ogg_uint32_t,
		0xce3dd482 as ::core::ffi::c_uint,
		0x959c0848 as ::core::ffi::c_uint,
		0xc7b4deb7 as ::core::ffi::c_uint,
		0x9c15027d as ::core::ffi::c_uint,
		0x70f76723 as ::core::ffi::c_int as ogg_uint32_t,
		0x2b56bbe9 as ::core::ffi::c_int as ogg_uint32_t,
		0xadf2b028 as ::core::ffi::c_uint,
		0xf6536ce2 as ::core::ffi::c_uint,
		0x1ab109bc as ::core::ffi::c_int as ogg_uint32_t,
		0x4110d576 as ::core::ffi::c_int as ogg_uint32_t,
		0xe190f663 as ::core::ffi::c_uint,
		0xba312aa9 as ::core::ffi::c_uint,
		0x56d34ff7 as ::core::ffi::c_int as ogg_uint32_t,
		0xd72933d as ::core::ffi::c_int as ogg_uint32_t,
		0x8bd698fc as ::core::ffi::c_uint,
		0xd0774436 as ::core::ffi::c_uint,
		0x3c952168 as ::core::ffi::c_int as ogg_uint32_t,
		0x6734fda2 as ::core::ffi::c_int as ogg_uint32_t,
		0x351c2b5d as ::core::ffi::c_int as ogg_uint32_t,
		0x6ebdf797 as ::core::ffi::c_int as ogg_uint32_t,
		0x825f92c9 as ::core::ffi::c_uint,
		0xd9fe4e03 as ::core::ffi::c_uint,
		0x5f5a45c2 as ::core::ffi::c_int as ogg_uint32_t,
		0x4fb9908 as ::core::ffi::c_int as ogg_uint32_t,
		0xe819fc56 as ::core::ffi::c_uint,
		0xb3b8209c as ::core::ffi::c_uint,
		0x4c4851a8 as ::core::ffi::c_int as ogg_uint32_t,
		0x17e98d62 as ::core::ffi::c_int as ogg_uint32_t,
		0xfb0be83c as ::core::ffi::c_uint,
		0xa0aa34f6 as ::core::ffi::c_uint,
		0x260e3f37 as ::core::ffi::c_int as ogg_uint32_t,
		0x7dafe3fd as ::core::ffi::c_int as ogg_uint32_t,
		0x914d86a3 as ::core::ffi::c_uint,
		0xcaec5a69 as ::core::ffi::c_uint,
		0x98c48c96 as ::core::ffi::c_uint,
		0xc365505c as ::core::ffi::c_uint,
		0x2f873502 as ::core::ffi::c_int as ogg_uint32_t,
		0x7426e9c8 as ::core::ffi::c_int as ogg_uint32_t,
		0xf282e209 as ::core::ffi::c_uint,
		0xa9233ec3 as ::core::ffi::c_uint,
		0x45c15b9d as ::core::ffi::c_int as ogg_uint32_t,
		0x1e608757 as ::core::ffi::c_int as ogg_uint32_t,
		0x79005533 as ::core::ffi::c_int as ogg_uint32_t,
		0x22a189f9 as ::core::ffi::c_int as ogg_uint32_t,
		0xce43eca7 as ::core::ffi::c_uint,
		0x95e2306d as ::core::ffi::c_uint,
		0x13463bac as ::core::ffi::c_int as ogg_uint32_t,
		0x48e7e766 as ::core::ffi::c_int as ogg_uint32_t,
		0xa4058238 as ::core::ffi::c_uint,
		0xffa45ef2 as ::core::ffi::c_uint,
		0xad8c880d as ::core::ffi::c_uint,
		0xf62d54c7 as ::core::ffi::c_uint,
		0x1acf3199 as ::core::ffi::c_int as ogg_uint32_t,
		0x416eed53 as ::core::ffi::c_int as ogg_uint32_t,
		0xc7cae692 as ::core::ffi::c_uint,
		0x9c6b3a58 as ::core::ffi::c_uint,
		0x70895f06 as ::core::ffi::c_int as ogg_uint32_t,
		0x2b2883cc as ::core::ffi::c_int as ogg_uint32_t,
		0xd4d8f2f8 as ::core::ffi::c_uint,
		0x8f792e32 as ::core::ffi::c_uint,
		0x639b4b6c as ::core::ffi::c_int as ogg_uint32_t,
		0x383a97a6 as ::core::ffi::c_int as ogg_uint32_t,
		0xbe9e9c67 as ::core::ffi::c_uint,
		0xe53f40ad as ::core::ffi::c_uint,
		0x9dd25f3 as ::core::ffi::c_int as ogg_uint32_t,
		0x527cf939 as ::core::ffi::c_int as ogg_uint32_t,
		0x542fc6 as ::core::ffi::c_int as ogg_uint32_t,
		0x5bf5f30c as ::core::ffi::c_int as ogg_uint32_t,
		0xb7179652 as ::core::ffi::c_uint,
		0xecb64a98 as ::core::ffi::c_uint,
		0x6a124159 as ::core::ffi::c_int as ogg_uint32_t,
		0x31b39d93 as ::core::ffi::c_int as ogg_uint32_t,
		0xdd51f8cd as ::core::ffi::c_uint,
		0x86f02407 as ::core::ffi::c_uint,
		0x26700712 as ::core::ffi::c_int as ogg_uint32_t,
		0x7dd1dbd8 as ::core::ffi::c_int as ogg_uint32_t,
		0x9133be86 as ::core::ffi::c_uint,
		0xca92624c as ::core::ffi::c_uint,
		0x4c36698d as ::core::ffi::c_int as ogg_uint32_t,
		0x1797b547 as ::core::ffi::c_int as ogg_uint32_t,
		0xfb75d019 as ::core::ffi::c_uint,
		0xa0d40cd3 as ::core::ffi::c_uint,
		0xf2fcda2c as ::core::ffi::c_uint,
		0xa95d06e6 as ::core::ffi::c_uint,
		0x45bf63b8 as ::core::ffi::c_int as ogg_uint32_t,
		0x1e1ebf72 as ::core::ffi::c_int as ogg_uint32_t,
		0x98bab4b3 as ::core::ffi::c_uint,
		0xc31b6879 as ::core::ffi::c_uint,
		0x2ff90d27 as ::core::ffi::c_int as ogg_uint32_t,
		0x7458d1ed as ::core::ffi::c_int as ogg_uint32_t,
		0x8ba8a0d9 as ::core::ffi::c_uint,
		0xd0097c13 as ::core::ffi::c_uint,
		0x3ceb194d as ::core::ffi::c_int as ogg_uint32_t,
		0x674ac587 as ::core::ffi::c_int as ogg_uint32_t,
		0xe1eece46 as ::core::ffi::c_uint,
		0xba4f128c as ::core::ffi::c_uint,
		0x56ad77d2 as ::core::ffi::c_int as ogg_uint32_t,
		0xd0cab18 as ::core::ffi::c_int as ogg_uint32_t,
		0x5f247de7 as ::core::ffi::c_int as ogg_uint32_t,
		0x485a12d as ::core::ffi::c_int as ogg_uint32_t,
		0xe867c473 as ::core::ffi::c_uint,
		0xb3c618b9 as ::core::ffi::c_uint,
		0x35621378 as ::core::ffi::c_int as ogg_uint32_t,
		0x6ec3cfb2 as ::core::ffi::c_int as ogg_uint32_t,
		0x8221aaec as ::core::ffi::c_uint,
		0xd9807626 as ::core::ffi::c_uint,
		0xc7e0f171 as ::core::ffi::c_uint,
		0x9c412dbb as ::core::ffi::c_uint,
		0x70a348e5 as ::core::ffi::c_int as ogg_uint32_t,
		0x2b02942f as ::core::ffi::c_int as ogg_uint32_t,
		0xada69fee as ::core::ffi::c_uint,
		0xf6074324 as ::core::ffi::c_uint,
		0x1ae5267a as ::core::ffi::c_int as ogg_uint32_t,
		0x4144fab0 as ::core::ffi::c_int as ogg_uint32_t,
		0x136c2c4f as ::core::ffi::c_int as ogg_uint32_t,
		0x48cdf085 as ::core::ffi::c_int as ogg_uint32_t,
		0xa42f95db as ::core::ffi::c_uint,
		0xff8e4911 as ::core::ffi::c_uint,
		0x792a42d0 as ::core::ffi::c_int as ogg_uint32_t,
		0x228b9e1a as ::core::ffi::c_int as ogg_uint32_t,
		0xce69fb44 as ::core::ffi::c_uint,
		0x95c8278e as ::core::ffi::c_uint,
		0x6a3856ba as ::core::ffi::c_int as ogg_uint32_t,
		0x31998a70 as ::core::ffi::c_int as ogg_uint32_t,
		0xdd7bef2e as ::core::ffi::c_uint,
		0x86da33e4 as ::core::ffi::c_uint,
		0x7e3825 as ::core::ffi::c_int as ogg_uint32_t,
		0x5bdfe4ef as ::core::ffi::c_int as ogg_uint32_t,
		0xb73d81b1 as ::core::ffi::c_uint,
		0xec9c5d7b as ::core::ffi::c_uint,
		0xbeb48b84 as ::core::ffi::c_uint,
		0xe515574e as ::core::ffi::c_uint,
		0x9f73210 as ::core::ffi::c_int as ogg_uint32_t,
		0x5256eeda as ::core::ffi::c_int as ogg_uint32_t,
		0xd4f2e51b as ::core::ffi::c_uint,
		0x8f5339d1 as ::core::ffi::c_uint,
		0x63b15c8f as ::core::ffi::c_int as ogg_uint32_t,
		0x38108045 as ::core::ffi::c_int as ogg_uint32_t,
		0x9890a350 as ::core::ffi::c_uint,
		0xc3317f9a as ::core::ffi::c_uint,
		0x2fd31ac4 as ::core::ffi::c_int as ogg_uint32_t,
		0x7472c60e as ::core::ffi::c_int as ogg_uint32_t,
		0xf2d6cdcf as ::core::ffi::c_uint,
		0xa9771105 as ::core::ffi::c_uint,
		0x4595745b as ::core::ffi::c_int as ogg_uint32_t,
		0x1e34a891 as ::core::ffi::c_int as ogg_uint32_t,
		0x4c1c7e6e as ::core::ffi::c_int as ogg_uint32_t,
		0x17bda2a4 as ::core::ffi::c_int as ogg_uint32_t,
		0xfb5fc7fa as ::core::ffi::c_uint,
		0xa0fe1b30 as ::core::ffi::c_uint,
		0x265a10f1 as ::core::ffi::c_int as ogg_uint32_t,
		0x7dfbcc3b as ::core::ffi::c_int as ogg_uint32_t,
		0x9119a965 as ::core::ffi::c_uint,
		0xcab875af as ::core::ffi::c_uint,
		0x3548049b as ::core::ffi::c_int as ogg_uint32_t,
		0x6ee9d851 as ::core::ffi::c_int as ogg_uint32_t,
		0x820bbd0f as ::core::ffi::c_uint,
		0xd9aa61c5 as ::core::ffi::c_uint,
		0x5f0e6a04 as ::core::ffi::c_int as ogg_uint32_t,
		0x4afb6ce as ::core::ffi::c_int as ogg_uint32_t,
		0xe84dd390 as ::core::ffi::c_uint,
		0xb3ec0f5a as ::core::ffi::c_uint,
		0xe1c4d9a5 as ::core::ffi::c_uint,
		0xba65056f as ::core::ffi::c_uint,
		0x56876031 as ::core::ffi::c_int as ogg_uint32_t,
		0xd26bcfb as ::core::ffi::c_int as ogg_uint32_t,
		0x8b82b73a as ::core::ffi::c_uint,
		0xd0236bf0 as ::core::ffi::c_uint,
		0x3cc10eae as ::core::ffi::c_int as ogg_uint32_t,
		0x6760d264 as ::core::ffi::c_int as ogg_uint32_t
	]
];
#[no_mangle]
pub unsafe extern "C" fn ogg_stream_init(
	mut os: *mut ogg_stream_state,
	mut serialno: ::core::ffi::c_int
) -> ::core::ffi::c_int {
	if !os.is_null() {
		memset(
			os as *mut ::core::ffi::c_void,
			0 as ::core::ffi::c_int,
			::core::mem::size_of::<ogg_stream_state>() as size_t
		);
		(*os).body_storage =
			(16 as ::core::ffi::c_int * 1024 as ::core::ffi::c_int) as ::core::ffi::c_long;
		(*os).lacing_storage = 1024 as ::core::ffi::c_long;
		(*os).body_data = malloc(
			((*os).body_storage as size_t)
				.wrapping_mul(::core::mem::size_of::<::core::ffi::c_uchar>() as size_t)
		) as *mut ::core::ffi::c_uchar;
		(*os).lacing_vals = malloc(
			((*os).lacing_storage as size_t)
				.wrapping_mul(::core::mem::size_of::<::core::ffi::c_int>() as size_t)
		) as *mut ::core::ffi::c_int;
		(*os).granule_vals = malloc(
			((*os).lacing_storage as size_t)
				.wrapping_mul(::core::mem::size_of::<ogg_int64_t>() as size_t)
		) as *mut ogg_int64_t;
		if (*os).body_data.is_null() || (*os).lacing_vals.is_null() || (*os).granule_vals.is_null()
		{
			ogg_stream_clear(os);
			return -(1 as ::core::ffi::c_int);
		}
		(*os).serialno = serialno as ::core::ffi::c_long;
		return 0 as ::core::ffi::c_int;
	}
	return -(1 as ::core::ffi::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn ogg_stream_check(mut os: *mut ogg_stream_state) -> ::core::ffi::c_int {
	if os.is_null() || (*os).body_data.is_null() {
		return -(1 as ::core::ffi::c_int);
	}
	return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ogg_stream_clear(mut os: *mut ogg_stream_state) -> ::core::ffi::c_int {
	if !os.is_null() {
		if !(*os).body_data.is_null() {
			free((*os).body_data as *mut ::core::ffi::c_void);
		}
		if !(*os).lacing_vals.is_null() {
			free((*os).lacing_vals as *mut ::core::ffi::c_void);
		}
		if !(*os).granule_vals.is_null() {
			free((*os).granule_vals as *mut ::core::ffi::c_void);
		}
		memset(
			os as *mut ::core::ffi::c_void,
			0 as ::core::ffi::c_int,
			::core::mem::size_of::<ogg_stream_state>() as size_t
		);
	}
	return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ogg_stream_destroy(mut os: *mut ogg_stream_state) -> ::core::ffi::c_int {
	if !os.is_null() {
		ogg_stream_clear(os);
		free(os as *mut ::core::ffi::c_void);
	}
	return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn _os_body_expand(
	mut os: *mut ogg_stream_state,
	mut needed: ::core::ffi::c_long
) -> ::core::ffi::c_int {
	if (*os).body_storage - needed <= (*os).body_fill {
		let mut body_storage: ::core::ffi::c_long = 0;
		let mut ret: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
		if (*os).body_storage > LONG_MAX - needed {
			ogg_stream_clear(os);
			return -(1 as ::core::ffi::c_int);
		}
		body_storage = (*os).body_storage + needed;
		if body_storage < LONG_MAX - 1024 as ::core::ffi::c_long {
			body_storage += 1024 as ::core::ffi::c_long;
		}
		ret = realloc(
			(*os).body_data as *mut ::core::ffi::c_void,
			(body_storage as size_t)
				.wrapping_mul(::core::mem::size_of::<::core::ffi::c_uchar>() as size_t)
		);
		if ret.is_null() {
			ogg_stream_clear(os);
			return -(1 as ::core::ffi::c_int);
		}
		(*os).body_storage = body_storage;
		(*os).body_data = ret as *mut ::core::ffi::c_uchar;
	}
	return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn _os_lacing_expand(
	mut os: *mut ogg_stream_state,
	mut needed: ::core::ffi::c_long
) -> ::core::ffi::c_int {
	if (*os).lacing_storage - needed <= (*os).lacing_fill {
		let mut lacing_storage: ::core::ffi::c_long = 0;
		let mut ret: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
		if (*os).lacing_storage > LONG_MAX - needed {
			ogg_stream_clear(os);
			return -(1 as ::core::ffi::c_int);
		}
		lacing_storage = (*os).lacing_storage + needed;
		if lacing_storage < LONG_MAX - 32 as ::core::ffi::c_long {
			lacing_storage += 32 as ::core::ffi::c_long;
		}
		ret = realloc(
			(*os).lacing_vals as *mut ::core::ffi::c_void,
			(lacing_storage as size_t)
				.wrapping_mul(::core::mem::size_of::<::core::ffi::c_int>() as size_t)
		);
		if ret.is_null() {
			ogg_stream_clear(os);
			return -(1 as ::core::ffi::c_int);
		}
		(*os).lacing_vals = ret as *mut ::core::ffi::c_int;
		ret = realloc(
			(*os).granule_vals as *mut ::core::ffi::c_void,
			(lacing_storage as size_t)
				.wrapping_mul(::core::mem::size_of::<ogg_int64_t>() as size_t)
		);
		if ret.is_null() {
			ogg_stream_clear(os);
			return -(1 as ::core::ffi::c_int);
		}
		(*os).granule_vals = ret as *mut ogg_int64_t;
		(*os).lacing_storage = lacing_storage;
	}
	return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn _os_update_crc(
	mut crc: ogg_uint32_t,
	mut buffer: *mut ::core::ffi::c_uchar,
	mut size: ::core::ffi::c_int
) -> ogg_uint32_t {
	while size >= 8 as ::core::ffi::c_int {
		crc ^= (*buffer.offset(0 as ::core::ffi::c_int as isize) as ogg_uint32_t)
			<< 24 as ::core::ffi::c_int
			| (*buffer.offset(1 as ::core::ffi::c_int as isize) as ogg_uint32_t)
				<< 16 as ::core::ffi::c_int
			| (*buffer.offset(2 as ::core::ffi::c_int as isize) as ogg_uint32_t)
				<< 8 as ::core::ffi::c_int
			| *buffer.offset(3 as ::core::ffi::c_int as isize) as ogg_uint32_t;
		crc = crc_lookup[7 as ::core::ffi::c_int as usize]
			[(crc >> 24 as ::core::ffi::c_int) as usize]
			^ crc_lookup[6 as ::core::ffi::c_int as usize]
				[(crc >> 16 as ::core::ffi::c_int & 0xff as ogg_uint32_t) as usize]
			^ crc_lookup[5 as ::core::ffi::c_int as usize]
				[(crc >> 8 as ::core::ffi::c_int & 0xff as ogg_uint32_t) as usize]
			^ crc_lookup[4 as ::core::ffi::c_int as usize][(crc & 0xff as ogg_uint32_t) as usize]
			^ crc_lookup[3 as ::core::ffi::c_int as usize]
				[*buffer.offset(4 as ::core::ffi::c_int as isize) as usize]
			^ crc_lookup[2 as ::core::ffi::c_int as usize]
				[*buffer.offset(5 as ::core::ffi::c_int as isize) as usize]
			^ crc_lookup[1 as ::core::ffi::c_int as usize]
				[*buffer.offset(6 as ::core::ffi::c_int as isize) as usize]
			^ crc_lookup[0 as ::core::ffi::c_int as usize]
				[*buffer.offset(7 as ::core::ffi::c_int as isize) as usize];
		buffer = buffer.offset(8 as ::core::ffi::c_int as isize);
		size -= 8 as ::core::ffi::c_int;
	}
	loop {
		let fresh2 = size;
		size = size - 1;
		if !(fresh2 != 0) {
			break;
		}
		let fresh3 = buffer;
		buffer = buffer.offset(1);
		crc = crc << 8 as ::core::ffi::c_int
			^ crc_lookup[0 as ::core::ffi::c_int as usize][(crc >> 24 as ::core::ffi::c_int
				& 0xff as ogg_uint32_t
				^ *fresh3 as ogg_uint32_t) as usize];
	}
	return crc;
}
#[no_mangle]
pub unsafe extern "C" fn ogg_page_checksum_set(mut og: *mut ogg_page) {
	if !og.is_null() {
		let mut crc_reg: ogg_uint32_t = 0 as ogg_uint32_t;
		*(*og).header.offset(22 as ::core::ffi::c_int as isize) = 0 as ::core::ffi::c_uchar;
		*(*og).header.offset(23 as ::core::ffi::c_int as isize) = 0 as ::core::ffi::c_uchar;
		*(*og).header.offset(24 as ::core::ffi::c_int as isize) = 0 as ::core::ffi::c_uchar;
		*(*og).header.offset(25 as ::core::ffi::c_int as isize) = 0 as ::core::ffi::c_uchar;
		crc_reg = _os_update_crc(
			crc_reg,
			(*og).header,
			(*og).header_len as ::core::ffi::c_int
		);
		crc_reg = _os_update_crc(crc_reg, (*og).body, (*og).body_len as ::core::ffi::c_int);
		*(*og).header.offset(22 as ::core::ffi::c_int as isize) =
			(crc_reg & 0xff as ogg_uint32_t) as ::core::ffi::c_uchar;
		*(*og).header.offset(23 as ::core::ffi::c_int as isize) =
			(crc_reg >> 8 as ::core::ffi::c_int & 0xff as ogg_uint32_t) as ::core::ffi::c_uchar;
		*(*og).header.offset(24 as ::core::ffi::c_int as isize) =
			(crc_reg >> 16 as ::core::ffi::c_int & 0xff as ogg_uint32_t) as ::core::ffi::c_uchar;
		*(*og).header.offset(25 as ::core::ffi::c_int as isize) =
			(crc_reg >> 24 as ::core::ffi::c_int & 0xff as ogg_uint32_t) as ::core::ffi::c_uchar;
	}
}
#[no_mangle]
pub unsafe extern "C" fn ogg_stream_iovecin(
	mut os: *mut ogg_stream_state,
	mut iov: *mut ogg_iovec_t,
	mut count: ::core::ffi::c_int,
	mut e_o_s: ::core::ffi::c_long,
	mut granulepos: ogg_int64_t
) -> ::core::ffi::c_int {
	let mut bytes: ::core::ffi::c_long = 0 as ::core::ffi::c_long;
	let mut lacing_vals: ::core::ffi::c_long = 0;
	let mut i: ::core::ffi::c_int = 0;
	if ogg_stream_check(os) != 0 {
		return -(1 as ::core::ffi::c_int);
	}
	if iov.is_null() {
		return 0 as ::core::ffi::c_int;
	}
	i = 0 as ::core::ffi::c_int;
	while i < count {
		if (*iov.offset(i as isize)).iov_len > LONG_MAX as size_t {
			return -(1 as ::core::ffi::c_int);
		}
		if bytes > LONG_MAX - (*iov.offset(i as isize)).iov_len as ::core::ffi::c_long {
			return -(1 as ::core::ffi::c_int);
		}
		bytes += (*iov.offset(i as isize)).iov_len as ::core::ffi::c_long;
		i += 1;
	}
	lacing_vals = bytes / 255 as ::core::ffi::c_long + 1 as ::core::ffi::c_long;
	if (*os).body_returned != 0 {
		(*os).body_fill -= (*os).body_returned;
		if (*os).body_fill != 0 {
			::core::ptr::copy(
				(*os).body_data.offset((*os).body_returned as isize),
				(*os).body_data,
				(*os).body_fill as usize
			);
		}
		(*os).body_returned = 0 as ::core::ffi::c_long;
	}
	if _os_body_expand(os, bytes) != 0 || _os_lacing_expand(os, lacing_vals) != 0 {
		return -(1 as ::core::ffi::c_int);
	}
	i = 0 as ::core::ffi::c_int;
	while i < count {
		memcpy(
			(*os).body_data.offset((*os).body_fill as isize) as *mut ::core::ffi::c_void,
			(*iov.offset(i as isize)).iov_base,
			(*iov.offset(i as isize)).iov_len
		);
		(*os).body_fill +=
			(*iov.offset(i as isize)).iov_len as ::core::ffi::c_int as ::core::ffi::c_long;
		i += 1;
	}
	i = 0 as ::core::ffi::c_int;
	while (i as ::core::ffi::c_long) < lacing_vals - 1 as ::core::ffi::c_long {
		*(*os)
			.lacing_vals
			.offset(((*os).lacing_fill + i as ::core::ffi::c_long) as isize) = 255 as ::core::ffi::c_int;
		*(*os)
			.granule_vals
			.offset(((*os).lacing_fill + i as ::core::ffi::c_long) as isize) = (*os).granulepos;
		i += 1;
	}
	*(*os)
		.lacing_vals
		.offset(((*os).lacing_fill + i as ::core::ffi::c_long) as isize) =
		(bytes % 255 as ::core::ffi::c_long) as ::core::ffi::c_int;
	let ref mut fresh0 = *(*os)
		.granule_vals
		.offset(((*os).lacing_fill + i as ::core::ffi::c_long) as isize);
	*fresh0 = granulepos;
	(*os).granulepos = *fresh0;
	*(*os).lacing_vals.offset((*os).lacing_fill as isize) |= 0x100 as ::core::ffi::c_int;
	(*os).lacing_fill += lacing_vals;
	(*os).packetno += 1;
	if e_o_s != 0 {
		(*os).e_o_s = 1 as ::core::ffi::c_int;
	}
	return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ogg_stream_packetin(
	mut os: *mut ogg_stream_state,
	mut op: *mut ogg_packet
) -> ::core::ffi::c_int {
	let mut iov: ogg_iovec_t = ogg_iovec_t {
		iov_base: ::core::ptr::null_mut::<::core::ffi::c_void>(),
		iov_len: 0
	};
	iov.iov_base = (*op).packet as *mut ::core::ffi::c_void;
	iov.iov_len = (*op).bytes as size_t;
	return ogg_stream_iovecin(
		os,
		&raw mut iov,
		1 as ::core::ffi::c_int,
		(*op).e_o_s,
		(*op).granulepos
	);
}
unsafe extern "C" fn ogg_stream_flush_i(
	mut os: *mut ogg_stream_state,
	mut og: *mut ogg_page,
	mut force: ::core::ffi::c_int,
	mut nfill: ::core::ffi::c_int
) -> ::core::ffi::c_int {
	let mut i: ::core::ffi::c_int = 0;
	let mut vals: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
	let mut maxvals: ::core::ffi::c_int = (if (*os).lacing_fill > 255 as ::core::ffi::c_long {
		255 as ::core::ffi::c_long
	} else {
		(*os).lacing_fill
	}) as ::core::ffi::c_int;
	let mut bytes: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
	let mut acc: ::core::ffi::c_long = 0 as ::core::ffi::c_long;
	let mut granule_pos: ogg_int64_t = -(1 as ::core::ffi::c_int) as ogg_int64_t;
	if ogg_stream_check(os) != 0 {
		return 0 as ::core::ffi::c_int;
	}
	if maxvals == 0 as ::core::ffi::c_int {
		return 0 as ::core::ffi::c_int;
	}
	if (*os).b_o_s == 0 as ::core::ffi::c_int {
		granule_pos = 0 as ogg_int64_t;
		vals = 0 as ::core::ffi::c_int;
		while vals < maxvals {
			if (*(*os).lacing_vals.offset(vals as isize) & 0xff as ::core::ffi::c_int)
				< 255 as ::core::ffi::c_int
			{
				vals += 1;
				break;
			} else {
				vals += 1;
			}
		}
	} else {
		let mut packets_done: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
		let mut packet_just_done: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
		vals = 0 as ::core::ffi::c_int;
		while vals < maxvals {
			if acc > nfill as ::core::ffi::c_long && packet_just_done >= 4 as ::core::ffi::c_int {
				force = 1 as ::core::ffi::c_int;
				break;
			} else {
				acc += (*(*os).lacing_vals.offset(vals as isize) & 0xff as ::core::ffi::c_int)
					as ::core::ffi::c_long;
				if (*(*os).lacing_vals.offset(vals as isize) & 0xff as ::core::ffi::c_int)
					< 255 as ::core::ffi::c_int
				{
					granule_pos = *(*os).granule_vals.offset(vals as isize);
					packets_done += 1;
					packet_just_done = packets_done;
				} else {
					packet_just_done = 0 as ::core::ffi::c_int;
				}
				vals += 1;
			}
		}
		if vals == 255 as ::core::ffi::c_int {
			force = 1 as ::core::ffi::c_int;
		}
	}
	if force == 0 {
		return 0 as ::core::ffi::c_int;
	}
	memcpy(
		&raw mut (*os).header as *mut ::core::ffi::c_uchar as *mut ::core::ffi::c_void,
		b"OggS\0" as *const u8 as *const ::core::ffi::c_char as *const ::core::ffi::c_void,
		4 as size_t
	);
	(*os).header[4 as ::core::ffi::c_int as usize] = 0 as ::core::ffi::c_uchar;
	(*os).header[5 as ::core::ffi::c_int as usize] = 0 as ::core::ffi::c_uchar;
	if *(*os).lacing_vals.offset(0 as ::core::ffi::c_int as isize) & 0x100 as ::core::ffi::c_int
		== 0 as ::core::ffi::c_int
	{
		(*os).header[5 as ::core::ffi::c_int as usize] =
			((*os).header[5 as ::core::ffi::c_int as usize] as ::core::ffi::c_int
				| 0x1 as ::core::ffi::c_int) as ::core::ffi::c_uchar;
	}
	if (*os).b_o_s == 0 as ::core::ffi::c_int {
		(*os).header[5 as ::core::ffi::c_int as usize] =
			((*os).header[5 as ::core::ffi::c_int as usize] as ::core::ffi::c_int
				| 0x2 as ::core::ffi::c_int) as ::core::ffi::c_uchar;
	}
	if (*os).e_o_s != 0 && (*os).lacing_fill == vals as ::core::ffi::c_long {
		(*os).header[5 as ::core::ffi::c_int as usize] =
			((*os).header[5 as ::core::ffi::c_int as usize] as ::core::ffi::c_int
				| 0x4 as ::core::ffi::c_int) as ::core::ffi::c_uchar;
	}
	(*os).b_o_s = 1 as ::core::ffi::c_int;
	i = 6 as ::core::ffi::c_int;
	while i < 14 as ::core::ffi::c_int {
		(*os).header[i as usize] = (granule_pos & 0xff as ogg_int64_t) as ::core::ffi::c_uchar;
		granule_pos >>= 8 as ::core::ffi::c_int;
		i += 1;
	}
	let mut serialno: ::core::ffi::c_long = (*os).serialno;
	i = 14 as ::core::ffi::c_int;
	while i < 18 as ::core::ffi::c_int {
		(*os).header[i as usize] = (serialno & 0xff as ::core::ffi::c_long) as ::core::ffi::c_uchar;
		serialno >>= 8 as ::core::ffi::c_int;
		i += 1;
	}
	if (*os).pageno == -(1 as ::core::ffi::c_int) as ::core::ffi::c_long {
		(*os).pageno = 0 as ::core::ffi::c_long;
	}
	let fresh1 = (*os).pageno;
	(*os).pageno = (*os).pageno + 1;
	let mut pageno: ::core::ffi::c_long = fresh1;
	i = 18 as ::core::ffi::c_int;
	while i < 22 as ::core::ffi::c_int {
		(*os).header[i as usize] = (pageno & 0xff as ::core::ffi::c_long) as ::core::ffi::c_uchar;
		pageno >>= 8 as ::core::ffi::c_int;
		i += 1;
	}
	(*os).header[22 as ::core::ffi::c_int as usize] = 0 as ::core::ffi::c_uchar;
	(*os).header[23 as ::core::ffi::c_int as usize] = 0 as ::core::ffi::c_uchar;
	(*os).header[24 as ::core::ffi::c_int as usize] = 0 as ::core::ffi::c_uchar;
	(*os).header[25 as ::core::ffi::c_int as usize] = 0 as ::core::ffi::c_uchar;
	(*os).header[26 as ::core::ffi::c_int as usize] =
		(vals & 0xff as ::core::ffi::c_int) as ::core::ffi::c_uchar;
	i = 0 as ::core::ffi::c_int;
	while i < vals {
		(*os).header[(i + 27 as ::core::ffi::c_int) as usize] =
			(*(*os).lacing_vals.offset(i as isize) & 0xff as ::core::ffi::c_int)
				as ::core::ffi::c_uchar;
		bytes += (*os).header[(i + 27 as ::core::ffi::c_int) as usize] as ::core::ffi::c_int;
		i += 1;
	}
	(*og).header = &raw mut (*os).header as *mut ::core::ffi::c_uchar;
	(*os).header_fill = vals + 27 as ::core::ffi::c_int;
	(*og).header_len = (*os).header_fill as ::core::ffi::c_long;
	(*og).body = (*os).body_data.offset((*os).body_returned as isize);
	(*og).body_len = bytes as ::core::ffi::c_long;
	(*os).lacing_fill -= vals as ::core::ffi::c_long;
	::core::ptr::copy(
		(*os).lacing_vals.offset(vals as isize),
		(*os).lacing_vals,
		(*os).lacing_fill as usize
	);
	::core::ptr::copy(
		(*os).granule_vals.offset(vals as isize),
		(*os).granule_vals,
		(*os).lacing_fill as usize
	);
	(*os).body_returned += bytes as ::core::ffi::c_long;
	ogg_page_checksum_set(og);
	return 1 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ogg_stream_flush(
	mut os: *mut ogg_stream_state,
	mut og: *mut ogg_page
) -> ::core::ffi::c_int {
	return ogg_stream_flush_i(os, og, 1 as ::core::ffi::c_int, 4096 as ::core::ffi::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn ogg_stream_flush_fill(
	mut os: *mut ogg_stream_state,
	mut og: *mut ogg_page,
	mut nfill: ::core::ffi::c_int
) -> ::core::ffi::c_int {
	return ogg_stream_flush_i(os, og, 1 as ::core::ffi::c_int, nfill);
}
#[no_mangle]
pub unsafe extern "C" fn ogg_stream_pageout(
	mut os: *mut ogg_stream_state,
	mut og: *mut ogg_page
) -> ::core::ffi::c_int {
	let mut force: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
	if ogg_stream_check(os) != 0 {
		return 0 as ::core::ffi::c_int;
	}
	if (*os).e_o_s != 0 && (*os).lacing_fill != 0 || (*os).lacing_fill != 0 && (*os).b_o_s == 0 {
		force = 1 as ::core::ffi::c_int;
	}
	return ogg_stream_flush_i(os, og, force, 4096 as ::core::ffi::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn ogg_stream_pageout_fill(
	mut os: *mut ogg_stream_state,
	mut og: *mut ogg_page,
	mut nfill: ::core::ffi::c_int
) -> ::core::ffi::c_int {
	let mut force: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
	if ogg_stream_check(os) != 0 {
		return 0 as ::core::ffi::c_int;
	}
	if (*os).e_o_s != 0 && (*os).lacing_fill != 0 || (*os).lacing_fill != 0 && (*os).b_o_s == 0 {
		force = 1 as ::core::ffi::c_int;
	}
	return ogg_stream_flush_i(os, og, force, nfill);
}
#[no_mangle]
pub unsafe extern "C" fn ogg_stream_eos(mut os: *mut ogg_stream_state) -> ::core::ffi::c_int {
	if ogg_stream_check(os) != 0 {
		return 1 as ::core::ffi::c_int;
	}
	return (*os).e_o_s;
}
#[no_mangle]
pub unsafe extern "C" fn ogg_sync_init(mut oy: *mut ogg_sync_state) -> ::core::ffi::c_int {
	if !oy.is_null() {
		(*oy).storage = -(1 as ::core::ffi::c_int);
		memset(
			oy as *mut ::core::ffi::c_void,
			0 as ::core::ffi::c_int,
			::core::mem::size_of::<ogg_sync_state>() as size_t
		);
	}
	return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ogg_sync_clear(mut oy: *mut ogg_sync_state) -> ::core::ffi::c_int {
	if !oy.is_null() {
		if !(*oy).data.is_null() {
			free((*oy).data as *mut ::core::ffi::c_void);
		}
		memset(
			oy as *mut ::core::ffi::c_void,
			0 as ::core::ffi::c_int,
			::core::mem::size_of::<ogg_sync_state>() as size_t
		);
	}
	return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ogg_sync_destroy(mut oy: *mut ogg_sync_state) -> ::core::ffi::c_int {
	if !oy.is_null() {
		ogg_sync_clear(oy);
		free(oy as *mut ::core::ffi::c_void);
	}
	return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ogg_sync_check(mut oy: *mut ogg_sync_state) -> ::core::ffi::c_int {
	if (*oy).storage < 0 as ::core::ffi::c_int {
		return -(1 as ::core::ffi::c_int);
	}
	return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ogg_sync_buffer(
	mut oy: *mut ogg_sync_state,
	mut size: ::core::ffi::c_long
) -> *mut ::core::ffi::c_char {
	if ogg_sync_check(oy) != 0 {
		return ::core::ptr::null_mut::<::core::ffi::c_char>();
	}
	if (*oy).returned != 0 {
		(*oy).fill -= (*oy).returned;
		if (*oy).fill > 0 as ::core::ffi::c_int {
			::core::ptr::copy(
				(*oy).data.offset((*oy).returned as isize),
				(*oy).data,
				(*oy).fill as usize
			);
		}
		(*oy).returned = 0 as ::core::ffi::c_int;
	}
	if size > ((*oy).storage - (*oy).fill) as ::core::ffi::c_long {
		let mut newsize: ::core::ffi::c_long = 0;
		let mut ret: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
		if size > (INT_MAX - 4096 as ::core::ffi::c_int - (*oy).fill) as ::core::ffi::c_long {
			ogg_sync_clear(oy);
			return ::core::ptr::null_mut::<::core::ffi::c_char>();
		}
		newsize = size + (*oy).fill as ::core::ffi::c_long + 4096 as ::core::ffi::c_long;
		if !(*oy).data.is_null() {
			ret = realloc((*oy).data as *mut ::core::ffi::c_void, newsize as size_t);
		} else {
			ret = malloc(newsize as size_t);
		}
		if ret.is_null() {
			ogg_sync_clear(oy);
			return ::core::ptr::null_mut::<::core::ffi::c_char>();
		}
		(*oy).data = ret as *mut ::core::ffi::c_uchar;
		(*oy).storage = newsize as ::core::ffi::c_int;
	}
	return ((*oy).data as *mut ::core::ffi::c_char).offset((*oy).fill as isize);
}
#[no_mangle]
pub unsafe extern "C" fn ogg_sync_wrote(
	mut oy: *mut ogg_sync_state,
	mut bytes: ::core::ffi::c_long
) -> ::core::ffi::c_int {
	if ogg_sync_check(oy) != 0 {
		return -(1 as ::core::ffi::c_int);
	}
	if (*oy).fill as ::core::ffi::c_long + bytes > (*oy).storage as ::core::ffi::c_long {
		return -(1 as ::core::ffi::c_int);
	}
	(*oy).fill = ((*oy).fill as ::core::ffi::c_long + bytes) as ::core::ffi::c_int;
	return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ogg_sync_pageseek(
	mut oy: *mut ogg_sync_state,
	mut og: *mut ogg_page
) -> ::core::ffi::c_long {
	let mut current_block: u64;
	let mut page: *mut ::core::ffi::c_uchar = (*oy).data.offset((*oy).returned as isize);
	let mut next: *mut ::core::ffi::c_uchar = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
	let mut bytes: ::core::ffi::c_long = ((*oy).fill - (*oy).returned) as ::core::ffi::c_long;
	if ogg_sync_check(oy) != 0 {
		return 0 as ::core::ffi::c_long;
	}
	if (*oy).headerbytes == 0 as ::core::ffi::c_int {
		let mut headerbytes: ::core::ffi::c_int = 0;
		let mut i: ::core::ffi::c_int = 0;
		if bytes < 27 as ::core::ffi::c_long {
			return 0 as ::core::ffi::c_long;
		}
		if memcmp(
			page as *const ::core::ffi::c_void,
			b"OggS\0" as *const u8 as *const ::core::ffi::c_char as *const ::core::ffi::c_void,
			4 as size_t
		) != 0
		{
			current_block = 17739541499954794539;
		} else {
			headerbytes = *page.offset(26 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
				+ 27 as ::core::ffi::c_int;
			if bytes < headerbytes as ::core::ffi::c_long {
				return 0 as ::core::ffi::c_long;
			}
			i = 0 as ::core::ffi::c_int;
			while i < *page.offset(26 as ::core::ffi::c_int as isize) as ::core::ffi::c_int {
				(*oy).bodybytes +=
					*page.offset((27 as ::core::ffi::c_int + i) as isize) as ::core::ffi::c_int;
				i += 1;
			}
			(*oy).headerbytes = headerbytes;
			current_block = 7651349459974463963;
		}
	} else {
		current_block = 7651349459974463963;
	}
	match current_block {
		7651349459974463963 => {
			if ((*oy).bodybytes + (*oy).headerbytes) as ::core::ffi::c_long > bytes {
				return 0 as ::core::ffi::c_long;
			}
			let mut chksum: [::core::ffi::c_char; 4] = [0; 4];
			let mut log: ogg_page = ogg_page {
				header: ::core::ptr::null_mut::<::core::ffi::c_uchar>(),
				header_len: 0,
				body: ::core::ptr::null_mut::<::core::ffi::c_uchar>(),
				body_len: 0
			};
			memcpy(
				&raw mut chksum as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void,
				page.offset(22 as ::core::ffi::c_int as isize) as *const ::core::ffi::c_void,
				4 as size_t
			);
			memset(
				page.offset(22 as ::core::ffi::c_int as isize) as *mut ::core::ffi::c_void,
				0 as ::core::ffi::c_int,
				4 as size_t
			);
			log.header = page;
			log.header_len = (*oy).headerbytes as ::core::ffi::c_long;
			log.body = page.offset((*oy).headerbytes as isize);
			log.body_len = (*oy).bodybytes as ::core::ffi::c_long;
			ogg_page_checksum_set(&raw mut log);
			if memcmp(
				&raw mut chksum as *mut ::core::ffi::c_char as *const ::core::ffi::c_void,
				page.offset(22 as ::core::ffi::c_int as isize) as *const ::core::ffi::c_void,
				4 as size_t
			) != 0
			{
				memcpy(
					page.offset(22 as ::core::ffi::c_int as isize) as *mut ::core::ffi::c_void,
					&raw mut chksum as *mut ::core::ffi::c_char as *const ::core::ffi::c_void,
					4 as size_t
				);
			} else {
				if !og.is_null() {
					(*og).header = page;
					(*og).header_len = (*oy).headerbytes as ::core::ffi::c_long;
					(*og).body = page.offset((*oy).headerbytes as isize);
					(*og).body_len = (*oy).bodybytes as ::core::ffi::c_long;
				}
				(*oy).unsynced = 0 as ::core::ffi::c_int;
				bytes = ((*oy).headerbytes + (*oy).bodybytes) as ::core::ffi::c_long;
				(*oy).returned =
					((*oy).returned as ::core::ffi::c_long + bytes) as ::core::ffi::c_int;
				(*oy).headerbytes = 0 as ::core::ffi::c_int;
				(*oy).bodybytes = 0 as ::core::ffi::c_int;
				return bytes;
			}
		}
		_ => {}
	}
	(*oy).headerbytes = 0 as ::core::ffi::c_int;
	(*oy).bodybytes = 0 as ::core::ffi::c_int;
	next = memchr(
		page.offset(1 as ::core::ffi::c_int as isize) as *const ::core::ffi::c_void,
		'O' as i32,
		(bytes - 1 as ::core::ffi::c_long) as size_t
	) as *mut ::core::ffi::c_uchar;
	if next.is_null() {
		next = (*oy).data.offset((*oy).fill as isize);
	}
	(*oy).returned = next.offset_from((*oy).data) as ::core::ffi::c_long as ::core::ffi::c_int;
	return -(next.offset_from(page) as ::core::ffi::c_long);
}
#[no_mangle]
pub unsafe extern "C" fn ogg_sync_pageout(
	mut oy: *mut ogg_sync_state,
	mut og: *mut ogg_page
) -> ::core::ffi::c_int {
	if ogg_sync_check(oy) != 0 {
		return 0 as ::core::ffi::c_int;
	}
	loop {
		let mut ret: ::core::ffi::c_long = ogg_sync_pageseek(oy, og);
		if ret > 0 as ::core::ffi::c_long {
			return 1 as ::core::ffi::c_int;
		}
		if ret == 0 as ::core::ffi::c_long {
			return 0 as ::core::ffi::c_int;
		}
		if (*oy).unsynced == 0 {
			(*oy).unsynced = 1 as ::core::ffi::c_int;
			return -(1 as ::core::ffi::c_int);
		}
	}
}
#[no_mangle]
pub unsafe extern "C" fn ogg_stream_pagein(
	mut os: *mut ogg_stream_state,
	mut og: *mut ogg_page
) -> ::core::ffi::c_int {
	let mut header: *mut ::core::ffi::c_uchar = (*og).header;
	let mut body: *mut ::core::ffi::c_uchar = (*og).body;
	let mut bodysize: ::core::ffi::c_long = (*og).body_len;
	let mut segptr: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
	let mut version: ::core::ffi::c_int = ogg_page_version(og);
	let mut continued: ::core::ffi::c_int = ogg_page_continued(og);
	let mut bos: ::core::ffi::c_int = ogg_page_bos(og);
	let mut eos: ::core::ffi::c_int = ogg_page_eos(og);
	let mut granulepos: ogg_int64_t = ogg_page_granulepos(og);
	let mut serialno: ::core::ffi::c_int = ogg_page_serialno(og);
	let mut pageno: ::core::ffi::c_long = ogg_page_pageno(og);
	let mut segments: ::core::ffi::c_int =
		*header.offset(26 as ::core::ffi::c_int as isize) as ::core::ffi::c_int;
	if ogg_stream_check(os) != 0 {
		return -(1 as ::core::ffi::c_int);
	}
	let mut lr: ::core::ffi::c_long = (*os).lacing_returned;
	let mut br: ::core::ffi::c_long = (*os).body_returned;
	if br != 0 {
		(*os).body_fill -= br;
		if (*os).body_fill != 0 {
			::core::ptr::copy(
				(*os).body_data.offset(br as isize),
				(*os).body_data,
				(*os).body_fill as usize
			);
		}
		(*os).body_returned = 0 as ::core::ffi::c_long;
	}
	if lr != 0 {
		if (*os).lacing_fill - lr != 0 {
			::core::ptr::copy(
				(*os).lacing_vals.offset(lr as isize),
				(*os).lacing_vals,
				((*os).lacing_fill - lr) as usize
			);
			::core::ptr::copy(
				(*os).granule_vals.offset(lr as isize),
				(*os).granule_vals,
				((*os).lacing_fill - lr) as usize
			);
		}
		(*os).lacing_fill -= lr;
		(*os).lacing_packet -= lr;
		(*os).lacing_returned = 0 as ::core::ffi::c_long;
	}
	if serialno as ::core::ffi::c_long != (*os).serialno {
		return -(1 as ::core::ffi::c_int);
	}
	if version > 0 as ::core::ffi::c_int {
		return -(1 as ::core::ffi::c_int);
	}
	if _os_lacing_expand(
		os,
		(segments + 1 as ::core::ffi::c_int) as ::core::ffi::c_long
	) != 0
	{
		return -(1 as ::core::ffi::c_int);
	}
	if pageno != (*os).pageno {
		let mut i: ::core::ffi::c_int = 0;
		i = (*os).lacing_packet as ::core::ffi::c_int;
		while (i as ::core::ffi::c_long) < (*os).lacing_fill {
			(*os).body_fill -= (*(*os).lacing_vals.offset(i as isize) & 0xff as ::core::ffi::c_int)
				as ::core::ffi::c_long;
			i += 1;
		}
		(*os).lacing_fill = (*os).lacing_packet;
		if (*os).pageno != -(1 as ::core::ffi::c_int) as ::core::ffi::c_long {
			let fresh4 = (*os).lacing_fill;
			(*os).lacing_fill = (*os).lacing_fill + 1;
			*(*os).lacing_vals.offset(fresh4 as isize) = 0x400 as ::core::ffi::c_int;
			(*os).lacing_packet += 1;
		}
	}
	if continued != 0 {
		if (*os).lacing_fill < 1 as ::core::ffi::c_long
			|| (*(*os)
				.lacing_vals
				.offset(((*os).lacing_fill - 1 as ::core::ffi::c_long) as isize)
				& 0xff as ::core::ffi::c_int)
				< 255 as ::core::ffi::c_int
			|| *(*os)
				.lacing_vals
				.offset(((*os).lacing_fill - 1 as ::core::ffi::c_long) as isize)
				== 0x400 as ::core::ffi::c_int
		{
			bos = 0 as ::core::ffi::c_int;
			while segptr < segments {
				let mut val: ::core::ffi::c_int = *header
					.offset((27 as ::core::ffi::c_int + segptr) as isize)
					as ::core::ffi::c_int;
				body = body.offset(val as isize);
				bodysize -= val as ::core::ffi::c_long;
				if val < 255 as ::core::ffi::c_int {
					segptr += 1;
					break;
				} else {
					segptr += 1;
				}
			}
		}
	}
	if bodysize != 0 {
		if _os_body_expand(os, bodysize) != 0 {
			return -(1 as ::core::ffi::c_int);
		}
		memcpy(
			(*os).body_data.offset((*os).body_fill as isize) as *mut ::core::ffi::c_void,
			body as *const ::core::ffi::c_void,
			bodysize as size_t
		);
		(*os).body_fill += bodysize;
	}
	let mut saved: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
	while segptr < segments {
		let mut val_0: ::core::ffi::c_int =
			*header.offset((27 as ::core::ffi::c_int + segptr) as isize) as ::core::ffi::c_int;
		*(*os).lacing_vals.offset((*os).lacing_fill as isize) = val_0;
		*(*os).granule_vals.offset((*os).lacing_fill as isize) =
			-(1 as ::core::ffi::c_int) as ogg_int64_t;
		if bos != 0 {
			*(*os).lacing_vals.offset((*os).lacing_fill as isize) |= 0x100 as ::core::ffi::c_int;
			bos = 0 as ::core::ffi::c_int;
		}
		if val_0 < 255 as ::core::ffi::c_int {
			saved = (*os).lacing_fill as ::core::ffi::c_int;
		}
		(*os).lacing_fill += 1;
		segptr += 1;
		if val_0 < 255 as ::core::ffi::c_int {
			(*os).lacing_packet = (*os).lacing_fill;
		}
	}
	if saved != -(1 as ::core::ffi::c_int) {
		*(*os).granule_vals.offset(saved as isize) = granulepos;
	}
	if eos != 0 {
		(*os).e_o_s = 1 as ::core::ffi::c_int;
		if (*os).lacing_fill > 0 as ::core::ffi::c_long {
			*(*os)
				.lacing_vals
				.offset(((*os).lacing_fill - 1 as ::core::ffi::c_long) as isize) |= 0x200 as ::core::ffi::c_int;
		}
	}
	(*os).pageno = pageno + 1 as ::core::ffi::c_long;
	return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ogg_sync_reset(mut oy: *mut ogg_sync_state) -> ::core::ffi::c_int {
	if ogg_sync_check(oy) != 0 {
		return -(1 as ::core::ffi::c_int);
	}
	(*oy).fill = 0 as ::core::ffi::c_int;
	(*oy).returned = 0 as ::core::ffi::c_int;
	(*oy).unsynced = 0 as ::core::ffi::c_int;
	(*oy).headerbytes = 0 as ::core::ffi::c_int;
	(*oy).bodybytes = 0 as ::core::ffi::c_int;
	return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ogg_stream_reset(mut os: *mut ogg_stream_state) -> ::core::ffi::c_int {
	if ogg_stream_check(os) != 0 {
		return -(1 as ::core::ffi::c_int);
	}
	(*os).body_fill = 0 as ::core::ffi::c_long;
	(*os).body_returned = 0 as ::core::ffi::c_long;
	(*os).lacing_fill = 0 as ::core::ffi::c_long;
	(*os).lacing_packet = 0 as ::core::ffi::c_long;
	(*os).lacing_returned = 0 as ::core::ffi::c_long;
	(*os).header_fill = 0 as ::core::ffi::c_int;
	(*os).e_o_s = 0 as ::core::ffi::c_int;
	(*os).b_o_s = 0 as ::core::ffi::c_int;
	(*os).pageno = -(1 as ::core::ffi::c_int) as ::core::ffi::c_long;
	(*os).packetno = 0 as ogg_int64_t;
	(*os).granulepos = 0 as ogg_int64_t;
	return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ogg_stream_reset_serialno(
	mut os: *mut ogg_stream_state,
	mut serialno: ::core::ffi::c_int
) -> ::core::ffi::c_int {
	if ogg_stream_check(os) != 0 {
		return -(1 as ::core::ffi::c_int);
	}
	ogg_stream_reset(os);
	(*os).serialno = serialno as ::core::ffi::c_long;
	return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn _packetout(
	mut os: *mut ogg_stream_state,
	mut op: *mut ogg_packet,
	mut adv: ::core::ffi::c_int
) -> ::core::ffi::c_int {
	let mut ptr: ::core::ffi::c_int = (*os).lacing_returned as ::core::ffi::c_int;
	if (*os).lacing_packet <= ptr as ::core::ffi::c_long {
		return 0 as ::core::ffi::c_int;
	}
	if *(*os).lacing_vals.offset(ptr as isize) & 0x400 as ::core::ffi::c_int != 0 {
		(*os).lacing_returned += 1;
		(*os).packetno += 1;
		return -(1 as ::core::ffi::c_int);
	}
	if op.is_null() && adv == 0 {
		return 1 as ::core::ffi::c_int;
	}
	let mut size: ::core::ffi::c_int =
		*(*os).lacing_vals.offset(ptr as isize) & 0xff as ::core::ffi::c_int;
	let mut bytes: ::core::ffi::c_long = size as ::core::ffi::c_long;
	let mut eos: ::core::ffi::c_int =
		*(*os).lacing_vals.offset(ptr as isize) & 0x200 as ::core::ffi::c_int;
	let mut bos: ::core::ffi::c_int =
		*(*os).lacing_vals.offset(ptr as isize) & 0x100 as ::core::ffi::c_int;
	while size == 255 as ::core::ffi::c_int {
		ptr += 1;
		let mut val: ::core::ffi::c_int = *(*os).lacing_vals.offset(ptr as isize);
		size = val & 0xff as ::core::ffi::c_int;
		if val & 0x200 as ::core::ffi::c_int != 0 {
			eos = 0x200 as ::core::ffi::c_int;
		}
		bytes += size as ::core::ffi::c_long;
	}
	if !op.is_null() {
		(*op).e_o_s = eos as ::core::ffi::c_long;
		(*op).b_o_s = bos as ::core::ffi::c_long;
		(*op).packet = (*os).body_data.offset((*os).body_returned as isize);
		(*op).packetno = (*os).packetno;
		(*op).granulepos = *(*os).granule_vals.offset(ptr as isize);
		(*op).bytes = bytes;
	}
	if adv != 0 {
		(*os).body_returned += bytes;
		(*os).lacing_returned = (ptr + 1 as ::core::ffi::c_int) as ::core::ffi::c_long;
		(*os).packetno += 1;
	}
	return 1 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ogg_stream_packetout(
	mut os: *mut ogg_stream_state,
	mut op: *mut ogg_packet
) -> ::core::ffi::c_int {
	if ogg_stream_check(os) != 0 {
		return 0 as ::core::ffi::c_int;
	}
	return _packetout(os, op, 1 as ::core::ffi::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn ogg_stream_packetpeek(
	mut os: *mut ogg_stream_state,
	mut op: *mut ogg_packet
) -> ::core::ffi::c_int {
	if ogg_stream_check(os) != 0 {
		return 0 as ::core::ffi::c_int;
	}
	return _packetout(os, op, 0 as ::core::ffi::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn ogg_packet_clear(mut op: *mut ogg_packet) {
	free((*op).packet as *mut ::core::ffi::c_void);
	memset(
		op as *mut ::core::ffi::c_void,
		0 as ::core::ffi::c_int,
		::core::mem::size_of::<ogg_packet>() as size_t
	);
}
pub const __INT_MAX__: ::core::ffi::c_int = 2147483647 as ::core::ffi::c_int;
pub const __LONG_MAX__: ::core::ffi::c_long = 9223372036854775807 as ::core::ffi::c_long;
