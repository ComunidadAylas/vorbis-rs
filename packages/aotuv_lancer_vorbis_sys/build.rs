use std::{env, env::current_dir};

#[cfg(feature = "build-time-bindgen")]
use bindgen::RustTarget;

fn main() {
	#[cfg(feature = "build-time-bindgen")]
	let ogg_vendor_path = current_dir()
		.expect("Could not get current working directory")
		.join("ogg_vendor");

	let vorbis_vendor_path = current_dir()
		.expect("Could not get current working directory")
		.join("vorbis_vendor");

	println!(
		"cargo:rerun-if-changed={}",
		vorbis_vendor_path
			.to_str()
			.expect("The vorbis_vendor submodule is in a non-UTF-8 path")
	);

	let vorbis_include_path = vorbis_vendor_path.join("include");
	println!("cargo:include={}", vorbis_include_path.to_str().unwrap());

	#[cfg(feature = "build-time-bindgen")]
	bindgen::Builder::default()
		.header(vorbis_include_path.join("vorbis/codec.h").to_str().unwrap())
		.header(
			vorbis_include_path
				.join("vorbis/vorbisenc.h")
				.to_str()
				.unwrap()
		)
		.header(
			vorbis_include_path
				.join("vorbis/vorbisfile.h")
				.to_str()
				.unwrap()
		)
		.disable_header_comment()
		.clang_arg(format!("-I{}", vorbis_include_path.to_str().unwrap()))
		.clang_arg(format!("-I{}/lib", vorbis_vendor_path.to_str().unwrap()))
		.clang_arg(format!("-I{}/include", ogg_vendor_path.to_str().unwrap()))
		.clang_arg("-DOV_EXCLUDE_STATIC_CALLBACKS")
		.allowlist_function("vorbis_.*")
		.allowlist_function("ov_.*")
		.blocklist_function("ov_f?open")
		.blocklist_function("ov_test")
		.allowlist_type("vorbis_.*")
		.allowlist_type("ovectl.*")
		.allowlist_type("OggVorbis.*")
		.allowlist_var("OV_.*")
		.allowlist_var("NOTOPEN")
		.allowlist_var("PARTOPEN")
		.allowlist_var("OPENED")
		.allowlist_var("STREAMSET")
		.allowlist_var("INITSET")
		.blocklist_item("_IO.*")
		.blocklist_item("FILE")
		.blocklist_item("ogg_.*")
		.blocklist_item("__.*")
		.rust_target(RustTarget::Stable_1_77)
		.layout_tests(false) // Type layout is platform-specific
		.generate()
		.expect("Unable to generate bindings")
		.write_to_file("src/bindings.rs")
		.unwrap();

	cc::Build::new()
		// libvorbis
		.file(vorbis_vendor_path.join("lib/mdct.c"))
		.file(vorbis_vendor_path.join("lib/smallft.c"))
		.file(vorbis_vendor_path.join("lib/block.c"))
		.file(vorbis_vendor_path.join("lib/envelope.c"))
		.file(vorbis_vendor_path.join("lib/window.c"))
		.file(vorbis_vendor_path.join("lib/lsp.c"))
		.file(vorbis_vendor_path.join("lib/lpc.c"))
		.file(vorbis_vendor_path.join("lib/analysis.c"))
		.file(vorbis_vendor_path.join("lib/synthesis.c"))
		.file(vorbis_vendor_path.join("lib/psy.c"))
		.file(vorbis_vendor_path.join("lib/info.c"))
		.file(vorbis_vendor_path.join("lib/floor1.c"))
		.file(vorbis_vendor_path.join("lib/floor0.c"))
		.file(vorbis_vendor_path.join("lib/cpu.c"))
		.file(vorbis_vendor_path.join("lib/xmmlib.c"))
		.file(vorbis_vendor_path.join("lib/res0.c"))
		.file(vorbis_vendor_path.join("lib/mapping0.c"))
		.file(vorbis_vendor_path.join("lib/registry.c"))
		.file(vorbis_vendor_path.join("lib/codebook.c"))
		.file(vorbis_vendor_path.join("lib/sharedbook.c"))
		.file(vorbis_vendor_path.join("lib/lookup.c"))
		.file(vorbis_vendor_path.join("lib/bitrate.c"))
		// libvorbisfile
		.file(vorbis_vendor_path.join("lib/vorbisfile.c"))
		// libvorbisenc
		.file(vorbis_vendor_path.join("lib/vorbisenc.c"))
		// Avoid warning flood from upstream code we'd rather not touch
		.warnings(false)
		.flag_if_supported("-Wno-discarded-qualifiers") // GCC
		.flag_if_supported("-Wno-incompatible-pointer-types-discards-qualifiers") // Clang
		.flag_if_supported("-Wno-int-conversion") // GCC and Clang
		.flag_if_supported("-Wno-implicit-int") // GCC
		.flag_if_supported("-Wno-return-type") // Clang
		// libogg headers
		.include(env::var_os("DEP_OGG_INCLUDE").unwrap())
		.include(vorbis_vendor_path.join("lib"))
		.include(vorbis_include_path)
		// Fixes building on Windows with the GCC toolchain, and maybe other
		// platforms with a modern C compiler
		.define("HAVE_STDINT_H", None)
		.compile("vorbis");
}
