use std::process::Command;
use std::path::{Path, PathBuf};
use std::env;
use std::io::{Write};
use std::ffi::OsStr;

pub fn main() {
	let out_dir = env::var("OUT_DIR").unwrap();
	let out_dir_path = Path::new(&out_dir);
	
	// Cargo link directives
	println!("cargo:rustc-link-search=native={}", out_dir);
	println!("cargo:rustc-link-lib=static=acpicasys");
	
	// For now we can't call bindgen in the build script.
	// Don't ask me why, but for some inexplicable reason one of
	// bindgen's dependencies, log, gets compiled with the target
	// triple instead of the host triple, like build-deps normally should.
	// Because std isn't available, this obviously fails.
	// I have literally not a single idea why it would do this.
	// But for now just take bindgen out of the crate's build-deps
	// and it works.
	
//	// Gen binds
//	let bindings = bindgen::builder()
//		.header("src_c/binds.h")
//		.clang_arg("-D__NELL_KERNEL")
//		.clang_arg("-v")
//		.clang_args(&[r#"-I./acpica/source/include/"#, "-nostdlib"])
//		.use_core()
//		.ctypes_prefix("::cty")
//		.rustfmt_bindings(true)
//		.parse_callbacks(Box::new(bindgen::CargoCallbacks))
//		.layout_tests(false)
//		.generate()
//		.expect("Failed to gen acpica rust bindings");
//	
//	bindings.write_to_file(&out_dir_path.join("binds.rs"))
//		.expect("Failed to write binds.rs");
	
	// Debugger and disassembler seem to not be part of the kernel subsystem compontents?
	//  They reference code from `common/` anyways, so I don't think they're safe to use
	
	let src_glob_patterns = &[
//		"acpica/source/common/ahids.c",
//		"acpica/source/common/ahpredef.c",
//		"acpica/source/common/ahtable.c",
//		"acpica/source/common/ahuuids.c",
		
//		"acpica/source/components/debugger/**/*.c",
//		"acpica/source/components/disassembler/**/*.c",
		
		"acpica/source/components/dispatcher/**/*.c",
		"acpica/source/components/events/**/*.c",
		"acpica/source/components/executer/**/*.c",
		"acpica/source/components/hardware/**/*.c",
		"acpica/source/components/namespace/**/*.c",
		"acpica/source/components/parser/**/*.c",
		"acpica/source/components/resources/**/*.c",
		"acpica/source/components/tables/**/*.c",
		"acpica/source/components/utilities/**/*.c",
	];
	
	let src_files = src_glob_patterns.iter().copied()
		.map(|p| glob::glob(p).unwrap())
		.flatten()
		.map(|p| p.expect("Glob error"))
		.collect::<Vec<PathBuf>>();
	
	// Cargo directives to observe source files
	for src_file in &src_files {
		println!("cargo:rerun-if-changed={}", src_file.as_path().to_str().unwrap());
	}
	
	// Cargo directives to observe headers
	for header_file in glob::glob("acpica/source/include/**/*.h").unwrap().map(|p| p.expect("Glob error")) {
		println!("cargo:rerun-if-changed={}", header_file.as_path().to_str().unwrap());
	}
	
	let mut out_files = Vec::<PathBuf>::with_capacity(src_files.len());
	
	for src_file in &src_files {
		let out_file: PathBuf = out_dir_path
			.join("ccout")
			.join(src_file.canonicalize().unwrap()
				.strip_prefix(Path::new("./acpica").canonicalize().unwrap())
			.unwrap()
			.with_extension("o"));
		
		std::fs::create_dir_all(out_file.as_path().parent().unwrap()).unwrap();
		
		let a = Command::new("clang")
			.arg("-v")
			.arg("-c")
			.arg("-Iacpica/source/include/")
			.arg("-D__NELL_KERNEL")
			.arg("--target=x86_64-none-none-gnu")
			.arg("-std=c11")
			.arg("-fpic")
			.arg("-nostdlib")
//			.arg("-nostdinc")
			.arg("-fno-builtin")
			.arg("-o").arg(format!("{}", out_file.display()))
			.arg(format!("{}", src_file.display()))
			.output()
			.expect("Invocation of clang failed");
		
		out_files.push(out_file);
		
		if !a.status.success() {
			std::io::stdout().write_all(&a.stdout).unwrap();
			std::io::stderr().write_all(&a.stderr).unwrap();
			
			panic!("Failed to compile acpica native lib");
		}
	}
	
	// Archive static lib
	let target_lib_path = out_dir_path.join("libacpicasys.a");
	
	let b = Command::new("llvm-ar")
		.arg("-rcs")
		.arg(&target_lib_path)
		.args(out_files.iter())
		.output()
		.expect("Failed to invoke archiver llvm-ar");
	
	if !b.status.success() {
		std::io::stdout().write_all(&b.stdout).unwrap();
		std::io::stderr().write_all(&b.stderr).unwrap();
		
		panic!("Failed to archive static {:?}", target_lib_path.file_name().unwrap_or(OsStr::new("<unknown>")));
	}
	
//	eprintln!("{}", target_lib_path.as_path().as_os_str().to_str().unwrap());
	
//	// Compile
//	cc::Build::new()
//		.no_default_flags(true)
//		.flag("-nostdlib")
//		.include("acpica/source/include")
//		.files(src_files)
//		.compile("acpica");
}
