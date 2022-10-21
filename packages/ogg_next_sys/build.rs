use git2::Repository;

fn main() {
	let repo = Repository::discover(".").expect("Could not discover current repository");

	let ogg_vendor_submodule = repo
		.find_submodule("ogg_vendor")
		.expect("Could not find ogg_vendor submodule");
	let ogg_vendor_path = repo.path().join("..").join(ogg_vendor_submodule.path());

	println!(
		"cargo:rerun-if-changed={}",
		ogg_vendor_path
			.to_str()
			.expect("The ogg_vendor submodule is in a non-UTF-8 path")
	);

	let include_path = ogg_vendor_path.join("include");
	println!("cargo:include={}", include_path.to_str().unwrap());

	#[cfg(feature = "build-time-bindgen")]
	bindgen::Builder::default()
		.header(include_path.join("ogg/ogg.h").to_str().unwrap())
		.disable_header_comment()
		.clang_arg("-I../../vendor/ogg/include")
		.size_t_is_usize(true)
		.allowlist_function("ogg.*")
		.allowlist_type("ogg.*")
		.blocklist_item("__.*")
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
