extern "C" {
	fn malloc(__size: size_t) -> *mut ::core::ffi::c_void;
	fn free(__ptr: *mut ::core::ffi::c_void);
	fn memset(
		__s: *mut ::core::ffi::c_void,
		__c: ::core::ffi::c_int,
		__n: size_t
	) -> *mut ::core::ffi::c_void;
	fn rint(__x: ::core::ffi::c_double) -> ::core::ffi::c_double;
}
pub type size_t = usize;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mdct_lookup {
	pub n: ::core::ffi::c_int,
	pub log2n: ::core::ffi::c_int,
	pub trig: *mut ::core::ffi::c_float,
	pub bitrev: *mut ::core::ffi::c_int,
	pub scale: ::core::ffi::c_float
}
#[no_mangle]
pub unsafe extern "C" fn mdct_init(mut lookup: *mut mdct_lookup, mut n: ::core::ffi::c_int) {
	let mut bitrev: *mut ::core::ffi::c_int = malloc(
		(::core::mem::size_of::<::core::ffi::c_int>() as size_t)
			.wrapping_mul((n / 4 as ::core::ffi::c_int) as size_t)
	) as *mut ::core::ffi::c_int;
	let mut T: *mut ::core::ffi::c_float = malloc(
		(::core::mem::size_of::<::core::ffi::c_float>() as size_t)
			.wrapping_mul((n + n / 4 as ::core::ffi::c_int) as size_t)
	) as *mut ::core::ffi::c_float;
	let mut i: ::core::ffi::c_int = 0;
	let mut n2: ::core::ffi::c_int = n >> 1 as ::core::ffi::c_int;
	(*lookup).log2n = rint((n as ::core::ffi::c_float as ::core::ffi::c_double).ln() / 2.0f64.ln())
		as ::core::ffi::c_int;
	let mut log2n: ::core::ffi::c_int = (*lookup).log2n;
	(*lookup).n = n;
	(*lookup).trig = T;
	(*lookup).bitrev = bitrev;
	i = 0 as ::core::ffi::c_int;
	while i < n / 4 as ::core::ffi::c_int {
		*T.offset((i * 2 as ::core::ffi::c_int) as isize) = (3.14159265358979323846f64
			/ n as ::core::ffi::c_double
			* (4 as ::core::ffi::c_int * i) as ::core::ffi::c_double)
			.cos() as ::core::ffi::c_float;
		*T.offset((i * 2 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize) =
			-(3.14159265358979323846f64 / n as ::core::ffi::c_double
				* (4 as ::core::ffi::c_int * i) as ::core::ffi::c_double)
				.sin() as ::core::ffi::c_float;
		*T.offset((n2 + i * 2 as ::core::ffi::c_int) as isize) = (3.14159265358979323846f64
			/ (2 as ::core::ffi::c_int * n) as ::core::ffi::c_double
			* (2 as ::core::ffi::c_int * i + 1 as ::core::ffi::c_int) as ::core::ffi::c_double)
			.cos() as ::core::ffi::c_float;
		*T.offset((n2 + i * 2 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize) =
			(3.14159265358979323846f64 / (2 as ::core::ffi::c_int * n) as ::core::ffi::c_double
				* (2 as ::core::ffi::c_int * i + 1 as ::core::ffi::c_int) as ::core::ffi::c_double)
				.sin() as ::core::ffi::c_float;
		i += 1;
	}
	i = 0 as ::core::ffi::c_int;
	while i < n / 8 as ::core::ffi::c_int {
		*T.offset((n + i * 2 as ::core::ffi::c_int) as isize) =
			((3.14159265358979323846f64 / n as ::core::ffi::c_double
				* (4 as ::core::ffi::c_int * i + 2 as ::core::ffi::c_int) as ::core::ffi::c_double)
				.cos() * 0.5f64) as ::core::ffi::c_float;
		*T.offset((n + i * 2 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize) =
			(-(3.14159265358979323846f64 / n as ::core::ffi::c_double
				* (4 as ::core::ffi::c_int * i + 2 as ::core::ffi::c_int) as ::core::ffi::c_double)
				.sin() * 0.5f64) as ::core::ffi::c_float;
		i += 1;
	}
	let mut mask: ::core::ffi::c_int =
		((1 as ::core::ffi::c_int) << log2n - 1 as ::core::ffi::c_int) - 1 as ::core::ffi::c_int;
	let mut i_0: ::core::ffi::c_int = 0;
	let mut j: ::core::ffi::c_int = 0;
	let mut msb: ::core::ffi::c_int = (1 as ::core::ffi::c_int) << log2n - 2 as ::core::ffi::c_int;
	i_0 = 0 as ::core::ffi::c_int;
	while i_0 < n / 8 as ::core::ffi::c_int {
		let mut acc: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
		j = 0 as ::core::ffi::c_int;
		while msb >> j != 0 {
			if msb >> j & i_0 != 0 {
				acc |= (1 as ::core::ffi::c_int) << j;
			}
			j += 1;
		}
		*bitrev.offset((i_0 * 2 as ::core::ffi::c_int) as isize) =
			(!acc & mask) - 1 as ::core::ffi::c_int;
		*bitrev.offset((i_0 * 2 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize) = acc;
		i_0 += 1;
	}
	(*lookup).scale = 4.0f32 / n as ::core::ffi::c_float;
}
#[inline]
unsafe extern "C" fn mdct_butterfly_8(mut x: *mut ::core::ffi::c_float) {
	let mut r0: ::core::ffi::c_float =
		*x.offset(6 as ::core::ffi::c_int as isize) + *x.offset(2 as ::core::ffi::c_int as isize);
	let mut r1: ::core::ffi::c_float =
		*x.offset(6 as ::core::ffi::c_int as isize) - *x.offset(2 as ::core::ffi::c_int as isize);
	let mut r2: ::core::ffi::c_float =
		*x.offset(4 as ::core::ffi::c_int as isize) + *x.offset(0 as ::core::ffi::c_int as isize);
	let mut r3: ::core::ffi::c_float =
		*x.offset(4 as ::core::ffi::c_int as isize) - *x.offset(0 as ::core::ffi::c_int as isize);
	*x.offset(6 as ::core::ffi::c_int as isize) = r0 + r2;
	*x.offset(4 as ::core::ffi::c_int as isize) = r0 - r2;
	r0 = *x.offset(5 as ::core::ffi::c_int as isize) - *x.offset(1 as ::core::ffi::c_int as isize);
	r2 = *x.offset(7 as ::core::ffi::c_int as isize) - *x.offset(3 as ::core::ffi::c_int as isize);
	*x.offset(0 as ::core::ffi::c_int as isize) = r1 + r0;
	*x.offset(2 as ::core::ffi::c_int as isize) = r1 - r0;
	r0 = *x.offset(5 as ::core::ffi::c_int as isize) + *x.offset(1 as ::core::ffi::c_int as isize);
	r1 = *x.offset(7 as ::core::ffi::c_int as isize) + *x.offset(3 as ::core::ffi::c_int as isize);
	*x.offset(3 as ::core::ffi::c_int as isize) = r2 + r3;
	*x.offset(1 as ::core::ffi::c_int as isize) = r2 - r3;
	*x.offset(7 as ::core::ffi::c_int as isize) = r1 + r0;
	*x.offset(5 as ::core::ffi::c_int as isize) = r1 - r0;
}
#[inline]
unsafe extern "C" fn mdct_butterfly_16(mut x: *mut ::core::ffi::c_float) {
	let mut r0: ::core::ffi::c_float =
		*x.offset(1 as ::core::ffi::c_int as isize) - *x.offset(9 as ::core::ffi::c_int as isize);
	let mut r1: ::core::ffi::c_float =
		*x.offset(0 as ::core::ffi::c_int as isize) - *x.offset(8 as ::core::ffi::c_int as isize);
	*x.offset(8 as ::core::ffi::c_int as isize) += *x.offset(0 as ::core::ffi::c_int as isize);
	*x.offset(9 as ::core::ffi::c_int as isize) += *x.offset(1 as ::core::ffi::c_int as isize);
	*x.offset(0 as ::core::ffi::c_int as isize) = (r0 + r1) * 0.70710678118654752441f32;
	*x.offset(1 as ::core::ffi::c_int as isize) = (r0 - r1) * 0.70710678118654752441f32;
	r0 = *x.offset(3 as ::core::ffi::c_int as isize) - *x.offset(11 as ::core::ffi::c_int as isize);
	r1 = *x.offset(10 as ::core::ffi::c_int as isize) - *x.offset(2 as ::core::ffi::c_int as isize);
	*x.offset(10 as ::core::ffi::c_int as isize) += *x.offset(2 as ::core::ffi::c_int as isize);
	*x.offset(11 as ::core::ffi::c_int as isize) += *x.offset(3 as ::core::ffi::c_int as isize);
	*x.offset(2 as ::core::ffi::c_int as isize) = r0;
	*x.offset(3 as ::core::ffi::c_int as isize) = r1;
	r0 = *x.offset(12 as ::core::ffi::c_int as isize) - *x.offset(4 as ::core::ffi::c_int as isize);
	r1 = *x.offset(13 as ::core::ffi::c_int as isize) - *x.offset(5 as ::core::ffi::c_int as isize);
	*x.offset(12 as ::core::ffi::c_int as isize) += *x.offset(4 as ::core::ffi::c_int as isize);
	*x.offset(13 as ::core::ffi::c_int as isize) += *x.offset(5 as ::core::ffi::c_int as isize);
	*x.offset(4 as ::core::ffi::c_int as isize) = (r0 - r1) * 0.70710678118654752441f32;
	*x.offset(5 as ::core::ffi::c_int as isize) = (r0 + r1) * 0.70710678118654752441f32;
	r0 = *x.offset(14 as ::core::ffi::c_int as isize) - *x.offset(6 as ::core::ffi::c_int as isize);
	r1 = *x.offset(15 as ::core::ffi::c_int as isize) - *x.offset(7 as ::core::ffi::c_int as isize);
	*x.offset(14 as ::core::ffi::c_int as isize) += *x.offset(6 as ::core::ffi::c_int as isize);
	*x.offset(15 as ::core::ffi::c_int as isize) += *x.offset(7 as ::core::ffi::c_int as isize);
	*x.offset(6 as ::core::ffi::c_int as isize) = r0;
	*x.offset(7 as ::core::ffi::c_int as isize) = r1;
	mdct_butterfly_8(x);
	mdct_butterfly_8(x.offset(8 as ::core::ffi::c_int as isize));
}
#[inline]
unsafe extern "C" fn mdct_butterfly_32(mut x: *mut ::core::ffi::c_float) {
	let mut r0: ::core::ffi::c_float =
		*x.offset(30 as ::core::ffi::c_int as isize) - *x.offset(14 as ::core::ffi::c_int as isize);
	let mut r1: ::core::ffi::c_float =
		*x.offset(31 as ::core::ffi::c_int as isize) - *x.offset(15 as ::core::ffi::c_int as isize);
	*x.offset(30 as ::core::ffi::c_int as isize) += *x.offset(14 as ::core::ffi::c_int as isize);
	*x.offset(31 as ::core::ffi::c_int as isize) += *x.offset(15 as ::core::ffi::c_int as isize);
	*x.offset(14 as ::core::ffi::c_int as isize) = r0;
	*x.offset(15 as ::core::ffi::c_int as isize) = r1;
	r0 =
		*x.offset(28 as ::core::ffi::c_int as isize) - *x.offset(12 as ::core::ffi::c_int as isize);
	r1 =
		*x.offset(29 as ::core::ffi::c_int as isize) - *x.offset(13 as ::core::ffi::c_int as isize);
	*x.offset(28 as ::core::ffi::c_int as isize) += *x.offset(12 as ::core::ffi::c_int as isize);
	*x.offset(29 as ::core::ffi::c_int as isize) += *x.offset(13 as ::core::ffi::c_int as isize);
	*x.offset(12 as ::core::ffi::c_int as isize) =
		r0 * 0.92387953251128675613f32 - r1 * 0.38268343236508977175f32;
	*x.offset(13 as ::core::ffi::c_int as isize) =
		r0 * 0.38268343236508977175f32 + r1 * 0.92387953251128675613f32;
	r0 =
		*x.offset(26 as ::core::ffi::c_int as isize) - *x.offset(10 as ::core::ffi::c_int as isize);
	r1 =
		*x.offset(27 as ::core::ffi::c_int as isize) - *x.offset(11 as ::core::ffi::c_int as isize);
	*x.offset(26 as ::core::ffi::c_int as isize) += *x.offset(10 as ::core::ffi::c_int as isize);
	*x.offset(27 as ::core::ffi::c_int as isize) += *x.offset(11 as ::core::ffi::c_int as isize);
	*x.offset(10 as ::core::ffi::c_int as isize) = (r0 - r1) * 0.70710678118654752441f32;
	*x.offset(11 as ::core::ffi::c_int as isize) = (r0 + r1) * 0.70710678118654752441f32;
	r0 = *x.offset(24 as ::core::ffi::c_int as isize) - *x.offset(8 as ::core::ffi::c_int as isize);
	r1 = *x.offset(25 as ::core::ffi::c_int as isize) - *x.offset(9 as ::core::ffi::c_int as isize);
	*x.offset(24 as ::core::ffi::c_int as isize) += *x.offset(8 as ::core::ffi::c_int as isize);
	*x.offset(25 as ::core::ffi::c_int as isize) += *x.offset(9 as ::core::ffi::c_int as isize);
	*x.offset(8 as ::core::ffi::c_int as isize) =
		r0 * 0.38268343236508977175f32 - r1 * 0.92387953251128675613f32;
	*x.offset(9 as ::core::ffi::c_int as isize) =
		r1 * 0.38268343236508977175f32 + r0 * 0.92387953251128675613f32;
	r0 = *x.offset(22 as ::core::ffi::c_int as isize) - *x.offset(6 as ::core::ffi::c_int as isize);
	r1 = *x.offset(7 as ::core::ffi::c_int as isize) - *x.offset(23 as ::core::ffi::c_int as isize);
	*x.offset(22 as ::core::ffi::c_int as isize) += *x.offset(6 as ::core::ffi::c_int as isize);
	*x.offset(23 as ::core::ffi::c_int as isize) += *x.offset(7 as ::core::ffi::c_int as isize);
	*x.offset(6 as ::core::ffi::c_int as isize) = r1;
	*x.offset(7 as ::core::ffi::c_int as isize) = r0;
	r0 = *x.offset(4 as ::core::ffi::c_int as isize) - *x.offset(20 as ::core::ffi::c_int as isize);
	r1 = *x.offset(5 as ::core::ffi::c_int as isize) - *x.offset(21 as ::core::ffi::c_int as isize);
	*x.offset(20 as ::core::ffi::c_int as isize) += *x.offset(4 as ::core::ffi::c_int as isize);
	*x.offset(21 as ::core::ffi::c_int as isize) += *x.offset(5 as ::core::ffi::c_int as isize);
	*x.offset(4 as ::core::ffi::c_int as isize) =
		r1 * 0.92387953251128675613f32 + r0 * 0.38268343236508977175f32;
	*x.offset(5 as ::core::ffi::c_int as isize) =
		r1 * 0.38268343236508977175f32 - r0 * 0.92387953251128675613f32;
	r0 = *x.offset(2 as ::core::ffi::c_int as isize) - *x.offset(18 as ::core::ffi::c_int as isize);
	r1 = *x.offset(3 as ::core::ffi::c_int as isize) - *x.offset(19 as ::core::ffi::c_int as isize);
	*x.offset(18 as ::core::ffi::c_int as isize) += *x.offset(2 as ::core::ffi::c_int as isize);
	*x.offset(19 as ::core::ffi::c_int as isize) += *x.offset(3 as ::core::ffi::c_int as isize);
	*x.offset(2 as ::core::ffi::c_int as isize) = (r1 + r0) * 0.70710678118654752441f32;
	*x.offset(3 as ::core::ffi::c_int as isize) = (r1 - r0) * 0.70710678118654752441f32;
	r0 = *x.offset(0 as ::core::ffi::c_int as isize) - *x.offset(16 as ::core::ffi::c_int as isize);
	r1 = *x.offset(1 as ::core::ffi::c_int as isize) - *x.offset(17 as ::core::ffi::c_int as isize);
	*x.offset(16 as ::core::ffi::c_int as isize) += *x.offset(0 as ::core::ffi::c_int as isize);
	*x.offset(17 as ::core::ffi::c_int as isize) += *x.offset(1 as ::core::ffi::c_int as isize);
	*x.offset(0 as ::core::ffi::c_int as isize) =
		r1 * 0.38268343236508977175f32 + r0 * 0.92387953251128675613f32;
	*x.offset(1 as ::core::ffi::c_int as isize) =
		r1 * 0.92387953251128675613f32 - r0 * 0.38268343236508977175f32;
	mdct_butterfly_16(x);
	mdct_butterfly_16(x.offset(16 as ::core::ffi::c_int as isize));
}
#[inline]
unsafe extern "C" fn mdct_butterfly_first(
	mut T: *mut ::core::ffi::c_float,
	mut x: *mut ::core::ffi::c_float,
	mut points: ::core::ffi::c_int
) {
	let mut x1: *mut ::core::ffi::c_float = x
		.offset(points as isize)
		.offset(-(8 as ::core::ffi::c_int as isize));
	let mut x2: *mut ::core::ffi::c_float = x
		.offset((points >> 1 as ::core::ffi::c_int) as isize)
		.offset(-(8 as ::core::ffi::c_int as isize));
	let mut r0: ::core::ffi::c_float = 0.;
	let mut r1: ::core::ffi::c_float = 0.;
	loop {
		r0 = *x1.offset(6 as ::core::ffi::c_int as isize)
			- *x2.offset(6 as ::core::ffi::c_int as isize);
		r1 = *x1.offset(7 as ::core::ffi::c_int as isize)
			- *x2.offset(7 as ::core::ffi::c_int as isize);
		*x1.offset(6 as ::core::ffi::c_int as isize) +=
			*x2.offset(6 as ::core::ffi::c_int as isize);
		*x1.offset(7 as ::core::ffi::c_int as isize) +=
			*x2.offset(7 as ::core::ffi::c_int as isize);
		*x2.offset(6 as ::core::ffi::c_int as isize) = r1
			* *T.offset(1 as ::core::ffi::c_int as isize)
			+ r0 * *T.offset(0 as ::core::ffi::c_int as isize);
		*x2.offset(7 as ::core::ffi::c_int as isize) = r1
			* *T.offset(0 as ::core::ffi::c_int as isize)
			- r0 * *T.offset(1 as ::core::ffi::c_int as isize);
		r0 = *x1.offset(4 as ::core::ffi::c_int as isize)
			- *x2.offset(4 as ::core::ffi::c_int as isize);
		r1 = *x1.offset(5 as ::core::ffi::c_int as isize)
			- *x2.offset(5 as ::core::ffi::c_int as isize);
		*x1.offset(4 as ::core::ffi::c_int as isize) +=
			*x2.offset(4 as ::core::ffi::c_int as isize);
		*x1.offset(5 as ::core::ffi::c_int as isize) +=
			*x2.offset(5 as ::core::ffi::c_int as isize);
		*x2.offset(4 as ::core::ffi::c_int as isize) = r1
			* *T.offset(5 as ::core::ffi::c_int as isize)
			+ r0 * *T.offset(4 as ::core::ffi::c_int as isize);
		*x2.offset(5 as ::core::ffi::c_int as isize) = r1
			* *T.offset(4 as ::core::ffi::c_int as isize)
			- r0 * *T.offset(5 as ::core::ffi::c_int as isize);
		r0 = *x1.offset(2 as ::core::ffi::c_int as isize)
			- *x2.offset(2 as ::core::ffi::c_int as isize);
		r1 = *x1.offset(3 as ::core::ffi::c_int as isize)
			- *x2.offset(3 as ::core::ffi::c_int as isize);
		*x1.offset(2 as ::core::ffi::c_int as isize) +=
			*x2.offset(2 as ::core::ffi::c_int as isize);
		*x1.offset(3 as ::core::ffi::c_int as isize) +=
			*x2.offset(3 as ::core::ffi::c_int as isize);
		*x2.offset(2 as ::core::ffi::c_int as isize) = r1
			* *T.offset(9 as ::core::ffi::c_int as isize)
			+ r0 * *T.offset(8 as ::core::ffi::c_int as isize);
		*x2.offset(3 as ::core::ffi::c_int as isize) = r1
			* *T.offset(8 as ::core::ffi::c_int as isize)
			- r0 * *T.offset(9 as ::core::ffi::c_int as isize);
		r0 = *x1.offset(0 as ::core::ffi::c_int as isize)
			- *x2.offset(0 as ::core::ffi::c_int as isize);
		r1 = *x1.offset(1 as ::core::ffi::c_int as isize)
			- *x2.offset(1 as ::core::ffi::c_int as isize);
		*x1.offset(0 as ::core::ffi::c_int as isize) +=
			*x2.offset(0 as ::core::ffi::c_int as isize);
		*x1.offset(1 as ::core::ffi::c_int as isize) +=
			*x2.offset(1 as ::core::ffi::c_int as isize);
		*x2.offset(0 as ::core::ffi::c_int as isize) = r1
			* *T.offset(13 as ::core::ffi::c_int as isize)
			+ r0 * *T.offset(12 as ::core::ffi::c_int as isize);
		*x2.offset(1 as ::core::ffi::c_int as isize) = r1
			* *T.offset(12 as ::core::ffi::c_int as isize)
			- r0 * *T.offset(13 as ::core::ffi::c_int as isize);
		x1 = x1.offset(-(8 as ::core::ffi::c_int as isize));
		x2 = x2.offset(-(8 as ::core::ffi::c_int as isize));
		T = T.offset(16 as ::core::ffi::c_int as isize);
		if !(x2 >= x) {
			break;
		}
	}
}
#[inline]
unsafe extern "C" fn mdct_butterfly_generic(
	mut T: *mut ::core::ffi::c_float,
	mut x: *mut ::core::ffi::c_float,
	mut points: ::core::ffi::c_int,
	mut trigint: ::core::ffi::c_int
) {
	let mut x1: *mut ::core::ffi::c_float = x
		.offset(points as isize)
		.offset(-(8 as ::core::ffi::c_int as isize));
	let mut x2: *mut ::core::ffi::c_float = x
		.offset((points >> 1 as ::core::ffi::c_int) as isize)
		.offset(-(8 as ::core::ffi::c_int as isize));
	let mut r0: ::core::ffi::c_float = 0.;
	let mut r1: ::core::ffi::c_float = 0.;
	loop {
		r0 = *x1.offset(6 as ::core::ffi::c_int as isize)
			- *x2.offset(6 as ::core::ffi::c_int as isize);
		r1 = *x1.offset(7 as ::core::ffi::c_int as isize)
			- *x2.offset(7 as ::core::ffi::c_int as isize);
		*x1.offset(6 as ::core::ffi::c_int as isize) +=
			*x2.offset(6 as ::core::ffi::c_int as isize);
		*x1.offset(7 as ::core::ffi::c_int as isize) +=
			*x2.offset(7 as ::core::ffi::c_int as isize);
		*x2.offset(6 as ::core::ffi::c_int as isize) = r1
			* *T.offset(1 as ::core::ffi::c_int as isize)
			+ r0 * *T.offset(0 as ::core::ffi::c_int as isize);
		*x2.offset(7 as ::core::ffi::c_int as isize) = r1
			* *T.offset(0 as ::core::ffi::c_int as isize)
			- r0 * *T.offset(1 as ::core::ffi::c_int as isize);
		T = T.offset(trigint as isize);
		r0 = *x1.offset(4 as ::core::ffi::c_int as isize)
			- *x2.offset(4 as ::core::ffi::c_int as isize);
		r1 = *x1.offset(5 as ::core::ffi::c_int as isize)
			- *x2.offset(5 as ::core::ffi::c_int as isize);
		*x1.offset(4 as ::core::ffi::c_int as isize) +=
			*x2.offset(4 as ::core::ffi::c_int as isize);
		*x1.offset(5 as ::core::ffi::c_int as isize) +=
			*x2.offset(5 as ::core::ffi::c_int as isize);
		*x2.offset(4 as ::core::ffi::c_int as isize) = r1
			* *T.offset(1 as ::core::ffi::c_int as isize)
			+ r0 * *T.offset(0 as ::core::ffi::c_int as isize);
		*x2.offset(5 as ::core::ffi::c_int as isize) = r1
			* *T.offset(0 as ::core::ffi::c_int as isize)
			- r0 * *T.offset(1 as ::core::ffi::c_int as isize);
		T = T.offset(trigint as isize);
		r0 = *x1.offset(2 as ::core::ffi::c_int as isize)
			- *x2.offset(2 as ::core::ffi::c_int as isize);
		r1 = *x1.offset(3 as ::core::ffi::c_int as isize)
			- *x2.offset(3 as ::core::ffi::c_int as isize);
		*x1.offset(2 as ::core::ffi::c_int as isize) +=
			*x2.offset(2 as ::core::ffi::c_int as isize);
		*x1.offset(3 as ::core::ffi::c_int as isize) +=
			*x2.offset(3 as ::core::ffi::c_int as isize);
		*x2.offset(2 as ::core::ffi::c_int as isize) = r1
			* *T.offset(1 as ::core::ffi::c_int as isize)
			+ r0 * *T.offset(0 as ::core::ffi::c_int as isize);
		*x2.offset(3 as ::core::ffi::c_int as isize) = r1
			* *T.offset(0 as ::core::ffi::c_int as isize)
			- r0 * *T.offset(1 as ::core::ffi::c_int as isize);
		T = T.offset(trigint as isize);
		r0 = *x1.offset(0 as ::core::ffi::c_int as isize)
			- *x2.offset(0 as ::core::ffi::c_int as isize);
		r1 = *x1.offset(1 as ::core::ffi::c_int as isize)
			- *x2.offset(1 as ::core::ffi::c_int as isize);
		*x1.offset(0 as ::core::ffi::c_int as isize) +=
			*x2.offset(0 as ::core::ffi::c_int as isize);
		*x1.offset(1 as ::core::ffi::c_int as isize) +=
			*x2.offset(1 as ::core::ffi::c_int as isize);
		*x2.offset(0 as ::core::ffi::c_int as isize) = r1
			* *T.offset(1 as ::core::ffi::c_int as isize)
			+ r0 * *T.offset(0 as ::core::ffi::c_int as isize);
		*x2.offset(1 as ::core::ffi::c_int as isize) = r1
			* *T.offset(0 as ::core::ffi::c_int as isize)
			- r0 * *T.offset(1 as ::core::ffi::c_int as isize);
		T = T.offset(trigint as isize);
		x1 = x1.offset(-(8 as ::core::ffi::c_int as isize));
		x2 = x2.offset(-(8 as ::core::ffi::c_int as isize));
		if !(x2 >= x) {
			break;
		}
	}
}
#[inline]
unsafe extern "C" fn mdct_butterflies(
	mut init: *mut mdct_lookup,
	mut x: *mut ::core::ffi::c_float,
	mut points: ::core::ffi::c_int
) {
	let mut T: *mut ::core::ffi::c_float = (*init).trig;
	let mut stages: ::core::ffi::c_int = (*init).log2n - 5 as ::core::ffi::c_int;
	let mut i: ::core::ffi::c_int = 0;
	let mut j: ::core::ffi::c_int = 0;
	stages -= 1;
	if stages > 0 as ::core::ffi::c_int {
		mdct_butterfly_first(T, x, points);
	}
	i = 1 as ::core::ffi::c_int;
	loop {
		stages -= 1;
		if !(stages > 0 as ::core::ffi::c_int) {
			break;
		}
		j = 0 as ::core::ffi::c_int;
		while j < (1 as ::core::ffi::c_int) << i {
			mdct_butterfly_generic(
				T,
				x.offset(((points >> i) * j) as isize),
				points >> i,
				(4 as ::core::ffi::c_int) << i
			);
			j += 1;
		}
		i += 1;
	}
	j = 0 as ::core::ffi::c_int;
	while j < points {
		mdct_butterfly_32(x.offset(j as isize));
		j += 32 as ::core::ffi::c_int;
	}
}
#[no_mangle]
pub unsafe extern "C" fn mdct_clear(mut l: *mut mdct_lookup) {
	if !l.is_null() {
		if !(*l).trig.is_null() {
			free((*l).trig as *mut ::core::ffi::c_void);
		}
		if !(*l).bitrev.is_null() {
			free((*l).bitrev as *mut ::core::ffi::c_void);
		}
		memset(
			l as *mut ::core::ffi::c_void,
			0 as ::core::ffi::c_int,
			::core::mem::size_of::<mdct_lookup>() as size_t
		);
	}
}
#[inline]
unsafe extern "C" fn mdct_bitreverse(mut init: *mut mdct_lookup, mut x: *mut ::core::ffi::c_float) {
	let mut n: ::core::ffi::c_int = (*init).n;
	let mut bit: *mut ::core::ffi::c_int = (*init).bitrev;
	let mut w0: *mut ::core::ffi::c_float = x;
	x = w0.offset((n >> 1 as ::core::ffi::c_int) as isize);
	let mut w1: *mut ::core::ffi::c_float = x;
	let mut T: *mut ::core::ffi::c_float = (*init).trig.offset(n as isize);
	loop {
		let mut x0: *mut ::core::ffi::c_float =
			x.offset(*bit.offset(0 as ::core::ffi::c_int as isize) as isize);
		let mut x1: *mut ::core::ffi::c_float =
			x.offset(*bit.offset(1 as ::core::ffi::c_int as isize) as isize);
		let mut r0: ::core::ffi::c_float = *x0.offset(1 as ::core::ffi::c_int as isize)
			- *x1.offset(1 as ::core::ffi::c_int as isize);
		let mut r1: ::core::ffi::c_float = *x0.offset(0 as ::core::ffi::c_int as isize)
			+ *x1.offset(0 as ::core::ffi::c_int as isize);
		let mut r2: ::core::ffi::c_float = r1 * *T.offset(0 as ::core::ffi::c_int as isize)
			+ r0 * *T.offset(1 as ::core::ffi::c_int as isize);
		let mut r3: ::core::ffi::c_float = r1 * *T.offset(1 as ::core::ffi::c_int as isize)
			- r0 * *T.offset(0 as ::core::ffi::c_int as isize);
		w1 = w1.offset(-(4 as ::core::ffi::c_int as isize));
		r0 = (*x0.offset(1 as ::core::ffi::c_int as isize)
			+ *x1.offset(1 as ::core::ffi::c_int as isize))
			* 0.5f32;
		r1 = (*x0.offset(0 as ::core::ffi::c_int as isize)
			- *x1.offset(0 as ::core::ffi::c_int as isize))
			* 0.5f32;
		*w0.offset(0 as ::core::ffi::c_int as isize) = r0 + r2;
		*w1.offset(2 as ::core::ffi::c_int as isize) = r0 - r2;
		*w0.offset(1 as ::core::ffi::c_int as isize) = r1 + r3;
		*w1.offset(3 as ::core::ffi::c_int as isize) = r3 - r1;
		x0 = x.offset(*bit.offset(2 as ::core::ffi::c_int as isize) as isize);
		x1 = x.offset(*bit.offset(3 as ::core::ffi::c_int as isize) as isize);
		r0 = *x0.offset(1 as ::core::ffi::c_int as isize)
			- *x1.offset(1 as ::core::ffi::c_int as isize);
		r1 = *x0.offset(0 as ::core::ffi::c_int as isize)
			+ *x1.offset(0 as ::core::ffi::c_int as isize);
		r2 = r1 * *T.offset(2 as ::core::ffi::c_int as isize)
			+ r0 * *T.offset(3 as ::core::ffi::c_int as isize);
		r3 = r1 * *T.offset(3 as ::core::ffi::c_int as isize)
			- r0 * *T.offset(2 as ::core::ffi::c_int as isize);
		r0 = (*x0.offset(1 as ::core::ffi::c_int as isize)
			+ *x1.offset(1 as ::core::ffi::c_int as isize))
			* 0.5f32;
		r1 = (*x0.offset(0 as ::core::ffi::c_int as isize)
			- *x1.offset(0 as ::core::ffi::c_int as isize))
			* 0.5f32;
		*w0.offset(2 as ::core::ffi::c_int as isize) = r0 + r2;
		*w1.offset(0 as ::core::ffi::c_int as isize) = r0 - r2;
		*w0.offset(3 as ::core::ffi::c_int as isize) = r1 + r3;
		*w1.offset(1 as ::core::ffi::c_int as isize) = r3 - r1;
		T = T.offset(4 as ::core::ffi::c_int as isize);
		bit = bit.offset(4 as ::core::ffi::c_int as isize);
		w0 = w0.offset(4 as ::core::ffi::c_int as isize);
		if !(w0 < w1) {
			break;
		}
	}
}
#[no_mangle]
pub unsafe extern "C" fn mdct_backward(
	mut init: *mut mdct_lookup,
	mut in_0: *mut ::core::ffi::c_float,
	mut out: *mut ::core::ffi::c_float
) {
	let mut n: ::core::ffi::c_int = (*init).n;
	let mut n2: ::core::ffi::c_int = n >> 1 as ::core::ffi::c_int;
	let mut n4: ::core::ffi::c_int = n >> 2 as ::core::ffi::c_int;
	let mut iX: *mut ::core::ffi::c_float = in_0
		.offset(n2 as isize)
		.offset(-(7 as ::core::ffi::c_int as isize));
	let mut oX: *mut ::core::ffi::c_float = out.offset(n2 as isize).offset(n4 as isize);
	let mut T: *mut ::core::ffi::c_float = (*init).trig.offset(n4 as isize);
	loop {
		oX = oX.offset(-(4 as ::core::ffi::c_int as isize));
		*oX.offset(0 as ::core::ffi::c_int as isize) = -*iX
			.offset(2 as ::core::ffi::c_int as isize)
			* *T.offset(3 as ::core::ffi::c_int as isize)
			- *iX.offset(0 as ::core::ffi::c_int as isize)
				* *T.offset(2 as ::core::ffi::c_int as isize);
		*oX.offset(1 as ::core::ffi::c_int as isize) = *iX.offset(0 as ::core::ffi::c_int as isize)
			* *T.offset(3 as ::core::ffi::c_int as isize)
			- *iX.offset(2 as ::core::ffi::c_int as isize)
				* *T.offset(2 as ::core::ffi::c_int as isize);
		*oX.offset(2 as ::core::ffi::c_int as isize) = -*iX
			.offset(6 as ::core::ffi::c_int as isize)
			* *T.offset(1 as ::core::ffi::c_int as isize)
			- *iX.offset(4 as ::core::ffi::c_int as isize)
				* *T.offset(0 as ::core::ffi::c_int as isize);
		*oX.offset(3 as ::core::ffi::c_int as isize) = *iX.offset(4 as ::core::ffi::c_int as isize)
			* *T.offset(1 as ::core::ffi::c_int as isize)
			- *iX.offset(6 as ::core::ffi::c_int as isize)
				* *T.offset(0 as ::core::ffi::c_int as isize);
		iX = iX.offset(-(8 as ::core::ffi::c_int as isize));
		T = T.offset(4 as ::core::ffi::c_int as isize);
		if !(iX >= in_0) {
			break;
		}
	}
	iX = in_0
		.offset(n2 as isize)
		.offset(-(8 as ::core::ffi::c_int as isize));
	oX = out.offset(n2 as isize).offset(n4 as isize);
	T = (*init).trig.offset(n4 as isize);
	loop {
		T = T.offset(-(4 as ::core::ffi::c_int as isize));
		*oX.offset(0 as ::core::ffi::c_int as isize) = *iX.offset(4 as ::core::ffi::c_int as isize)
			* *T.offset(3 as ::core::ffi::c_int as isize)
			+ *iX.offset(6 as ::core::ffi::c_int as isize)
				* *T.offset(2 as ::core::ffi::c_int as isize);
		*oX.offset(1 as ::core::ffi::c_int as isize) = *iX.offset(4 as ::core::ffi::c_int as isize)
			* *T.offset(2 as ::core::ffi::c_int as isize)
			- *iX.offset(6 as ::core::ffi::c_int as isize)
				* *T.offset(3 as ::core::ffi::c_int as isize);
		*oX.offset(2 as ::core::ffi::c_int as isize) = *iX.offset(0 as ::core::ffi::c_int as isize)
			* *T.offset(1 as ::core::ffi::c_int as isize)
			+ *iX.offset(2 as ::core::ffi::c_int as isize)
				* *T.offset(0 as ::core::ffi::c_int as isize);
		*oX.offset(3 as ::core::ffi::c_int as isize) = *iX.offset(0 as ::core::ffi::c_int as isize)
			* *T.offset(0 as ::core::ffi::c_int as isize)
			- *iX.offset(2 as ::core::ffi::c_int as isize)
				* *T.offset(1 as ::core::ffi::c_int as isize);
		iX = iX.offset(-(8 as ::core::ffi::c_int as isize));
		oX = oX.offset(4 as ::core::ffi::c_int as isize);
		if !(iX >= in_0) {
			break;
		}
	}
	mdct_butterflies(init, out.offset(n2 as isize), n2);
	mdct_bitreverse(init, out);
	let mut oX1: *mut ::core::ffi::c_float = out.offset(n2 as isize).offset(n4 as isize);
	let mut oX2: *mut ::core::ffi::c_float = out.offset(n2 as isize).offset(n4 as isize);
	let mut iX_0: *mut ::core::ffi::c_float = out;
	T = (*init).trig.offset(n2 as isize);
	loop {
		oX1 = oX1.offset(-(4 as ::core::ffi::c_int as isize));
		*oX1.offset(3 as ::core::ffi::c_int as isize) = *iX_0
			.offset(0 as ::core::ffi::c_int as isize)
			* *T.offset(1 as ::core::ffi::c_int as isize)
			- *iX_0.offset(1 as ::core::ffi::c_int as isize)
				* *T.offset(0 as ::core::ffi::c_int as isize);
		*oX2.offset(0 as ::core::ffi::c_int as isize) = -(*iX_0
			.offset(0 as ::core::ffi::c_int as isize)
			* *T.offset(0 as ::core::ffi::c_int as isize)
			+ *iX_0.offset(1 as ::core::ffi::c_int as isize)
				* *T.offset(1 as ::core::ffi::c_int as isize));
		*oX1.offset(2 as ::core::ffi::c_int as isize) = *iX_0
			.offset(2 as ::core::ffi::c_int as isize)
			* *T.offset(3 as ::core::ffi::c_int as isize)
			- *iX_0.offset(3 as ::core::ffi::c_int as isize)
				* *T.offset(2 as ::core::ffi::c_int as isize);
		*oX2.offset(1 as ::core::ffi::c_int as isize) = -(*iX_0
			.offset(2 as ::core::ffi::c_int as isize)
			* *T.offset(2 as ::core::ffi::c_int as isize)
			+ *iX_0.offset(3 as ::core::ffi::c_int as isize)
				* *T.offset(3 as ::core::ffi::c_int as isize));
		*oX1.offset(1 as ::core::ffi::c_int as isize) = *iX_0
			.offset(4 as ::core::ffi::c_int as isize)
			* *T.offset(5 as ::core::ffi::c_int as isize)
			- *iX_0.offset(5 as ::core::ffi::c_int as isize)
				* *T.offset(4 as ::core::ffi::c_int as isize);
		*oX2.offset(2 as ::core::ffi::c_int as isize) = -(*iX_0
			.offset(4 as ::core::ffi::c_int as isize)
			* *T.offset(4 as ::core::ffi::c_int as isize)
			+ *iX_0.offset(5 as ::core::ffi::c_int as isize)
				* *T.offset(5 as ::core::ffi::c_int as isize));
		*oX1.offset(0 as ::core::ffi::c_int as isize) = *iX_0
			.offset(6 as ::core::ffi::c_int as isize)
			* *T.offset(7 as ::core::ffi::c_int as isize)
			- *iX_0.offset(7 as ::core::ffi::c_int as isize)
				* *T.offset(6 as ::core::ffi::c_int as isize);
		*oX2.offset(3 as ::core::ffi::c_int as isize) = -(*iX_0
			.offset(6 as ::core::ffi::c_int as isize)
			* *T.offset(6 as ::core::ffi::c_int as isize)
			+ *iX_0.offset(7 as ::core::ffi::c_int as isize)
				* *T.offset(7 as ::core::ffi::c_int as isize));
		oX2 = oX2.offset(4 as ::core::ffi::c_int as isize);
		iX_0 = iX_0.offset(8 as ::core::ffi::c_int as isize);
		T = T.offset(8 as ::core::ffi::c_int as isize);
		if !(iX_0 < oX1) {
			break;
		}
	}
	iX_0 = out.offset(n2 as isize).offset(n4 as isize);
	oX1 = out.offset(n4 as isize);
	oX2 = oX1;
	loop {
		oX1 = oX1.offset(-(4 as ::core::ffi::c_int as isize));
		iX_0 = iX_0.offset(-(4 as ::core::ffi::c_int as isize));
		let ref mut fresh0 = *oX1.offset(3 as ::core::ffi::c_int as isize);
		*fresh0 = *iX_0.offset(3 as ::core::ffi::c_int as isize);
		*oX2.offset(0 as ::core::ffi::c_int as isize) = -*fresh0;
		let ref mut fresh1 = *oX1.offset(2 as ::core::ffi::c_int as isize);
		*fresh1 = *iX_0.offset(2 as ::core::ffi::c_int as isize);
		*oX2.offset(1 as ::core::ffi::c_int as isize) = -*fresh1;
		let ref mut fresh2 = *oX1.offset(1 as ::core::ffi::c_int as isize);
		*fresh2 = *iX_0.offset(1 as ::core::ffi::c_int as isize);
		*oX2.offset(2 as ::core::ffi::c_int as isize) = -*fresh2;
		let ref mut fresh3 = *oX1.offset(0 as ::core::ffi::c_int as isize);
		*fresh3 = *iX_0.offset(0 as ::core::ffi::c_int as isize);
		*oX2.offset(3 as ::core::ffi::c_int as isize) = -*fresh3;
		oX2 = oX2.offset(4 as ::core::ffi::c_int as isize);
		if !(oX2 < iX_0) {
			break;
		}
	}
	iX_0 = out.offset(n2 as isize).offset(n4 as isize);
	oX1 = out.offset(n2 as isize).offset(n4 as isize);
	oX2 = out.offset(n2 as isize);
	loop {
		oX1 = oX1.offset(-(4 as ::core::ffi::c_int as isize));
		*oX1.offset(0 as ::core::ffi::c_int as isize) =
			*iX_0.offset(3 as ::core::ffi::c_int as isize);
		*oX1.offset(1 as ::core::ffi::c_int as isize) =
			*iX_0.offset(2 as ::core::ffi::c_int as isize);
		*oX1.offset(2 as ::core::ffi::c_int as isize) =
			*iX_0.offset(1 as ::core::ffi::c_int as isize);
		*oX1.offset(3 as ::core::ffi::c_int as isize) =
			*iX_0.offset(0 as ::core::ffi::c_int as isize);
		iX_0 = iX_0.offset(4 as ::core::ffi::c_int as isize);
		if !(oX1 > oX2) {
			break;
		}
	}
}
#[no_mangle]
pub unsafe extern "C" fn mdct_forward(
	mut init: *mut mdct_lookup,
	mut in_0: *mut ::core::ffi::c_float,
	mut out: *mut ::core::ffi::c_float
) {
	let mut alloca_allocations: Vec<Vec<u8>> = Vec::new();
	let mut n: ::core::ffi::c_int = (*init).n;
	let mut n2: ::core::ffi::c_int = n >> 1 as ::core::ffi::c_int;
	let mut n4: ::core::ffi::c_int = n >> 2 as ::core::ffi::c_int;
	let mut n8: ::core::ffi::c_int = n >> 3 as ::core::ffi::c_int;
	alloca_allocations.push(::std::vec::from_elem(
		0,
		(n as usize).wrapping_mul(::core::mem::size_of::<::core::ffi::c_float>() as usize) as usize
	));
	let mut w: *mut ::core::ffi::c_float =
		alloca_allocations.last_mut().unwrap().as_mut_ptr() as *mut ::core::ffi::c_float;
	let mut w2: *mut ::core::ffi::c_float = w.offset(n2 as isize);
	let mut r0: ::core::ffi::c_float = 0.;
	let mut r1: ::core::ffi::c_float = 0.;
	let mut x0: *mut ::core::ffi::c_float = in_0.offset(n2 as isize).offset(n4 as isize);
	let mut x1: *mut ::core::ffi::c_float = x0.offset(1 as ::core::ffi::c_int as isize);
	let mut T: *mut ::core::ffi::c_float = (*init).trig.offset(n2 as isize);
	let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
	i = 0 as ::core::ffi::c_int;
	while i < n8 {
		x0 = x0.offset(-(4 as ::core::ffi::c_int as isize));
		T = T.offset(-(2 as ::core::ffi::c_int as isize));
		r0 = *x0.offset(2 as ::core::ffi::c_int as isize)
			+ *x1.offset(0 as ::core::ffi::c_int as isize);
		r1 = *x0.offset(0 as ::core::ffi::c_int as isize)
			+ *x1.offset(2 as ::core::ffi::c_int as isize);
		*w2.offset(i as isize) = r1 * *T.offset(1 as ::core::ffi::c_int as isize)
			+ r0 * *T.offset(0 as ::core::ffi::c_int as isize);
		*w2.offset((i + 1 as ::core::ffi::c_int) as isize) = r1
			* *T.offset(0 as ::core::ffi::c_int as isize)
			- r0 * *T.offset(1 as ::core::ffi::c_int as isize);
		x1 = x1.offset(4 as ::core::ffi::c_int as isize);
		i += 2 as ::core::ffi::c_int;
	}
	x1 = in_0.offset(1 as ::core::ffi::c_int as isize);
	while i < n2 - n8 {
		T = T.offset(-(2 as ::core::ffi::c_int as isize));
		x0 = x0.offset(-(4 as ::core::ffi::c_int as isize));
		r0 = *x0.offset(2 as ::core::ffi::c_int as isize)
			- *x1.offset(0 as ::core::ffi::c_int as isize);
		r1 = *x0.offset(0 as ::core::ffi::c_int as isize)
			- *x1.offset(2 as ::core::ffi::c_int as isize);
		*w2.offset(i as isize) = r1 * *T.offset(1 as ::core::ffi::c_int as isize)
			+ r0 * *T.offset(0 as ::core::ffi::c_int as isize);
		*w2.offset((i + 1 as ::core::ffi::c_int) as isize) = r1
			* *T.offset(0 as ::core::ffi::c_int as isize)
			- r0 * *T.offset(1 as ::core::ffi::c_int as isize);
		x1 = x1.offset(4 as ::core::ffi::c_int as isize);
		i += 2 as ::core::ffi::c_int;
	}
	x0 = in_0.offset(n as isize);
	while i < n2 {
		T = T.offset(-(2 as ::core::ffi::c_int as isize));
		x0 = x0.offset(-(4 as ::core::ffi::c_int as isize));
		r0 = -*x0.offset(2 as ::core::ffi::c_int as isize)
			- *x1.offset(0 as ::core::ffi::c_int as isize);
		r1 = -*x0.offset(0 as ::core::ffi::c_int as isize)
			- *x1.offset(2 as ::core::ffi::c_int as isize);
		*w2.offset(i as isize) = r1 * *T.offset(1 as ::core::ffi::c_int as isize)
			+ r0 * *T.offset(0 as ::core::ffi::c_int as isize);
		*w2.offset((i + 1 as ::core::ffi::c_int) as isize) = r1
			* *T.offset(0 as ::core::ffi::c_int as isize)
			- r0 * *T.offset(1 as ::core::ffi::c_int as isize);
		x1 = x1.offset(4 as ::core::ffi::c_int as isize);
		i += 2 as ::core::ffi::c_int;
	}
	mdct_butterflies(init, w.offset(n2 as isize), n2);
	mdct_bitreverse(init, w);
	T = (*init).trig.offset(n2 as isize);
	x0 = out.offset(n2 as isize);
	i = 0 as ::core::ffi::c_int;
	while i < n4 {
		x0 = x0.offset(-1);
		*out.offset(i as isize) = (*w.offset(0 as ::core::ffi::c_int as isize)
			* *T.offset(0 as ::core::ffi::c_int as isize)
			+ *w.offset(1 as ::core::ffi::c_int as isize)
				* *T.offset(1 as ::core::ffi::c_int as isize))
			* (*init).scale;
		*x0.offset(0 as ::core::ffi::c_int as isize) = (*w
			.offset(0 as ::core::ffi::c_int as isize)
			* *T.offset(1 as ::core::ffi::c_int as isize)
			- *w.offset(1 as ::core::ffi::c_int as isize)
				* *T.offset(0 as ::core::ffi::c_int as isize))
			* (*init).scale;
		w = w.offset(2 as ::core::ffi::c_int as isize);
		T = T.offset(2 as ::core::ffi::c_int as isize);
		i += 1;
	}
}
