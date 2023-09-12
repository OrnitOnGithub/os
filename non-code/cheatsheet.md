# Compile
- rustup default nightly
- cargo build --target x86_64-rust_os.json (useless I think)
- compile rust std for bare metal: cargo build -Z build-std --target x86_64-rust_os.json (Also not sure if this is actually needed)

# Run
or use scripts run.sh & runc.sh
- cargo bootimage
- qemu-system-x86_64 -drive format=raw,file=target/x86_64-rust_os/debug/bootimage-rust_os.bin