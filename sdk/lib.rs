#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![no_std]

#[cfg(not(feature = "std"))]
include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
