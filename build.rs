fn main () {
    println!("cargo:rerun-if-changed=memory.x");
    println!("cargo:rerun-if-changed=riscv-link.x");
}