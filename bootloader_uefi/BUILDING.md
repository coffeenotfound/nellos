## How to build `bootloader_uefi`

`cargo build --target x86_64-unknown-uefi`

* `-Z build-std=core,alloc` cross compiles `std` and `alloc` for the uefi target
* `-Z build-std-features=compiler-builtins-mem` causes the `compiler-builtins` crate
	to export `memcpy` et al. which are required by the uefi crate

## Running qemu
~~`qemu-system-x86_64 --bios $QEMU/_ovmf/RELEASEX64_OVMF.fd -drive file=fat:rw:a,format=raw -net none`~~
`qemu-system-x86_64 --bios $QEMU/_ovmf/RELEASEX64_OVMF.fd -drive file="run/boot.img",format=raw -net none`
