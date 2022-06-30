#[cfg(all(any(target_arch = "riscv32", target_arch = "riscv64"), target_os = "none"))]
pub mod riscv;