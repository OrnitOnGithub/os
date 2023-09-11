# OS
An "OS" made primarily with Philipp Oppermann's blog

# Compile
This might be wrong idk lol

- compile rust std for bare metal:
  - cargo build -Z build-std --target x86_64-rust_os.json

also switch to nightly i think 

-then compile & run the project:
  - cargo bootimage
  - qemu-system-x86_64 -drive format=raw,file=target/x86_64-rust_os/debug/bootimage-rust_os.bin
