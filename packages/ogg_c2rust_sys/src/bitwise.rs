extern "C" {
	fn memset(
		__s: *mut ::core::ffi::c_void,
		__c: ::core::ffi::c_int,
		__n: size_t
	) -> *mut ::core::ffi::c_void;
	fn malloc(__size: size_t) -> *mut ::core::ffi::c_void;
	fn realloc(__ptr: *mut ::core::ffi::c_void, __size: size_t) -> *mut ::core::ffi::c_void;
	fn free(__ptr: *mut ::core::ffi::c_void);
}
pub type size_t = usize;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct oggpack_buffer {
	pub endbyte: ::core::ffi::c_long,
	pub endbit: ::core::ffi::c_int,
	pub buffer: *mut ::core::ffi::c_uchar,
	pub ptr: *mut ::core::ffi::c_uchar,
	pub storage: ::core::ffi::c_long
}
pub const LONG_MAX: ::core::ffi::c_long = __LONG_MAX__;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const BUFFER_INCREMENT: ::core::ffi::c_int = 256 as ::core::ffi::c_int;
static mut mask: [::core::ffi::c_ulong; 33] = [
	0 as ::core::ffi::c_int as ::core::ffi::c_ulong,
	0x1 as ::core::ffi::c_int as ::core::ffi::c_ulong,
	0x3 as ::core::ffi::c_int as ::core::ffi::c_ulong,
	0x7 as ::core::ffi::c_int as ::core::ffi::c_ulong,
	0xf as ::core::ffi::c_int as ::core::ffi::c_ulong,
	0x1f as ::core::ffi::c_int as ::core::ffi::c_ulong,
	0x3f as ::core::ffi::c_int as ::core::ffi::c_ulong,
	0x7f as ::core::ffi::c_int as ::core::ffi::c_ulong,
	0xff as ::core::ffi::c_int as ::core::ffi::c_ulong,
	0x1ff as ::core::ffi::c_int as ::core::ffi::c_ulong,
	0x3ff as ::core::ffi::c_int as ::core::ffi::c_ulong,
	0x7ff as ::core::ffi::c_int as ::core::ffi::c_ulong,
	0xfff as ::core::ffi::c_int as ::core::ffi::c_ulong,
	0x1fff as ::core::ffi::c_int as ::core::ffi::c_ulong,
	0x3fff as ::core::ffi::c_int as ::core::ffi::c_ulong,
	0x7fff as ::core::ffi::c_int as ::core::ffi::c_ulong,
	0xffff as ::core::ffi::c_int as ::core::ffi::c_ulong,
	0x1ffff as ::core::ffi::c_int as ::core::ffi::c_ulong,
	0x3ffff as ::core::ffi::c_int as ::core::ffi::c_ulong,
	0x7ffff as ::core::ffi::c_int as ::core::ffi::c_ulong,
	0xfffff as ::core::ffi::c_int as ::core::ffi::c_ulong,
	0x1fffff as ::core::ffi::c_int as ::core::ffi::c_ulong,
	0x3fffff as ::core::ffi::c_int as ::core::ffi::c_ulong,
	0x7fffff as ::core::ffi::c_int as ::core::ffi::c_ulong,
	0xffffff as ::core::ffi::c_int as ::core::ffi::c_ulong,
	0x1ffffff as ::core::ffi::c_int as ::core::ffi::c_ulong,
	0x3ffffff as ::core::ffi::c_int as ::core::ffi::c_ulong,
	0x7ffffff as ::core::ffi::c_int as ::core::ffi::c_ulong,
	0xfffffff as ::core::ffi::c_int as ::core::ffi::c_ulong,
	0x1fffffff as ::core::ffi::c_int as ::core::ffi::c_ulong,
	0x3fffffff as ::core::ffi::c_int as ::core::ffi::c_ulong,
	0x7fffffff as ::core::ffi::c_int as ::core::ffi::c_ulong,
	0xffffffff as ::core::ffi::c_uint as ::core::ffi::c_ulong
];
static mut mask8B: [::core::ffi::c_uint; 9] = [
	0 as ::core::ffi::c_int as ::core::ffi::c_uint,
	0x80 as ::core::ffi::c_int as ::core::ffi::c_uint,
	0xc0 as ::core::ffi::c_int as ::core::ffi::c_uint,
	0xe0 as ::core::ffi::c_int as ::core::ffi::c_uint,
	0xf0 as ::core::ffi::c_int as ::core::ffi::c_uint,
	0xf8 as ::core::ffi::c_int as ::core::ffi::c_uint,
	0xfc as ::core::ffi::c_int as ::core::ffi::c_uint,
	0xfe as ::core::ffi::c_int as ::core::ffi::c_uint,
	0xff as ::core::ffi::c_int as ::core::ffi::c_uint
];
#[no_mangle]
pub unsafe extern "C" fn oggpack_writeinit(mut b: *mut oggpack_buffer) {
	memset(
		b as *mut ::core::ffi::c_void,
		0 as ::core::ffi::c_int,
		::core::mem::size_of::<oggpack_buffer>() as size_t
	);
	(*b).buffer = malloc(BUFFER_INCREMENT as size_t) as *mut ::core::ffi::c_uchar;
	(*b).ptr = (*b).buffer;
	*(*b).buffer.offset(0 as ::core::ffi::c_int as isize) = '\0' as i32 as ::core::ffi::c_uchar;
	(*b).storage = BUFFER_INCREMENT as ::core::ffi::c_long;
}
#[no_mangle]
pub unsafe extern "C" fn oggpackB_writeinit(mut b: *mut oggpack_buffer) {
	oggpack_writeinit(b);
}
#[no_mangle]
pub unsafe extern "C" fn oggpack_writecheck(mut b: *mut oggpack_buffer) -> ::core::ffi::c_int {
	if (*b).ptr.is_null() || (*b).storage == 0 {
		return -(1 as ::core::ffi::c_int);
	}
	return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn oggpackB_writecheck(mut b: *mut oggpack_buffer) -> ::core::ffi::c_int {
	return oggpack_writecheck(b);
}
#[no_mangle]
pub unsafe extern "C" fn oggpack_writetrunc(
	mut b: *mut oggpack_buffer,
	mut bits: ::core::ffi::c_long
) {
	let mut bytes: ::core::ffi::c_long = bits >> 3 as ::core::ffi::c_int;
	if !(*b).ptr.is_null() {
		bits -= bytes * 8 as ::core::ffi::c_long;
		(*b).ptr = (*b).buffer.offset(bytes as isize);
		(*b).endbit = bits as ::core::ffi::c_int;
		(*b).endbyte = bytes;
		*(*b).ptr =
			(*(*b).ptr as ::core::ffi::c_ulong & mask[bits as usize]) as ::core::ffi::c_uchar;
	}
}
#[no_mangle]
pub unsafe extern "C" fn oggpackB_writetrunc(
	mut b: *mut oggpack_buffer,
	mut bits: ::core::ffi::c_long
) {
	let mut bytes: ::core::ffi::c_long = bits >> 3 as ::core::ffi::c_int;
	if !(*b).ptr.is_null() {
		bits -= bytes * 8 as ::core::ffi::c_long;
		(*b).ptr = (*b).buffer.offset(bytes as isize);
		(*b).endbit = bits as ::core::ffi::c_int;
		(*b).endbyte = bytes;
		*(*b).ptr =
			(*(*b).ptr as ::core::ffi::c_uint & mask8B[bits as usize]) as ::core::ffi::c_uchar;
	}
}
#[no_mangle]
pub unsafe extern "C" fn oggpack_write(
	mut b: *mut oggpack_buffer,
	mut value: ::core::ffi::c_ulong,
	mut bits: ::core::ffi::c_int
) {
	let mut current_block: u64;
	if !(bits < 0 as ::core::ffi::c_int || bits > 32 as ::core::ffi::c_int) {
		if (*b).endbyte >= (*b).storage - 4 as ::core::ffi::c_long {
			let mut ret: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
			if (*b).ptr.is_null() {
				return;
			}
			if (*b).storage > LONG_MAX - BUFFER_INCREMENT as ::core::ffi::c_long {
				current_block = 18437152838830023355;
			} else {
				ret = realloc(
					(*b).buffer as *mut ::core::ffi::c_void,
					((*b).storage + BUFFER_INCREMENT as ::core::ffi::c_long) as size_t
				);
				if ret.is_null() {
					current_block = 18437152838830023355;
				} else {
					(*b).buffer = ret as *mut ::core::ffi::c_uchar;
					(*b).storage += BUFFER_INCREMENT as ::core::ffi::c_long;
					(*b).ptr = (*b).buffer.offset((*b).endbyte as isize);
					current_block = 17216689946888361452;
				}
			}
		} else {
			current_block = 17216689946888361452;
		}
		match current_block {
			18437152838830023355 => {}
			_ => {
				value &= mask[bits as usize];
				bits += (*b).endbit;
				let ref mut fresh0 = *(*b).ptr.offset(0 as ::core::ffi::c_int as isize);
				*fresh0 = (*fresh0 as ::core::ffi::c_ulong | value << (*b).endbit)
					as ::core::ffi::c_uchar;
				if bits >= 8 as ::core::ffi::c_int {
					*(*b).ptr.offset(1 as ::core::ffi::c_int as isize) =
						(value >> 8 as ::core::ffi::c_int - (*b).endbit) as ::core::ffi::c_uchar;
					if bits >= 16 as ::core::ffi::c_int {
						*(*b).ptr.offset(2 as ::core::ffi::c_int as isize) = (value
							>> 16 as ::core::ffi::c_int - (*b).endbit)
							as ::core::ffi::c_uchar;
						if bits >= 24 as ::core::ffi::c_int {
							*(*b).ptr.offset(3 as ::core::ffi::c_int as isize) = (value
								>> 24 as ::core::ffi::c_int - (*b).endbit)
								as ::core::ffi::c_uchar;
							if bits >= 32 as ::core::ffi::c_int {
								if (*b).endbit != 0 {
									*(*b).ptr.offset(4 as ::core::ffi::c_int as isize) = (value
										>> 32 as ::core::ffi::c_int - (*b).endbit)
										as ::core::ffi::c_uchar;
								} else {
									*(*b).ptr.offset(4 as ::core::ffi::c_int as isize) =
										0 as ::core::ffi::c_uchar;
								}
							}
						}
					}
				}
				(*b).endbyte += (bits / 8 as ::core::ffi::c_int) as ::core::ffi::c_long;
				(*b).ptr = (*b).ptr.offset((bits / 8 as ::core::ffi::c_int) as isize);
				(*b).endbit = bits & 7 as ::core::ffi::c_int;
				return;
			}
		}
	}
	oggpack_writeclear(b);
}
#[no_mangle]
pub unsafe extern "C" fn oggpackB_write(
	mut b: *mut oggpack_buffer,
	mut value: ::core::ffi::c_ulong,
	mut bits: ::core::ffi::c_int
) {
	let mut current_block: u64;
	if !(bits < 0 as ::core::ffi::c_int || bits > 32 as ::core::ffi::c_int) {
		if (*b).endbyte >= (*b).storage - 4 as ::core::ffi::c_long {
			let mut ret: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
			if (*b).ptr.is_null() {
				return;
			}
			if (*b).storage > LONG_MAX - BUFFER_INCREMENT as ::core::ffi::c_long {
				current_block = 2123180106933083702;
			} else {
				ret = realloc(
					(*b).buffer as *mut ::core::ffi::c_void,
					((*b).storage + BUFFER_INCREMENT as ::core::ffi::c_long) as size_t
				);
				if ret.is_null() {
					current_block = 2123180106933083702;
				} else {
					(*b).buffer = ret as *mut ::core::ffi::c_uchar;
					(*b).storage += BUFFER_INCREMENT as ::core::ffi::c_long;
					(*b).ptr = (*b).buffer.offset((*b).endbyte as isize);
					current_block = 17216689946888361452;
				}
			}
		} else {
			current_block = 17216689946888361452;
		}
		match current_block {
			2123180106933083702 => {}
			_ => {
				value = (value & mask[bits as usize]) << 32 as ::core::ffi::c_int - bits;
				bits += (*b).endbit;
				let ref mut fresh1 = *(*b).ptr.offset(0 as ::core::ffi::c_int as isize);
				*fresh1 = (*fresh1 as ::core::ffi::c_ulong
					| value >> 24 as ::core::ffi::c_int + (*b).endbit)
					as ::core::ffi::c_uchar;
				if bits >= 8 as ::core::ffi::c_int {
					*(*b).ptr.offset(1 as ::core::ffi::c_int as isize) =
						(value >> 16 as ::core::ffi::c_int + (*b).endbit) as ::core::ffi::c_uchar;
					if bits >= 16 as ::core::ffi::c_int {
						*(*b).ptr.offset(2 as ::core::ffi::c_int as isize) = (value
							>> 8 as ::core::ffi::c_int + (*b).endbit)
							as ::core::ffi::c_uchar;
						if bits >= 24 as ::core::ffi::c_int {
							*(*b).ptr.offset(3 as ::core::ffi::c_int as isize) =
								(value >> (*b).endbit) as ::core::ffi::c_uchar;
							if bits >= 32 as ::core::ffi::c_int {
								if (*b).endbit != 0 {
									*(*b).ptr.offset(4 as ::core::ffi::c_int as isize) = (value
										<< 8 as ::core::ffi::c_int - (*b).endbit)
										as ::core::ffi::c_uchar;
								} else {
									*(*b).ptr.offset(4 as ::core::ffi::c_int as isize) =
										0 as ::core::ffi::c_uchar;
								}
							}
						}
					}
				}
				(*b).endbyte += (bits / 8 as ::core::ffi::c_int) as ::core::ffi::c_long;
				(*b).ptr = (*b).ptr.offset((bits / 8 as ::core::ffi::c_int) as isize);
				(*b).endbit = bits & 7 as ::core::ffi::c_int;
				return;
			}
		}
	}
	oggpack_writeclear(b);
}
#[no_mangle]
pub unsafe extern "C" fn oggpack_writealign(mut b: *mut oggpack_buffer) {
	let mut bits: ::core::ffi::c_int = 8 as ::core::ffi::c_int - (*b).endbit;
	if bits < 8 as ::core::ffi::c_int {
		oggpack_write(b, 0 as ::core::ffi::c_ulong, bits);
	}
}
#[no_mangle]
pub unsafe extern "C" fn oggpackB_writealign(mut b: *mut oggpack_buffer) {
	let mut bits: ::core::ffi::c_int = 8 as ::core::ffi::c_int - (*b).endbit;
	if bits < 8 as ::core::ffi::c_int {
		oggpackB_write(b, 0 as ::core::ffi::c_ulong, bits);
	}
}
unsafe extern "C" fn oggpack_writecopy_helper(
	mut b: *mut oggpack_buffer,
	mut source: *mut ::core::ffi::c_void,
	mut bits: ::core::ffi::c_long,
	mut w: Option<
		unsafe extern "C" fn(*mut oggpack_buffer, ::core::ffi::c_ulong, ::core::ffi::c_int) -> ()
	>,
	mut msb: ::core::ffi::c_int
) {
	let mut current_block: u64;
	let mut ptr: *mut ::core::ffi::c_uchar = source as *mut ::core::ffi::c_uchar;
	let mut bytes: ::core::ffi::c_long = bits / 8 as ::core::ffi::c_long;
	let mut pbytes: ::core::ffi::c_long =
		((*b).endbit as ::core::ffi::c_long + bits) / 8 as ::core::ffi::c_long;
	bits -= bytes * 8 as ::core::ffi::c_long;
	if (*b).endbyte + pbytes >= (*b).storage {
		let mut ret: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
		if (*b).ptr.is_null() {
			current_block = 5881729868994960529;
		} else if (*b).storage > (*b).endbyte + pbytes + BUFFER_INCREMENT as ::core::ffi::c_long {
			current_block = 5881729868994960529;
		} else {
			(*b).storage = (*b).endbyte + pbytes + BUFFER_INCREMENT as ::core::ffi::c_long;
			ret = realloc(
				(*b).buffer as *mut ::core::ffi::c_void,
				(*b).storage as size_t
			);
			if ret.is_null() {
				current_block = 5881729868994960529;
			} else {
				(*b).buffer = ret as *mut ::core::ffi::c_uchar;
				(*b).ptr = (*b).buffer.offset((*b).endbyte as isize);
				current_block = 13513818773234778473;
			}
		}
		match current_block {
			13513818773234778473 => {}
			_ => {
				oggpack_writeclear(b);
				return;
			}
		}
	}
	if (*b).endbit != 0 {
		let mut i: ::core::ffi::c_int = 0;
		i = 0 as ::core::ffi::c_int;
		while (i as ::core::ffi::c_long) < bytes {
			w.expect("non-null function pointer")(
				b,
				*ptr.offset(i as isize) as ::core::ffi::c_ulong,
				8 as ::core::ffi::c_int
			);
			i += 1;
		}
	} else {
		::core::ptr::copy(
			source as *const ::core::ffi::c_uchar,
			(*b).ptr,
			bytes as usize
		);
		(*b).ptr = (*b).ptr.offset(bytes as isize);
		(*b).endbyte += bytes;
		*(*b).ptr = 0 as ::core::ffi::c_uchar;
	}
	if bits != 0 {
		if msb != 0 {
			w.expect("non-null function pointer")(
				b,
				(*ptr.offset(bytes as isize) as ::core::ffi::c_int
					>> 8 as ::core::ffi::c_long - bits) as ::core::ffi::c_ulong,
				bits as ::core::ffi::c_int
			);
		} else {
			w.expect("non-null function pointer")(
				b,
				*ptr.offset(bytes as isize) as ::core::ffi::c_ulong,
				bits as ::core::ffi::c_int
			);
		}
	}
}
#[no_mangle]
pub unsafe extern "C" fn oggpack_writecopy(
	mut b: *mut oggpack_buffer,
	mut source: *mut ::core::ffi::c_void,
	mut bits: ::core::ffi::c_long
) {
	oggpack_writecopy_helper(
		b,
		source,
		bits,
		Some(
			oggpack_write
				as unsafe extern "C" fn(
					*mut oggpack_buffer,
					::core::ffi::c_ulong,
					::core::ffi::c_int
				) -> ()
		),
		0 as ::core::ffi::c_int
	);
}
#[no_mangle]
pub unsafe extern "C" fn oggpackB_writecopy(
	mut b: *mut oggpack_buffer,
	mut source: *mut ::core::ffi::c_void,
	mut bits: ::core::ffi::c_long
) {
	oggpack_writecopy_helper(
		b,
		source,
		bits,
		Some(
			oggpackB_write
				as unsafe extern "C" fn(
					*mut oggpack_buffer,
					::core::ffi::c_ulong,
					::core::ffi::c_int
				) -> ()
		),
		1 as ::core::ffi::c_int
	);
}
#[no_mangle]
pub unsafe extern "C" fn oggpack_reset(mut b: *mut oggpack_buffer) {
	if (*b).ptr.is_null() {
		return;
	}
	(*b).ptr = (*b).buffer;
	*(*b).buffer.offset(0 as ::core::ffi::c_int as isize) = 0 as ::core::ffi::c_uchar;
	(*b).endbyte = 0 as ::core::ffi::c_long;
	(*b).endbit = (*b).endbyte as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn oggpackB_reset(mut b: *mut oggpack_buffer) {
	oggpack_reset(b);
}
#[no_mangle]
pub unsafe extern "C" fn oggpack_writeclear(mut b: *mut oggpack_buffer) {
	if !(*b).buffer.is_null() {
		free((*b).buffer as *mut ::core::ffi::c_void);
	}
	memset(
		b as *mut ::core::ffi::c_void,
		0 as ::core::ffi::c_int,
		::core::mem::size_of::<oggpack_buffer>() as size_t
	);
}
#[no_mangle]
pub unsafe extern "C" fn oggpackB_writeclear(mut b: *mut oggpack_buffer) {
	oggpack_writeclear(b);
}
#[no_mangle]
pub unsafe extern "C" fn oggpack_readinit(
	mut b: *mut oggpack_buffer,
	mut buf: *mut ::core::ffi::c_uchar,
	mut bytes: ::core::ffi::c_int
) {
	memset(
		b as *mut ::core::ffi::c_void,
		0 as ::core::ffi::c_int,
		::core::mem::size_of::<oggpack_buffer>() as size_t
	);
	(*b).ptr = buf;
	(*b).buffer = (*b).ptr;
	(*b).storage = bytes as ::core::ffi::c_long;
}
#[no_mangle]
pub unsafe extern "C" fn oggpackB_readinit(
	mut b: *mut oggpack_buffer,
	mut buf: *mut ::core::ffi::c_uchar,
	mut bytes: ::core::ffi::c_int
) {
	oggpack_readinit(b, buf, bytes);
}
#[no_mangle]
pub unsafe extern "C" fn oggpack_look(
	mut b: *mut oggpack_buffer,
	mut bits: ::core::ffi::c_int
) -> ::core::ffi::c_long {
	let mut ret: ::core::ffi::c_ulong = 0;
	let mut m: ::core::ffi::c_ulong = 0;
	if bits < 0 as ::core::ffi::c_int || bits > 32 as ::core::ffi::c_int {
		return -(1 as ::core::ffi::c_int) as ::core::ffi::c_long;
	}
	m = mask[bits as usize];
	bits += (*b).endbit;
	if (*b).endbyte >= (*b).storage - 4 as ::core::ffi::c_long {
		if (*b).endbyte
			> (*b).storage
				- (bits + 7 as ::core::ffi::c_int >> 3 as ::core::ffi::c_int) as ::core::ffi::c_long
		{
			return -(1 as ::core::ffi::c_int) as ::core::ffi::c_long;
		} else if bits == 0 {
			return 0 as ::core::ffi::c_long;
		}
	}
	ret = (*(*b).ptr.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int >> (*b).endbit)
		as ::core::ffi::c_ulong;
	if bits > 8 as ::core::ffi::c_int {
		ret |= ((*(*b).ptr.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int)
			<< 8 as ::core::ffi::c_int - (*b).endbit) as ::core::ffi::c_ulong;
		if bits > 16 as ::core::ffi::c_int {
			ret |= ((*(*b).ptr.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int)
				<< 16 as ::core::ffi::c_int - (*b).endbit) as ::core::ffi::c_ulong;
			if bits > 24 as ::core::ffi::c_int {
				ret |= ((*(*b).ptr.offset(3 as ::core::ffi::c_int as isize) as ::core::ffi::c_int)
					<< 24 as ::core::ffi::c_int - (*b).endbit) as ::core::ffi::c_ulong;
				if bits > 32 as ::core::ffi::c_int && (*b).endbit != 0 {
					ret |= ((*(*b).ptr.offset(4 as ::core::ffi::c_int as isize)
						as ::core::ffi::c_int)
						<< 32 as ::core::ffi::c_int - (*b).endbit) as ::core::ffi::c_ulong;
				}
			}
		}
	}
	return (m & ret) as ::core::ffi::c_long;
}
#[no_mangle]
pub unsafe extern "C" fn oggpackB_look(
	mut b: *mut oggpack_buffer,
	mut bits: ::core::ffi::c_int
) -> ::core::ffi::c_long {
	let mut ret: ::core::ffi::c_ulong = 0;
	let mut m: ::core::ffi::c_int = 32 as ::core::ffi::c_int - bits;
	if m < 0 as ::core::ffi::c_int || m > 32 as ::core::ffi::c_int {
		return -(1 as ::core::ffi::c_int) as ::core::ffi::c_long;
	}
	bits += (*b).endbit;
	if (*b).endbyte >= (*b).storage - 4 as ::core::ffi::c_long {
		if (*b).endbyte
			> (*b).storage
				- (bits + 7 as ::core::ffi::c_int >> 3 as ::core::ffi::c_int) as ::core::ffi::c_long
		{
			return -(1 as ::core::ffi::c_int) as ::core::ffi::c_long;
		} else if bits == 0 {
			return 0 as ::core::ffi::c_long;
		}
	}
	ret = ((*(*b).ptr.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int)
		<< 24 as ::core::ffi::c_int + (*b).endbit) as ::core::ffi::c_ulong;
	if bits > 8 as ::core::ffi::c_int {
		ret |= ((*(*b).ptr.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int)
			<< 16 as ::core::ffi::c_int + (*b).endbit) as ::core::ffi::c_ulong;
		if bits > 16 as ::core::ffi::c_int {
			ret |= ((*(*b).ptr.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int)
				<< 8 as ::core::ffi::c_int + (*b).endbit) as ::core::ffi::c_ulong;
			if bits > 24 as ::core::ffi::c_int {
				ret |= ((*(*b).ptr.offset(3 as ::core::ffi::c_int as isize) as ::core::ffi::c_int)
					<< (*b).endbit) as ::core::ffi::c_ulong;
				if bits > 32 as ::core::ffi::c_int && (*b).endbit != 0 {
					ret |= (*(*b).ptr.offset(4 as ::core::ffi::c_int as isize)
						as ::core::ffi::c_int
						>> 8 as ::core::ffi::c_int - (*b).endbit) as ::core::ffi::c_ulong;
				}
			}
		}
	}
	return ((ret & 0xffffffff as ::core::ffi::c_ulong)
		>> (m >> 1 as ::core::ffi::c_int)
		>> (m + 1 as ::core::ffi::c_int >> 1 as ::core::ffi::c_int)) as ::core::ffi::c_long;
}
#[no_mangle]
pub unsafe extern "C" fn oggpack_look1(mut b: *mut oggpack_buffer) -> ::core::ffi::c_long {
	if (*b).endbyte >= (*b).storage {
		return -(1 as ::core::ffi::c_int) as ::core::ffi::c_long;
	}
	return (*(*b).ptr.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int >> (*b).endbit
		& 1 as ::core::ffi::c_int) as ::core::ffi::c_long;
}
#[no_mangle]
pub unsafe extern "C" fn oggpackB_look1(mut b: *mut oggpack_buffer) -> ::core::ffi::c_long {
	if (*b).endbyte >= (*b).storage {
		return -(1 as ::core::ffi::c_int) as ::core::ffi::c_long;
	}
	return (*(*b).ptr.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
		>> 7 as ::core::ffi::c_int - (*b).endbit
		& 1 as ::core::ffi::c_int) as ::core::ffi::c_long;
}
#[no_mangle]
pub unsafe extern "C" fn oggpack_adv(mut b: *mut oggpack_buffer, mut bits: ::core::ffi::c_int) {
	bits += (*b).endbit;
	if (*b).endbyte
		> (*b).storage
			- (bits + 7 as ::core::ffi::c_int >> 3 as ::core::ffi::c_int) as ::core::ffi::c_long
	{
		(*b).ptr = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
		(*b).endbyte = (*b).storage;
		(*b).endbit = 1 as ::core::ffi::c_int;
		return;
	} else {
		(*b).ptr = (*b).ptr.offset((bits / 8 as ::core::ffi::c_int) as isize);
		(*b).endbyte += (bits / 8 as ::core::ffi::c_int) as ::core::ffi::c_long;
		(*b).endbit = bits & 7 as ::core::ffi::c_int;
		return;
	};
}
#[no_mangle]
pub unsafe extern "C" fn oggpackB_adv(mut b: *mut oggpack_buffer, mut bits: ::core::ffi::c_int) {
	oggpack_adv(b, bits);
}
#[no_mangle]
pub unsafe extern "C" fn oggpack_adv1(mut b: *mut oggpack_buffer) {
	(*b).endbit += 1;
	if (*b).endbit > 7 as ::core::ffi::c_int {
		(*b).endbit = 0 as ::core::ffi::c_int;
		(*b).ptr = (*b).ptr.offset(1);
		(*b).endbyte += 1;
	}
}
#[no_mangle]
pub unsafe extern "C" fn oggpackB_adv1(mut b: *mut oggpack_buffer) {
	oggpack_adv1(b);
}
#[no_mangle]
pub unsafe extern "C" fn oggpack_read(
	mut b: *mut oggpack_buffer,
	mut bits: ::core::ffi::c_int
) -> ::core::ffi::c_long {
	let mut current_block: u64;
	let mut ret: ::core::ffi::c_long = 0;
	let mut m: ::core::ffi::c_ulong = 0;
	if !(bits < 0 as ::core::ffi::c_int || bits > 32 as ::core::ffi::c_int) {
		m = mask[bits as usize];
		bits += (*b).endbit;
		if (*b).endbyte >= (*b).storage - 4 as ::core::ffi::c_long {
			if (*b).endbyte
				> (*b).storage
					- (bits + 7 as ::core::ffi::c_int >> 3 as ::core::ffi::c_int)
						as ::core::ffi::c_long
			{
				current_block = 6792820664770128024;
			} else {
				if bits == 0 {
					return 0 as ::core::ffi::c_long;
				}
				current_block = 4906268039856690917;
			}
		} else {
			current_block = 4906268039856690917;
		}
		match current_block {
			6792820664770128024 => {}
			_ => {
				ret = (*(*b).ptr.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
					>> (*b).endbit) as ::core::ffi::c_long;
				if bits > 8 as ::core::ffi::c_int {
					ret |= ((*(*b).ptr.offset(1 as ::core::ffi::c_int as isize)
						as ::core::ffi::c_int)
						<< 8 as ::core::ffi::c_int - (*b).endbit) as ::core::ffi::c_long;
					if bits > 16 as ::core::ffi::c_int {
						ret |= ((*(*b).ptr.offset(2 as ::core::ffi::c_int as isize)
							as ::core::ffi::c_int)
							<< 16 as ::core::ffi::c_int - (*b).endbit)
							as ::core::ffi::c_long;
						if bits > 24 as ::core::ffi::c_int {
							ret |= ((*(*b).ptr.offset(3 as ::core::ffi::c_int as isize)
								as ::core::ffi::c_int) << 24 as ::core::ffi::c_int
								- (*b).endbit) as ::core::ffi::c_long;
							if bits > 32 as ::core::ffi::c_int && (*b).endbit != 0 {
								ret |= ((*(*b).ptr.offset(4 as ::core::ffi::c_int as isize)
									as ::core::ffi::c_int) << 32 as ::core::ffi::c_int
									- (*b).endbit) as ::core::ffi::c_long;
							}
						}
					}
				}
				ret = (ret as ::core::ffi::c_ulong & m) as ::core::ffi::c_long;
				(*b).ptr = (*b).ptr.offset((bits / 8 as ::core::ffi::c_int) as isize);
				(*b).endbyte += (bits / 8 as ::core::ffi::c_int) as ::core::ffi::c_long;
				(*b).endbit = bits & 7 as ::core::ffi::c_int;
				return ret;
			}
		}
	}
	(*b).ptr = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
	(*b).endbyte = (*b).storage;
	(*b).endbit = 1 as ::core::ffi::c_int;
	return -(1 as ::core::ffi::c_long);
}
#[no_mangle]
pub unsafe extern "C" fn oggpackB_read(
	mut b: *mut oggpack_buffer,
	mut bits: ::core::ffi::c_int
) -> ::core::ffi::c_long {
	let mut current_block: u64;
	let mut ret: ::core::ffi::c_long = 0;
	let mut m: ::core::ffi::c_long = (32 as ::core::ffi::c_int - bits) as ::core::ffi::c_long;
	if !(m < 0 as ::core::ffi::c_long || m > 32 as ::core::ffi::c_long) {
		bits += (*b).endbit;
		if (*b).endbyte + 4 as ::core::ffi::c_long >= (*b).storage {
			if (*b).endbyte
				> (*b).storage
					- (bits + 7 as ::core::ffi::c_int >> 3 as ::core::ffi::c_int)
						as ::core::ffi::c_long
			{
				current_block = 2347419518545282166;
			} else {
				if bits == 0 {
					return 0 as ::core::ffi::c_long;
				}
				current_block = 15619007995458559411;
			}
		} else {
			current_block = 15619007995458559411;
		}
		match current_block {
			2347419518545282166 => {}
			_ => {
				ret = ((*(*b).ptr.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int)
					<< 24 as ::core::ffi::c_int + (*b).endbit) as ::core::ffi::c_long;
				if bits > 8 as ::core::ffi::c_int {
					ret |= ((*(*b).ptr.offset(1 as ::core::ffi::c_int as isize)
						as ::core::ffi::c_int)
						<< 16 as ::core::ffi::c_int + (*b).endbit) as ::core::ffi::c_long;
					if bits > 16 as ::core::ffi::c_int {
						ret |= ((*(*b).ptr.offset(2 as ::core::ffi::c_int as isize)
							as ::core::ffi::c_int)
							<< 8 as ::core::ffi::c_int + (*b).endbit)
							as ::core::ffi::c_long;
						if bits > 24 as ::core::ffi::c_int {
							ret |= ((*(*b).ptr.offset(3 as ::core::ffi::c_int as isize)
								as ::core::ffi::c_int) << (*b).endbit)
								as ::core::ffi::c_long;
							if bits > 32 as ::core::ffi::c_int && (*b).endbit != 0 {
								ret |= (*(*b).ptr.offset(4 as ::core::ffi::c_int as isize)
									as ::core::ffi::c_int >> 8 as ::core::ffi::c_int
									- (*b).endbit) as ::core::ffi::c_long;
							}
						}
					}
				}
				ret = ((ret as ::core::ffi::c_ulong & 0xffffffff as ::core::ffi::c_ulong)
					>> (m >> 1 as ::core::ffi::c_int)
					>> (m + 1 as ::core::ffi::c_long >> 1 as ::core::ffi::c_int))
					as ::core::ffi::c_long;
				(*b).ptr = (*b).ptr.offset((bits / 8 as ::core::ffi::c_int) as isize);
				(*b).endbyte += (bits / 8 as ::core::ffi::c_int) as ::core::ffi::c_long;
				(*b).endbit = bits & 7 as ::core::ffi::c_int;
				return ret;
			}
		}
	}
	(*b).ptr = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
	(*b).endbyte = (*b).storage;
	(*b).endbit = 1 as ::core::ffi::c_int;
	return -(1 as ::core::ffi::c_long);
}
#[no_mangle]
pub unsafe extern "C" fn oggpack_read1(mut b: *mut oggpack_buffer) -> ::core::ffi::c_long {
	let mut ret: ::core::ffi::c_long = 0;
	if (*b).endbyte >= (*b).storage {
		(*b).ptr = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
		(*b).endbyte = (*b).storage;
		(*b).endbit = 1 as ::core::ffi::c_int;
		return -(1 as ::core::ffi::c_long);
	} else {
		ret = (*(*b).ptr.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
			>> (*b).endbit
			& 1 as ::core::ffi::c_int) as ::core::ffi::c_long;
		(*b).endbit += 1;
		if (*b).endbit > 7 as ::core::ffi::c_int {
			(*b).endbit = 0 as ::core::ffi::c_int;
			(*b).ptr = (*b).ptr.offset(1);
			(*b).endbyte += 1;
		}
		return ret;
	};
}
#[no_mangle]
pub unsafe extern "C" fn oggpackB_read1(mut b: *mut oggpack_buffer) -> ::core::ffi::c_long {
	let mut ret: ::core::ffi::c_long = 0;
	if (*b).endbyte >= (*b).storage {
		(*b).ptr = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
		(*b).endbyte = (*b).storage;
		(*b).endbit = 1 as ::core::ffi::c_int;
		return -(1 as ::core::ffi::c_long);
	} else {
		ret = (*(*b).ptr.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
			>> 7 as ::core::ffi::c_int - (*b).endbit
			& 1 as ::core::ffi::c_int) as ::core::ffi::c_long;
		(*b).endbit += 1;
		if (*b).endbit > 7 as ::core::ffi::c_int {
			(*b).endbit = 0 as ::core::ffi::c_int;
			(*b).ptr = (*b).ptr.offset(1);
			(*b).endbyte += 1;
		}
		return ret;
	};
}
#[no_mangle]
pub unsafe extern "C" fn oggpack_bytes(mut b: *mut oggpack_buffer) -> ::core::ffi::c_long {
	return (*b).endbyte
		+ (((*b).endbit + 7 as ::core::ffi::c_int) / 8 as ::core::ffi::c_int)
			as ::core::ffi::c_long;
}
#[no_mangle]
pub unsafe extern "C" fn oggpack_bits(mut b: *mut oggpack_buffer) -> ::core::ffi::c_long {
	return (*b).endbyte * 8 as ::core::ffi::c_long + (*b).endbit as ::core::ffi::c_long;
}
#[no_mangle]
pub unsafe extern "C" fn oggpackB_bytes(mut b: *mut oggpack_buffer) -> ::core::ffi::c_long {
	return oggpack_bytes(b);
}
#[no_mangle]
pub unsafe extern "C" fn oggpackB_bits(mut b: *mut oggpack_buffer) -> ::core::ffi::c_long {
	return oggpack_bits(b);
}
#[no_mangle]
pub unsafe extern "C" fn oggpack_get_buffer(
	mut b: *mut oggpack_buffer
) -> *mut ::core::ffi::c_uchar {
	return (*b).buffer;
}
#[no_mangle]
pub unsafe extern "C" fn oggpackB_get_buffer(
	mut b: *mut oggpack_buffer
) -> *mut ::core::ffi::c_uchar {
	return oggpack_get_buffer(b);
}
pub const __LONG_MAX__: ::core::ffi::c_long = 9223372036854775807 as ::core::ffi::c_long;
