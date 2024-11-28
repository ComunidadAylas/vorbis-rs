pub type ogg_int16_t = i16;
pub type ogg_uint16_t = u16;
pub type ogg_int32_t = i32;
pub type ogg_uint32_t = u32;
pub type ogg_int64_t = i64;
pub type ogg_uint64_t = u64;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ogg_iovec_t {
	pub iov_base: *mut ::std::os::raw::c_void,
	pub iov_len: usize
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
	["Size of ogg_iovec_t"][::std::mem::size_of::<ogg_iovec_t>() - 16usize];
	["Alignment of ogg_iovec_t"][::std::mem::align_of::<ogg_iovec_t>() - 8usize];
	["Offset of field: ogg_iovec_t::iov_base"]
		[::std::mem::offset_of!(ogg_iovec_t, iov_base) - 0usize];
	["Offset of field: ogg_iovec_t::iov_len"]
		[::std::mem::offset_of!(ogg_iovec_t, iov_len) - 8usize];
};
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct oggpack_buffer {
	pub endbyte: ::std::os::raw::c_long,
	pub endbit: ::std::os::raw::c_int,
	pub buffer: *mut ::std::os::raw::c_uchar,
	pub ptr: *mut ::std::os::raw::c_uchar,
	pub storage: ::std::os::raw::c_long
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
	["Size of oggpack_buffer"][::std::mem::size_of::<oggpack_buffer>() - 40usize];
	["Alignment of oggpack_buffer"][::std::mem::align_of::<oggpack_buffer>() - 8usize];
	["Offset of field: oggpack_buffer::endbyte"]
		[::std::mem::offset_of!(oggpack_buffer, endbyte) - 0usize];
	["Offset of field: oggpack_buffer::endbit"]
		[::std::mem::offset_of!(oggpack_buffer, endbit) - 8usize];
	["Offset of field: oggpack_buffer::buffer"]
		[::std::mem::offset_of!(oggpack_buffer, buffer) - 16usize];
	["Offset of field: oggpack_buffer::ptr"][::std::mem::offset_of!(oggpack_buffer, ptr) - 24usize];
	["Offset of field: oggpack_buffer::storage"]
		[::std::mem::offset_of!(oggpack_buffer, storage) - 32usize];
};
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ogg_page {
	pub header: *mut ::std::os::raw::c_uchar,
	pub header_len: ::std::os::raw::c_long,
	pub body: *mut ::std::os::raw::c_uchar,
	pub body_len: ::std::os::raw::c_long
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
	["Size of ogg_page"][::std::mem::size_of::<ogg_page>() - 32usize];
	["Alignment of ogg_page"][::std::mem::align_of::<ogg_page>() - 8usize];
	["Offset of field: ogg_page::header"][::std::mem::offset_of!(ogg_page, header) - 0usize];
	["Offset of field: ogg_page::header_len"]
		[::std::mem::offset_of!(ogg_page, header_len) - 8usize];
	["Offset of field: ogg_page::body"][::std::mem::offset_of!(ogg_page, body) - 16usize];
	["Offset of field: ogg_page::body_len"][::std::mem::offset_of!(ogg_page, body_len) - 24usize];
};
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ogg_stream_state {
	pub body_data: *mut ::std::os::raw::c_uchar,
	pub body_storage: ::std::os::raw::c_long,
	pub body_fill: ::std::os::raw::c_long,
	pub body_returned: ::std::os::raw::c_long,
	pub lacing_vals: *mut ::std::os::raw::c_int,
	pub granule_vals: *mut ogg_int64_t,
	pub lacing_storage: ::std::os::raw::c_long,
	pub lacing_fill: ::std::os::raw::c_long,
	pub lacing_packet: ::std::os::raw::c_long,
	pub lacing_returned: ::std::os::raw::c_long,
	pub header: [::std::os::raw::c_uchar; 282usize],
	pub header_fill: ::std::os::raw::c_int,
	pub e_o_s: ::std::os::raw::c_int,
	pub b_o_s: ::std::os::raw::c_int,
	pub serialno: ::std::os::raw::c_long,
	pub pageno: ::std::os::raw::c_long,
	pub packetno: ogg_int64_t,
	pub granulepos: ogg_int64_t
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
	["Size of ogg_stream_state"][::std::mem::size_of::<ogg_stream_state>() - 408usize];
	["Alignment of ogg_stream_state"][::std::mem::align_of::<ogg_stream_state>() - 8usize];
	["Offset of field: ogg_stream_state::body_data"]
		[::std::mem::offset_of!(ogg_stream_state, body_data) - 0usize];
	["Offset of field: ogg_stream_state::body_storage"]
		[::std::mem::offset_of!(ogg_stream_state, body_storage) - 8usize];
	["Offset of field: ogg_stream_state::body_fill"]
		[::std::mem::offset_of!(ogg_stream_state, body_fill) - 16usize];
	["Offset of field: ogg_stream_state::body_returned"]
		[::std::mem::offset_of!(ogg_stream_state, body_returned) - 24usize];
	["Offset of field: ogg_stream_state::lacing_vals"]
		[::std::mem::offset_of!(ogg_stream_state, lacing_vals) - 32usize];
	["Offset of field: ogg_stream_state::granule_vals"]
		[::std::mem::offset_of!(ogg_stream_state, granule_vals) - 40usize];
	["Offset of field: ogg_stream_state::lacing_storage"]
		[::std::mem::offset_of!(ogg_stream_state, lacing_storage) - 48usize];
	["Offset of field: ogg_stream_state::lacing_fill"]
		[::std::mem::offset_of!(ogg_stream_state, lacing_fill) - 56usize];
	["Offset of field: ogg_stream_state::lacing_packet"]
		[::std::mem::offset_of!(ogg_stream_state, lacing_packet) - 64usize];
	["Offset of field: ogg_stream_state::lacing_returned"]
		[::std::mem::offset_of!(ogg_stream_state, lacing_returned) - 72usize];
	["Offset of field: ogg_stream_state::header"]
		[::std::mem::offset_of!(ogg_stream_state, header) - 80usize];
	["Offset of field: ogg_stream_state::header_fill"]
		[::std::mem::offset_of!(ogg_stream_state, header_fill) - 364usize];
	["Offset of field: ogg_stream_state::e_o_s"]
		[::std::mem::offset_of!(ogg_stream_state, e_o_s) - 368usize];
	["Offset of field: ogg_stream_state::b_o_s"]
		[::std::mem::offset_of!(ogg_stream_state, b_o_s) - 372usize];
	["Offset of field: ogg_stream_state::serialno"]
		[::std::mem::offset_of!(ogg_stream_state, serialno) - 376usize];
	["Offset of field: ogg_stream_state::pageno"]
		[::std::mem::offset_of!(ogg_stream_state, pageno) - 384usize];
	["Offset of field: ogg_stream_state::packetno"]
		[::std::mem::offset_of!(ogg_stream_state, packetno) - 392usize];
	["Offset of field: ogg_stream_state::granulepos"]
		[::std::mem::offset_of!(ogg_stream_state, granulepos) - 400usize];
};
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ogg_packet {
	pub packet: *mut ::std::os::raw::c_uchar,
	pub bytes: ::std::os::raw::c_long,
	pub b_o_s: ::std::os::raw::c_long,
	pub e_o_s: ::std::os::raw::c_long,
	pub granulepos: ogg_int64_t,
	pub packetno: ogg_int64_t
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
	["Size of ogg_packet"][::std::mem::size_of::<ogg_packet>() - 48usize];
	["Alignment of ogg_packet"][::std::mem::align_of::<ogg_packet>() - 8usize];
	["Offset of field: ogg_packet::packet"][::std::mem::offset_of!(ogg_packet, packet) - 0usize];
	["Offset of field: ogg_packet::bytes"][::std::mem::offset_of!(ogg_packet, bytes) - 8usize];
	["Offset of field: ogg_packet::b_o_s"][::std::mem::offset_of!(ogg_packet, b_o_s) - 16usize];
	["Offset of field: ogg_packet::e_o_s"][::std::mem::offset_of!(ogg_packet, e_o_s) - 24usize];
	["Offset of field: ogg_packet::granulepos"]
		[::std::mem::offset_of!(ogg_packet, granulepos) - 32usize];
	["Offset of field: ogg_packet::packetno"]
		[::std::mem::offset_of!(ogg_packet, packetno) - 40usize];
};
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ogg_sync_state {
	pub data: *mut ::std::os::raw::c_uchar,
	pub storage: ::std::os::raw::c_int,
	pub fill: ::std::os::raw::c_int,
	pub returned: ::std::os::raw::c_int,
	pub unsynced: ::std::os::raw::c_int,
	pub headerbytes: ::std::os::raw::c_int,
	pub bodybytes: ::std::os::raw::c_int
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
	["Size of ogg_sync_state"][::std::mem::size_of::<ogg_sync_state>() - 32usize];
	["Alignment of ogg_sync_state"][::std::mem::align_of::<ogg_sync_state>() - 8usize];
	["Offset of field: ogg_sync_state::data"]
		[::std::mem::offset_of!(ogg_sync_state, data) - 0usize];
	["Offset of field: ogg_sync_state::storage"]
		[::std::mem::offset_of!(ogg_sync_state, storage) - 8usize];
	["Offset of field: ogg_sync_state::fill"]
		[::std::mem::offset_of!(ogg_sync_state, fill) - 12usize];
	["Offset of field: ogg_sync_state::returned"]
		[::std::mem::offset_of!(ogg_sync_state, returned) - 16usize];
	["Offset of field: ogg_sync_state::unsynced"]
		[::std::mem::offset_of!(ogg_sync_state, unsynced) - 20usize];
	["Offset of field: ogg_sync_state::headerbytes"]
		[::std::mem::offset_of!(ogg_sync_state, headerbytes) - 24usize];
	["Offset of field: ogg_sync_state::bodybytes"]
		[::std::mem::offset_of!(ogg_sync_state, bodybytes) - 28usize];
};
extern "C" {
	pub fn oggpack_writeinit(b: *mut oggpack_buffer);
}
extern "C" {
	pub fn oggpack_writecheck(b: *mut oggpack_buffer) -> ::std::os::raw::c_int;
}
extern "C" {
	pub fn oggpack_writetrunc(b: *mut oggpack_buffer, bits: ::std::os::raw::c_long);
}
extern "C" {
	pub fn oggpack_writealign(b: *mut oggpack_buffer);
}
extern "C" {
	pub fn oggpack_writecopy(
		b: *mut oggpack_buffer,
		source: *mut ::std::os::raw::c_void,
		bits: ::std::os::raw::c_long
	);
}
extern "C" {
	pub fn oggpack_reset(b: *mut oggpack_buffer);
}
extern "C" {
	pub fn oggpack_writeclear(b: *mut oggpack_buffer);
}
extern "C" {
	pub fn oggpack_readinit(
		b: *mut oggpack_buffer,
		buf: *mut ::std::os::raw::c_uchar,
		bytes: ::std::os::raw::c_int
	);
}
extern "C" {
	pub fn oggpack_write(
		b: *mut oggpack_buffer,
		value: ::std::os::raw::c_ulong,
		bits: ::std::os::raw::c_int
	);
}
extern "C" {
	pub fn oggpack_look(
		b: *mut oggpack_buffer,
		bits: ::std::os::raw::c_int
	) -> ::std::os::raw::c_long;
}
extern "C" {
	pub fn oggpack_look1(b: *mut oggpack_buffer) -> ::std::os::raw::c_long;
}
extern "C" {
	pub fn oggpack_adv(b: *mut oggpack_buffer, bits: ::std::os::raw::c_int);
}
extern "C" {
	pub fn oggpack_adv1(b: *mut oggpack_buffer);
}
extern "C" {
	pub fn oggpack_read(
		b: *mut oggpack_buffer,
		bits: ::std::os::raw::c_int
	) -> ::std::os::raw::c_long;
}
extern "C" {
	pub fn oggpack_read1(b: *mut oggpack_buffer) -> ::std::os::raw::c_long;
}
extern "C" {
	pub fn oggpack_bytes(b: *mut oggpack_buffer) -> ::std::os::raw::c_long;
}
extern "C" {
	pub fn oggpack_bits(b: *mut oggpack_buffer) -> ::std::os::raw::c_long;
}
extern "C" {
	pub fn oggpack_get_buffer(b: *mut oggpack_buffer) -> *mut ::std::os::raw::c_uchar;
}
extern "C" {
	pub fn oggpackB_writeinit(b: *mut oggpack_buffer);
}
extern "C" {
	pub fn oggpackB_writecheck(b: *mut oggpack_buffer) -> ::std::os::raw::c_int;
}
extern "C" {
	pub fn oggpackB_writetrunc(b: *mut oggpack_buffer, bits: ::std::os::raw::c_long);
}
extern "C" {
	pub fn oggpackB_writealign(b: *mut oggpack_buffer);
}
extern "C" {
	pub fn oggpackB_writecopy(
		b: *mut oggpack_buffer,
		source: *mut ::std::os::raw::c_void,
		bits: ::std::os::raw::c_long
	);
}
extern "C" {
	pub fn oggpackB_reset(b: *mut oggpack_buffer);
}
extern "C" {
	pub fn oggpackB_writeclear(b: *mut oggpack_buffer);
}
extern "C" {
	pub fn oggpackB_readinit(
		b: *mut oggpack_buffer,
		buf: *mut ::std::os::raw::c_uchar,
		bytes: ::std::os::raw::c_int
	);
}
extern "C" {
	pub fn oggpackB_write(
		b: *mut oggpack_buffer,
		value: ::std::os::raw::c_ulong,
		bits: ::std::os::raw::c_int
	);
}
extern "C" {
	pub fn oggpackB_look(
		b: *mut oggpack_buffer,
		bits: ::std::os::raw::c_int
	) -> ::std::os::raw::c_long;
}
extern "C" {
	pub fn oggpackB_look1(b: *mut oggpack_buffer) -> ::std::os::raw::c_long;
}
extern "C" {
	pub fn oggpackB_adv(b: *mut oggpack_buffer, bits: ::std::os::raw::c_int);
}
extern "C" {
	pub fn oggpackB_adv1(b: *mut oggpack_buffer);
}
extern "C" {
	pub fn oggpackB_read(
		b: *mut oggpack_buffer,
		bits: ::std::os::raw::c_int
	) -> ::std::os::raw::c_long;
}
extern "C" {
	pub fn oggpackB_read1(b: *mut oggpack_buffer) -> ::std::os::raw::c_long;
}
extern "C" {
	pub fn oggpackB_bytes(b: *mut oggpack_buffer) -> ::std::os::raw::c_long;
}
extern "C" {
	pub fn oggpackB_bits(b: *mut oggpack_buffer) -> ::std::os::raw::c_long;
}
extern "C" {
	pub fn oggpackB_get_buffer(b: *mut oggpack_buffer) -> *mut ::std::os::raw::c_uchar;
}
extern "C" {
	pub fn ogg_stream_packetin(
		os: *mut ogg_stream_state,
		op: *mut ogg_packet
	) -> ::std::os::raw::c_int;
}
extern "C" {
	pub fn ogg_stream_iovecin(
		os: *mut ogg_stream_state,
		iov: *mut ogg_iovec_t,
		count: ::std::os::raw::c_int,
		e_o_s: ::std::os::raw::c_long,
		granulepos: ogg_int64_t
	) -> ::std::os::raw::c_int;
}
extern "C" {
	pub fn ogg_stream_pageout(
		os: *mut ogg_stream_state,
		og: *mut ogg_page
	) -> ::std::os::raw::c_int;
}
extern "C" {
	pub fn ogg_stream_pageout_fill(
		os: *mut ogg_stream_state,
		og: *mut ogg_page,
		nfill: ::std::os::raw::c_int
	) -> ::std::os::raw::c_int;
}
extern "C" {
	pub fn ogg_stream_flush(os: *mut ogg_stream_state, og: *mut ogg_page) -> ::std::os::raw::c_int;
}
extern "C" {
	pub fn ogg_stream_flush_fill(
		os: *mut ogg_stream_state,
		og: *mut ogg_page,
		nfill: ::std::os::raw::c_int
	) -> ::std::os::raw::c_int;
}
extern "C" {
	pub fn ogg_sync_init(oy: *mut ogg_sync_state) -> ::std::os::raw::c_int;
}
extern "C" {
	pub fn ogg_sync_clear(oy: *mut ogg_sync_state) -> ::std::os::raw::c_int;
}
extern "C" {
	pub fn ogg_sync_reset(oy: *mut ogg_sync_state) -> ::std::os::raw::c_int;
}
extern "C" {
	pub fn ogg_sync_destroy(oy: *mut ogg_sync_state) -> ::std::os::raw::c_int;
}
extern "C" {
	pub fn ogg_sync_check(oy: *mut ogg_sync_state) -> ::std::os::raw::c_int;
}
extern "C" {
	pub fn ogg_sync_buffer(
		oy: *mut ogg_sync_state,
		size: ::std::os::raw::c_long
	) -> *mut ::std::os::raw::c_char;
}
extern "C" {
	pub fn ogg_sync_wrote(
		oy: *mut ogg_sync_state,
		bytes: ::std::os::raw::c_long
	) -> ::std::os::raw::c_int;
}
extern "C" {
	pub fn ogg_sync_pageseek(oy: *mut ogg_sync_state, og: *mut ogg_page) -> ::std::os::raw::c_long;
}
extern "C" {
	pub fn ogg_sync_pageout(oy: *mut ogg_sync_state, og: *mut ogg_page) -> ::std::os::raw::c_int;
}
extern "C" {
	pub fn ogg_stream_pagein(os: *mut ogg_stream_state, og: *mut ogg_page)
		-> ::std::os::raw::c_int;
}
extern "C" {
	pub fn ogg_stream_packetout(
		os: *mut ogg_stream_state,
		op: *mut ogg_packet
	) -> ::std::os::raw::c_int;
}
extern "C" {
	pub fn ogg_stream_packetpeek(
		os: *mut ogg_stream_state,
		op: *mut ogg_packet
	) -> ::std::os::raw::c_int;
}
extern "C" {
	pub fn ogg_stream_init(
		os: *mut ogg_stream_state,
		serialno: ::std::os::raw::c_int
	) -> ::std::os::raw::c_int;
}
extern "C" {
	pub fn ogg_stream_clear(os: *mut ogg_stream_state) -> ::std::os::raw::c_int;
}
extern "C" {
	pub fn ogg_stream_reset(os: *mut ogg_stream_state) -> ::std::os::raw::c_int;
}
extern "C" {
	pub fn ogg_stream_reset_serialno(
		os: *mut ogg_stream_state,
		serialno: ::std::os::raw::c_int
	) -> ::std::os::raw::c_int;
}
extern "C" {
	pub fn ogg_stream_destroy(os: *mut ogg_stream_state) -> ::std::os::raw::c_int;
}
extern "C" {
	pub fn ogg_stream_check(os: *mut ogg_stream_state) -> ::std::os::raw::c_int;
}
extern "C" {
	pub fn ogg_stream_eos(os: *mut ogg_stream_state) -> ::std::os::raw::c_int;
}
extern "C" {
	pub fn ogg_page_checksum_set(og: *mut ogg_page);
}
extern "C" {
	pub fn ogg_page_version(og: *const ogg_page) -> ::std::os::raw::c_int;
}
extern "C" {
	pub fn ogg_page_continued(og: *const ogg_page) -> ::std::os::raw::c_int;
}
extern "C" {
	pub fn ogg_page_bos(og: *const ogg_page) -> ::std::os::raw::c_int;
}
extern "C" {
	pub fn ogg_page_eos(og: *const ogg_page) -> ::std::os::raw::c_int;
}
extern "C" {
	pub fn ogg_page_granulepos(og: *const ogg_page) -> ogg_int64_t;
}
extern "C" {
	pub fn ogg_page_serialno(og: *const ogg_page) -> ::std::os::raw::c_int;
}
extern "C" {
	pub fn ogg_page_pageno(og: *const ogg_page) -> ::std::os::raw::c_long;
}
extern "C" {
	pub fn ogg_page_packets(og: *const ogg_page) -> ::std::os::raw::c_int;
}
extern "C" {
	pub fn ogg_packet_clear(op: *mut ogg_packet);
}
