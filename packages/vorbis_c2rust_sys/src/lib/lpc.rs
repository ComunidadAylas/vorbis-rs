extern "C" {
	fn memset(
		__s: *mut ::core::ffi::c_void,
		__c: ::core::ffi::c_int,
		__n: size_t
	) -> *mut ::core::ffi::c_void;
}
pub type size_t = usize;
#[no_mangle]
pub unsafe extern "C" fn vorbis_lpc_from_data(
	mut data: *mut ::core::ffi::c_float,
	mut lpci: *mut ::core::ffi::c_float,
	mut n: ::core::ffi::c_int,
	mut m: ::core::ffi::c_int
) -> ::core::ffi::c_float {
	// Allocated as `Vec<c_double>` (rather than a byte buffer reinterpreted as `*mut c_double`)
	// so that the buffer is properly aligned for `c_double` access, as required by Miri
	let mut alloca_allocations: Vec<Vec<::core::ffi::c_double>> = Vec::new();
	alloca_allocations.push(::std::vec::from_elem(
		0.0,
		(m + 1 as ::core::ffi::c_int) as usize
	));
	let mut aut: *mut ::core::ffi::c_double = alloca_allocations.last_mut().unwrap().as_mut_ptr();
	alloca_allocations.push(::std::vec::from_elem(0.0, m as usize));
	let mut lpc: *mut ::core::ffi::c_double = alloca_allocations.last_mut().unwrap().as_mut_ptr();
	let mut error: ::core::ffi::c_double = 0.;
	let mut epsilon: ::core::ffi::c_double = 0.;
	let mut i: ::core::ffi::c_int = 0;
	let mut j: ::core::ffi::c_int = 0;
	j = m + 1 as ::core::ffi::c_int;
	loop {
		let fresh0 = j;
		j = j - 1;
		if !(fresh0 != 0) {
			break;
		}
		let mut d: ::core::ffi::c_double = 0 as ::core::ffi::c_int as ::core::ffi::c_double;
		i = j;
		while i < n {
			d += *data.offset(i as isize) as ::core::ffi::c_double
				* *data.offset((i - j) as isize) as ::core::ffi::c_double;
			i += 1;
		}
		*aut.offset(j as isize) = d;
	}
	error = *aut.offset(0 as ::core::ffi::c_int as isize) * (1.0f64 + 1e-10f64);
	epsilon = 1e-9f64 * *aut.offset(0 as ::core::ffi::c_int as isize) + 1e-10f64;
	i = 0 as ::core::ffi::c_int;
	while i < m {
		let mut r: ::core::ffi::c_double = -*aut.offset((i + 1 as ::core::ffi::c_int) as isize);
		if error < epsilon {
			memset(
				lpc.offset(i as isize) as *mut ::core::ffi::c_void,
				0 as ::core::ffi::c_int,
				((m - i) as size_t)
					.wrapping_mul(::core::mem::size_of::<::core::ffi::c_double>() as size_t)
			);
			break;
		} else {
			j = 0 as ::core::ffi::c_int;
			while j < i {
				r -= *lpc.offset(j as isize) * *aut.offset((i - j) as isize);
				j += 1;
			}
			r /= error;
			*lpc.offset(i as isize) = r;
			j = 0 as ::core::ffi::c_int;
			while j < i / 2 as ::core::ffi::c_int {
				let mut tmp: ::core::ffi::c_double = *lpc.offset(j as isize);
				*lpc.offset(j as isize) +=
					r * *lpc.offset((i - 1 as ::core::ffi::c_int - j) as isize);
				*lpc.offset((i - 1 as ::core::ffi::c_int - j) as isize) += r * tmp;
				j += 1;
			}
			if i & 1 as ::core::ffi::c_int != 0 {
				*lpc.offset(j as isize) += *lpc.offset(j as isize) * r;
			}
			error *= 1.0f64 - r * r;
			i += 1;
		}
	}
	let mut g: ::core::ffi::c_double = 0.99f64;
	let mut damp: ::core::ffi::c_double = g;
	j = 0 as ::core::ffi::c_int;
	while j < m {
		*lpc.offset(j as isize) *= damp;
		damp *= g;
		j += 1;
	}
	j = 0 as ::core::ffi::c_int;
	while j < m {
		*lpci.offset(j as isize) = *lpc.offset(j as isize) as ::core::ffi::c_float;
		j += 1;
	}
	return error as ::core::ffi::c_float;
}
#[no_mangle]
pub unsafe extern "C" fn vorbis_lpc_predict(
	mut coeff: *mut ::core::ffi::c_float,
	mut prime: *mut ::core::ffi::c_float,
	mut m: ::core::ffi::c_int,
	mut data: *mut ::core::ffi::c_float,
	mut n: ::core::ffi::c_long
) {
	// Allocated as `Vec<c_float>` (rather than a byte buffer reinterpreted as `*mut c_float`) so
	// that the buffer is properly aligned for `c_float` access, as required by Miri
	let mut alloca_allocations: Vec<Vec<::core::ffi::c_float>> = Vec::new();
	let mut i: ::core::ffi::c_long = 0;
	let mut j: ::core::ffi::c_long = 0;
	let mut o: ::core::ffi::c_long = 0;
	let mut p: ::core::ffi::c_long = 0;
	let mut y: ::core::ffi::c_float = 0.;
	alloca_allocations.push(::std::vec::from_elem(
		0.0,
		(m as ::core::ffi::c_long + n) as usize
	));
	let mut work: *mut ::core::ffi::c_float = alloca_allocations.last_mut().unwrap().as_mut_ptr();
	if prime.is_null() {
		i = 0 as ::core::ffi::c_long;
		while i < m as ::core::ffi::c_long {
			*work.offset(i as isize) = 0.0f32;
			i += 1;
		}
	} else {
		i = 0 as ::core::ffi::c_long;
		while i < m as ::core::ffi::c_long {
			*work.offset(i as isize) = *prime.offset(i as isize);
			i += 1;
		}
	}
	i = 0 as ::core::ffi::c_long;
	while i < n {
		y = 0 as ::core::ffi::c_int as ::core::ffi::c_float;
		o = i;
		p = m as ::core::ffi::c_long;
		j = 0 as ::core::ffi::c_long;
		while j < m as ::core::ffi::c_long {
			let fresh1 = o;
			o = o + 1;
			p -= 1;
			y -= *work.offset(fresh1 as isize) * *coeff.offset(p as isize);
			j += 1;
		}
		let ref mut fresh2 = *work.offset(o as isize);
		*fresh2 = y;
		*data.offset(i as isize) = *fresh2;
		i += 1;
	}
}
