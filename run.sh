# Run without compiling.
# Remember to chmod +x this and run it without sudo.
sudo qemu-system-x86_64 -drive format=raw,file=target/x86_64-rust_os/debug/bootimage-rust_os.bin # Run in qemu