extern "C" {
	fn malloc(__size: size_t) -> *mut ::core::ffi::c_void;
	fn calloc(__nmemb: size_t, __size: size_t) -> *mut ::core::ffi::c_void;
	fn free(__ptr: *mut ::core::ffi::c_void);
	fn qsort(
		__base: *mut ::core::ffi::c_void,
		__nmemb: size_t,
		__size: size_t,
		__compar: __compar_fn_t
	);
	fn ldexp(__x: ::core::ffi::c_double, __exponent: ::core::ffi::c_int) -> ::core::ffi::c_double;
	fn fabs(__x: ::core::ffi::c_double) -> ::core::ffi::c_double;
	fn floor(__x: ::core::ffi::c_double) -> ::core::ffi::c_double;
	fn rint(__x: ::core::ffi::c_double) -> ::core::ffi::c_double;
	fn memset(
		__s: *mut ::core::ffi::c_void,
		__c: ::core::ffi::c_int,
		__n: size_t
	) -> *mut ::core::ffi::c_void;
}
pub type size_t = usize;
pub type __uint16_t = u16;
pub type __int32_t = i32;
pub type __uint32_t = u32;
pub type __int64_t = i64;
pub type int32_t = __int32_t;
pub type int64_t = __int64_t;
pub type __compar_fn_t = Option<
	unsafe extern "C" fn(
		*const ::core::ffi::c_void,
		*const ::core::ffi::c_void
	) -> ::core::ffi::c_int
>;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
pub type ogg_uint16_t = uint16_t;
pub type ogg_int32_t = int32_t;
pub type ogg_uint32_t = uint32_t;
pub type ogg_int64_t = int64_t;
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
#[no_mangle]
pub unsafe extern "C" fn ov_ilog(mut v: ogg_uint32_t) -> ::core::ffi::c_int {
	let mut ret: ::core::ffi::c_int = 0;
	ret = 0 as ::core::ffi::c_int;
	while v != 0 {
		v >>= 1 as ::core::ffi::c_int;
		ret += 1;
	}
	return ret;
}
pub const VQ_FMAN: ::core::ffi::c_int = 21 as ::core::ffi::c_int;
pub const VQ_FEXP_BIAS: ::core::ffi::c_int = 768 as ::core::ffi::c_int;
#[no_mangle]
pub unsafe extern "C" fn _float32_pack(mut val: ::core::ffi::c_float) -> ::core::ffi::c_long {
	let mut sign: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
	let mut exp: ::core::ffi::c_long = 0;
	let mut mant: ::core::ffi::c_long = 0;
	if val < 0 as ::core::ffi::c_int as ::core::ffi::c_float {
		sign = 0x80000000 as ::core::ffi::c_uint as ::core::ffi::c_int;
		val = -val;
	}
	exp = floor((val as ::core::ffi::c_double).ln() / (2.0f64).ln() + 0.001f64)
		as ::core::ffi::c_long;
	mant = rint(ldexp(
		val as ::core::ffi::c_double,
		((VQ_FMAN - 1 as ::core::ffi::c_int) as ::core::ffi::c_long - exp) as ::core::ffi::c_int
	)) as ::core::ffi::c_long;
	exp = (exp + VQ_FEXP_BIAS as ::core::ffi::c_long) << VQ_FMAN;
	return sign as ::core::ffi::c_long | exp | mant;
}
#[no_mangle]
pub unsafe extern "C" fn _float32_unpack(mut val: ::core::ffi::c_long) -> ::core::ffi::c_float {
	let mut mant: ::core::ffi::c_double =
		(val & 0x1fffff as ::core::ffi::c_long) as ::core::ffi::c_double;
	let mut sign: ::core::ffi::c_int =
		(val & 0x80000000 as ::core::ffi::c_uint as ::core::ffi::c_long) as ::core::ffi::c_int;
	let mut exp: ::core::ffi::c_long = (val & 0x7fe00000 as ::core::ffi::c_long) >> VQ_FMAN;
	if sign != 0 {
		mant = -mant;
	}
	exp = exp
		- (VQ_FMAN - 1 as ::core::ffi::c_int) as ::core::ffi::c_long
		- VQ_FEXP_BIAS as ::core::ffi::c_long;
	if exp > 63 as ::core::ffi::c_long {
		exp = 63 as ::core::ffi::c_long;
	}
	if exp < -(63 as ::core::ffi::c_int) as ::core::ffi::c_long {
		exp = -(63 as ::core::ffi::c_int) as ::core::ffi::c_long;
	}
	return ldexp(mant, exp as ::core::ffi::c_int) as ::core::ffi::c_float;
}
#[no_mangle]
pub unsafe extern "C" fn _make_words(
	mut l: *mut ::core::ffi::c_char,
	mut n: ::core::ffi::c_long,
	mut sparsecount: ::core::ffi::c_long
) -> *mut ogg_uint32_t {
	let mut i: ::core::ffi::c_long = 0;
	let mut j: ::core::ffi::c_long = 0;
	let mut count: ::core::ffi::c_long = 0 as ::core::ffi::c_long;
	let mut marker: [ogg_uint32_t; 33] = [0; 33];
	let mut r: *mut ogg_uint32_t = malloc(
		((if sparsecount != 0 { sparsecount } else { n }) as size_t)
			.wrapping_mul(::core::mem::size_of::<ogg_uint32_t>() as size_t)
	) as *mut ogg_uint32_t;
	memset(
		&raw mut marker as *mut ogg_uint32_t as *mut ::core::ffi::c_void,
		0 as ::core::ffi::c_int,
		::core::mem::size_of::<[ogg_uint32_t; 33]>() as size_t
	);
	i = 0 as ::core::ffi::c_long;
	while i < n {
		let mut length: ::core::ffi::c_long = *l.offset(i as isize) as ::core::ffi::c_long;
		if length > 0 as ::core::ffi::c_long {
			let mut entry: ogg_uint32_t = marker[length as usize];
			if length < 32 as ::core::ffi::c_long && entry >> length != 0 {
				free(r as *mut ::core::ffi::c_void);
				return ::core::ptr::null_mut::<ogg_uint32_t>();
			}
			let fresh0 = count;
			count = count + 1;
			*r.offset(fresh0 as isize) = entry;
			j = length;
			while j > 0 as ::core::ffi::c_long {
				if marker[j as usize] & 1 as ogg_uint32_t != 0 {
					if j == 1 as ::core::ffi::c_long {
						marker[1 as ::core::ffi::c_int as usize] =
							marker[1 as ::core::ffi::c_int as usize].wrapping_add(1);
					} else {
						marker[j as usize] = marker[(j - 1 as ::core::ffi::c_long) as usize]
							<< 1 as ::core::ffi::c_int;
					}
					break;
				} else {
					marker[j as usize] = marker[j as usize].wrapping_add(1);
					j -= 1;
				}
			}
			j = length + 1 as ::core::ffi::c_long;
			while j < 33 as ::core::ffi::c_long {
				if !(marker[j as usize] >> 1 as ::core::ffi::c_int == entry) {
					break;
				}
				entry = marker[j as usize];
				marker[j as usize] =
					marker[(j - 1 as ::core::ffi::c_long) as usize] << 1 as ::core::ffi::c_int;
				j += 1;
			}
		} else if sparsecount == 0 as ::core::ffi::c_long {
			count += 1;
		}
		i += 1;
	}
	if !(count == 1 as ::core::ffi::c_long
		&& marker[2 as ::core::ffi::c_int as usize] == 2 as ogg_uint32_t)
	{
		i = 1 as ::core::ffi::c_long;
		while i < 33 as ::core::ffi::c_long {
			if marker[i as usize] as ::core::ffi::c_ulong
				& 0xffffffff as ::core::ffi::c_ulong >> 32 as ::core::ffi::c_long - i
				!= 0
			{
				free(r as *mut ::core::ffi::c_void);
				return ::core::ptr::null_mut::<ogg_uint32_t>();
			}
			i += 1;
		}
	}
	i = 0 as ::core::ffi::c_long;
	count = 0 as ::core::ffi::c_long;
	while i < n {
		let mut temp: ogg_uint32_t = 0 as ogg_uint32_t;
		j = 0 as ::core::ffi::c_long;
		while j < *l.offset(i as isize) as ::core::ffi::c_long {
			temp <<= 1 as ::core::ffi::c_int;
			temp |= *r.offset(count as isize) >> j & 1 as ogg_uint32_t;
			j += 1;
		}
		if sparsecount != 0 {
			if *l.offset(i as isize) != 0 {
				let fresh1 = count;
				count = count + 1;
				*r.offset(fresh1 as isize) = temp;
			}
		} else {
			let fresh2 = count;
			count = count + 1;
			*r.offset(fresh2 as isize) = temp;
		}
		i += 1;
	}
	return r;
}
unsafe extern "C" fn dec_make_words(
	mut l: *mut ::core::ffi::c_schar,
	mut n: ::core::ffi::c_long,
	mut sparsecount: ::core::ffi::c_long
) -> *mut ogg_int64_t {
	let mut i: ::core::ffi::c_long = 0;
	let mut j: ::core::ffi::c_long = 0;
	let mut count: ::core::ffi::c_long = 0 as ::core::ffi::c_long;
	let mut marker: [ogg_uint32_t; 33] = [0; 33];
	let mut r: *mut ogg_int64_t = malloc(
		((if sparsecount != 0 { sparsecount } else { n }) as size_t)
			.wrapping_mul(::core::mem::size_of::<ogg_int64_t>() as size_t)
	) as *mut ogg_int64_t;
	if r.is_null() {
		return ::core::ptr::null_mut::<ogg_int64_t>();
	}
	memset(
		&raw mut marker as *mut ogg_uint32_t as *mut ::core::ffi::c_void,
		0 as ::core::ffi::c_int,
		::core::mem::size_of::<[ogg_uint32_t; 33]>() as size_t
	);
	i = 0 as ::core::ffi::c_long;
	while i < n {
		let mut length: ::core::ffi::c_long = *l.offset(i as isize) as ::core::ffi::c_long;
		if length > 0 as ::core::ffi::c_long {
			let mut entry: ogg_uint32_t = marker[length as usize];
			if length < 32 as ::core::ffi::c_long && entry >> length != 0 {
				free(r as *mut ::core::ffi::c_void);
				return ::core::ptr::null_mut::<ogg_int64_t>();
			}
			let fresh4 = count;
			count = count + 1;
			*r.offset(fresh4 as isize) = (entry as ogg_int64_t)
				<< 32 as ::core::ffi::c_int - *l.offset(i as isize) as ::core::ffi::c_int
					+ 24 as ::core::ffi::c_int
				| i as ogg_int64_t;
			j = length;
			while j > 0 as ::core::ffi::c_long {
				if marker[j as usize] & 1 as ogg_uint32_t != 0 {
					if j == 1 as ::core::ffi::c_long {
						marker[1 as ::core::ffi::c_int as usize] =
							marker[1 as ::core::ffi::c_int as usize].wrapping_add(1);
					} else {
						marker[j as usize] = marker[(j - 1 as ::core::ffi::c_long) as usize]
							<< 1 as ::core::ffi::c_int;
					}
					break;
				} else {
					marker[j as usize] = marker[j as usize].wrapping_add(1);
					j -= 1;
				}
			}
			j = length + 1 as ::core::ffi::c_long;
			while j < 33 as ::core::ffi::c_long {
				if !(marker[j as usize] >> 1 as ::core::ffi::c_int == entry) {
					break;
				}
				entry = marker[j as usize];
				marker[j as usize] =
					marker[(j - 1 as ::core::ffi::c_long) as usize] << 1 as ::core::ffi::c_int;
				j += 1;
			}
		} else if sparsecount == 0 as ::core::ffi::c_long {
			count += 1;
		}
		i += 1;
	}
	if !(count == 1 as ::core::ffi::c_long
		&& marker[2 as ::core::ffi::c_int as usize] == 2 as ogg_uint32_t)
	{
		i = 1 as ::core::ffi::c_long;
		while i < 33 as ::core::ffi::c_long {
			if marker[i as usize] as ::core::ffi::c_ulong
				& 0xffffffff as ::core::ffi::c_ulong >> 32 as ::core::ffi::c_long - i
				!= 0
			{
				free(r as *mut ::core::ffi::c_void);
				return ::core::ptr::null_mut::<ogg_int64_t>();
			}
			i += 1;
		}
	}
	return r;
}
#[no_mangle]
pub unsafe extern "C" fn _book_maptype1_quantvals(
	mut dim: ::core::ffi::c_long,
	mut entries: ::core::ffi::c_long
) -> ::core::ffi::c_long {
	let mut vals: ::core::ffi::c_long = 0;
	if entries < 1 as ::core::ffi::c_long {
		return 0 as ::core::ffi::c_long;
	}
	vals = floor(
		(entries as ::core::ffi::c_float as ::core::ffi::c_double)
			.powf((1.0f32 / dim as ::core::ffi::c_float) as ::core::ffi::c_double)
	) as ::core::ffi::c_long;
	if vals < 1 as ::core::ffi::c_long {
		vals = 1 as ::core::ffi::c_long;
	}
	loop {
		let mut acc: ::core::ffi::c_long = 1 as ::core::ffi::c_long;
		let mut acc1: ::core::ffi::c_long = 1 as ::core::ffi::c_long;
		let mut i: ::core::ffi::c_int = 0;
		i = 0 as ::core::ffi::c_int;
		while (i as ::core::ffi::c_long) < dim {
			if entries / vals < acc {
				break;
			}
			acc *= vals;
			if LONG_MAX / (vals + 1 as ::core::ffi::c_long) < acc1 {
				acc1 = LONG_MAX;
			} else {
				acc1 *= vals + 1 as ::core::ffi::c_long;
			}
			i += 1;
		}
		if i as ::core::ffi::c_long >= dim && acc <= entries && acc1 > entries {
			return vals;
		} else if (i as ::core::ffi::c_long) < dim || acc > entries {
			vals -= 1;
		} else {
			vals += 1;
		}
	}
}
#[no_mangle]
pub unsafe extern "C" fn _book_unquantize(
	mut r: *mut ::core::ffi::c_float,
	mut b: *const dec_codebook,
	mut n: ogg_int32_t,
	mut sparsemap: *mut ogg_int64_t
) {
	let mut i: ogg_int32_t = 0;
	let mut j: ogg_int32_t = 0;
	let mut k: ogg_int32_t = 0;
	if (*b).maptype as ::core::ffi::c_int == 1 as ::core::ffi::c_int
		|| (*b).maptype as ::core::ffi::c_int == 2 as ::core::ffi::c_int
	{
		let mut quantvals: ogg_int32_t = 0;
		let mut mindel: ::core::ffi::c_float = _float32_unpack((*b).q_min as ::core::ffi::c_long);
		let mut delta: ::core::ffi::c_float = _float32_unpack((*b).q_delta as ::core::ffi::c_long);
		match (*b).maptype as ::core::ffi::c_int {
			1 => {
				quantvals = _book_maptype1_quantvals(
					(*b).dim as ::core::ffi::c_long,
					(*b).entries as ::core::ffi::c_long
				) as ogg_int32_t;
				i = 0 as ::core::ffi::c_int as ogg_int32_t;
				while i < n {
					let mut last: ::core::ffi::c_float = 0.0f32;
					let mut indexdiv: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
					j = (if !sparsemap.is_null() {
						*sparsemap.offset(i as isize) & 0xffffff as ogg_int64_t
					} else {
						i as ogg_int64_t
					}) as ::core::ffi::c_long as ogg_int32_t;
					k = 0 as ::core::ffi::c_int as ogg_int32_t;
					while k < (*b).dim as ogg_int32_t {
						let mut index: ogg_int32_t = j / indexdiv as ogg_int32_t % quantvals;
						let mut val: ::core::ffi::c_float =
							*(*b).quantlist.offset(index as isize) as ::core::ffi::c_float;
						val = (fabs(val as ::core::ffi::c_double) * delta as ::core::ffi::c_double
							+ mindel as ::core::ffi::c_double
							+ last as ::core::ffi::c_double) as ::core::ffi::c_float;
						if (*b).q_sequencep != 0 {
							last = val;
						}
						*r.offset((i * (*b).dim as ogg_int32_t + k) as isize) = val;
						indexdiv *= quantvals as ::core::ffi::c_int;
						k += 1;
					}
					i += 1;
				}
			}
			2 => {
				i = 0 as ::core::ffi::c_int as ogg_int32_t;
				while i < n {
					let mut last_0: ::core::ffi::c_float = 0.0f32;
					j = (if !sparsemap.is_null() {
						*sparsemap.offset(i as isize) & 0xffffff as ogg_int64_t
					} else {
						i as ogg_int64_t
					}) as ogg_int32_t;
					k = 0 as ::core::ffi::c_int as ogg_int32_t;
					while k < (*b).dim as ogg_int32_t {
						let mut val_0: ::core::ffi::c_float = *(*b)
							.quantlist
							.offset((j * (*b).dim as ogg_int32_t + k) as isize)
							as ::core::ffi::c_float;
						val_0 = (fabs(val_0 as ::core::ffi::c_double)
							* delta as ::core::ffi::c_double
							+ mindel as ::core::ffi::c_double
							+ last_0 as ::core::ffi::c_double) as ::core::ffi::c_float;
						if (*b).q_sequencep != 0 {
							last_0 = val_0;
						}
						*r.offset((i * (*b).dim as ogg_int32_t + k) as isize) = val_0;
						k += 1;
					}
					i += 1;
				}
			}
			_ => {}
		}
	}
}
#[no_mangle]
pub unsafe extern "C" fn vorbis_staticbook_destroy(mut b: *mut static_codebook) {
	if (*b).allocedp != 0 {
		if !(*b).quantlist.is_null() {
			free((*b).quantlist as *mut ::core::ffi::c_void);
		}
		if !(*b).lengthlist.is_null() {
			free((*b).lengthlist as *mut ::core::ffi::c_void);
		}
		memset(
			b as *mut ::core::ffi::c_void,
			0 as ::core::ffi::c_int,
			::core::mem::size_of::<static_codebook>() as size_t
		);
		free(b as *mut ::core::ffi::c_void);
	}
}
#[no_mangle]
pub unsafe extern "C" fn vorbis_book_clear(mut b: *mut codebook) {
	if !(*b).valuelist.is_null() {
		free((*b).valuelist as *mut ::core::ffi::c_void);
	}
	if !(*b).codelist.is_null() {
		free((*b).codelist as *mut ::core::ffi::c_void);
	}
	memset(
		b as *mut ::core::ffi::c_void,
		0 as ::core::ffi::c_int,
		::core::mem::size_of::<codebook>() as size_t
	);
}
#[no_mangle]
pub unsafe extern "C" fn vorbis_decbook_clear(mut b: *mut dec_codebook) {
	if !(*b).quantlist.is_null() {
		free((*b).quantlist as *mut ::core::ffi::c_void);
	}
	if !(*b).firsttable.is_null() {
		free((*b).firsttable as *mut ::core::ffi::c_void);
	}
	if !(*b).codelist.is_null() {
		free((*b).codelist as *mut ::core::ffi::c_void);
	}
	if !(*b).codelengths.is_null() {
		free((*b).codelengths as *mut ::core::ffi::c_void);
	}
	if !(*b).index.is_null() {
		free((*b).index as *mut ::core::ffi::c_void);
	}
	if !(*b).valuelist.is_null() {
		free((*b).valuelist as *mut ::core::ffi::c_void);
	}
	memset(
		b as *mut ::core::ffi::c_void,
		0 as ::core::ffi::c_int,
		::core::mem::size_of::<dec_codebook>() as size_t
	);
}
#[no_mangle]
pub unsafe extern "C" fn vorbis_book_init_encode(
	mut c: *mut codebook,
	mut s: *const static_codebook
) -> ::core::ffi::c_int {
	memset(
		c as *mut ::core::ffi::c_void,
		0 as ::core::ffi::c_int,
		::core::mem::size_of::<codebook>() as size_t
	);
	(*c).c = s;
	(*c).entries = (*s).entries;
	(*c).used_entries = (*s).entries;
	(*c).dim = (*s).dim;
	(*c).codelist = _make_words((*s).lengthlist, (*s).entries, 0 as ::core::ffi::c_long);
	(*c).quantvals = _book_maptype1_quantvals((*s).dim, (*s).entries) as ::core::ffi::c_int;
	(*c).minval = rint(_float32_unpack((*s).q_min) as ::core::ffi::c_double) as ::core::ffi::c_int;
	(*c).delta = rint(_float32_unpack((*s).q_delta) as ::core::ffi::c_double) as ::core::ffi::c_int;
	return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn bitreverse(mut x: ogg_uint32_t) -> ogg_uint32_t {
	x = ((x >> 16 as ::core::ffi::c_int) as ::core::ffi::c_ulong & 0xffff as ::core::ffi::c_ulong
		| (x << 16 as ::core::ffi::c_int) as ::core::ffi::c_ulong
			& 0xffff0000 as ::core::ffi::c_ulong) as ogg_uint32_t;
	x = ((x >> 8 as ::core::ffi::c_int) as ::core::ffi::c_ulong & 0xff00ff as ::core::ffi::c_ulong
		| (x << 8 as ::core::ffi::c_int) as ::core::ffi::c_ulong
			& 0xff00ff00 as ::core::ffi::c_ulong) as ogg_uint32_t;
	x = ((x >> 4 as ::core::ffi::c_int) as ::core::ffi::c_ulong & 0xf0f0f0f as ::core::ffi::c_ulong
		| (x << 4 as ::core::ffi::c_int) as ::core::ffi::c_ulong
			& 0xf0f0f0f0 as ::core::ffi::c_ulong) as ogg_uint32_t;
	x = ((x >> 2 as ::core::ffi::c_int) as ::core::ffi::c_ulong
		& 0x33333333 as ::core::ffi::c_ulong
		| (x << 2 as ::core::ffi::c_int) as ::core::ffi::c_ulong
			& 0xcccccccc as ::core::ffi::c_ulong) as ogg_uint32_t;
	return ((x >> 1 as ::core::ffi::c_int) as ::core::ffi::c_ulong
		& 0x55555555 as ::core::ffi::c_ulong
		| (x << 1 as ::core::ffi::c_int) as ::core::ffi::c_ulong
			& 0xaaaaaaaa as ::core::ffi::c_ulong) as ogg_uint32_t;
}
unsafe extern "C" fn sort64a(
	mut a: *const ::core::ffi::c_void,
	mut b: *const ::core::ffi::c_void
) -> ::core::ffi::c_int {
	return (*(a as *mut ogg_int64_t) > *(b as *mut ogg_int64_t)) as ::core::ffi::c_int
		- (*(a as *mut ogg_int64_t) < *(b as *mut ogg_int64_t)) as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn vorbis_book_init_decode(mut c: *mut dec_codebook) -> ::core::ffi::c_int {
	let mut current_block: u64;
	let mut i: ogg_int32_t = 0;
	let mut j: ogg_int32_t = 0;
	let mut n: ogg_int32_t = 0;
	let mut codes: *mut ogg_int64_t = ::core::ptr::null_mut::<ogg_int64_t>();
	if !(*c).codelist.is_null() {
		return 0 as ::core::ffi::c_int;
	}
	if !(*c).codelengths.is_null() {
		n = 0 as ::core::ffi::c_int as ogg_int32_t;
		i = 0 as ::core::ffi::c_int as ogg_int32_t;
		while i < (*c).entries {
			if *(*c).codelengths.offset(i as isize) as ::core::ffi::c_int > 0 as ::core::ffi::c_int
			{
				n += 1;
			}
			i += 1;
		}
		if n > 0 as ogg_int32_t {
			let mut codelengths: *mut ::core::ffi::c_schar =
				::core::ptr::null_mut::<::core::ffi::c_schar>();
			codes = dec_make_words(
				(*c).codelengths,
				(*c).entries as ::core::ffi::c_long,
				n as ::core::ffi::c_long
			);
			if codes.is_null() {
				current_block = 7011271194244703442;
			} else {
				::core::slice::from_raw_parts_mut(codes, n as usize).sort_unstable();
				(*c).codelist = malloc(
					(n as size_t).wrapping_mul(::core::mem::size_of::<ogg_uint32_t>() as size_t)
				) as *mut ogg_uint32_t;
				if (*c).codelist.is_null() {
					current_block = 7011271194244703442;
				} else {
					i = 0 as ::core::ffi::c_int as ogg_int32_t;
					while i < n {
						*(*c).codelist.offset(i as isize) =
							(*codes.offset(i as isize) >> 24 as ::core::ffi::c_int) as ogg_uint32_t;
						i += 1;
					}
					if (*c).maptype as ::core::ffi::c_int == 1 as ::core::ffi::c_int
						|| (*c).maptype as ::core::ffi::c_int == 2 as ::core::ffi::c_int
					{
						(*c).valuelist =
							malloc((((*c).dim as ogg_int32_t * n) as size_t).wrapping_mul(
								::core::mem::size_of::<::core::ffi::c_float>() as size_t
							)) as *mut ::core::ffi::c_float;
						if (*c).valuelist.is_null() {
							current_block = 7011271194244703442;
						} else {
							_book_unquantize((*c).valuelist, c, n, codes);
							free((*c).quantlist as *mut ::core::ffi::c_void);
							(*c).quantlist = ::core::ptr::null_mut::<ogg_uint16_t>();
							current_block = 26972500619410423;
						}
					} else {
						current_block = 26972500619410423;
					}
					match current_block {
						7011271194244703442 => {}
						_ => {
							(*c).index = malloc(
								(n as size_t)
									.wrapping_mul(::core::mem::size_of::<ogg_int32_t>() as size_t)
							) as *mut ogg_int32_t;
							if (*c).index.is_null() {
								current_block = 7011271194244703442;
							} else {
								codelengths =
									malloc((n as size_t).wrapping_mul(::core::mem::size_of::<
										::core::ffi::c_schar
									>() as size_t)) as *mut ::core::ffi::c_schar;
								if codelengths.is_null() {
									current_block = 7011271194244703442;
								} else {
									(*c).minlength = 32 as ::core::ffi::c_schar;
									(*c).maxlength = 0 as ::core::ffi::c_schar;
									i = 0 as ::core::ffi::c_int as ogg_int32_t;
									while i < n {
										j = (*codes.offset(i as isize) & 0xffffff as ogg_int64_t)
											as ogg_int32_t;
										*(*c).index.offset(i as isize) = j;
										*codelengths.offset(i as isize) =
											*(*c).codelengths.offset(j as isize);
										if (*codelengths.offset(i as isize) as ::core::ffi::c_int)
											< (*c).minlength as ::core::ffi::c_int
										{
											(*c).minlength = *codelengths.offset(i as isize);
										}
										if *codelengths.offset(i as isize) as ::core::ffi::c_int
											> (*c).maxlength as ::core::ffi::c_int
										{
											(*c).maxlength = *codelengths.offset(i as isize);
										}
										i += 1;
									}
									free(codes as *mut ::core::ffi::c_void);
									codes = ::core::ptr::null_mut::<ogg_int64_t>();
									free((*c).codelengths as *mut ::core::ffi::c_void);
									(*c).codelengths = codelengths;
									current_block = 168769493162332264;
								}
							}
						}
					}
				}
			}
		} else {
			current_block = 168769493162332264;
		}
	} else {
		n = (*c).entries;
		if n > 0 as ogg_int32_t {
			let mut prev_entry: ogg_uint32_t = 0;
			let mut code: ogg_uint32_t = 0;
			let mut nlengths: ::core::ffi::c_int = 0;
			let mut length: ::core::ffi::c_int = 0;
			let mut l: ::core::ffi::c_int = 0;
			nlengths = (*c).maxlength as ::core::ffi::c_int - (*c).minlength as ::core::ffi::c_int
				+ 1 as ::core::ffi::c_int;
			(*c).codelist = malloc(
				(nlengths as size_t).wrapping_mul(::core::mem::size_of::<ogg_uint32_t>() as size_t)
			) as *mut ogg_uint32_t;
			if (*c).codelist.is_null() {
				current_block = 7011271194244703442;
			} else {
				prev_entry = 0 as ogg_uint32_t;
				code = 0 as ogg_uint32_t;
				length = (*c).minlength as ::core::ffi::c_int;
				l = 0 as ::core::ffi::c_int;
				loop {
					if !(l < nlengths) {
						current_block = 1423531122933789233;
						break;
					}
					let mut nentries: ogg_uint32_t = 0;
					nentries =
						(*(*c).index.offset(l as isize) as ogg_uint32_t).wrapping_sub(prev_entry);
					if (l + 1 as ::core::ffi::c_int) < nlengths
						&& nentries as ::core::ffi::c_ulong
							> (0xffffffff as ::core::ffi::c_ulong)
								.wrapping_sub(code as ::core::ffi::c_ulong)
								>> 32 as ::core::ffi::c_int - length
					{
						current_block = 7011271194244703442;
						break;
					}
					code = code.wrapping_add(nentries << 32 as ::core::ffi::c_int - length);
					*(*c).codelist.offset(l as isize) = code.wrapping_sub(1 as ogg_uint32_t);
					prev_entry = *(*c).index.offset(l as isize) as ogg_uint32_t;
					l += 1;
					length += 1;
				}
				match current_block {
					7011271194244703442 => {}
					_ => {
						if *(*c)
							.codelist
							.offset((nlengths - 1 as ::core::ffi::c_int) as isize)
							as ::core::ffi::c_ulong
							!= 0xffffffff as ::core::ffi::c_ulong
						{
							if n != 1 as ogg_int32_t
								|| (*c).maxlength as ::core::ffi::c_int != 1 as ::core::ffi::c_int
							{
								current_block = 7011271194244703442;
							} else {
								current_block = 5892776923941496671;
							}
						} else {
							current_block = 5892776923941496671;
						}
						match current_block {
							7011271194244703442 => {}
							_ => {
								if (*c).maptype as ::core::ffi::c_int == 1 as ::core::ffi::c_int
									|| (*c).maptype as ::core::ffi::c_int == 2 as ::core::ffi::c_int
								{
									(*c).valuelist = malloc(
										(((*c).dim as ogg_int32_t * n) as size_t).wrapping_mul(
											::core::mem::size_of::<::core::ffi::c_float>()
												as size_t
										)
									) as *mut ::core::ffi::c_float;
									if (*c).valuelist.is_null() {
										current_block = 7011271194244703442;
									} else {
										_book_unquantize(
											(*c).valuelist,
											c,
											n,
											::core::ptr::null_mut::<ogg_int64_t>()
										);
										free((*c).quantlist as *mut ::core::ffi::c_void);
										(*c).quantlist = ::core::ptr::null_mut::<ogg_uint16_t>();
										current_block = 168769493162332264;
									}
								} else {
									current_block = 168769493162332264;
								}
							}
						}
					}
				}
			}
		} else {
			current_block = 168769493162332264;
		}
	}
	match current_block {
		168769493162332264 => {
			if n > 0 as ogg_int32_t {
				if n == 1 as ogg_int32_t
					&& (*c).maxlength as ::core::ffi::c_int == 1 as ::core::ffi::c_int
				{
					(*c).firsttablen = 1 as ::core::ffi::c_schar;
					(*c).firsttable = calloc(
						2 as size_t,
						::core::mem::size_of::<ogg_uint32_t>() as size_t
					) as *mut ogg_uint32_t;
					if (*c).firsttable.is_null() {
						current_block = 7011271194244703442;
					} else {
						let ref mut fresh3 =
							*(*c).firsttable.offset(1 as ::core::ffi::c_int as isize);
						*fresh3 = 1 as ogg_uint32_t;
						*(*c).firsttable.offset(0 as ::core::ffi::c_int as isize) = *fresh3;
						current_block = 12608488225262500095;
					}
				} else {
					let mut used_bits: ::core::ffi::c_int = 0;
					let mut tabn: ::core::ffi::c_int = 0;
					used_bits = ov_ilog(n as ogg_uint32_t);
					(*c).firsttablen =
						(used_bits - 4 as ::core::ffi::c_int) as ::core::ffi::c_schar;
					if ((*c).firsttablen as ::core::ffi::c_int) < 5 as ::core::ffi::c_int {
						(*c).firsttablen = 5 as ::core::ffi::c_schar;
					}
					if ((*c).firsttablen as ::core::ffi::c_int)
						< (*c).minlength as ::core::ffi::c_int + 1 as ::core::ffi::c_int
					{
						(*c).firsttablen = ((*c).minlength as ::core::ffi::c_int
							+ 1 as ::core::ffi::c_int)
							as ::core::ffi::c_schar;
					}
					if (*c).firsttablen as ::core::ffi::c_int > (*c).maxlength as ::core::ffi::c_int
					{
						(*c).firsttablen = (*c).maxlength;
					}
					if (*c).codelengths.is_null()
						&& (*c).minlength as ::core::ffi::c_int > 8 as ::core::ffi::c_int
					{
						(*c).firsttablen = ((*c).maxlength as ::core::ffi::c_int
							- (*c).minlength as ::core::ffi::c_int
							+ 1 as ::core::ffi::c_int)
							as ::core::ffi::c_schar;
					}
					if (*c).firsttablen as ::core::ffi::c_int > 8 as ::core::ffi::c_int {
						(*c).firsttablen = 8 as ::core::ffi::c_schar;
					}
					tabn = (1 as ::core::ffi::c_int) << (*c).firsttablen as ::core::ffi::c_int;
					(*c).firsttable = calloc(
						tabn as size_t,
						::core::mem::size_of::<ogg_uint32_t>() as size_t
					) as *mut ogg_uint32_t;
					if (*c).firsttable.is_null() {
						current_block = 7011271194244703442;
					} else {
						if !(*c).codelengths.is_null() {
							i = 0 as ::core::ffi::c_int as ogg_int32_t;
							while i < n {
								if *(*c).codelengths.offset(i as isize) as ::core::ffi::c_int
									<= (*c).firsttablen as ::core::ffi::c_int
								{
									let mut orig: ogg_uint32_t =
										bitreverse(*(*c).codelist.offset(i as isize));
									j = 0 as ::core::ffi::c_int as ogg_int32_t;
									while j
										< (1 as ogg_int32_t)
											<< (*c).firsttablen as ::core::ffi::c_int
												- *(*c).codelengths.offset(i as isize)
													as ::core::ffi::c_int
									{
										*(*c).firsttable.offset(
											(orig
												| (j << *(*c).codelengths.offset(i as isize)
													as ::core::ffi::c_int)
													as ogg_uint32_t) as isize
										) = (i << 6 as ::core::ffi::c_int
											| *(*c).codelengths.offset(i as isize) as ogg_int32_t)
											as ogg_uint32_t;
										j += 1;
									}
								}
								i += 1;
							}
							let mut mask: ogg_uint32_t = ((0xfffffffe as ::core::ffi::c_ulong)
								<< 31 as ::core::ffi::c_int
									- (*c).firsttablen as ::core::ffi::c_int)
								as ogg_uint32_t;
							let mut lo: ::core::ffi::c_long = 0 as ::core::ffi::c_long;
							let mut hi: ::core::ffi::c_long = 0 as ::core::ffi::c_long;
							let mut hint_shift: ::core::ffi::c_int = 0;
							(*c).hi_max = n;
							hint_shift = if used_bits > 15 as ::core::ffi::c_int {
								used_bits - 15 as ::core::ffi::c_int
							} else {
								0 as ::core::ffi::c_int
							};
							(*c).hint_shift = hint_shift as ::core::ffi::c_schar;
							i = 0 as ::core::ffi::c_int as ogg_int32_t;
							while i < tabn as ogg_int32_t {
								let mut word: ogg_uint32_t = (i as ogg_uint32_t)
									<< 32 as ::core::ffi::c_int
										- (*c).firsttablen as ::core::ffi::c_int;
								if *(*c).firsttable.offset(bitreverse(word) as isize)
									== 0 as ogg_uint32_t
								{
									while (lo + 1 as ::core::ffi::c_long) < n as ::core::ffi::c_long
										&& *(*c)
											.codelist
											.offset((lo + 1 as ::core::ffi::c_long) as isize)
											<= word
									{
										lo += 1;
									}
									while hi < n as ::core::ffi::c_long
										&& word >= *(*c).codelist.offset(hi as isize) & mask
									{
										hi += 1;
									}
									let mut loval: ::core::ffi::c_ulong =
										(lo >> hint_shift) as ::core::ffi::c_ulong;
									let mut hival: ::core::ffi::c_ulong = (n as ::core::ffi::c_long
										- hi >> hint_shift)
										as ::core::ffi::c_ulong;
									if loval > 0x7fff as ::core::ffi::c_ulong {
										loval = 0x7fff as ::core::ffi::c_ulong;
									}
									if hival > 0x7fff as ::core::ffi::c_ulong {
										hival = 0x7fff as ::core::ffi::c_ulong;
									}
									*(*c).firsttable.offset(bitreverse(word) as isize) =
										(0x80000000 as ::core::ffi::c_ulong
											| loval << 15 as ::core::ffi::c_int
											| hival) as ogg_uint32_t;
								}
								i += 1;
							}
						} else {
							let mut code_0: ogg_uint32_t = 0;
							let mut nlengths_0: ::core::ffi::c_int = 0;
							let mut length_0: ::core::ffi::c_int = 0;
							let mut l_0: ::core::ffi::c_int = 0;
							nlengths_0 = (*c).maxlength as ::core::ffi::c_int
								- (*c).minlength as ::core::ffi::c_int
								+ 1 as ::core::ffi::c_int;
							(*c).hi_max = (nlengths_0 - 1 as ::core::ffi::c_int) as ogg_int32_t;
							(*c).hint_shift = 0 as ::core::ffi::c_schar;
							length_0 = (*c).minlength as ::core::ffi::c_int;
							code_0 = 0 as ogg_uint32_t;
							l_0 = 0 as ::core::ffi::c_int;
							i = l_0 as ogg_int32_t;
							while length_0 <= (*c).firsttablen as ::core::ffi::c_int {
								while i < *(*c).index.offset(l_0 as isize) {
									let mut orig_0: ogg_uint32_t =
										bitreverse(code_0 << 32 as ::core::ffi::c_int - length_0);
									j = 0 as ::core::ffi::c_int as ogg_int32_t;
									while j
										< (1 as ogg_int32_t)
											<< (*c).firsttablen as ::core::ffi::c_int - length_0
									{
										*(*c).firsttable.offset(
											((j << length_0) as ogg_uint32_t | orig_0) as isize
										) = (i << 6 as ::core::ffi::c_int | length_0 as ogg_int32_t)
											as ogg_uint32_t;
										j += 1;
									}
									i += 1;
									code_0 = code_0.wrapping_add(1);
								}
								code_0 <<= 1 as ::core::ffi::c_int;
								l_0 += 1;
								length_0 += 1;
							}
							if l_0 < nlengths_0 {
								let mut lo_0: ::core::ffi::c_int = l_0;
								loop {
									let mut nleft: ogg_uint32_t = 0;
									let mut slot_count: ogg_uint32_t = 0;
									nleft = (*(*c).codelist.offset(l_0 as isize)
										>> 32 as ::core::ffi::c_int - length_0)
										.wrapping_sub(code_0)
										.wrapping_add(1 as ogg_uint32_t);
									slot_count = (1 as ::core::ffi::c_int as ogg_uint32_t)
										<< length_0 - (*c).firsttablen as ::core::ffi::c_int;
									if nleft >= slot_count {
										let mut word_0: ogg_uint32_t = code_0
											>> length_0 - (*c).firsttablen as ::core::ffi::c_int;
										*(*c).firsttable.offset(bitreverse(
											word_0
												<< 32 as ::core::ffi::c_int
													- (*c).firsttablen as ::core::ffi::c_int
										) as isize) = (0x80000000 as ::core::ffi::c_ulong
											| ((lo_0 as ogg_uint32_t) << 15 as ::core::ffi::c_int)
												as ::core::ffi::c_ulong | (nlengths_0
											- 1 as ::core::ffi::c_int
											- l_0)
											as ::core::ffi::c_ulong) as ogg_uint32_t;
										code_0 = code_0.wrapping_add(slot_count);
										lo_0 = l_0;
									} else {
										l_0 += 1;
										length_0 += 1;
										code_0 <<= 1 as ::core::ffi::c_int;
										if nleft == 0 as ogg_uint32_t {
											lo_0 = l_0;
										}
									}
									if !(l_0 < nlengths_0) {
										break;
									}
								}
							}
						}
						current_block = 12608488225262500095;
					}
				}
			} else {
				current_block = 12608488225262500095;
			}
			match current_block {
				7011271194244703442 => {}
				_ => return 0 as ::core::ffi::c_int
			}
		}
		_ => {}
	}
	if !codes.is_null() {
		free(codes as *mut ::core::ffi::c_void);
	}
	return -(1 as ::core::ffi::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn vorbis_book_codeword(
	mut book: *mut codebook,
	mut entry: ::core::ffi::c_int
) -> ::core::ffi::c_long {
	if !(*book).c.is_null() {
		return *(*book).codelist.offset(entry as isize) as ::core::ffi::c_long;
	}
	return -(1 as ::core::ffi::c_int) as ::core::ffi::c_long;
}
#[no_mangle]
pub unsafe extern "C" fn vorbis_book_codelen(
	mut book: *mut codebook,
	mut entry: ::core::ffi::c_int
) -> ::core::ffi::c_long {
	if !(*book).c.is_null() {
		return *(*(*book).c).lengthlist.offset(entry as isize) as ::core::ffi::c_long;
	}
	return -(1 as ::core::ffi::c_int) as ::core::ffi::c_long;
}
pub const LONG_MAX: ::core::ffi::c_long = __LONG_MAX__;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const __LONG_MAX__: ::core::ffi::c_long = 9223372036854775807 as ::core::ffi::c_long;
