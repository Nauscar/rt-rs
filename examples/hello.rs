#![no_std]
#![no_main]

use rt::*;

#[entry]
fn main() -> ! {
    println!("Hello, world!");
    loop {}
}