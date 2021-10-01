#![no_std]
mod arch;
mod panic;
mod print;
mod entry;

#[cfg(not(target_os = "none"))]
compile_error!("Select a build target in .cargo/config, or try cargo build --target <target_arch>");
pub use entry::entry as entry;

#[cfg(all(target_arch = "arm", target_os = "none"))]
pub use cortex_m_semihosting::hio as stdout;
#[cfg(all(any(target_arch = "riscv32", target_arch = "riscv64"), target_os = "none"))]
pub use crate::arch::riscv::uart::Uart as stdout;

pub use print::*;