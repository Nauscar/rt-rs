use entry::entry as entry;

extern "Rust" {
    fn main();
}

#[entry]
fn _start() -> ! {
    unsafe { main() }
    panic!()
}