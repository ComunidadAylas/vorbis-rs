extern "C" {
	fn acos(__x: ::core::ffi::c_double) -> ::core::ffi::c_double;
	fn cos(__x: ::core::ffi::c_double) -> ::core::ffi::c_double;
	fn exp(__x: ::core::ffi::c_double) -> ::core::ffi::c_double;
	fn sqrt(__x: ::core::ffi::c_double) -> ::core::ffi::c_double;
	fn fabs(__x: ::core::ffi::c_double) -> ::core::ffi::c_double;
	fn qsort(
		__base: *mut ::core::ffi::c_void,
		__nmemb: size_t,
		__size: size_t,
		__compar: __compar_fn_t
	);
}
pub type size_t = usize;
pub type __compar_fn_t = Option<
	unsafe extern "C" fn(
		*const ::core::ffi::c_void,
		*const ::core::ffi::c_void
	) -> ::core::ffi::c_int
>;
pub const M_PI: ::core::ffi::c_double = 3.14159265358979323846f64;
#[no_mangle]
pub unsafe extern "C" fn vorbis_lsp_to_curve(
	mut curve: *mut ::core::ffi::c_float,
	mut map: *mut ::core::ffi::c_int,
	mut n: ::core::ffi::c_int,
	mut ln: ::core::ffi::c_int,
	mut lsp: *mut ::core::ffi::c_float,
	mut m: ::core::ffi::c_int,
	mut amp: ::core::ffi::c_float,
	mut ampoffset: ::core::ffi::c_float
) {
	let mut i: ::core::ffi::c_int = 0;
	let mut wdel: ::core::ffi::c_float =
		(M_PI / ln as ::core::ffi::c_double) as ::core::ffi::c_float;
	i = 0 as ::core::ffi::c_int;
	while i < m {
		*lsp.offset(i as isize) = (2.0f64 * cos(*lsp.offset(i as isize) as ::core::ffi::c_double))
			as ::core::ffi::c_float;
		i += 1;
	}
	i = 0 as ::core::ffi::c_int;
	while i < n {
		let mut j: ::core::ffi::c_int = 0;
		let mut k: ::core::ffi::c_int = *map.offset(i as isize);
		let mut p: ::core::ffi::c_float = 0.5f32;
		let mut q: ::core::ffi::c_float = 0.5f32;
		let mut w: ::core::ffi::c_float = (2.0f64
			* cos((wdel * k as ::core::ffi::c_float) as ::core::ffi::c_double))
			as ::core::ffi::c_float;
		j = 1 as ::core::ffi::c_int;
		while j < m {
			q *= w - *lsp.offset((j - 1 as ::core::ffi::c_int) as isize);
			p *= w - *lsp.offset(j as isize);
			j += 2 as ::core::ffi::c_int;
		}
		if j == m {
			q *= w - *lsp.offset((j - 1 as ::core::ffi::c_int) as isize);
			p *= p * (4.0f32 - w * w);
			q *= q;
		} else {
			p *= p * (2.0f32 - w);
			q *= q * (2.0f32 + w);
		}
		q = exp(
			(amp as ::core::ffi::c_double / sqrt((p + q) as ::core::ffi::c_double)
				- ampoffset as ::core::ffi::c_double)
				* 0.11512925f64
		) as ::core::ffi::c_float;
		*curve.offset(i as isize) *= q;
		loop {
			i += 1;
			if !(*map.offset(i as isize) == k) {
				break;
			}
			*curve.offset(i as isize) *= q;
		}
	}
}
unsafe extern "C" fn cheby(mut g: *mut ::core::ffi::c_float, mut ord: ::core::ffi::c_int) {
	let mut i: ::core::ffi::c_int = 0;
	let mut j: ::core::ffi::c_int = 0;
	*g.offset(0 as ::core::ffi::c_int as isize) *= 0.5f32;
	i = 2 as ::core::ffi::c_int;
	while i <= ord {
		j = ord;
		while j >= i {
			*g.offset((j - 2 as ::core::ffi::c_int) as isize) -= *g.offset(j as isize);
			*g.offset(j as isize) += *g.offset(j as isize);
			j -= 1;
		}
		i += 1;
	}
}
unsafe extern "C" fn comp(
	mut a: *const ::core::ffi::c_void,
	mut b: *const ::core::ffi::c_void
) -> ::core::ffi::c_int {
	return (*(a as *mut ::core::ffi::c_float) < *(b as *mut ::core::ffi::c_float))
		as ::core::ffi::c_int
		- (*(a as *mut ::core::ffi::c_float) > *(b as *mut ::core::ffi::c_float))
			as ::core::ffi::c_int;
}
pub const EPSILON: ::core::ffi::c_double = 10e-7f64;
unsafe extern "C" fn Laguerre_With_Deflation(
	mut a: *mut ::core::ffi::c_float,
	mut ord: ::core::ffi::c_int,
	mut r: *mut ::core::ffi::c_float
) -> ::core::ffi::c_int {
	let mut alloca_allocations: Vec<Vec<u8>> = Vec::new();
	let mut i: ::core::ffi::c_int = 0;
	let mut m: ::core::ffi::c_int = 0;
	alloca_allocations.push(::std::vec::from_elem(
		0,
		(::core::mem::size_of::<::core::ffi::c_double>() as usize)
			.wrapping_mul((ord + 1 as ::core::ffi::c_int) as usize) as usize
	));
	let mut defl: *mut ::core::ffi::c_double =
		alloca_allocations.last_mut().unwrap().as_mut_ptr() as *mut ::core::ffi::c_double;
	i = 0 as ::core::ffi::c_int;
	while i <= ord {
		*defl.offset(i as isize) = *a.offset(i as isize) as ::core::ffi::c_double;
		i += 1;
	}
	m = ord;
	while m > 0 as ::core::ffi::c_int {
		let mut new: ::core::ffi::c_double = 0.0f64;
		let mut delta: ::core::ffi::c_double = 0.;
		loop {
			let mut p: ::core::ffi::c_double = *defl.offset(m as isize);
			let mut pp: ::core::ffi::c_double = 0.0f64;
			let mut ppp: ::core::ffi::c_double = 0.0f64;
			let mut denom: ::core::ffi::c_double = 0.;
			i = m;
			while i > 0 as ::core::ffi::c_int {
				ppp = new * ppp + pp;
				pp = new * pp + p;
				p = new * p + *defl.offset((i - 1 as ::core::ffi::c_int) as isize);
				i -= 1;
			}
			denom = (m - 1 as ::core::ffi::c_int) as ::core::ffi::c_double
				* ((m - 1 as ::core::ffi::c_int) as ::core::ffi::c_double * pp * pp
					- m as ::core::ffi::c_double * p * ppp);
			if denom < 0 as ::core::ffi::c_int as ::core::ffi::c_double {
				return -(1 as ::core::ffi::c_int);
			}
			if pp > 0 as ::core::ffi::c_int as ::core::ffi::c_double {
				denom = pp + sqrt(denom);
				if denom < EPSILON {
					denom = EPSILON;
				}
			} else {
				denom = pp - sqrt(denom);
				if denom > -10e-7f64 {
					denom = -10e-7f64;
				}
			}
			delta = m as ::core::ffi::c_double * p / denom;
			new -= delta;
			if delta < 0.0f64 {
				delta *= -(1 as ::core::ffi::c_int) as ::core::ffi::c_double;
			}
			if fabs(delta / new) < 10e-12f64 {
				break;
			}
		}
		*r.offset((m - 1 as ::core::ffi::c_int) as isize) = new as ::core::ffi::c_float;
		i = m;
		while i > 0 as ::core::ffi::c_int {
			*defl.offset((i - 1 as ::core::ffi::c_int) as isize) += new * *defl.offset(i as isize);
			i -= 1;
		}
		defl = defl.offset(1);
		m -= 1;
	}
	return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn Newton_Raphson(
	mut a: *mut ::core::ffi::c_float,
	mut ord: ::core::ffi::c_int,
	mut r: *mut ::core::ffi::c_float
) -> ::core::ffi::c_int {
	let mut alloca_allocations: Vec<Vec<u8>> = Vec::new();
	let mut i: ::core::ffi::c_int = 0;
	let mut k: ::core::ffi::c_int = 0;
	let mut count: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
	let mut error: ::core::ffi::c_double = 1.0f64;
	alloca_allocations.push(::std::vec::from_elem(
		0,
		(ord as usize).wrapping_mul(::core::mem::size_of::<::core::ffi::c_double>() as usize)
			as usize
	));
	let mut root: *mut ::core::ffi::c_double =
		alloca_allocations.last_mut().unwrap().as_mut_ptr() as *mut ::core::ffi::c_double;
	i = 0 as ::core::ffi::c_int;
	while i < ord {
		*root.offset(i as isize) = *r.offset(i as isize) as ::core::ffi::c_double;
		i += 1;
	}
	while error > 1e-20f64 {
		error = 0 as ::core::ffi::c_int as ::core::ffi::c_double;
		i = 0 as ::core::ffi::c_int;
		while i < ord {
			let mut pp: ::core::ffi::c_double = 0.0f64;
			let mut delta: ::core::ffi::c_double = 0.;
			let mut rooti: ::core::ffi::c_double = *root.offset(i as isize);
			let mut p: ::core::ffi::c_double = *a.offset(ord as isize) as ::core::ffi::c_double;
			k = ord - 1 as ::core::ffi::c_int;
			while k >= 0 as ::core::ffi::c_int {
				pp = pp * rooti + p;
				p = p * rooti + *a.offset(k as isize) as ::core::ffi::c_double;
				k -= 1;
			}
			delta = p / pp;
			*root.offset(i as isize) -= delta;
			error += delta * delta;
			i += 1;
		}
		if count > 40 as ::core::ffi::c_int {
			return -(1 as ::core::ffi::c_int);
		}
		count += 1;
	}
	i = 0 as ::core::ffi::c_int;
	while i < ord {
		*r.offset(i as isize) = *root.offset(i as isize) as ::core::ffi::c_float;
		i += 1;
	}
	return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn vorbis_lpc_to_lsp(
	mut lpc: *mut ::core::ffi::c_float,
	mut lsp: *mut ::core::ffi::c_float,
	mut m: ::core::ffi::c_int
) -> ::core::ffi::c_int {
	let mut alloca_allocations: Vec<Vec<u8>> = Vec::new();
	let mut order2: ::core::ffi::c_int = m + 1 as ::core::ffi::c_int >> 1 as ::core::ffi::c_int;
	let mut g1_order: ::core::ffi::c_int = 0;
	let mut g2_order: ::core::ffi::c_int = 0;
	alloca_allocations.push(::std::vec::from_elem(
		0,
		(::core::mem::size_of::<::core::ffi::c_float>() as usize)
			.wrapping_mul((order2 + 1 as ::core::ffi::c_int) as usize) as usize
	));
	let mut g1: *mut ::core::ffi::c_float =
		alloca_allocations.last_mut().unwrap().as_mut_ptr() as *mut ::core::ffi::c_float;
	alloca_allocations.push(::std::vec::from_elem(
		0,
		(::core::mem::size_of::<::core::ffi::c_float>() as usize)
			.wrapping_mul((order2 + 1 as ::core::ffi::c_int) as usize) as usize
	));
	let mut g2: *mut ::core::ffi::c_float =
		alloca_allocations.last_mut().unwrap().as_mut_ptr() as *mut ::core::ffi::c_float;
	alloca_allocations.push(::std::vec::from_elem(
		0,
		(::core::mem::size_of::<::core::ffi::c_float>() as usize)
			.wrapping_mul((order2 + 1 as ::core::ffi::c_int) as usize) as usize
	));
	let mut g1r: *mut ::core::ffi::c_float =
		alloca_allocations.last_mut().unwrap().as_mut_ptr() as *mut ::core::ffi::c_float;
	alloca_allocations.push(::std::vec::from_elem(
		0,
		(::core::mem::size_of::<::core::ffi::c_float>() as usize)
			.wrapping_mul((order2 + 1 as ::core::ffi::c_int) as usize) as usize
	));
	let mut g2r: *mut ::core::ffi::c_float =
		alloca_allocations.last_mut().unwrap().as_mut_ptr() as *mut ::core::ffi::c_float;
	let mut i: ::core::ffi::c_int = 0;
	g1_order = m + 1 as ::core::ffi::c_int >> 1 as ::core::ffi::c_int;
	g2_order = m >> 1 as ::core::ffi::c_int;
	*g1.offset(g1_order as isize) = 1.0f32;
	i = 1 as ::core::ffi::c_int;
	while i <= g1_order {
		*g1.offset((g1_order - i) as isize) =
			*lpc.offset((i - 1 as ::core::ffi::c_int) as isize) + *lpc.offset((m - i) as isize);
		i += 1;
	}
	*g2.offset(g2_order as isize) = 1.0f32;
	i = 1 as ::core::ffi::c_int;
	while i <= g2_order {
		*g2.offset((g2_order - i) as isize) =
			*lpc.offset((i - 1 as ::core::ffi::c_int) as isize) - *lpc.offset((m - i) as isize);
		i += 1;
	}
	if g1_order > g2_order {
		i = 2 as ::core::ffi::c_int;
		while i <= g2_order {
			*g2.offset((g2_order - i) as isize) +=
				*g2.offset((g2_order - i + 2 as ::core::ffi::c_int) as isize);
			i += 1;
		}
	} else {
		i = 1 as ::core::ffi::c_int;
		while i <= g1_order {
			*g1.offset((g1_order - i) as isize) -=
				*g1.offset((g1_order - i + 1 as ::core::ffi::c_int) as isize);
			i += 1;
		}
		i = 1 as ::core::ffi::c_int;
		while i <= g2_order {
			*g2.offset((g2_order - i) as isize) +=
				*g2.offset((g2_order - i + 1 as ::core::ffi::c_int) as isize);
			i += 1;
		}
	}
	cheby(g1, g1_order);
	cheby(g2, g2_order);
	if Laguerre_With_Deflation(g1, g1_order, g1r) != 0
		|| Laguerre_With_Deflation(g2, g2_order, g2r) != 0
	{
		return -(1 as ::core::ffi::c_int);
	}
	Newton_Raphson(g1, g1_order, g1r);
	Newton_Raphson(g2, g2_order, g2r);
	::core::slice::from_raw_parts_mut(g1r, g1_order as usize)
		.sort_by(|a, b| a.partial_cmp(b).unwrap_or(::core::cmp::Ordering::Equal));
	::core::slice::from_raw_parts_mut(g2r, g2_order as usize)
		.sort_by(|a, b| a.partial_cmp(b).unwrap_or(::core::cmp::Ordering::Equal));
	i = 0 as ::core::ffi::c_int;
	while i < g1_order {
		*lsp.offset((i * 2 as ::core::ffi::c_int) as isize) =
			acos(*g1r.offset(i as isize) as ::core::ffi::c_double) as ::core::ffi::c_float;
		i += 1;
	}
	i = 0 as ::core::ffi::c_int;
	while i < g2_order {
		*lsp.offset((i * 2 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize) =
			acos(*g2r.offset(i as isize) as ::core::ffi::c_double) as ::core::ffi::c_float;
		i += 1;
	}
	return 0 as ::core::ffi::c_int;
}
