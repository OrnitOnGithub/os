# Compile and run.
argo bootimage # Compile and turn into bootimage using cargo crate "bootimage"
sudo qemu-system-x86_64 -drive format=raw,file=target/x86_64-rust_os/debug/bootimage-rust_os.bin # Run in qemu