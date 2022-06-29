#[cfg(all(any(target_arch = "riscv32", target_arch = "riscv64"), target_os = "none"))]
pub mod riscv;

#[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), target_os = "none"))]
pub mod x86;
