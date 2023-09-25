# OS

A Rust kernel made primarily with Philipp Oppermann's blog and the OSDEV wiki.


# Setup

pre-requirements for fedora:
- sudo dnf groupinstall "Development Tools"

To setup the environment run:
- rustup override set nightly
- rustup component add rust-src --toolchain nightly-x86_64-unknown-linux-gnu
- rustup component add llvm-tools-preview
- cargo install bootimage
- dnf install qemu
- ./runc.sh (or run.sh if you don't want to compile beforehand)
  - cargo bootimage
  - qemu-system-x86_64 -drive format=raw,file=target/x86_64-rust_os/debug/bootimage-rust_os.bin

also this was in my notes and i have no clue why
- compile rust std for bare metal:
  - cargo build -Z build-std --target x86_64-rust_os.json
