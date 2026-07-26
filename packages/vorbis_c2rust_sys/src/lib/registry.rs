// These bundles are genuine intra-crate statics (not real `extern "C"` data), so they are
// brought into scope with a plain `use` instead of being redeclared as `extern "C" { static ...; }`
// items. Doing so keeps their true definitions (and sizes) visible to tools such as Miri, which
// do not support treating cross-module Rust statics as opaque external symbols
use ogg_c2rust_sys::oggpack_buffer;

use super::{
	block::{alloc_chain, vorbis_block, vorbis_dsp_state},
	floor0::floor0_exportbundle,
	floor1::floor1_exportbundle,
	info::vorbis_info,
	mapping0::mapping0_exportbundle,
	res0::{residue0_exportbundle, residue1_exportbundle, residue2_exportbundle}
};
pub type __int64_t = i64;
pub type int64_t = __int64_t;
pub type ogg_int64_t = int64_t;
pub type vorbis_look_floor = ();
pub type vorbis_look_residue = ();
pub type vorbis_info_floor = ();
pub type vorbis_info_residue = ();
pub type vorbis_info_mapping = ();
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
#[no_mangle]
pub static mut _floor_P: [*const vorbis_func_floor; 2] = unsafe {
	[
		&raw const floor0_exportbundle,
		&raw const floor1_exportbundle
	]
};
#[no_mangle]
pub static mut _residue_P: [*const vorbis_func_residue; 3] = unsafe {
	[
		&raw const residue0_exportbundle,
		&raw const residue1_exportbundle,
		&raw const residue2_exportbundle
	]
};
#[no_mangle]
pub static mut _mapping_P: [*const vorbis_func_mapping; 1] =
	unsafe { [&raw const mapping0_exportbundle] };
