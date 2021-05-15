#!/bin/bash

set -e

# Ensure script dir
cd "$(dirname "$0")"

# Compile bootloader
(
	echo ""
	echo ""
	echo "Building bootloader..."
	cd "../bootloader_uefi"
	cargo build --target x86_64-unknown-uefi
)

# Compile kernel
(
	echo ""
	echo ""
	echo "Building kernel..."
	cd "../kernel"
	cargo build --target x86_64-none-kernel-nabi.json
)

bootloader_efi_path="$(pwd)/../bootloader_uefi/target/x86_64-unknown-uefi/debug/bootloader_uefi.efi"
kernel_elf_path="$(pwd)/../kernel/target/x86_64-none-kernel-nabi/debug/kernel.elf"
echo "bootloader efi path: $bootloader_efi_path"
echo "kernel elf path: $kernel_elf_path"

# Build disk image
(
	echo ""
	echo ""
	echo "Making disk image..."
	cd "../tools/makediskimg"
	cargo run -- --bootloaderefi="$bootloader_efi_path" --kernelelf="$kernel_elf_path"
)

# Prepare run dir
mkdir -p "./run"
test -e "./run/*" && rm -r "./run/*"

#mkdir -p "./run/hda"
#cp "./target/x86_64-unknown-uefi/debug/bootloader_uefi.efi" "./run/hda"

# Copy disk image
cp "../tools/makediskimg/build/boot.img" "run/"

# Run qemu
echo ""
echo ""
echo "Running QEMU"
qemu-system-x86_64 --bios $QEMU/_ovmf/RELEASEX64_OVMF.fd -m 512 -smp 2 -drive file="run/boot.img",format=raw -net none
#qemu-system-x86_64 --bios $QEMU/_ovmf/RELEASEX64_OVMF.fd -m 512 -smp 2 -drive file="run/boot.img",format=raw -net none -s -S
