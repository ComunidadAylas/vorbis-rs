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
#[test]
fn bindgen_test_layout_ogg_iovec_t() {
	assert_eq!(
		::std::mem::size_of::<ogg_iovec_t>(),
		16usize,
		concat!("Size of: ", stringify!(ogg_iovec_t))
	);
	assert_eq!(
		::std::mem::align_of::<ogg_iovec_t>(),
		8usize,
		concat!("Alignment of ", stringify!(ogg_iovec_t))
	);
	fn test_field_iov_base() {
		assert_eq!(
			unsafe {
				let uninit = ::std::mem::MaybeUninit::<ogg_iovec_t>::uninit();
				let ptr = uninit.as_ptr();
				::std::ptr::addr_of!((*ptr).iov_base) as usize - ptr as usize
			},
			0usize,
			concat!(
				"Offset of field: ",
				stringify!(ogg_iovec_t),
				"::",
				stringify!(iov_base)
			)
		);
	}
	test_field_iov_base();
	fn test_field_iov_len() {
		assert_eq!(
			unsafe {
				let uninit = ::std::mem::MaybeUninit::<ogg_iovec_t>::uninit();
				let ptr = uninit.as_ptr();
				::std::ptr::addr_of!((*ptr).iov_len) as usize - ptr as usize
			},
			8usize,
			concat!(
				"Offset of field: ",
				stringify!(ogg_iovec_t),
				"::",
				stringify!(iov_len)
			)
		);
	}
	test_field_iov_len();
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct oggpack_buffer {
	pub endbyte: ::std::os::raw::c_long,
	pub endbit: ::std::os::raw::c_int,
	pub buffer: *mut ::std::os::raw::c_uchar,
	pub ptr: *mut ::std::os::raw::c_uchar,
	pub storage: ::std::os::raw::c_long
}
#[test]
fn bindgen_test_layout_oggpack_buffer() {
	assert_eq!(
		::std::mem::size_of::<oggpack_buffer>(),
		40usize,
		concat!("Size of: ", stringify!(oggpack_buffer))
	);
	assert_eq!(
		::std::mem::align_of::<oggpack_buffer>(),
		8usize,
		concat!("Alignment of ", stringify!(oggpack_buffer))
	);
	fn test_field_endbyte() {
		assert_eq!(
			unsafe {
				let uninit = ::std::mem::MaybeUninit::<oggpack_buffer>::uninit();
				let ptr = uninit.as_ptr();
				::std::ptr::addr_of!((*ptr).endbyte) as usize - ptr as usize
			},
			0usize,
			concat!(
				"Offset of field: ",
				stringify!(oggpack_buffer),
				"::",
				stringify!(endbyte)
			)
		);
	}
	test_field_endbyte();
	fn test_field_endbit() {
		assert_eq!(
			unsafe {
				let uninit = ::std::mem::MaybeUninit::<oggpack_buffer>::uninit();
				let ptr = uninit.as_ptr();
				::std::ptr::addr_of!((*ptr).endbit) as usize - ptr as usize
			},
			8usize,
			concat!(
				"Offset of field: ",
				stringify!(oggpack_buffer),
				"::",
				stringify!(endbit)
			)
		);
	}
	test_field_endbit();
	fn test_field_buffer() {
		assert_eq!(
			unsafe {
				let uninit = ::std::mem::MaybeUninit::<oggpack_buffer>::uninit();
				let ptr = uninit.as_ptr();
				::std::ptr::addr_of!((*ptr).buffer) as usize - ptr as usize
			},
			16usize,
			concat!(
				"Offset of field: ",
				stringify!(oggpack_buffer),
				"::",
				stringify!(buffer)
			)
		);
	}
	test_field_buffer();
	fn test_field_ptr() {
		assert_eq!(
			unsafe {
				let uninit = ::std::mem::MaybeUninit::<oggpack_buffer>::uninit();
				let ptr = uninit.as_ptr();
				::std::ptr::addr_of!((*ptr).ptr) as usize - ptr as usize
			},
			24usize,
			concat!(
				"Offset of field: ",
				stringify!(oggpack_buffer),
				"::",
				stringify!(ptr)
			)
		);
	}
	test_field_ptr();
	fn test_field_storage() {
		assert_eq!(
			unsafe {
				let uninit = ::std::mem::MaybeUninit::<oggpack_buffer>::uninit();
				let ptr = uninit.as_ptr();
				::std::ptr::addr_of!((*ptr).storage) as usize - ptr as usize
			},
			32usize,
			concat!(
				"Offset of field: ",
				stringify!(oggpack_buffer),
				"::",
				stringify!(storage)
			)
		);
	}
	test_field_storage();
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ogg_page {
	pub header: *mut ::std::os::raw::c_uchar,
	pub header_len: ::std::os::raw::c_long,
	pub body: *mut ::std::os::raw::c_uchar,
	pub body_len: ::std::os::raw::c_long
}
#[test]
fn bindgen_test_layout_ogg_page() {
	assert_eq!(
		::std::mem::size_of::<ogg_page>(),
		32usize,
		concat!("Size of: ", stringify!(ogg_page))
	);
	assert_eq!(
		::std::mem::align_of::<ogg_page>(),
		8usize,
		concat!("Alignment of ", stringify!(ogg_page))
	);
	fn test_field_header() {
		assert_eq!(
			unsafe {
				let uninit = ::std::mem::MaybeUninit::<ogg_page>::uninit();
				let ptr = uninit.as_ptr();
				::std::ptr::addr_of!((*ptr).header) as usize - ptr as usize
			},
			0usize,
			concat!(
				"Offset of field: ",
				stringify!(ogg_page),
				"::",
				stringify!(header)
			)
		);
	}
	test_field_header();
	fn test_field_header_len() {
		assert_eq!(
			unsafe {
				let uninit = ::std::mem::MaybeUninit::<ogg_page>::uninit();
				let ptr = uninit.as_ptr();
				::std::ptr::addr_of!((*ptr).header_len) as usize - ptr as usize
			},
			8usize,
			concat!(
				"Offset of field: ",
				stringify!(ogg_page),
				"::",
				stringify!(header_len)
			)
		);
	}
	test_field_header_len();
	fn test_field_body() {
		assert_eq!(
			unsafe {
				let uninit = ::std::mem::MaybeUninit::<ogg_page>::uninit();
				let ptr = uninit.as_ptr();
				::std::ptr::addr_of!((*ptr).body) as usize - ptr as usize
			},
			16usize,
			concat!(
				"Offset of field: ",
				stringify!(ogg_page),
				"::",
				stringify!(body)
			)
		);
	}
	test_field_body();
	fn test_field_body_len() {
		assert_eq!(
			unsafe {
				let uninit = ::std::mem::MaybeUninit::<ogg_page>::uninit();
				let ptr = uninit.as_ptr();
				::std::ptr::addr_of!((*ptr).body_len) as usize - ptr as usize
			},
			24usize,
			concat!(
				"Offset of field: ",
				stringify!(ogg_page),
				"::",
				stringify!(body_len)
			)
		);
	}
	test_field_body_len();
}
#[repr(C)]
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
#[test]
fn bindgen_test_layout_ogg_stream_state() {
	assert_eq!(
		::std::mem::size_of::<ogg_stream_state>(),
		408usize,
		concat!("Size of: ", stringify!(ogg_stream_state))
	);
	assert_eq!(
		::std::mem::align_of::<ogg_stream_state>(),
		8usize,
		concat!("Alignment of ", stringify!(ogg_stream_state))
	);
	fn test_field_body_data() {
		assert_eq!(
			unsafe {
				let uninit = ::std::mem::MaybeUninit::<ogg_stream_state>::uninit();
				let ptr = uninit.as_ptr();
				::std::ptr::addr_of!((*ptr).body_data) as usize - ptr as usize
			},
			0usize,
			concat!(
				"Offset of field: ",
				stringify!(ogg_stream_state),
				"::",
				stringify!(body_data)
			)
		);
	}
	test_field_body_data();
	fn test_field_body_storage() {
		assert_eq!(
			unsafe {
				let uninit = ::std::mem::MaybeUninit::<ogg_stream_state>::uninit();
				let ptr = uninit.as_ptr();
				::std::ptr::addr_of!((*ptr).body_storage) as usize - ptr as usize
			},
			8usize,
			concat!(
				"Offset of field: ",
				stringify!(ogg_stream_state),
				"::",
				stringify!(body_storage)
			)
		);
	}
	test_field_body_storage();
	fn test_field_body_fill() {
		assert_eq!(
			unsafe {
				let uninit = ::std::mem::MaybeUninit::<ogg_stream_state>::uninit();
				let ptr = uninit.as_ptr();
				::std::ptr::addr_of!((*ptr).body_fill) as usize - ptr as usize
			},
			16usize,
			concat!(
				"Offset of field: ",
				stringify!(ogg_stream_state),
				"::",
				stringify!(body_fill)
			)
		);
	}
	test_field_body_fill();
	fn test_field_body_returned() {
		assert_eq!(
			unsafe {
				let uninit = ::std::mem::MaybeUninit::<ogg_stream_state>::uninit();
				let ptr = uninit.as_ptr();
				::std::ptr::addr_of!((*ptr).body_returned) as usize - ptr as usize
			},
			24usize,
			concat!(
				"Offset of field: ",
				stringify!(ogg_stream_state),
				"::",
				stringify!(body_returned)
			)
		);
	}
	test_field_body_returned();
	fn test_field_lacing_vals() {
		assert_eq!(
			unsafe {
				let uninit = ::std::mem::MaybeUninit::<ogg_stream_state>::uninit();
				let ptr = uninit.as_ptr();
				::std::ptr::addr_of!((*ptr).lacing_vals) as usize - ptr as usize
			},
			32usize,
			concat!(
				"Offset of field: ",
				stringify!(ogg_stream_state),
				"::",
				stringify!(lacing_vals)
			)
		);
	}
	test_field_lacing_vals();
	fn test_field_granule_vals() {
		assert_eq!(
			unsafe {
				let uninit = ::std::mem::MaybeUninit::<ogg_stream_state>::uninit();
				let ptr = uninit.as_ptr();
				::std::ptr::addr_of!((*ptr).granule_vals) as usize - ptr as usize
			},
			40usize,
			concat!(
				"Offset of field: ",
				stringify!(ogg_stream_state),
				"::",
				stringify!(granule_vals)
			)
		);
	}
	test_field_granule_vals();
	fn test_field_lacing_storage() {
		assert_eq!(
			unsafe {
				let uninit = ::std::mem::MaybeUninit::<ogg_stream_state>::uninit();
				let ptr = uninit.as_ptr();
				::std::ptr::addr_of!((*ptr).lacing_storage) as usize - ptr as usize
			},
			48usize,
			concat!(
				"Offset of field: ",
				stringify!(ogg_stream_state),
				"::",
				stringify!(lacing_storage)
			)
		);
	}
	test_field_lacing_storage();
	fn test_field_lacing_fill() {
		assert_eq!(
			unsafe {
				let uninit = ::std::mem::MaybeUninit::<ogg_stream_state>::uninit();
				let ptr = uninit.as_ptr();
				::std::ptr::addr_of!((*ptr).lacing_fill) as usize - ptr as usize
			},
			56usize,
			concat!(
				"Offset of field: ",
				stringify!(ogg_stream_state),
				"::",
				stringify!(lacing_fill)
			)
		);
	}
	test_field_lacing_fill();
	fn test_field_lacing_packet() {
		assert_eq!(
			unsafe {
				let uninit = ::std::mem::MaybeUninit::<ogg_stream_state>::uninit();
				let ptr = uninit.as_ptr();
				::std::ptr::addr_of!((*ptr).lacing_packet) as usize - ptr as usize
			},
			64usize,
			concat!(
				"Offset of field: ",
				stringify!(ogg_stream_state),
				"::",
				stringify!(lacing_packet)
			)
		);
	}
	test_field_lacing_packet();
	fn test_field_lacing_returned() {
		assert_eq!(
			unsafe {
				let uninit = ::std::mem::MaybeUninit::<ogg_stream_state>::uninit();
				let ptr = uninit.as_ptr();
				::std::ptr::addr_of!((*ptr).lacing_returned) as usize - ptr as usize
			},
			72usize,
			concat!(
				"Offset of field: ",
				stringify!(ogg_stream_state),
				"::",
				stringify!(lacing_returned)
			)
		);
	}
	test_field_lacing_returned();
	fn test_field_header() {
		assert_eq!(
			unsafe {
				let uninit = ::std::mem::MaybeUninit::<ogg_stream_state>::uninit();
				let ptr = uninit.as_ptr();
				::std::ptr::addr_of!((*ptr).header) as usize - ptr as usize
			},
			80usize,
			concat!(
				"Offset of field: ",
				stringify!(ogg_stream_state),
				"::",
				stringify!(header)
			)
		);
	}
	test_field_header();
	fn test_field_header_fill() {
		assert_eq!(
			unsafe {
				let uninit = ::std::mem::MaybeUninit::<ogg_stream_state>::uninit();
				let ptr = uninit.as_ptr();
				::std::ptr::addr_of!((*ptr).header_fill) as usize - ptr as usize
			},
			364usize,
			concat!(
				"Offset of field: ",
				stringify!(ogg_stream_state),
				"::",
				stringify!(header_fill)
			)
		);
	}
	test_field_header_fill();
	fn test_field_e_o_s() {
		assert_eq!(
			unsafe {
				let uninit = ::std::mem::MaybeUninit::<ogg_stream_state>::uninit();
				let ptr = uninit.as_ptr();
				::std::ptr::addr_of!((*ptr).e_o_s) as usize - ptr as usize
			},
			368usize,
			concat!(
				"Offset of field: ",
				stringify!(ogg_stream_state),
				"::",
				stringify!(e_o_s)
			)
		);
	}
	test_field_e_o_s();
	fn test_field_b_o_s() {
		assert_eq!(
			unsafe {
				let uninit = ::std::mem::MaybeUninit::<ogg_stream_state>::uninit();
				let ptr = uninit.as_ptr();
				::std::ptr::addr_of!((*ptr).b_o_s) as usize - ptr as usize
			},
			372usize,
			concat!(
				"Offset of field: ",
				stringify!(ogg_stream_state),
				"::",
				stringify!(b_o_s)
			)
		);
	}
	test_field_b_o_s();
	fn test_field_serialno() {
		assert_eq!(
			unsafe {
				let uninit = ::std::mem::MaybeUninit::<ogg_stream_state>::uninit();
				let ptr = uninit.as_ptr();
				::std::ptr::addr_of!((*ptr).serialno) as usize - ptr as usize
			},
			376usize,
			concat!(
				"Offset of field: ",
				stringify!(ogg_stream_state),
				"::",
				stringify!(serialno)
			)
		);
	}
	test_field_serialno();
	fn test_field_pageno() {
		assert_eq!(
			unsafe {
				let uninit = ::std::mem::MaybeUninit::<ogg_stream_state>::uninit();
				let ptr = uninit.as_ptr();
				::std::ptr::addr_of!((*ptr).pageno) as usize - ptr as usize
			},
			384usize,
			concat!(
				"Offset of field: ",
				stringify!(ogg_stream_state),
				"::",
				stringify!(pageno)
			)
		);
	}
	test_field_pageno();
	fn test_field_packetno() {
		assert_eq!(
			unsafe {
				let uninit = ::std::mem::MaybeUninit::<ogg_stream_state>::uninit();
				let ptr = uninit.as_ptr();
				::std::ptr::addr_of!((*ptr).packetno) as usize - ptr as usize
			},
			392usize,
			concat!(
				"Offset of field: ",
				stringify!(ogg_stream_state),
				"::",
				stringify!(packetno)
			)
		);
	}
	test_field_packetno();
	fn test_field_granulepos() {
		assert_eq!(
			unsafe {
				let uninit = ::std::mem::MaybeUninit::<ogg_stream_state>::uninit();
				let ptr = uninit.as_ptr();
				::std::ptr::addr_of!((*ptr).granulepos) as usize - ptr as usize
			},
			400usize,
			concat!(
				"Offset of field: ",
				stringify!(ogg_stream_state),
				"::",
				stringify!(granulepos)
			)
		);
	}
	test_field_granulepos();
}
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
#[test]
fn bindgen_test_layout_ogg_packet() {
	assert_eq!(
		::std::mem::size_of::<ogg_packet>(),
		48usize,
		concat!("Size of: ", stringify!(ogg_packet))
	);
	assert_eq!(
		::std::mem::align_of::<ogg_packet>(),
		8usize,
		concat!("Alignment of ", stringify!(ogg_packet))
	);
	fn test_field_packet() {
		assert_eq!(
			unsafe {
				let uninit = ::std::mem::MaybeUninit::<ogg_packet>::uninit();
				let ptr = uninit.as_ptr();
				::std::ptr::addr_of!((*ptr).packet) as usize - ptr as usize
			},
			0usize,
			concat!(
				"Offset of field: ",
				stringify!(ogg_packet),
				"::",
				stringify!(packet)
			)
		);
	}
	test_field_packet();
	fn test_field_bytes() {
		assert_eq!(
			unsafe {
				let uninit = ::std::mem::MaybeUninit::<ogg_packet>::uninit();
				let ptr = uninit.as_ptr();
				::std::ptr::addr_of!((*ptr).bytes) as usize - ptr as usize
			},
			8usize,
			concat!(
				"Offset of field: ",
				stringify!(ogg_packet),
				"::",
				stringify!(bytes)
			)
		);
	}
	test_field_bytes();
	fn test_field_b_o_s() {
		assert_eq!(
			unsafe {
				let uninit = ::std::mem::MaybeUninit::<ogg_packet>::uninit();
				let ptr = uninit.as_ptr();
				::std::ptr::addr_of!((*ptr).b_o_s) as usize - ptr as usize
			},
			16usize,
			concat!(
				"Offset of field: ",
				stringify!(ogg_packet),
				"::",
				stringify!(b_o_s)
			)
		);
	}
	test_field_b_o_s();
	fn test_field_e_o_s() {
		assert_eq!(
			unsafe {
				let uninit = ::std::mem::MaybeUninit::<ogg_packet>::uninit();
				let ptr = uninit.as_ptr();
				::std::ptr::addr_of!((*ptr).e_o_s) as usize - ptr as usize
			},
			24usize,
			concat!(
				"Offset of field: ",
				stringify!(ogg_packet),
				"::",
				stringify!(e_o_s)
			)
		);
	}
	test_field_e_o_s();
	fn test_field_granulepos() {
		assert_eq!(
			unsafe {
				let uninit = ::std::mem::MaybeUninit::<ogg_packet>::uninit();
				let ptr = uninit.as_ptr();
				::std::ptr::addr_of!((*ptr).granulepos) as usize - ptr as usize
			},
			32usize,
			concat!(
				"Offset of field: ",
				stringify!(ogg_packet),
				"::",
				stringify!(granulepos)
			)
		);
	}
	test_field_granulepos();
	fn test_field_packetno() {
		assert_eq!(
			unsafe {
				let uninit = ::std::mem::MaybeUninit::<ogg_packet>::uninit();
				let ptr = uninit.as_ptr();
				::std::ptr::addr_of!((*ptr).packetno) as usize - ptr as usize
			},
			40usize,
			concat!(
				"Offset of field: ",
				stringify!(ogg_packet),
				"::",
				stringify!(packetno)
			)
		);
	}
	test_field_packetno();
}
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
#[test]
fn bindgen_test_layout_ogg_sync_state() {
	assert_eq!(
		::std::mem::size_of::<ogg_sync_state>(),
		32usize,
		concat!("Size of: ", stringify!(ogg_sync_state))
	);
	assert_eq!(
		::std::mem::align_of::<ogg_sync_state>(),
		8usize,
		concat!("Alignment of ", stringify!(ogg_sync_state))
	);
	fn test_field_data() {
		assert_eq!(
			unsafe {
				let uninit = ::std::mem::MaybeUninit::<ogg_sync_state>::uninit();
				let ptr = uninit.as_ptr();
				::std::ptr::addr_of!((*ptr).data) as usize - ptr as usize
			},
			0usize,
			concat!(
				"Offset of field: ",
				stringify!(ogg_sync_state),
				"::",
				stringify!(data)
			)
		);
	}
	test_field_data();
	fn test_field_storage() {
		assert_eq!(
			unsafe {
				let uninit = ::std::mem::MaybeUninit::<ogg_sync_state>::uninit();
				let ptr = uninit.as_ptr();
				::std::ptr::addr_of!((*ptr).storage) as usize - ptr as usize
			},
			8usize,
			concat!(
				"Offset of field: ",
				stringify!(ogg_sync_state),
				"::",
				stringify!(storage)
			)
		);
	}
	test_field_storage();
	fn test_field_fill() {
		assert_eq!(
			unsafe {
				let uninit = ::std::mem::MaybeUninit::<ogg_sync_state>::uninit();
				let ptr = uninit.as_ptr();
				::std::ptr::addr_of!((*ptr).fill) as usize - ptr as usize
			},
			12usize,
			concat!(
				"Offset of field: ",
				stringify!(ogg_sync_state),
				"::",
				stringify!(fill)
			)
		);
	}
	test_field_fill();
	fn test_field_returned() {
		assert_eq!(
			unsafe {
				let uninit = ::std::mem::MaybeUninit::<ogg_sync_state>::uninit();
				let ptr = uninit.as_ptr();
				::std::ptr::addr_of!((*ptr).returned) as usize - ptr as usize
			},
			16usize,
			concat!(
				"Offset of field: ",
				stringify!(ogg_sync_state),
				"::",
				stringify!(returned)
			)
		);
	}
	test_field_returned();
	fn test_field_unsynced() {
		assert_eq!(
			unsafe {
				let uninit = ::std::mem::MaybeUninit::<ogg_sync_state>::uninit();
				let ptr = uninit.as_ptr();
				::std::ptr::addr_of!((*ptr).unsynced) as usize - ptr as usize
			},
			20usize,
			concat!(
				"Offset of field: ",
				stringify!(ogg_sync_state),
				"::",
				stringify!(unsynced)
			)
		);
	}
	test_field_unsynced();
	fn test_field_headerbytes() {
		assert_eq!(
			unsafe {
				let uninit = ::std::mem::MaybeUninit::<ogg_sync_state>::uninit();
				let ptr = uninit.as_ptr();
				::std::ptr::addr_of!((*ptr).headerbytes) as usize - ptr as usize
			},
			24usize,
			concat!(
				"Offset of field: ",
				stringify!(ogg_sync_state),
				"::",
				stringify!(headerbytes)
			)
		);
	}
	test_field_headerbytes();
	fn test_field_bodybytes() {
		assert_eq!(
			unsafe {
				let uninit = ::std::mem::MaybeUninit::<ogg_sync_state>::uninit();
				let ptr = uninit.as_ptr();
				::std::ptr::addr_of!((*ptr).bodybytes) as usize - ptr as usize
			},
			28usize,
			concat!(
				"Offset of field: ",
				stringify!(ogg_sync_state),
				"::",
				stringify!(bodybytes)
			)
		);
	}
	test_field_bodybytes();
}
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
