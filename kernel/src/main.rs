#![no_std]
#![no_main]

#![feature(alloc_error_handler)]
//#![feature(generic_associated_types)]

#![feature(lang_items)]

extern crate alloc;
extern crate core;

use core::panic::PanicInfo;
//use core::sync::atomic::{AtomicBool, Ordering::*};

use crate::global_alloc::KernelGlobalAlloc;

pub mod global_alloc;
pub mod fallible;

#[global_allocator]
static KERNEL_GLOBAL_ALLOC: KernelGlobalAlloc = KernelGlobalAlloc::new();

//#[deprecated(note = "Don't use! Only here to force the linker to keep other exported functions in the binary.")]
#[cfg(target_arch = "x86_64")]
#[no_mangle]
pub extern "C" fn _start() -> u32 {
	4225666
//	unimplemented!();
}

#[cfg(target_arch = "aarch64")]
#[no_mangle]
pub extern "C" fn _start() {
	unimplemented!();
}

//#[cfg(target_arch = "x86_64")]
//fn start_x86_64_uefi() -> ! {
//	loop {}
//}

//#[cfg(target_arch = "aarch64")]
//fn start_aarch64_uefi() -> ! {
//	unimplemented!()
//}

#[panic_handler]
fn kernel_panic_handler(_info: &PanicInfo) -> ! {
//	/// Flag for checking reentrant panics, i.e. when
//	/// code called from this panic handler again panics.
//	// TODO: This is a bug! panic_handler may be called from multiple threads (maybe running on different processors) so this should really use a KernelThreadLocal
//	static PANIC_REENTRANT: AtomicBool = AtomicBool::new(false);
	
	loop {}
}

#[alloc_error_handler]
fn kernel_alloc_error_handler(_layout: core::alloc::Layout) -> ! {
	panic!("Unfallible global allocation failed (this is a bug, global allocation is forbidden)")
}

//#[lang = "eh_personality"]
//#[no_mangle]
//pub extern "C" fn rust_eh_personality() {
//	
//}
