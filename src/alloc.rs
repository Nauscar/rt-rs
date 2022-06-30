use static_alloc::Bump;

#[global_allocator]
static A: Bump<[u8; 1 << 10]> = Bump::uninit();

#[alloc_error_handler]
fn alloc_error_handler(_: core::alloc::Layout) -> ! {
    panic!()
}