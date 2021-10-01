#[cfg(feature = "halt")]
use panic_halt as _;

#[cfg(feature = "abort")]
use panic_abort as _;