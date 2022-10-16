use color_eyre::eyre::{bail, Result};
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::Path;
use std::process::Command;
use std::{env, fs};

fn main() -> Result<()> {
    color_eyre::install()?;

    let src_dir = env::var("CARGO_MANIFEST_DIR")?;
    let src_dir = Path::new(&src_dir);
    let out_dir = env::var("OUT_DIR")?;
    let out_dir = Path::new(&out_dir);
    let build_cores = env::var("NIX_BUILD_CORES");
    let build_cores = build_cores.as_deref().unwrap_or("4");
    let build_dir = out_dir.join("build");

    fs::create_dir_all(&build_dir)?;
    env::set_current_dir(&build_dir)?;

    let mut cmake = Command::new("cmake");
    cmake.arg(src_dir);
    if !cmake.status()?.success() {
        bail!("cmake exited unsucessfully");
    }

    let mut make = Command::new("make");
    make.arg(format!("--jobs={build_cores}"));
    if !make.status()?.success() {
        bail!("make exited unsucessfully");
    }

    println!("cargo:rerun-if-changed=CMakeLists.txt");
    println!("cargo:rerun-if-changed=pico_sdk_import.cmake");
    if env::var_os("CARGO_FEATURE_STD").is_none() {
        println!("cargo:rustc-link-search=native={}", build_dir.display());
        println!("cargo:rustc-link-lib=drone_raspberrypi_pico_sdk");
    }

    let mut bindings = bindgen::Builder::default()
        .use_core()
        .ctypes_prefix("::core::ffi")
        .clang_args(env::var("EXTRA_CLANG_CFLAGS").unwrap_or_default().split_ascii_whitespace())
        .header(src_dir.join("wrapper.h").to_string_lossy())
        .parse_callbacks(Box::new(bindgen::CargoCallbacks));

    let flags = File::open(build_dir.join("CMakeFiles/drone_raspberrypi_pico_sdk.dir/flags.make"))?;
    for line in BufReader::new(flags).lines() {
        let line = line?;
        if let Some(args) = line
            .strip_prefix("C_DEFINES = ")
            .or_else(|| line.strip_prefix("C_INCLUDES = "))
            .or_else(|| line.strip_prefix("C_FLAGS = "))
        {
            bindings = bindings.clang_args(args.split_ascii_whitespace());
        }
    }

    bindings.generate()?.write_to_file(out_dir.join("bindings.rs"))?;

    Ok(())
}
