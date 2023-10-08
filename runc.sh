# Compile into bootimage and run.
# Remember to chmod +x this and run it without sudo.
cargo bootimage # Compile and turn into bootimage using cargo crate "bootimage"
echo ""
sudo qemu-system-x86_64 -drive format=raw,file=target/x86_64-rust_os/debug/bootimage-rust_os.bin -drive file=emulated/nvm.img,if=none,id=nvm -device nvme,serial=deadbeef,drive=nvm