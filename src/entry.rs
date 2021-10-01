#[cfg(all(target_arch = "arm", target_os = "none"))]
pub use cortex_m_rt::entry as entry;
#[cfg(all(any(target_arch = "riscv32", target_arch = "riscv64"), target_os = "none"))]
pub use riscv_rt::entry as entry;