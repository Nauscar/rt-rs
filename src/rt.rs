#[cfg(all(target_arch = "arm", target_os = "none"))]
use cortex_m_rt as _;
#[cfg(all(any(target_arch = "riscv32", target_arch = "riscv64"), target_os = "none"))]
use riscv_rt as _;
#[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), target_os = "none"))]
use bootloader as _;