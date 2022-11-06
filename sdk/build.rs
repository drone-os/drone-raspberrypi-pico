use color_eyre::eyre::{bail, Result, WrapErr};
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::Path;
use std::process::Command;
use std::{env, fs, io};

const FLAGS_PATH: &str = "CMakeFiles/drone_raspberrypi_pico_sdk.dir/flags.make";
const BOOT2_PATH: &str = "pico-sdk/src/rp2_common/boot_stage2/bs2_default_padded_checksummed.S";

fn main() -> Result<()> {
    color_eyre::install()?;

    let src_dir = env::var("CARGO_MANIFEST_DIR")?;
    let src_dir = Path::new(&src_dir);
    let out_dir = env::var("OUT_DIR")?;
    let out_dir = Path::new(&out_dir);
    let build_dir = out_dir.join("build");

    println!("cargo:rerun-if-changed=wrapper.h");
    println!("cargo:rerun-if-changed=wrapper.c");
    println!("cargo:rerun-if-changed=CMakeLists.txt");
    println!("cargo:rerun-if-changed=pico_sdk_import.cmake");
    if env::var_os("CARGO_FEATURE_STD").is_some() {
        return Ok(());
    }
    println!("cargo:rustc-link-search=native={}", build_dir.display());
    println!("cargo:rustc-link-lib=drone_raspberrypi_pico_sdk");

    fs::create_dir_all(&build_dir)?;
    env::set_current_dir(&build_dir)?;
    configure(src_dir)?;
    build()?;
    let c_flags = parse_flags(&build_dir.join(FLAGS_PATH))?;
    generate_bindings(&out_dir.join("bindings.rs"), &src_dir.join("wrapper.h"), &c_flags)?;
    generate_boot2(&out_dir.join("boot2.rs"), &build_dir.join(BOOT2_PATH))?;

    Ok(())
}

fn configure(src_dir: &Path) -> Result<()> {
    let mut cmake = Command::new("cmake");
    cmake.arg(src_dir);
    if !cmake.status().wrap_err("running cmake")?.success() {
        bail!("cmake exited unsucessfully");
    }
    Ok(())
}

fn build() -> Result<()> {
    let build_cores = env::var("NIX_BUILD_CORES");
    let build_cores = build_cores.as_deref().unwrap_or("4");
    let mut make = Command::new("make");
    make.arg(format!("--jobs={build_cores}"));
    if !make.status().wrap_err("running make")?.success() {
        bail!("make exited unsucessfully");
    }
    Ok(())
}

fn parse_flags(flags: &Path) -> Result<Vec<String>> {
    let mut c_flags = Vec::new();
    for line in BufReader::new(File::open(flags)?).lines() {
        let line = line.wrap_err("reading flags.make")?;
        let flags = ["C_DEFINES = ", "C_INCLUDES = ", "C_FLAGS = "]
            .iter()
            .find_map(|prefix| line.strip_prefix(prefix))
            .into_iter()
            .flat_map(|line| line.split_ascii_whitespace())
            .map(ToOwned::to_owned);
        c_flags.extend(flags);
    }
    Ok(c_flags)
}

fn generate_bindings(bindings: &Path, wrapper: &Path, c_flags: &[String]) -> Result<()> {
    bindgen::Builder::default()
        .use_core()
        .ctypes_prefix("::core::ffi")
        .clang_args(env::var("EXTRA_CLANG_CFLAGS").unwrap_or_default().split_ascii_whitespace())
        .clang_args(c_flags)
        .header(wrapper.display().to_string())
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .wrap_err("generating bindings")?
        .write_to_file(bindings)
        .wrap_err("writing bindings")?;
    Ok(())
}

fn generate_boot2(boot2: &Path, bs2_default: &Path) -> Result<()> {
    let mut boot2 = File::create(boot2)?;
    writeln!(boot2, "#[macro_export]")?;
    writeln!(boot2, "macro_rules! boot2 {{")?;
    writeln!(boot2, "    () => {{")?;
    writeln!(boot2, "        ::core::arch::global_asm!(r#\"")?;
    io::copy(&mut File::open(bs2_default)?, &mut boot2)?;
    writeln!(boot2, "        \"#);")?;
    writeln!(boot2, "    }};")?;
    writeln!(boot2, "}}")?;
    Ok(())
}
