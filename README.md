# rt
Rust runtimes for no_std.

## Dependencies
Qemu
```sh
pacman -S qemu qemu-arch-extra
```
```sh
apt install qemu qemu-system-riscv32 qemu-system-riscv64
```

## Examples
```sh
cargo run --release --example <example>
```
Where example:
- entry
- hello

Build targets available in .cargo/config.toml