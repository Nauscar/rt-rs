#[cfg(all(target_arch = "arm", target_os = "none"))]
use cortex_m_semihosting::hio::HostStream as Stdio;
#[cfg(all(any(target_arch = "riscv32", target_arch = "riscv64"), target_os = "none"))]
use crate::arch::riscv::uart::Uart as Stdio;
#[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), target_os = "none"))]
use uart_16550::SerialPort as Stdio;

use lazy_static::lazy_static;
use spin::Mutex;

#[cfg(all(target_arch = "arm", target_os = "none"))]
lazy_static! {
    pub static ref WRITER: Mutex<Stdio> = Mutex::new(
		cortex_m_semihosting::hio::hstdout().unwrap()
	);
}

#[cfg(all(any(target_arch = "riscv32", target_arch = "riscv64"), target_os = "none"))]
lazy_static! {
	pub static ref WRITER: Mutex<Stdio> = Mutex::new(
		#[cfg(all(target_arch = "arm", target_os = "none"))]
		Stdio::new(0x1000_0000)
	);
}

#[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), target_os = "none"))]
lazy_static! {
	pub static ref WRITER: Mutex<Stdio> = Mutex::new(
		unsafe { Stdio::new(0x3f8) }
	);
}

#[macro_export]
macro_rules! print
{
	($($args:tt)+) => ({
			use core::fmt::Write;
			WRITER.lock().write_fmt(format_args!($($args)+)).unwrap();
			});
}
#[macro_export]
macro_rules! println
{
	() => ({
		   print!("\r\n")
		   });
	($fmt:expr) => ({
			print!(concat!($fmt, "\r\n"))
			});
	($fmt:expr, $($args:tt)+) => ({
			print!(concat!($fmt, "\r\n"), $($args)+)
			});
}