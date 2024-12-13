use std::env::current_dir;

#[cfg(feature = "build-time-bindgen")]
use bindgen::RustTarget;

fn main() {
	let ogg_vendor_path = current_dir()
		.expect("Could not get current working directory")
		.join("ogg_vendor");

	println!(
		"cargo:rerun-if-changed={}",
		ogg_vendor_path.to_str().unwrap()
	);

	let include_path = ogg_vendor_path.join("include");
	println!("cargo:include={}", include_path.to_str().unwrap());

	#[cfg(feature = "build-time-bindgen")]
	bindgen::Builder::default()
		.header(include_path.join("ogg/ogg.h").to_str().unwrap())
		.disable_header_comment()
		.clang_arg(format!("-I{}/include", ogg_vendor_path.to_str().unwrap()))
		.allowlist_function("ogg.*")
		.allowlist_type("ogg.*")
		.blocklist_item("__.*")
		.rust_target(RustTarget::Stable_1_82)
		.layout_tests(false) // Type layout is platform-specific
		.generate()
		.expect("Unable to generate bindings")
		.write_to_file("src/bindings.rs")
		.unwrap();

	cc::Build::new()
		.file(ogg_vendor_path.join("src/bitwise.c"))
		.file(ogg_vendor_path.join("src/framing.c"))
		.include(include_path)
		.compile("ogg");
}
