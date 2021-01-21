#![no_std]

pub type KernelEntryFn = unsafe extern "sysv64" fn(uefi_rs::Handle, uefi_rs::prelude::SystemTable<uefi_rs::prelude::Boot>) -> !;

#[repr(C)]
pub struct BootData {
	
}
