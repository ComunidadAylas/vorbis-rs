extern "C" {
	fn oggpack_reset(b: *mut oggpack_buffer);
	fn oggpack_bytes(b: *mut oggpack_buffer) -> ::core::ffi::c_long;
	fn oggpack_get_buffer(b: *mut oggpack_buffer) -> *mut ::core::ffi::c_uchar;
	fn vorbis_bitrate_managed(vb: *mut vorbis_block) -> ::core::ffi::c_int;
}
use ogg_c2rust_sys::{ogg_packet, oggpack_buffer};

use super::{
	block::{alloc_chain, vorbis_block, vorbis_dsp_state},
	info::vorbis_info,
	registry::_mapping_P
};
pub type __int64_t = i64;
pub type int64_t = __int64_t;
pub type ogg_int64_t = int64_t;
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
pub type vorbis_info_mapping = ();
#[derive(Copy, Clone)]
#[repr(C)]
pub struct vorbis_block_internal {
	pub pcmdelay: *mut *mut ::core::ffi::c_float,
	pub ampmax: ::core::ffi::c_float,
	pub blocktype: ::core::ffi::c_int,
	pub packetblob: [*mut oggpack_buffer; 15]
}
pub const OV_EINVAL: ::core::ffi::c_int = -(131 as ::core::ffi::c_int);
#[no_mangle]
pub unsafe extern "C" fn vorbis_analysis(
	mut vb: *mut vorbis_block,
	mut op: *mut ogg_packet
) -> ::core::ffi::c_int {
	let mut ret: ::core::ffi::c_int = 0;
	let mut i: ::core::ffi::c_int = 0;
	let mut vbi: *mut vorbis_block_internal = (*vb).internal as *mut vorbis_block_internal;
	(*vb).glue_bits = 0 as ::core::ffi::c_long;
	(*vb).time_bits = 0 as ::core::ffi::c_long;
	(*vb).floor_bits = 0 as ::core::ffi::c_long;
	(*vb).res_bits = 0 as ::core::ffi::c_long;
	i = 0 as ::core::ffi::c_int;
	while i < PACKETBLOBS {
		oggpack_reset((*vbi).packetblob[i as usize]);
		i += 1;
	}
	ret = (**(&raw const _mapping_P as *const *const vorbis_func_mapping)
		.offset(0 as ::core::ffi::c_int as isize))
	.forward
	.expect("non-null function pointer")(vb as *mut vorbis_block);
	if ret != 0 {
		return ret;
	}
	if !op.is_null() {
		if vorbis_bitrate_managed(vb) != 0 {
			return -(131 as ::core::ffi::c_int);
		}
		(*op).packet = oggpack_get_buffer(&raw mut (*vb).opb);
		(*op).bytes = oggpack_bytes(&raw mut (*vb).opb);
		(*op).b_o_s = 0 as ::core::ffi::c_long;
		(*op).e_o_s = (*vb).eofflag as ::core::ffi::c_long;
		(*op).granulepos = (*vb).granulepos;
		(*op).packetno = (*vb).sequence;
	}
	return 0 as ::core::ffi::c_int;
}
pub const PACKETBLOBS: ::core::ffi::c_int = 15 as ::core::ffi::c_int;
