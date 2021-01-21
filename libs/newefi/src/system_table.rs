
pub struct SystemTable {
	
}

impl SystemTable {
//	pub fn get_memory_map() ->
}

#[doc(alias = "EFI_SYSTEM_TABLE")]
#[repr(C)]
pub struct RawSystemTable {
	pub header: RawTableHeader,
	pub firmware_vendor: *mut u16,
	
}

#[doc(alias = "EFI_TABLE_HEADER")]
#[repr(C)]
pub struct RawTableHeader {
	
}

#[doc(alias = "EFI_MEMORY_DESCRIPTOR")]
#[repr(C)]
pub struct RawMemoryDescriptor {
	pub desc_type: u32,
	pub physical_start: u64,
	pub virtual_start: u64,
	pub number_of_pages: u64,
	pub attributes: u64,
}
