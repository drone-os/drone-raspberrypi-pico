//! SVD bindings generator for Raspberry Pi Pico SDK.

#![warn(clippy::pedantic)]
#![allow(clippy::missing_errors_doc, clippy::missing_panics_doc)]

mod sio;

pub use drone_config::{bail, Result};
use drone_svd::{Device, Generator};
use std::env;
use std::fs::File;
use std::path::Path;

/// Generates code for register mappings.
pub fn generate_regs(pool_number: usize, pool_size: usize) -> Result<()> {
    let out_dir = env::var("OUT_DIR")?;
    let out_dir = Path::new(&out_dir);
    let dev = svd_parse()?;
    let mut output = File::create(out_dir.join("svd_regs.rs"))?;
    generator().generate_regs(&mut output, dev, pool_number, pool_size)
}

/// Generates code for register tokens struct.
pub fn generate_index() -> Result<()> {
    let out_dir = env::var("OUT_DIR")?;
    let out_dir = Path::new(&out_dir);
    let dev = svd_parse()?;
    let mut reg_output = File::create(out_dir.join("svd_reg_index.rs"))?;
    generator().generate_index(&mut reg_output, dev)
}

fn generator() -> Generator<'static> {
    Generator::new("rp2040_reg_tokens")
}

fn svd_parse() -> Result<Device> {
    let pico_sdk_path = env::var("PICO_SDK_PATH")?;
    let pico_sdk_path = Path::new(&pico_sdk_path);
    let mut dev = drone_svd::parse(pico_sdk_path.join("src/rp2040/hardware_regs/rp2040.svd"))?;
    sio::patch(&mut dev);
    Ok(dev)
}
