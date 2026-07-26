extern "C" {
	fn malloc(__size: size_t) -> *mut ::core::ffi::c_void;
	fn memcpy(
		__dest: *mut ::core::ffi::c_void,
		__src: *const ::core::ffi::c_void,
		__n: size_t
	) -> *mut ::core::ffi::c_void;
	fn oggpack_write(b: *mut oggpack_buffer, value: ::core::ffi::c_ulong, bits: ::core::ffi::c_int);
	fn oggpack_look(b: *mut oggpack_buffer, bits: ::core::ffi::c_int) -> ::core::ffi::c_long;
	fn oggpack_adv(b: *mut oggpack_buffer, bits: ::core::ffi::c_int);
	fn oggpack_read(b: *mut oggpack_buffer, bits: ::core::ffi::c_int) -> ::core::ffi::c_long;
	fn oggpack_bytes(b: *mut oggpack_buffer) -> ::core::ffi::c_long;
	fn _book_maptype1_quantvals(
		dim: ::core::ffi::c_long,
		entries: ::core::ffi::c_long
	) -> ::core::ffi::c_long;
	fn ov_ilog(v: ogg_uint32_t) -> ::core::ffi::c_int;
}
use ogg_c2rust_sys::oggpack_buffer;
pub type size_t = usize;
pub type __uint16_t = u16;
pub type __int32_t = i32;
pub type __uint32_t = u32;
pub type int32_t = __int32_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
pub type ogg_uint16_t = uint16_t;
pub type ogg_int32_t = int32_t;
pub type ogg_uint32_t = uint32_t;
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
pub unsafe extern "C" fn vorbis_staticbook_pack(
	mut c: *const static_codebook,
	mut opb: *mut oggpack_buffer
) -> ::core::ffi::c_int {
	let mut i: ::core::ffi::c_long = 0;
	let mut j: ::core::ffi::c_long = 0;
	let mut ordered: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
	oggpack_write(
		opb,
		0x564342 as ::core::ffi::c_ulong,
		24 as ::core::ffi::c_int
	);
	oggpack_write(
		opb,
		(*c).dim as ::core::ffi::c_ulong,
		16 as ::core::ffi::c_int
	);
	oggpack_write(
		opb,
		(*c).entries as ::core::ffi::c_ulong,
		24 as ::core::ffi::c_int
	);
	i = 1 as ::core::ffi::c_long;
	while i < (*c).entries {
		if *(*c)
			.lengthlist
			.offset((i - 1 as ::core::ffi::c_long) as isize) as ::core::ffi::c_int
			== 0 as ::core::ffi::c_int
			|| (*(*c).lengthlist.offset(i as isize) as ::core::ffi::c_int)
				< *(*c)
					.lengthlist
					.offset((i - 1 as ::core::ffi::c_long) as isize) as ::core::ffi::c_int
		{
			break;
		}
		i += 1;
	}
	if i == (*c).entries {
		ordered = 1 as ::core::ffi::c_int;
	}
	if ordered != 0 {
		let mut count: ::core::ffi::c_long = 0 as ::core::ffi::c_long;
		oggpack_write(opb, 1 as ::core::ffi::c_ulong, 1 as ::core::ffi::c_int);
		oggpack_write(
			opb,
			(*(*c).lengthlist.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
				- 1 as ::core::ffi::c_int) as ::core::ffi::c_ulong,
			5 as ::core::ffi::c_int
		);
		i = 1 as ::core::ffi::c_long;
		while i < (*c).entries {
			let mut this: ::core::ffi::c_char = *(*c).lengthlist.offset(i as isize);
			let mut last: ::core::ffi::c_char = *(*c)
				.lengthlist
				.offset((i - 1 as ::core::ffi::c_long) as isize);
			if this as ::core::ffi::c_int > last as ::core::ffi::c_int {
				j = last as ::core::ffi::c_long;
				while j < this as ::core::ffi::c_long {
					oggpack_write(
						opb,
						(i - count) as ::core::ffi::c_ulong,
						ov_ilog(((*c).entries - count) as ogg_uint32_t)
					);
					count = i;
					j += 1;
				}
			}
			i += 1;
		}
		oggpack_write(
			opb,
			(i - count) as ::core::ffi::c_ulong,
			ov_ilog(((*c).entries - count) as ogg_uint32_t)
		);
	} else {
		oggpack_write(opb, 0 as ::core::ffi::c_ulong, 1 as ::core::ffi::c_int);
		i = 0 as ::core::ffi::c_long;
		while i < (*c).entries {
			if *(*c).lengthlist.offset(i as isize) as ::core::ffi::c_int == 0 as ::core::ffi::c_int
			{
				break;
			}
			i += 1;
		}
		if i == (*c).entries {
			oggpack_write(opb, 0 as ::core::ffi::c_ulong, 1 as ::core::ffi::c_int);
			i = 0 as ::core::ffi::c_long;
			while i < (*c).entries {
				oggpack_write(
					opb,
					(*(*c).lengthlist.offset(i as isize) as ::core::ffi::c_int
						- 1 as ::core::ffi::c_int) as ::core::ffi::c_ulong,
					5 as ::core::ffi::c_int
				);
				i += 1;
			}
		} else {
			oggpack_write(opb, 1 as ::core::ffi::c_ulong, 1 as ::core::ffi::c_int);
			i = 0 as ::core::ffi::c_long;
			while i < (*c).entries {
				if *(*c).lengthlist.offset(i as isize) as ::core::ffi::c_int
					== 0 as ::core::ffi::c_int
				{
					oggpack_write(opb, 0 as ::core::ffi::c_ulong, 1 as ::core::ffi::c_int);
				} else {
					oggpack_write(opb, 1 as ::core::ffi::c_ulong, 1 as ::core::ffi::c_int);
					oggpack_write(
						opb,
						(*(*c).lengthlist.offset(i as isize) as ::core::ffi::c_int
							- 1 as ::core::ffi::c_int) as ::core::ffi::c_ulong,
						5 as ::core::ffi::c_int
					);
				}
				i += 1;
			}
		}
	}
	oggpack_write(
		opb,
		(*c).maptype as ::core::ffi::c_ulong,
		4 as ::core::ffi::c_int
	);
	match (*c).maptype {
		0 => {}
		1 | 2 => {
			if (*c).quantlist.is_null() {
				return -(1 as ::core::ffi::c_int);
			}
			oggpack_write(
				opb,
				(*c).q_min as ::core::ffi::c_ulong,
				32 as ::core::ffi::c_int
			);
			oggpack_write(
				opb,
				(*c).q_delta as ::core::ffi::c_ulong,
				32 as ::core::ffi::c_int
			);
			oggpack_write(
				opb,
				((*c).q_quant - 1 as ::core::ffi::c_int) as ::core::ffi::c_ulong,
				4 as ::core::ffi::c_int
			);
			oggpack_write(
				opb,
				(*c).q_sequencep as ::core::ffi::c_ulong,
				1 as ::core::ffi::c_int
			);
			let mut quantvals: ::core::ffi::c_int = 0;
			match (*c).maptype {
				1 => {
					quantvals =
						_book_maptype1_quantvals((*c).dim, (*c).entries) as ::core::ffi::c_int;
				}
				2 => {
					quantvals = ((*c).entries * (*c).dim) as ::core::ffi::c_int;
				}
				_ => {
					quantvals = -(1 as ::core::ffi::c_int);
				}
			}
			i = 0 as ::core::ffi::c_long;
			while i < quantvals as ::core::ffi::c_long {
				oggpack_write(
					opb,
					(*(*c).quantlist.offset(i as isize)).abs() as ::core::ffi::c_ulong,
					(*c).q_quant
				);
				i += 1;
			}
		}
		_ => return -(1 as ::core::ffi::c_int)
	}
	return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn vorbis_decbook_unpack(
	mut c: *mut dec_codebook,
	mut opb: *mut oggpack_buffer
) -> ::core::ffi::c_int {
	let mut current_block: u64;
	let mut i: ::core::ffi::c_long = 0;
	if !(oggpack_read(opb, 24 as ::core::ffi::c_int) != 0x564342 as ::core::ffi::c_long) {
		(*c).dim = oggpack_read(opb, 16 as ::core::ffi::c_int) as ::core::ffi::c_schar;
		(*c).entries = oggpack_read(opb, 24 as ::core::ffi::c_int) as ogg_int32_t;
		if !((*c).entries == -(1 as ogg_int32_t)) {
			if !(ov_ilog((*c).dim as ogg_uint32_t) + ov_ilog((*c).entries as ogg_uint32_t)
				> 24 as ::core::ffi::c_int)
			{
				match oggpack_read(opb, 1 as ::core::ffi::c_int) as ::core::ffi::c_int {
					0 => {
						current_block = 7095457783677275021;
						match current_block {
							7095457783677275021 => {
								let mut unused: ::core::ffi::c_long = 0;
								unused = oggpack_read(opb, 1 as ::core::ffi::c_int);
								if ((*c).entries
									* (if unused != 0 {
										1 as ogg_int32_t
									} else {
										5 as ogg_int32_t
									}) + 7 as ogg_int32_t >> 3 as ::core::ffi::c_int)
									as ::core::ffi::c_long > (*opb).storage - oggpack_bytes(opb)
								{
									current_block = 17445631040686328021;
								} else {
									(*c).codelengths = malloc(
										(::core::mem::size_of::<::core::ffi::c_schar>() as size_t)
											.wrapping_mul((*c).entries as size_t)
									) as *mut ::core::ffi::c_schar;
									if (*c).codelengths.is_null() {
										current_block = 17445631040686328021;
									} else if unused != 0 {
										i = 0 as ::core::ffi::c_long;
										loop {
											if !(i < (*c).entries as ::core::ffi::c_long) {
												current_block = 10891380440665537214;
												break;
											}
											if oggpack_read(opb, 1 as ::core::ffi::c_int) != 0 {
												let mut num: ::core::ffi::c_long =
													oggpack_read(opb, 5 as ::core::ffi::c_int);
												if num
													== -(1 as ::core::ffi::c_int)
														as ::core::ffi::c_long
												{
													current_block = 17445631040686328021;
													break;
												}
												*(*c).codelengths.offset(i as isize) = (num
													+ 1 as ::core::ffi::c_long)
													as ::core::ffi::c_schar;
											} else {
												*(*c).codelengths.offset(i as isize) =
													0 as ::core::ffi::c_schar;
											}
											i += 1;
										}
									} else {
										i = 0 as ::core::ffi::c_long;
										loop {
											if !(i < (*c).entries as ::core::ffi::c_long) {
												current_block = 10891380440665537214;
												break;
											}
											let mut num_0: ::core::ffi::c_long =
												oggpack_read(opb, 5 as ::core::ffi::c_int);
											if num_0
												== -(1 as ::core::ffi::c_int) as ::core::ffi::c_long
											{
												current_block = 17445631040686328021;
												break;
											}
											*(*c).codelengths.offset(i as isize) = (num_0
												+ 1 as ::core::ffi::c_long)
												as ::core::ffi::c_schar;
											i += 1;
										}
									}
								}
							}
							_ => {
								let mut cum_entries: [ogg_int32_t; 32] = [0; 32];
								let mut minlength: ::core::ffi::c_long =
									oggpack_read(opb, 5 as ::core::ffi::c_int)
										+ 1 as ::core::ffi::c_long;
								let mut maxlength: ::core::ffi::c_long = 0;
								let mut length: ::core::ffi::c_long = 0;
								if minlength == 0 as ::core::ffi::c_long {
									current_block = 17445631040686328021;
								} else {
									length = minlength;
									maxlength = length;
									i = 0 as ::core::ffi::c_long;
									loop {
										if !(i < (*c).entries as ::core::ffi::c_long) {
											current_block = 7245201122033322888;
											break;
										}
										let mut num_1: ::core::ffi::c_long = oggpack_read(
											opb,
											ov_ilog(
												((*c).entries as ::core::ffi::c_long - i)
													as ogg_uint32_t
											)
										);
										if num_1
											== -(1 as ::core::ffi::c_int) as ::core::ffi::c_long
										{
											current_block = 17445631040686328021;
											break;
										}
										if length > 32 as ::core::ffi::c_long
											|| num_1 > (*c).entries as ::core::ffi::c_long - i
											|| num_1 > 0 as ::core::ffi::c_long
												&& num_1 - 1 as ::core::ffi::c_long
													>> length - 1 as ::core::ffi::c_long > 1
													as ::core::ffi::c_long
										{
											current_block = 17445631040686328021;
											break;
										}
										i += num_1;
										cum_entries[(length - minlength) as usize] =
											i as ogg_int32_t;
										if i == 0 as ::core::ffi::c_long {
											minlength += 1;
										}
										maxlength = length;
										length += 1;
									}
									match current_block {
										17445631040686328021 => {}
										_ => {
											(*c).minlength = minlength as ::core::ffi::c_schar;
											(*c).maxlength = maxlength as ::core::ffi::c_schar;
											(*c).index = malloc(
												((maxlength - minlength + 1 as ::core::ffi::c_long)
													as size_t)
													.wrapping_mul(
														::core::mem::size_of::<ogg_int32_t>()
															as size_t
													)
											) as *mut ogg_int32_t;
											if (*c).index.is_null() {
												current_block = 17445631040686328021;
											} else {
												memcpy(
													(*c).index as *mut ::core::ffi::c_void,
													&raw mut cum_entries as *mut ogg_int32_t
														as *const ::core::ffi::c_void,
													((maxlength - minlength
														+ 1 as ::core::ffi::c_long)
														as size_t)
														.wrapping_mul(::core::mem::size_of::<
															ogg_int32_t
														>() as size_t)
												);
												current_block = 10891380440665537214;
											}
										}
									}
								}
							}
						}
						match current_block {
							17445631040686328021 => {}
							_ => {
								(*c).maptype = oggpack_read(opb, 4 as ::core::ffi::c_int)
									as ::core::ffi::c_schar;
								match (*c).maptype as ::core::ffi::c_int {
									0 => {
										current_block = 7252614138838059896;
										match current_block {
											10730382391303010067 => {
												(*c).q_min =
													oggpack_read(opb, 32 as ::core::ffi::c_int)
														as ogg_uint32_t;
												(*c).q_delta =
													oggpack_read(opb, 32 as ::core::ffi::c_int)
														as ogg_uint32_t;
												(*c).q_quant =
													(oggpack_read(opb, 4 as ::core::ffi::c_int)
														as ::core::ffi::c_schar
														as ::core::ffi::c_int + 1
														as ::core::ffi::c_int)
														as ::core::ffi::c_schar;
												(*c).q_sequencep =
													oggpack_read(opb, 1 as ::core::ffi::c_int)
														as ::core::ffi::c_schar;
												if (*c).q_sequencep as ::core::ffi::c_int
													== -(1 as ::core::ffi::c_int)
												{
													current_block = 17445631040686328021;
												} else {
													let mut q: ::core::ffi::c_long =
														0 as ::core::ffi::c_long;
													let mut quantvals: ::core::ffi::c_int =
														0 as ::core::ffi::c_int;
													match (*c).maptype as ::core::ffi::c_int {
														1 => {
															quantvals = (if (*c).dim
																as ::core::ffi::c_int
																== 0 as ::core::ffi::c_int
															{
																0 as ::core::ffi::c_long
															} else {
																_book_maptype1_quantvals(
																	(*c).dim as ::core::ffi::c_long,
																	(*c).entries
																		as ::core::ffi::c_long
																)
															})
																as ::core::ffi::c_int;
														}
														2 => {
															quantvals = ((*c).entries
																* (*c).dim as ogg_int32_t)
																as ::core::ffi::c_int;
														}
														_ => {}
													}
													if (quantvals
														* (*c).q_quant as ::core::ffi::c_int + 7
														as ::core::ffi::c_int >> 3
														as ::core::ffi::c_int)
														as ::core::ffi::c_long > (*opb).storage
														- oggpack_bytes(opb)
													{
														current_block = 17445631040686328021;
													} else {
														(*c).quantlist = malloc(
															(::core::mem::size_of::<ogg_uint16_t>()
																as size_t)
																.wrapping_mul(quantvals as size_t)
														)
															as *mut ogg_uint16_t;
														if (*c).quantlist.is_null() {
															current_block = 17445631040686328021;
														} else {
															i = 0 as ::core::ffi::c_long;
															while i < quantvals
																as ::core::ffi::c_long
															{
																q = oggpack_read(
																	opb,
																	(*c).q_quant
																		as ::core::ffi::c_int
																);
																*(*c)
																	.quantlist
																	.offset(i as isize) = q as ogg_uint16_t;
																i += 1;
															}
															if q == -(1 as ::core::ffi::c_int)
																as ::core::ffi::c_long
															{
																current_block =
																	17445631040686328021;
															} else {
																current_block = 7252614138838059896;
															}
														}
													}
												}
											}
											_ => {}
										}
										match current_block {
											17445631040686328021 => {}
											_ => return 0 as ::core::ffi::c_int
										}
									}
									1 | 2 => {
										current_block = 10730382391303010067;
										match current_block {
											10730382391303010067 => {
												(*c).q_min =
													oggpack_read(opb, 32 as ::core::ffi::c_int)
														as ogg_uint32_t;
												(*c).q_delta =
													oggpack_read(opb, 32 as ::core::ffi::c_int)
														as ogg_uint32_t;
												(*c).q_quant =
													(oggpack_read(opb, 4 as ::core::ffi::c_int)
														as ::core::ffi::c_schar
														as ::core::ffi::c_int + 1
														as ::core::ffi::c_int)
														as ::core::ffi::c_schar;
												(*c).q_sequencep =
													oggpack_read(opb, 1 as ::core::ffi::c_int)
														as ::core::ffi::c_schar;
												if (*c).q_sequencep as ::core::ffi::c_int
													== -(1 as ::core::ffi::c_int)
												{
													current_block = 17445631040686328021;
												} else {
													let mut q: ::core::ffi::c_long =
														0 as ::core::ffi::c_long;
													let mut quantvals: ::core::ffi::c_int =
														0 as ::core::ffi::c_int;
													match (*c).maptype as ::core::ffi::c_int {
														1 => {
															quantvals = (if (*c).dim
																as ::core::ffi::c_int
																== 0 as ::core::ffi::c_int
															{
																0 as ::core::ffi::c_long
															} else {
																_book_maptype1_quantvals(
																	(*c).dim as ::core::ffi::c_long,
																	(*c).entries
																		as ::core::ffi::c_long
																)
															})
																as ::core::ffi::c_int;
														}
														2 => {
															quantvals = ((*c).entries
																* (*c).dim as ogg_int32_t)
																as ::core::ffi::c_int;
														}
														_ => {}
													}
													if (quantvals
														* (*c).q_quant as ::core::ffi::c_int + 7
														as ::core::ffi::c_int >> 3
														as ::core::ffi::c_int)
														as ::core::ffi::c_long > (*opb).storage
														- oggpack_bytes(opb)
													{
														current_block = 17445631040686328021;
													} else {
														(*c).quantlist = malloc(
															(::core::mem::size_of::<ogg_uint16_t>()
																as size_t)
																.wrapping_mul(quantvals as size_t)
														)
															as *mut ogg_uint16_t;
														if (*c).quantlist.is_null() {
															current_block = 17445631040686328021;
														} else {
															i = 0 as ::core::ffi::c_long;
															while i < quantvals
																as ::core::ffi::c_long
															{
																q = oggpack_read(
																	opb,
																	(*c).q_quant
																		as ::core::ffi::c_int
																);
																*(*c)
																	.quantlist
																	.offset(i as isize) = q as ogg_uint16_t;
																i += 1;
															}
															if q == -(1 as ::core::ffi::c_int)
																as ::core::ffi::c_long
															{
																current_block =
																	17445631040686328021;
															} else {
																current_block = 7252614138838059896;
															}
														}
													}
												}
											}
											_ => {}
										}
										match current_block {
											17445631040686328021 => {}
											_ => return 0 as ::core::ffi::c_int
										}
									}
									_ => {}
								}
							}
						}
					}
					1 => {
						current_block = 11307063007268554308;
						match current_block {
							7095457783677275021 => {
								let mut unused: ::core::ffi::c_long = 0;
								unused = oggpack_read(opb, 1 as ::core::ffi::c_int);
								if ((*c).entries
									* (if unused != 0 {
										1 as ogg_int32_t
									} else {
										5 as ogg_int32_t
									}) + 7 as ogg_int32_t >> 3 as ::core::ffi::c_int)
									as ::core::ffi::c_long > (*opb).storage - oggpack_bytes(opb)
								{
									current_block = 17445631040686328021;
								} else {
									(*c).codelengths = malloc(
										(::core::mem::size_of::<::core::ffi::c_schar>() as size_t)
											.wrapping_mul((*c).entries as size_t)
									) as *mut ::core::ffi::c_schar;
									if (*c).codelengths.is_null() {
										current_block = 17445631040686328021;
									} else if unused != 0 {
										i = 0 as ::core::ffi::c_long;
										loop {
											if !(i < (*c).entries as ::core::ffi::c_long) {
												current_block = 10891380440665537214;
												break;
											}
											if oggpack_read(opb, 1 as ::core::ffi::c_int) != 0 {
												let mut num: ::core::ffi::c_long =
													oggpack_read(opb, 5 as ::core::ffi::c_int);
												if num
													== -(1 as ::core::ffi::c_int)
														as ::core::ffi::c_long
												{
													current_block = 17445631040686328021;
													break;
												}
												*(*c).codelengths.offset(i as isize) = (num
													+ 1 as ::core::ffi::c_long)
													as ::core::ffi::c_schar;
											} else {
												*(*c).codelengths.offset(i as isize) =
													0 as ::core::ffi::c_schar;
											}
											i += 1;
										}
									} else {
										i = 0 as ::core::ffi::c_long;
										loop {
											if !(i < (*c).entries as ::core::ffi::c_long) {
												current_block = 10891380440665537214;
												break;
											}
											let mut num_0: ::core::ffi::c_long =
												oggpack_read(opb, 5 as ::core::ffi::c_int);
											if num_0
												== -(1 as ::core::ffi::c_int) as ::core::ffi::c_long
											{
												current_block = 17445631040686328021;
												break;
											}
											*(*c).codelengths.offset(i as isize) = (num_0
												+ 1 as ::core::ffi::c_long)
												as ::core::ffi::c_schar;
											i += 1;
										}
									}
								}
							}
							_ => {
								let mut cum_entries: [ogg_int32_t; 32] = [0; 32];
								let mut minlength: ::core::ffi::c_long =
									oggpack_read(opb, 5 as ::core::ffi::c_int)
										+ 1 as ::core::ffi::c_long;
								let mut maxlength: ::core::ffi::c_long = 0;
								let mut length: ::core::ffi::c_long = 0;
								if minlength == 0 as ::core::ffi::c_long {
									current_block = 17445631040686328021;
								} else {
									length = minlength;
									maxlength = length;
									i = 0 as ::core::ffi::c_long;
									loop {
										if !(i < (*c).entries as ::core::ffi::c_long) {
											current_block = 7245201122033322888;
											break;
										}
										let mut num_1: ::core::ffi::c_long = oggpack_read(
											opb,
											ov_ilog(
												((*c).entries as ::core::ffi::c_long - i)
													as ogg_uint32_t
											)
										);
										if num_1
											== -(1 as ::core::ffi::c_int) as ::core::ffi::c_long
										{
											current_block = 17445631040686328021;
											break;
										}
										if length > 32 as ::core::ffi::c_long
											|| num_1 > (*c).entries as ::core::ffi::c_long - i
											|| num_1 > 0 as ::core::ffi::c_long
												&& num_1 - 1 as ::core::ffi::c_long
													>> length - 1 as ::core::ffi::c_long > 1
													as ::core::ffi::c_long
										{
											current_block = 17445631040686328021;
											break;
										}
										i += num_1;
										cum_entries[(length - minlength) as usize] =
											i as ogg_int32_t;
										if i == 0 as ::core::ffi::c_long {
											minlength += 1;
										}
										maxlength = length;
										length += 1;
									}
									match current_block {
										17445631040686328021 => {}
										_ => {
											(*c).minlength = minlength as ::core::ffi::c_schar;
											(*c).maxlength = maxlength as ::core::ffi::c_schar;
											(*c).index = malloc(
												((maxlength - minlength + 1 as ::core::ffi::c_long)
													as size_t)
													.wrapping_mul(
														::core::mem::size_of::<ogg_int32_t>()
															as size_t
													)
											) as *mut ogg_int32_t;
											if (*c).index.is_null() {
												current_block = 17445631040686328021;
											} else {
												memcpy(
													(*c).index as *mut ::core::ffi::c_void,
													&raw mut cum_entries as *mut ogg_int32_t
														as *const ::core::ffi::c_void,
													((maxlength - minlength
														+ 1 as ::core::ffi::c_long)
														as size_t)
														.wrapping_mul(::core::mem::size_of::<
															ogg_int32_t
														>() as size_t)
												);
												current_block = 10891380440665537214;
											}
										}
									}
								}
							}
						}
						match current_block {
							17445631040686328021 => {}
							_ => {
								(*c).maptype = oggpack_read(opb, 4 as ::core::ffi::c_int)
									as ::core::ffi::c_schar;
								match (*c).maptype as ::core::ffi::c_int {
									0 => {
										current_block = 7252614138838059896;
										match current_block {
											10730382391303010067 => {
												(*c).q_min =
													oggpack_read(opb, 32 as ::core::ffi::c_int)
														as ogg_uint32_t;
												(*c).q_delta =
													oggpack_read(opb, 32 as ::core::ffi::c_int)
														as ogg_uint32_t;
												(*c).q_quant =
													(oggpack_read(opb, 4 as ::core::ffi::c_int)
														as ::core::ffi::c_schar
														as ::core::ffi::c_int + 1
														as ::core::ffi::c_int)
														as ::core::ffi::c_schar;
												(*c).q_sequencep =
													oggpack_read(opb, 1 as ::core::ffi::c_int)
														as ::core::ffi::c_schar;
												if (*c).q_sequencep as ::core::ffi::c_int
													== -(1 as ::core::ffi::c_int)
												{
													current_block = 17445631040686328021;
												} else {
													let mut q: ::core::ffi::c_long =
														0 as ::core::ffi::c_long;
													let mut quantvals: ::core::ffi::c_int =
														0 as ::core::ffi::c_int;
													match (*c).maptype as ::core::ffi::c_int {
														1 => {
															quantvals = (if (*c).dim
																as ::core::ffi::c_int
																== 0 as ::core::ffi::c_int
															{
																0 as ::core::ffi::c_long
															} else {
																_book_maptype1_quantvals(
																	(*c).dim as ::core::ffi::c_long,
																	(*c).entries
																		as ::core::ffi::c_long
																)
															})
																as ::core::ffi::c_int;
														}
														2 => {
															quantvals = ((*c).entries
																* (*c).dim as ogg_int32_t)
																as ::core::ffi::c_int;
														}
														_ => {}
													}
													if (quantvals
														* (*c).q_quant as ::core::ffi::c_int + 7
														as ::core::ffi::c_int >> 3
														as ::core::ffi::c_int)
														as ::core::ffi::c_long > (*opb).storage
														- oggpack_bytes(opb)
													{
														current_block = 17445631040686328021;
													} else {
														(*c).quantlist = malloc(
															(::core::mem::size_of::<ogg_uint16_t>()
																as size_t)
																.wrapping_mul(quantvals as size_t)
														)
															as *mut ogg_uint16_t;
														if (*c).quantlist.is_null() {
															current_block = 17445631040686328021;
														} else {
															i = 0 as ::core::ffi::c_long;
															while i < quantvals
																as ::core::ffi::c_long
															{
																q = oggpack_read(
																	opb,
																	(*c).q_quant
																		as ::core::ffi::c_int
																);
																*(*c)
																	.quantlist
																	.offset(i as isize) = q as ogg_uint16_t;
																i += 1;
															}
															if q == -(1 as ::core::ffi::c_int)
																as ::core::ffi::c_long
															{
																current_block =
																	17445631040686328021;
															} else {
																current_block = 7252614138838059896;
															}
														}
													}
												}
											}
											_ => {}
										}
										match current_block {
											17445631040686328021 => {}
											_ => return 0 as ::core::ffi::c_int
										}
									}
									1 | 2 => {
										current_block = 10730382391303010067;
										match current_block {
											10730382391303010067 => {
												(*c).q_min =
													oggpack_read(opb, 32 as ::core::ffi::c_int)
														as ogg_uint32_t;
												(*c).q_delta =
													oggpack_read(opb, 32 as ::core::ffi::c_int)
														as ogg_uint32_t;
												(*c).q_quant =
													(oggpack_read(opb, 4 as ::core::ffi::c_int)
														as ::core::ffi::c_schar
														as ::core::ffi::c_int + 1
														as ::core::ffi::c_int)
														as ::core::ffi::c_schar;
												(*c).q_sequencep =
													oggpack_read(opb, 1 as ::core::ffi::c_int)
														as ::core::ffi::c_schar;
												if (*c).q_sequencep as ::core::ffi::c_int
													== -(1 as ::core::ffi::c_int)
												{
													current_block = 17445631040686328021;
												} else {
													let mut q: ::core::ffi::c_long =
														0 as ::core::ffi::c_long;
													let mut quantvals: ::core::ffi::c_int =
														0 as ::core::ffi::c_int;
													match (*c).maptype as ::core::ffi::c_int {
														1 => {
															quantvals = (if (*c).dim
																as ::core::ffi::c_int
																== 0 as ::core::ffi::c_int
															{
																0 as ::core::ffi::c_long
															} else {
																_book_maptype1_quantvals(
																	(*c).dim as ::core::ffi::c_long,
																	(*c).entries
																		as ::core::ffi::c_long
																)
															})
																as ::core::ffi::c_int;
														}
														2 => {
															quantvals = ((*c).entries
																* (*c).dim as ogg_int32_t)
																as ::core::ffi::c_int;
														}
														_ => {}
													}
													if (quantvals
														* (*c).q_quant as ::core::ffi::c_int + 7
														as ::core::ffi::c_int >> 3
														as ::core::ffi::c_int)
														as ::core::ffi::c_long > (*opb).storage
														- oggpack_bytes(opb)
													{
														current_block = 17445631040686328021;
													} else {
														(*c).quantlist = malloc(
															(::core::mem::size_of::<ogg_uint16_t>()
																as size_t)
																.wrapping_mul(quantvals as size_t)
														)
															as *mut ogg_uint16_t;
														if (*c).quantlist.is_null() {
															current_block = 17445631040686328021;
														} else {
															i = 0 as ::core::ffi::c_long;
															while i < quantvals
																as ::core::ffi::c_long
															{
																q = oggpack_read(
																	opb,
																	(*c).q_quant
																		as ::core::ffi::c_int
																);
																*(*c)
																	.quantlist
																	.offset(i as isize) = q as ogg_uint16_t;
																i += 1;
															}
															if q == -(1 as ::core::ffi::c_int)
																as ::core::ffi::c_long
															{
																current_block =
																	17445631040686328021;
															} else {
																current_block = 7252614138838059896;
															}
														}
													}
												}
											}
											_ => {}
										}
										match current_block {
											17445631040686328021 => {}
											_ => return 0 as ::core::ffi::c_int
										}
									}
									_ => {}
								}
							}
						}
					}
					_ => {}
				}
			}
		}
	}
	return -(1 as ::core::ffi::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn vorbis_book_encode(
	mut book: *mut codebook,
	mut a: ::core::ffi::c_int,
	mut b: *mut oggpack_buffer
) -> ::core::ffi::c_int {
	if a < 0 as ::core::ffi::c_int || a as ::core::ffi::c_long >= (*(*book).c).entries {
		return 0 as ::core::ffi::c_int;
	}
	oggpack_write(
		b,
		*(*book).codelist.offset(a as isize) as ::core::ffi::c_ulong,
		*(*(*book).c).lengthlist.offset(a as isize) as ::core::ffi::c_int
	);
	return *(*(*book).c).lengthlist.offset(a as isize) as ::core::ffi::c_int;
}
unsafe extern "C" fn bitreverse(mut x: ogg_uint32_t) -> ogg_uint32_t {
	x = x >> 16 as ::core::ffi::c_int & 0xffff as ogg_uint32_t
		| x << 16 as ::core::ffi::c_int & 0xffff0000 as ogg_uint32_t;
	x = x >> 8 as ::core::ffi::c_int & 0xff00ff as ogg_uint32_t
		| x << 8 as ::core::ffi::c_int & 0xff00ff00 as ogg_uint32_t;
	x = x >> 4 as ::core::ffi::c_int & 0xf0f0f0f as ogg_uint32_t
		| x << 4 as ::core::ffi::c_int & 0xf0f0f0f0 as ogg_uint32_t;
	x = x >> 2 as ::core::ffi::c_int & 0x33333333 as ogg_uint32_t
		| x << 2 as ::core::ffi::c_int & 0xcccccccc as ogg_uint32_t;
	return x >> 1 as ::core::ffi::c_int & 0x55555555 as ogg_uint32_t
		| x << 1 as ::core::ffi::c_int & 0xaaaaaaaa as ogg_uint32_t;
}
#[inline]
unsafe extern "C" fn decode_packed_entry_number(
	mut book: *mut dec_codebook,
	mut b: *mut oggpack_buffer
) -> ::core::ffi::c_long {
	let mut testword: ogg_uint32_t = 0;
	let mut read: ::core::ffi::c_int = (*book).maxlength as ::core::ffi::c_int;
	let mut lo: ::core::ffi::c_long = 0;
	let mut hi: ::core::ffi::c_long = 0;
	let mut lok: ::core::ffi::c_long = oggpack_look(b, (*book).firsttablen as ::core::ffi::c_int);
	if lok >= 0 as ::core::ffi::c_long {
		let mut entry: ::core::ffi::c_long =
			*(*book).firsttable.offset(lok as isize) as ::core::ffi::c_long;
		if entry as ::core::ffi::c_ulong & 0x80000000 as ::core::ffi::c_ulong != 0 {
			lo = (entry >> 15 as ::core::ffi::c_int & 0x7fff as ::core::ffi::c_long)
				<< (*book).hint_shift as ::core::ffi::c_int;
			hi = (*book).hi_max as ::core::ffi::c_long
				- ((entry & 0x7fff as ::core::ffi::c_long)
					<< (*book).hint_shift as ::core::ffi::c_int);
		} else {
			oggpack_adv(
				b,
				(entry & 0x3f as ::core::ffi::c_long) as ::core::ffi::c_int
			);
			return entry >> 6 as ::core::ffi::c_int;
		}
	} else {
		lo = 0 as ::core::ffi::c_long;
		hi = (*book).hi_max as ::core::ffi::c_long;
	}
	lok = oggpack_look(b, read);
	while lok < 0 as ::core::ffi::c_long && read > 1 as ::core::ffi::c_int {
		read -= 1;
		lok = oggpack_look(b, read);
	}
	if lok < 0 as ::core::ffi::c_long {
		return -(1 as ::core::ffi::c_int) as ::core::ffi::c_long;
	}
	testword = bitreverse(lok as ogg_uint32_t);
	if (*book).codelengths.is_null() {
		let mut length: ::core::ffi::c_int = 0;
		while testword > *(*book).codelist.offset(lo as isize) {
			lo += 1;
		}
		length = lo as ::core::ffi::c_int + (*book).minlength as ::core::ffi::c_int;
		if length <= read {
			let mut entry_0: ::core::ffi::c_long =
				(*(*book).index.offset(lo as isize) as ogg_uint32_t).wrapping_sub(
					((*(*book).codelist.offset(lo as isize)).wrapping_sub(testword)
						>> 32 as ::core::ffi::c_int - length)
						.wrapping_add(1 as ogg_uint32_t)
				) as ::core::ffi::c_long;
			oggpack_adv(b, length);
			return entry_0;
		}
	} else {
		while hi - lo > 1 as ::core::ffi::c_long {
			let mut p: ::core::ffi::c_long = hi - lo >> 1 as ::core::ffi::c_int;
			let mut test: ::core::ffi::c_long = (*(*book).codelist.offset((lo + p) as isize)
				> testword) as ::core::ffi::c_int
				as ::core::ffi::c_long;
			lo += p & test - 1 as ::core::ffi::c_long;
			hi -= p & -test;
		}
		if *(*book).codelengths.offset(lo as isize) as ::core::ffi::c_int <= read {
			oggpack_adv(
				b,
				*(*book).codelengths.offset(lo as isize) as ::core::ffi::c_int
			);
			return lo;
		}
	}
	oggpack_adv(b, read);
	return -(1 as ::core::ffi::c_int) as ::core::ffi::c_long;
}
#[no_mangle]
pub unsafe extern "C" fn vorbis_book_decode(
	mut book: *mut dec_codebook,
	mut b: *mut oggpack_buffer
) -> ::core::ffi::c_long {
	if !(*book).codelist.is_null() {
		let mut packed_entry: ::core::ffi::c_long = decode_packed_entry_number(book, b);
		if packed_entry >= 0 as ::core::ffi::c_long {
			if !(*book).codelengths.is_null() {
				return *(*book).index.offset(packed_entry as isize) as ::core::ffi::c_long;
			}
			return packed_entry;
		}
	}
	return -(1 as ::core::ffi::c_int) as ::core::ffi::c_long;
}
#[no_mangle]
pub unsafe extern "C" fn vorbis_book_decodevs_add(
	mut book: *mut dec_codebook,
	mut a: *mut ::core::ffi::c_float,
	mut b: *mut oggpack_buffer,
	mut n: ::core::ffi::c_int
) -> ::core::ffi::c_long {
	let mut alloca_allocations: Vec<Vec<u8>> = Vec::new();
	if !(*book).codelist.is_null() {
		let mut step: ::core::ffi::c_int = n / (*book).dim as ::core::ffi::c_int;
		alloca_allocations.push(::std::vec::from_elem(
			0,
			(::core::mem::size_of::<::core::ffi::c_long>() as usize).wrapping_mul(step as usize)
				as usize
		));
		let mut entry: *mut ::core::ffi::c_long =
			alloca_allocations.last_mut().unwrap().as_mut_ptr() as *mut ::core::ffi::c_long;
		alloca_allocations.push(::std::vec::from_elem(
			0,
			(::core::mem::size_of::<*mut ::core::ffi::c_float>() as usize)
				.wrapping_mul(step as usize) as usize
		));
		let mut t: *mut *mut ::core::ffi::c_float =
			alloca_allocations.last_mut().unwrap().as_mut_ptr() as *mut *mut ::core::ffi::c_float;
		let mut i: ::core::ffi::c_int = 0;
		let mut j: ::core::ffi::c_int = 0;
		let mut o: ::core::ffi::c_int = 0;
		i = 0 as ::core::ffi::c_int;
		while i < step {
			*entry.offset(i as isize) = decode_packed_entry_number(book, b);
			if *entry.offset(i as isize) == -(1 as ::core::ffi::c_int) as ::core::ffi::c_long {
				return -(1 as ::core::ffi::c_int) as ::core::ffi::c_long;
			}
			let ref mut fresh0 = *t.offset(i as isize);
			*fresh0 = (*book)
				.valuelist
				.offset((*entry.offset(i as isize) * (*book).dim as ::core::ffi::c_long) as isize);
			i += 1;
		}
		i = 0 as ::core::ffi::c_int;
		o = 0 as ::core::ffi::c_int;
		while i < (*book).dim as ::core::ffi::c_int {
			j = 0 as ::core::ffi::c_int;
			while o + j < n && j < step {
				*a.offset((o + j) as isize) += *(*t.offset(j as isize)).offset(i as isize);
				j += 1;
			}
			i += 1;
			o += step;
		}
	}
	return 0 as ::core::ffi::c_long;
}
#[no_mangle]
pub unsafe extern "C" fn vorbis_book_decodev_add(
	mut book: *mut dec_codebook,
	mut a: *mut ::core::ffi::c_float,
	mut b: *mut oggpack_buffer,
	mut n: ::core::ffi::c_int
) -> ::core::ffi::c_long {
	if !(*book).codelist.is_null() {
		let mut i: ::core::ffi::c_int = 0;
		let mut j: ::core::ffi::c_int = 0;
		let mut entry: ::core::ffi::c_int = 0;
		let mut t: *mut ::core::ffi::c_float = ::core::ptr::null_mut::<::core::ffi::c_float>();
		i = 0 as ::core::ffi::c_int;
		while i < n {
			entry = decode_packed_entry_number(book, b) as ::core::ffi::c_int;
			if entry == -(1 as ::core::ffi::c_int) {
				return -(1 as ::core::ffi::c_int) as ::core::ffi::c_long;
			}
			t = (*book)
				.valuelist
				.offset((entry * (*book).dim as ::core::ffi::c_int) as isize);
			j = 0 as ::core::ffi::c_int;
			while i < n && j < (*book).dim as ::core::ffi::c_int {
				let fresh4 = j;
				j = j + 1;
				let fresh5 = i;
				i = i + 1;
				*a.offset(fresh5 as isize) += *t.offset(fresh4 as isize);
			}
		}
	}
	return 0 as ::core::ffi::c_long;
}
#[no_mangle]
pub unsafe extern "C" fn vorbis_book_decodev_set(
	mut book: *mut dec_codebook,
	mut a: *mut ::core::ffi::c_float,
	mut b: *mut oggpack_buffer,
	mut n: ::core::ffi::c_int
) -> ::core::ffi::c_long {
	if !(*book).codelist.is_null() {
		let mut i: ::core::ffi::c_int = 0;
		let mut j: ::core::ffi::c_int = 0;
		let mut entry: ::core::ffi::c_int = 0;
		let mut t: *mut ::core::ffi::c_float = ::core::ptr::null_mut::<::core::ffi::c_float>();
		i = 0 as ::core::ffi::c_int;
		while i < n {
			entry = decode_packed_entry_number(book, b) as ::core::ffi::c_int;
			if entry == -(1 as ::core::ffi::c_int) {
				return -(1 as ::core::ffi::c_int) as ::core::ffi::c_long;
			}
			t = (*book)
				.valuelist
				.offset((entry * (*book).dim as ::core::ffi::c_int) as isize);
			j = 0 as ::core::ffi::c_int;
			while i < n && j < (*book).dim as ::core::ffi::c_int {
				let fresh1 = j;
				j = j + 1;
				let fresh2 = i;
				i = i + 1;
				*a.offset(fresh2 as isize) = *t.offset(fresh1 as isize);
			}
		}
	} else {
		let mut i_0: ::core::ffi::c_int = 0;
		i_0 = 0 as ::core::ffi::c_int;
		while i_0 < n {
			let fresh3 = i_0;
			i_0 = i_0 + 1;
			*a.offset(fresh3 as isize) = 0.0f32;
		}
	}
	return 0 as ::core::ffi::c_long;
}
#[no_mangle]
pub unsafe extern "C" fn vorbis_book_decodevv_add(
	mut book: *mut dec_codebook,
	mut a: *mut *mut ::core::ffi::c_float,
	mut offset: ::core::ffi::c_long,
	mut ch: ::core::ffi::c_int,
	mut b: *mut oggpack_buffer,
	mut n: ::core::ffi::c_int
) -> ::core::ffi::c_long {
	let mut i: ::core::ffi::c_long = 0;
	let mut j: ::core::ffi::c_long = 0;
	let mut entry: ::core::ffi::c_long = 0;
	let mut chptr: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
	if !(*book).codelist.is_null() {
		let mut m: ::core::ffi::c_int =
			((offset + n as ::core::ffi::c_long) / ch as ::core::ffi::c_long) as ::core::ffi::c_int;
		i = offset / ch as ::core::ffi::c_long;
		while i < m as ::core::ffi::c_long {
			entry = decode_packed_entry_number(book, b);
			if entry == -(1 as ::core::ffi::c_int) as ::core::ffi::c_long {
				return -(1 as ::core::ffi::c_int) as ::core::ffi::c_long;
			}
			let mut t: *const ::core::ffi::c_float = (*book)
				.valuelist
				.offset((entry * (*book).dim as ::core::ffi::c_long) as isize);
			j = 0 as ::core::ffi::c_long;
			while i < m as ::core::ffi::c_long && j < (*book).dim as ::core::ffi::c_long {
				let fresh6 = chptr;
				chptr = chptr + 1;
				*(*a.offset(fresh6 as isize)).offset(i as isize) += *t.offset(j as isize);
				if chptr == ch {
					chptr = 0 as ::core::ffi::c_int;
					i += 1;
				}
				j += 1;
			}
		}
	}
	return 0 as ::core::ffi::c_long;
}
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
