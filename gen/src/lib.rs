//! Code generator for Raspberry Pi Pico SDK.

#![warn(clippy::pedantic)]
#![allow(clippy::missing_errors_doc, clippy::missing_panics_doc)]

mod clocks;
mod dma;
mod sio;
mod spi;
mod syscfg;

pub use drone_config::{bail, Result};
use drone_svd::{Device, Generator};
use std::env;
use std::fs::File;
use std::path::Path;

const WRAPPED_BUILTINS: &[&str] = &[
    // pico_mem_ops
    "memcpy",
    "memset",
    "__aeabi_memcpy",
    "__aeabi_memset",
    "__aeabi_memcpy4",
    "__aeabi_memset4",
    "__aeabi_memcpy8",
    "__aeabi_memset8",
    // pico_bit_ops
    "__clzsi2",      // gcc
    "__clzsi2",      // gcc
    "__clzdi2",      // gcc
    "__ctzsi2",      // gcc
    "__ctzdi2",      // gcc
    "__popcountsi2", // gcc
    "__popcountdi2", // gcc
    "__clz",         // armclang
    "__clzl",        // armclang
    "__clzsi2",      // armclang
    "__clzll",       // armclang
    // pico_int64_ops
    "__aeabi_lmul",
    // pico_divider
    "__aeabi_idiv",
    "__aeabi_idivmod",
    "__aeabi_ldivmod",
    "__aeabi_uidiv",
    "__aeabi_uidivmod",
    "__aeabi_uldivmod",
    // pico_float
    "__aeabi_fadd",
    "__aeabi_fdiv",
    "__aeabi_fmul",
    "__aeabi_frsub",
    "__aeabi_fsub",
    "__aeabi_cfcmpeq",
    "__aeabi_cfrcmple",
    "__aeabi_cfcmple",
    "__aeabi_fcmpeq",
    "__aeabi_fcmplt",
    "__aeabi_fcmple",
    "__aeabi_fcmpge",
    "__aeabi_fcmpgt",
    "__aeabi_fcmpun",
    "__aeabi_i2f",
    "__aeabi_l2f",
    "__aeabi_ui2f",
    "__aeabi_ul2f",
    "__aeabi_f2iz",
    "__aeabi_f2lz",
    "__aeabi_f2uiz",
    "__aeabi_f2ulz",
    "__aeabi_f2d",
    "sqrtf",
    "cosf",
    "sinf",
    "tanf",
    "atan2f",
    "expf",
    "logf",
    "ldexpf",
    "copysignf",
    "truncf",
    "floorf",
    "ceilf",
    "roundf",
    "sincosf", // gnu
    "asinf",
    "acosf",
    "atanf",
    "sinhf",
    "coshf",
    "tanhf",
    "asinhf",
    "acoshf",
    "atanhf",
    "exp2f",
    "log2f",
    "exp10f",
    "log10f",
    "powf",
    "powintf", // gnu
    "hypotf",
    "cbrtf",
    "fmodf",
    "dremf",
    "remainderf",
    "remquof",
    "expm1f",
    "log1pf",
    "fmaf",
    // pico_double
    "__aeabi_dadd",
    "__aeabi_ddiv",
    "__aeabi_dmul",
    "__aeabi_drsub",
    "__aeabi_dsub",
    "__aeabi_cdcmpeq",
    "__aeabi_cdrcmple",
    "__aeabi_cdcmple",
    "__aeabi_dcmpeq",
    "__aeabi_dcmplt",
    "__aeabi_dcmple",
    "__aeabi_dcmpge",
    "__aeabi_dcmpgt",
    "__aeabi_dcmpun",
    "__aeabi_i2d",
    "__aeabi_l2d",
    "__aeabi_ui2d",
    "__aeabi_ul2d",
    "__aeabi_d2iz",
    "__aeabi_d2lz",
    "__aeabi_d2uiz",
    "__aeabi_d2ulz",
    "__aeabi_d2f",
    "sqrt",
    "cos",
    "sin",
    "tan",
    "atan2",
    "exp",
    "log",
    "ldexp",
    "copysign",
    "trunc",
    "floor",
    "ceil",
    "round",
    "sincos", // gnu
    "asin",
    "acos",
    "atan",
    "sinh",
    "cosh",
    "tanh",
    "asinh",
    "acosh",
    "atanh",
    "exp2",
    "log2",
    "exp10",
    "log10",
    "pow",
    "powint", // gnu
    "hypot",
    "cbrt",
    "fmod",
    "drem",
    "remainder",
    "remquo",
    "expm1",
    "log1p",
    "fma",
];

const SIO_CORE_REGS: &[&str] = &[
    "FIFO_ST",
    "FIFO_WR",
    "FIFO_RD",
    "SPINLOCK_ST",
    "SPINLOCK0",
    "SPINLOCK1",
    "SPINLOCK2",
    "SPINLOCK3",
    "SPINLOCK4",
    "SPINLOCK5",
    "SPINLOCK6",
    "SPINLOCK7",
    "SPINLOCK8",
    "SPINLOCK9",
    "SPINLOCK10",
    "SPINLOCK11",
    "SPINLOCK12",
    "SPINLOCK13",
    "SPINLOCK14",
    "SPINLOCK15",
    "SPINLOCK16",
    "SPINLOCK17",
    "SPINLOCK18",
    "SPINLOCK19",
    "SPINLOCK20",
    "SPINLOCK21",
    "SPINLOCK22",
    "SPINLOCK23",
    "SPINLOCK24",
    "SPINLOCK25",
    "SPINLOCK26",
    "SPINLOCK27",
    "SPINLOCK28",
    "SPINLOCK29",
    "SPINLOCK30",
    "SPINLOCK31",
    "DIV_UDIVIDEND",
    "DIV_UDIVISOR",
    "DIV_SDIVIDEND",
    "DIV_SDIVISOR",
    "DIV_QUOTIENT",
    "DIV_REMAINDER",
    "DIV_CSR",
    "INTERP0_ACCUM0",
    "INTERP0_ACCUM1",
    "INTERP0_BASE0",
    "INTERP0_BASE1",
    "INTERP0_BASE2",
    "INTERP0_POP_LANE0",
    "INTERP0_POP_LANE1",
    "INTERP0_POP_FULL",
    "INTERP0_PEEK_LANE0",
    "INTERP0_PEEK_LANE1",
    "INTERP0_PEEK_FULL",
    "INTERP0_CTRL_LANE0",
    "INTERP0_CTRL_LANE1",
    "INTERP0_ACCUM0_ADD",
    "INTERP0_ACCUM1_ADD",
    "INTERP0_BASE_1AND0",
    "INTERP1_ACCUM0",
    "INTERP1_ACCUM1",
    "INTERP1_BASE0",
    "INTERP1_BASE1",
    "INTERP1_BASE2",
    "INTERP1_POP_LANE0",
    "INTERP1_POP_LANE1",
    "INTERP1_POP_FULL",
    "INTERP1_PEEK_LANE0",
    "INTERP1_PEEK_LANE1",
    "INTERP1_PEEK_FULL",
    "INTERP1_CTRL_LANE0",
    "INTERP1_CTRL_LANE1",
    "INTERP1_ACCUM0_ADD",
    "INTERP1_ACCUM1_ADD",
    "INTERP1_BASE_1AND0",
];

/// Injects optimized versions for some standard compiler builtins.
///
/// RP2040 bootrom contains a library of optimized common functions. This
/// function sets up the final binary to use those optimized functions instead
/// of standard ones.
pub fn replace_builtins() {
    if env::var_os("CARGO_FEATURE_HOST").is_some() {
        return;
    }
    for builtin in WRAPPED_BUILTINS {
        println!("cargo:rustc-link-arg=--wrap={builtin}");
    }
}

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
    let mut generator = Generator::new("rp2040_reg_tokens");
    generator.core_regs(
        "rp2040_core_reg_tokens",
        "drone_cortexm::map::cortexm_reg_tokens",
        |peripheral, names| {
            peripheral == "SIO" && names.iter().any(|name| SIO_CORE_REGS.contains(&name.as_str()))
        },
    );
    generator.register_traits_callback(|peripheral, _, _| {
        (peripheral != "SIO").then(|| "RegAtomicAlias".to_string()).into_iter().collect()
    });
    generator
}

fn svd_parse() -> Result<Device> {
    let pico_sdk_path = env::var("PICO_SDK_PATH")?;
    let pico_sdk_path = Path::new(&pico_sdk_path);
    let mut dev = drone_svd::parse(pico_sdk_path.join("src/rp2040/hardware_regs/rp2040.svd"))?;
    clocks::patch(&mut dev);
    dma::patch(&mut dev);
    sio::patch(&mut dev);
    spi::patch(&mut dev);
    syscfg::patch(&mut dev);
    Ok(dev)
}
