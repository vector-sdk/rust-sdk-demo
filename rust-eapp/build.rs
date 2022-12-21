/// Build options
///
// SPDX-License-Identifier: MIT
// Copyright (C) 2022 VTT Technical Research Centre of Finland Ltd

fn main() {
    // Include linker script that defines memory areas required by the
    // enclave application crate
    println!("cargo:rustc-link-arg=-T");
    println!("cargo:rustc-link-arg=./rust-eapp/eapp.lds");

    // If heap is used, 'HEAP_SIZE' symbol is used to define its size in bytes.
    // The size must be multiple of the system page size (4096 bytes). Default
    // value is 0.

    println!("cargo:rustc-link-arg=--defsym");
    println!("cargo:rustc-link-arg=HEAP_SIZE=8192");

    // If not(feature = "heap_rt"), then ecall buffer sizes must be defined here
    println!("cargo:rustc-link-arg=--defsym");
    println!("cargo:rustc-link-arg=ECALL_INPUT_SIZE=1024");

    println!("cargo:rustc-link-arg=--defsym");
    println!("cargo:rustc-link-arg=ECALL_OUTPUT_SIZE=2048");
}
