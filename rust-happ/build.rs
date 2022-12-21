use std::env;

fn main() {
    let target = env::var("TARGET").unwrap();

    // Statically link for RISC-V, but dynamically link for x86*
    if target.contains("riscv32") || target.contains("riscv64") {
        println!("cargo:rustc-link-arg=-static");
    }
}
