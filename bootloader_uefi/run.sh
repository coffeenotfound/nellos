#!/bin/bash

# Ensure script dir
cd "$(dirname "$0")"

# Compile bootloader
cargo build --target x86_64-unknown-uefi -Z build-std=core,alloc -Z build-std-features=compiler-builtins-mem

# Prepare run dir
mkdir -p "./run"
test -e "./run/*" && rm -r "./run/*"

mkdir -p "./run/hda"
cp "./target/x86_64-unknown-uefi/debug/bootloader_uefi.efi" "./run/hda"

# Run qemu
#qemu-system-x86_64 --bios "$QEMU/_ovmf/RELEASEX64_OVMF.fd" -drive file=fat:rw:"./run/hda",format=raw -net none
qemu-system-x86_64 --bios $QEMU/_ovmf/RELEASEX64_OVMF.fd -drive file="run/boot.img",format=raw -net none