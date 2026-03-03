//! Unity's Il2Cpp runtime bindings

#![allow(
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unsafe_op_in_unsafe_fn
)]

#[cfg(not(doc))]
include!(concat!(env!("OUT_DIR"), "/il2cpp_sys.rs"));
