use std::env;

use git2::Repository;

fn main() {
	let repo = Repository::discover(".").expect("Could not discover current repository");

	let vorbis_vendor_submodule = repo
		.find_submodule("vorbis_vendor")
		.expect("Could not find vorbis_vendor submodule");
	let vorbis_vendor_path = repo.path().join("..").join(vorbis_vendor_submodule.path());

	// libvorbis depends on libogg. libogg is built first due to our Cargo.toml dependency on ogg_sys,
	// but the Cargo linking instructions in its build script do not affect us
	println!("cargo:rustc-link-lib=static=ogg");

	println!(
		"cargo:rerun-if-changed={}",
		vorbis_vendor_path
			.to_str()
			.expect("The vorbis_vendor submodule is in a non-UTF-8 path")
	);

	let include_path = vorbis_vendor_path.join("include");
	println!("cargo:include={}", include_path.to_str().unwrap());

	#[cfg(feature = "build-time-bindgen")]
	bindgen::Builder::default()
		.header(include_path.join("vorbis/codec.h").to_str().unwrap())
		.header(include_path.join("vorbis/vorbisenc.h").to_str().unwrap())
		.header(include_path.join("vorbis/vorbisfile.h").to_str().unwrap())
		.disable_header_comment()
		.clang_arg("-I../../vorbis_vendor/include")
		.clang_arg("-I../../vorbis_vendor/lib")
		.clang_arg("-I../../ogg_vendor/include")
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
		.size_t_is_usize(true)
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
		.flag_if_supported("-Wno-discarded-qualifiers")
		.flag_if_supported("-Wno-int-conversion")
		.flag_if_supported("-Wno-implicit-int")
		// libogg headers
		.include(env::var_os("DEP_OGG_INCLUDE").unwrap())
		.include(vorbis_vendor_path.join("lib"))
		.include(include_path)
		// Fixes building on Windows with the GCC toolchain, and maybe other
		// platforms with a modern C compiler
		.define("HAVE_STDINT_H", None)
		.compile("vorbis");
}
