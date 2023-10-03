# OS

A Rust kernel made primarily with Philipp Oppermann's blog and the OSDEV wiki and the revelations in my dreams.


# Setup

for Fedora:
- sudo dnf groupinstall "Development Tools"
- rustup override set nightly
- rustup component add rust-src --toolchain nightly-x86_64-unknown-linux-gnu
- rustup component add llvm-tools-preview
- cargo install bootimage
- dnf install qemu
- ./runc.sh (or run.sh if you don't want to compile beforehand)
  - cargo bootimage
  - qemu-system-x86_64 -drive format=raw,file=target/x86_64-rust_os/debug/bootimage-rust_os.bin

for Debian:
- sudo dnf groupinstall "Development Tools"
- rustup override set nightly
- rustup component add rust-src --toolchain nightly-x86_64-unknown-linux-gnu
- rustup component add llvm-tools-preview
- cargo install bootimage
- apt install qemu-utils qemu-system-x86 qemu-system-gui

also this was in my notes and i have no clue why
- compile rust std for bare metal:
  - cargo build -Z build-std --target x86_64-rust_os.json
