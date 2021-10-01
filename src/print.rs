#[macro_export]
macro_rules! print
{
	($($args:tt)+) => ({
			use core::fmt::Write;
            #[cfg(all(target_arch = "arm", target_os = "none"))]
            let _ = write!(stdout::hstdout().unwrap(), $($args)+);
            #[cfg(all(any(target_arch = "riscv32", target_arch = "riscv64"), target_os = "none"))]
			let _ = write!(rt::stdout::new(0x1000_0000), $($args)+);
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