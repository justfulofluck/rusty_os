# RustyOS

A minimal experimental OS/scheduler written in Rust for the ESP32-C3 (RISC-V).

## Structure

- `src/` — main project: basic task scheduler with two example tasks
- `rusty_os/` — companion Rust crate
- `document.txt` — setup notes (toolchain, Wokwi simulator)

## Build & Run

```sh
rustup target add riscv32imc-unknown-none-elf
cargo build
cargo run   # flashes via espflash; or use the Wokwi VS Code simulator
```

## License

No license specified yet.
