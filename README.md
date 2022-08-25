# Zeta_OS
It's an OS for the Teensy4.1 in Rust

Install the necessary cargo components:
```
cargo install cargo-binutils cargo-make
rustup component add llvm-tools-preview
```

Flash to the teensy:
```
cargo make flash
```
