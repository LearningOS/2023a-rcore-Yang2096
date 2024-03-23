cargo build
rust-objcopy --strip-all target/riscv64gc-unknown-none-elf/debug/os -O binary target/riscv64gc-unknown-none-elf/debug/os.bin
timeout --foreground 5s qemu-system-riscv64 \
    -machine virt \
    -nographic \
    -bios ../bootloader/rustsbi-qemu.bin \
    -device loader,file=target/riscv64gc-unknown-none-elf/debug/os.bin,addr=0x80200000