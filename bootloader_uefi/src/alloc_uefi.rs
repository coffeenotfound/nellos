
use core::alloc::{GlobalAlloc, Layout};
use core::mem;
use core::ptr;
use core::ffi;

use atomic::{Atomic, Ordering};
use atomic::Ordering::*;
use uefi::table::boot::{BootServices, MemoryType};
use crate::alloc_uefi::table::RawBootServicesTable;

type AllocPoolFn = extern "efiapi" fn(pool_type: MemoryType, size: usize, buffer: *mut *mut u8) -> uefi::Status;
type FreePoolFn = extern "efiapi" fn(buffer: *mut u8) -> uefi::Status;

pub struct GlobalAllocBootUefi {
	alloc_pool_fn: Atomic<usize>,
	free_pool_fn: Atomic<usize>,
}

impl GlobalAllocBootUefi {
	pub const fn new_uninit() -> Self {
		Self {
			alloc_pool_fn: Atomic::new(0x0), // Should be `ptr::null_mut() as _` but that doesn't work here for dumb reasons
			free_pool_fn: Atomic::new(0x0),
		}
	}
	
	pub unsafe fn init_boot_alloc(&self, boot_services: &BootServices) {
		self.alloc_pool_fn.store(mem::transmute::<_, &RawBootServicesTable>(boot_services).allocate_pool as usize, SeqCst);
		self.free_pool_fn.store(mem::transmute::<_, &RawBootServicesTable>(boot_services).free_pool as usize, SeqCst);
//		self.alloc_pool_fn.store(get_boot_service_fn_ptr(boot_services, 5) as _, SeqCst);
//		self.free_pool_fn.store(get_boot_service_fn_ptr(boot_services, 6) as _, SeqCst);
	}
	
	pub unsafe fn exit_boot_alloc(&self) {
		self.alloc_pool_fn.store(ptr::null::<usize>() as _, SeqCst);
		self.free_pool_fn.store(ptr::null::<usize>() as _, SeqCst);
	}
}

unsafe impl GlobalAlloc for GlobalAllocBootUefi {
	unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
		let p = self.alloc_pool_fn.load(Ordering::SeqCst) as *const ffi::c_void;
		if !p.is_null() {
			let mut buf_ptr = ptr::null_mut::<u8>();
			if (mem::transmute::<_, AllocPoolFn>(p))(MemoryType::LOADER_DATA, layout.size(), &mut buf_ptr).is_success() {
				return buf_ptr;
			}
		}
		ptr::null_mut()
	}
	
	unsafe fn dealloc(&self, ptr: *mut u8, _layout: Layout) {
		let p = self.free_pool_fn.load(Ordering::SeqCst) as *const ffi::c_void;
		if !p.is_null() {
			let _ = (mem::transmute::<_, FreePoolFn>(p))(ptr);
		}
	}
}

pub unsafe fn get_boot_service_fn_ptr(boot_services: &BootServices, fn_idx: usize) -> *const ffi::c_void {
	let byte_offset = mem::size_of::<uefi::table::Header>() + fn_idx * mem::size_of::<usize>();
	
	mem::transmute((boot_services as *const _ as *const ffi::c_void)
		.offset(byte_offset as isize))
}

mod table {
	use uefi::table::Header;
	use uefi::table::boot::{Tpl, MemoryType, MemoryDescriptor, MemoryMapKey, EventType};
	use uefi::{Status, Event, Handle, Guid};
	use core::ffi::c_void;
	use uefi::proto::loaded_image::DevicePath;
	
	#[repr(C)]
	pub struct RawBootServicesTable {
		pub header: Header,
		
		// Task Priority services
		pub raise_tpl: unsafe extern "efiapi" fn(new_tpl: Tpl) -> Tpl,
		pub restore_tpl: unsafe extern "efiapi" fn(old_tpl: Tpl),
		
		// Memory allocation functions
		pub allocate_pages: extern "efiapi" fn(
			alloc_ty: u32,
			mem_ty: MemoryType,
			count: usize,
			addr: &mut u64,
		) -> Status,
		pub free_pages: extern "efiapi" fn(addr: u64, pages: usize) -> Status,
		pub get_memory_map: unsafe extern "efiapi" fn(
			size: &mut usize,
			map: *mut MemoryDescriptor,
			key: &mut MemoryMapKey,
			desc_size: &mut usize,
			desc_version: &mut u32,
		) -> Status,
		pub allocate_pool: extern "efiapi" fn(pool_type: MemoryType, size: usize, buffer: &mut *mut u8) -> Status,
		pub free_pool: extern "efiapi" fn(buffer: *mut u8) -> Status,
		
		// Event & timer functions
		pub create_event: unsafe extern "efiapi" fn(
			ty: EventType,
			notify_tpl: Tpl,
			notify_func: Option<EventNotifyFn>,
			notify_ctx: *mut c_void,
			event: *mut Event,
		) -> Status,
		pub set_timer: unsafe extern "efiapi" fn(event: Event, ty: u32, trigger_time: u64) -> Status,
		pub wait_for_event: unsafe extern "efiapi" fn(
			number_of_events: usize,
			events: *mut Event,
			out_index: *mut usize,
		) -> Status,
		pub signal_event: usize,
		pub close_event: usize,
		pub check_event: usize,
		
		// Protocol handlers
		pub install_protocol_interface: usize,
		pub reinstall_protocol_interface: usize,
		pub uninstall_protocol_interface: usize,
		pub handle_protocol: extern "efiapi" fn(handle: Handle, proto: &Guid, out_proto: &mut *mut c_void) -> Status,
		pub _reserved: usize,
		pub register_protocol_notify: usize,
		pub locate_handle: unsafe extern "efiapi" fn(
			search_ty: i32,
			proto: *const Guid,
			key: *mut c_void,
			buf_sz: &mut usize,
			buf: *mut Handle,
		) -> Status,
		pub locate_device_path: unsafe extern "efiapi" fn(
			proto: &Guid,
			device_path: &mut *mut DevicePath,
			out_handle: *mut Handle,
		) -> Status,
		pub install_configuration_table: usize,
		
		// Image services
		pub load_image: usize,
		pub start_image: usize,
		pub exit: usize,
		pub unload_image: usize,
		pub exit_boot_services: unsafe extern "efiapi" fn(image_handle: Handle, map_key: MemoryMapKey) -> Status,
		
		// Misc services
		pub get_next_monotonic_count: usize,
		pub stall: extern "efiapi" fn(microseconds: usize) -> Status,
		pub set_watchdog_timer: unsafe extern "efiapi" fn(
			timeout: usize,
			watchdog_code: u64,
			data_size: usize,
			watchdog_data: *const u16,
		) -> Status,
		
		// Driver support services
		pub connect_controller: usize,
		pub disconnect_controller: usize,
		
		// Protocol open / close services
		pub open_protocol: usize,
		pub close_protocol: usize,
		pub open_protocol_information: usize,
		
		// Library services
		pub protocols_per_handle: usize,
		pub locate_handle_buffer: usize,
		pub locate_protocol: extern "efiapi" fn(
			proto: &Guid,
			registration: *mut c_void,
			out_proto: &mut *mut c_void,
		) -> Status,
		pub install_multiple_protocol_interfaces: usize,
		pub uninstall_multiple_protocol_interfaces: usize,
		
		// CRC services
		pub calculate_crc32: usize,
		
		// Misc services
		pub copy_mem: unsafe extern "efiapi" fn(dest: *mut u8, src: *const u8, len: usize),
		pub set_mem: unsafe extern "efiapi" fn(buffer: *mut u8, len: usize, value: u8),
		
		// New event functions (UEFI 2.0 or newer)
		pub create_event_ex: usize,
	}
	
	pub type EventNotifyFn = unsafe extern "efiapi" fn(event: Event, context: *mut c_void);
}
