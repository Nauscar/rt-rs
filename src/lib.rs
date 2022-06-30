#![feature(alloc_error_handler)]
#![no_std]

mod alloc;
mod arch;
mod panic;
pub mod print;
mod entry;
mod rt;

#[cfg(not(target_os = "none"))]
compile_error!("Select a build target in .cargo/config, or try cargo build --target <target_arch>");

pub use print::*;