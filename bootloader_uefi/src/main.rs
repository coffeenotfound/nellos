#![no_std]
#![no_main]

#![feature(abi_efiapi)]
#![feature(maybe_uninit_slice)]
#![feature(alloc_error_handler)]
#![feature(allocator_api)]
#![feature(const_raw_ptr_to_usize_cast)]
#![feature(array_methods)]

extern crate alloc;
extern crate core;

use alloc::boxed::Box;
use alloc::vec::Vec;
use core::cell::UnsafeCell;
use core::fmt::Write;
use core::iter::FromIterator;
use core::mem::{self, MaybeUninit};
use core::ptr;
use core::sync::atomic::{AtomicPtr, Ordering::*};

use atomic::Atomic;
use elf_rs::Elf;
use uefi::Handle;
use uefi::prelude::entry;
use uefi::proto::loaded_image::DevicePath;
use uefi::proto::media::file::{Directory, File, FileAttribute, FileMode, FileType, RegularFile};
use uefi::proto::media::fs::SimpleFileSystem;
use uefi::table::boot::SearchType;

use crate::alloc_uefi::GlobalAllocBootUefi;

pub mod more_uefi;
pub mod alloc_uefi;

mod uefip {
	pub use uefi::data_types::Char16;
	pub use uefi::prelude::*;
	pub use uefi::proto::console::text::{Color, Output};
}

#[global_allocator]
static GLOBAL_ALLOC_BOOT: GlobalAllocBootUefi = GlobalAllocBootUefi::new_uninit();

static STATIC_BOOT_SERVICES_PTR: AtomicPtr<uefip::BootServices> = AtomicPtr::new(ptr::null_mut());

static TEST_STDOUT_PTR: Atomic<usize> = Atomic::new(0x0);

#[entry]
fn efi_main(_img_handle: uefip::Handle, sys_table: uefip::SystemTable<uefip::Boot>) -> uefip::Status {
	// Store static boot services ptr
	// (This should ideally be done as early as possible
	//  so the panic_handler has a valid pointer, should smth. panic)
	STATIC_BOOT_SERVICES_PTR.store(sys_table.boot_services() as *const _ as *mut _, SeqCst);
	
	// Init global alloc
	unsafe {
		GLOBAL_ALLOC_BOOT.init_boot_alloc(sys_table.boot_services());
	}
	
	// Get uefi stdout handle
	let stdout = sys_table.stdout();
	
	// DEBUG:
	TEST_STDOUT_PTR.store((&mut *stdout) as *mut _ as usize, SeqCst);
	
	// Log hello world
//	stdout.set_color(uefip::Color::White, uefip::Color::Blue).unwrap().unwrap();
	writeln!(stdout, "Hello World (from bootloader_uefi!)").unwrap();
	writeln!(stdout).unwrap();
	
	// Find boot part handle
	let mut boot_part_sfs_prot = Option::<&UnsafeCell<SimpleFileSystem>>::None;
	{
		let sfs_handles = {
			let buf_len = sys_table.boot_services()
				.locate_handle(SearchType::from_proto::<SimpleFileSystem>(), None)
				.unwrap().unwrap();
			
			let mut handle_buf = Vec::from_iter((0..buf_len).map(|_| MaybeUninit::<Handle>::zeroed()));
			let handle_buf_slice_init = unsafe {
				MaybeUninit::slice_assume_init_mut(handle_buf.as_mut_slice())
			};
			sys_table.boot_services()
				.locate_handle(SearchType::from_proto::<SimpleFileSystem>(), Some(handle_buf_slice_init))
				.unwrap().unwrap();
			
//			let handle_buf = unsafe {
//				mem::transmute::<_, Vec<Handle>>(handle_buf)
//			};
			unsafe {
				mem::transmute::<_, Vec<Handle>>(handle_buf)
			}
		};
		
		for &handle in &sfs_handles {
			match sys_table.boot_services().handle_protocol::<DevicePath>(handle) {
				Ok(succ) => {
					let (_status, dp_prot) = succ.split();
					let dev_path = unsafe {&*dp_prot.get()};
					
					writeln!(stdout, "handle {:?}", unsafe {mem::transmute::<_, *const u8>(handle)}).unwrap();
					
					let mut node_ref = dev_path;
					loop {
						let (dev_type, subtype) = unsafe {
							(mem::transmute_copy::<_, RawDeviceType>(&node_ref.device_type),
							mem::transmute_copy::<_, RawDeviceSubTypeEnd>(&node_ref.sub_type))
						};
						let node_len = u16::from_le_bytes(node_ref.length);
						
						writeln!(stdout, "  {:?} {} len={}", node_ref.device_type, unsafe {mem::transmute::<_, u8>(subtype)}, node_len).unwrap();
						
						if let RawDeviceType::Media = dev_type {
							if subtype as u8 == 1 {
								let sig_type;
								let partn_sig;
								unsafe {
									sig_type = *node_ref.as_ptr().byte_offset(41).cast::<u8>();
									partn_sig = *node_ref.as_ptr().byte_offset(24).cast::<[u8; 16]>();
								}
								
								let boot_partn_guid_bytes = 0xA4A4A4A4_A4A4_A4A4_A4A4_A4A4A4A4A4A4u128.to_le_bytes();
								
								const SIG_TYPE_GUID_PARTN: u8 = 0x02;
								if sig_type == SIG_TYPE_GUID_PARTN && partn_sig == boot_partn_guid_bytes {
									boot_part_sfs_prot = Some(
										sys_table.boot_services()
											.handle_protocol::<SimpleFileSystem>(handle)
											.unwrap().unwrap()
									);
									
									writeln!(stdout, "  found boot partn, handle is the boot part handle").unwrap();
								}
							}
						}
						
						if let RawDeviceType::End = dev_type {
							break;
						} else {
							node_ref = unsafe {
								&*node_ref.as_ptr().byte_offset(node_len as isize)
							}
						}
					}
				}
				Err(_) => writeln!(stdout, "dev doesn't support device_handle protocol").unwrap(),
			}
		}
	}
	
//	// DEBUG:
//	writeln!(stdout, "got handles:").unwrap();
//	for h in &sfs_handles {
//		writeln!(stdout, "handle {:?}", unsafe {mem::transmute_copy::<_, usize>(h)}).unwrap();	
//	}
	
	let boot_part_sfs_prot = boot_part_sfs_prot
		.expect("Failed to find boot partn handle");
	
	// Load kernel
	let kernel_file = {
		let sfs_prot = unsafe {&mut *boot_part_sfs_prot.get()};
		let mut directory = sfs_prot.open_volume().unwrap().unwrap();
		
		// DEBUG: Print entries
		let mut buf = Box::new([0u8; 512]);
		directory.reset_entry_readout().unwrap().unwrap();
		
		while let Some(entry_info) = directory.read_entry(buf.as_mut_slice()).unwrap().unwrap() {
			writeln!(stdout, "file \"{}\"", entry_info.file_name()).unwrap();
		}
		
		// Load kernel.elf
		read_sfs_file(&mut directory, "kernel.elf").unwrap()
	};
	
	writeln!(stdout, "kernel.elf file size: {}", kernel_file.len()).unwrap();
	
	// Parse kernel elf
	let kernel_elf = Elf::from_bytes(&kernel_file).unwrap();
	let kernel_elf = if let Elf::Elf64(elf) = kernel_elf {
		elf
	} else {
		panic!("kernel.elf isn't a valid elf64");
	};
	
	writeln!(stdout).unwrap();
	writeln!(stdout, "kernel entry point {:08x}", kernel_elf.header().entry_point()).unwrap();
	
	// DEBUG: Try jumping to the kernel
	let entrypoint_addr = kernel_elf.header().entry_point();
	
	type KernelEntryFn = unsafe extern "C" fn() -> u32;
	let magic = unsafe {mem::transmute::<_, KernelEntryFn>(kernel_file.as_ptr().byte_offset(0x1b8710 as isize))()};
	
	writeln!(stdout, "Called kernel, got magic {}", magic).unwrap();
	
//	let mut magic: u32;
//	unsafe {
//		asm!(
//			"jmp {0}"
//			in(reg) entrypoint_addr,
//			out("ax") magic,
//		);
//	}
	
	// Return status to firmware
	uefi::Status::SUCCESS
}

fn read_sfs_file(root: &mut Directory, path: &str) -> Result<Vec<u8>, ()> {
	let file_handle = root.open(path, FileMode::Read, FileAttribute::from_bits(0).unwrap())
		.expect("Failed to open file").unwrap();
	
	if let FileType::Regular(mut file) = file_handle.into_type().unwrap().unwrap() {
		// Query size and alloc buffer
//		let (_, size) = file.read(&mut []).unwrap().split();
		file.set_position(RegularFile::END_OF_FILE).unwrap().unwrap();
		let size = file.get_position().unwrap().unwrap() as usize;
		
//		let buf_ptr = alloc::alloc::Global::default().allocate(core::alloc::Layout::from_size_align(size, 1).unwrap()).unwrap();
//		let mut data = Vec::from(unsafe {Box::from_raw(buf_ptr.as_ptr())});
		let mut data = Vec::with_capacity(size);
		unsafe {data.set_len(size)};
		
		// Read file
		file.set_position(0).unwrap().unwrap();
		match file.read(&mut data).map_err(|_| ())?.split() {
			(s, _) if s.is_success() => Ok(()),
			(_, _) => Err(()),
		}?;
		
		Ok(data)
	} else {
		Err(())
	}
}

#[panic_handler]
fn panic_handler(info: &core::panic::PanicInfo) -> ! {
	// Try load static boot_services pointer
	let maybe_boot_srvc = unsafe {
		STATIC_BOOT_SERVICES_PTR.load(SeqCst)
			.as_ref()
	};
	
	// DEBUG: Print panic info
	let stdout = unsafe {(TEST_STDOUT_PTR.load(SeqCst) as *mut uefip::Output).as_mut()};
	
	if let Some(stdout) = stdout {
		let _ = stdout.set_color(uefip::Color::LightRed, uefip::Color::Black);
		let _ = write!(stdout, "{}", info);
	} else {
		// Well, shit...
	}
	
	if let Some(boot_srvc) = maybe_boot_srvc {
		type FnEfiExit = extern "efiapi" fn(image_handle: uefip::Handle, exit_status: uefip::Status, exit_data_size: usize, exit_data: *const uefip::Char16);
		
		let exit_fn_offset = 
			mem::size_of::<uefi::table::Header>()
			+ 24 * mem::size_of::<usize>();
		let _exit_fn_ptr: FnEfiExit = unsafe {
			mem::transmute((boot_srvc as *const _ as *const u8)
				.offset(exit_fn_offset as isize))
		};
		
		// exit_fn_ptr()
		
		loop {}
	}
	else {
		// We didn't have a valid boot_services pointer (that's a bug!)
		// so just loop forever and chill
		loop {}
	}
}

#[alloc_error_handler]
fn alloc_error_handler(layout: core::alloc::Layout) -> ! {
    panic!("Memory allocation failed: {:?}", layout)
}

trait AsPtr {
	fn as_ptr(&self) -> *const Self;
}
trait AsPtrMut {
	fn as_ptr_mut(&mut self) -> *mut Self;
}
impl<T> AsPtr for T {
	fn as_ptr(&self) -> *const Self {
		self as *const Self
	}
}
impl<T> AsPtrMut for T {
	fn as_ptr_mut(&mut self) -> *mut Self {
		self as *mut Self
	}
}

trait PtrOpsExt {
	unsafe fn byte_offset(self, offset: isize) -> Self;
}
impl<T> PtrOpsExt for *const T {
	unsafe fn byte_offset(self, offset: isize) -> Self {
		self.cast::<u8>()
			.offset(offset)
			.cast()
	}
}
impl<T> PtrOpsExt for *mut T {
	unsafe fn byte_offset(self, offset: isize) -> Self {
		self.cast::<u8>()
			.offset(offset)
			.cast()
	}
}

#[repr(u8)]
#[derive(Debug, Copy, Clone)]
pub enum RawDeviceType {
	Hardware = 0x01,
	ACPI = 0x02,
	Messaging = 0x03,
	Media = 0x04,
	BIOSBootSpec = 0x05,
	End = 0x7F,
}

#[repr(u8)]
#[derive(Debug, Copy, Clone)]
pub enum RawDeviceSubTypeEnd {
	EndInstance = 0x01,
	EndEntire = 0xFF,
}

//pub struct DevPathChain {
//	path_ptr: *const DevPath,
//}
//
//pub struct DevPathIter {
//	
//}
//impl Iterator for DevPathIter {
//	type Item = RawDevPathNode;
//	
//	fn next(&mut self) -> Option<Self::Item> {
//		unimplemented!()
//	}
//}
//
//pub struct RawDevPathNode {
//	
//}
