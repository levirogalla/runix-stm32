use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::{env, ffi::OsStr};
use cc;

fn main() {
    let mut target = env::var("TARGET").unwrap();

    // When using a custom target JSON, `$TARGET` contains the path to that JSON file. By
    // convention, these files are named after the actual target triple, eg.
    // `thumbv7m-customos-elf.json`, so we extract the file stem here to allow custom target specs.
    let path = Path::new(&target);

    // Put the linker script somewhere the linker can find it
    let out = &PathBuf::from(env::var_os("OUT_DIR").unwrap());
    let link_x = include_bytes!("src/boot/link.x");
    let mut f = File::create(out.join("link.x")).unwrap();
    f.write_all(link_x).unwrap();


    println!("cargo:rustc-check-cfg=cfg(has_fpu)");

    if target.ends_with("-eabihf") {
        println!("cargo:rustc-cfg=has_fpu");
    }

    println!("cargo:rustc-link-search={}", out.display());

    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=src/boot/link.x");

    // compile in asm files
    cc::Build::new()
        .file("src/entry.s")
        .compile("entry");
}
