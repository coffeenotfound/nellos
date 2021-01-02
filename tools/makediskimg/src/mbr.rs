use std::mem;
use std::intrinsics::transmute;

pub type MbrOsType = u8;

pub mod os_types {
	use super::MbrOsType;
	
	pub const UEFI_SYSTEM: MbrOsType = 0xEF;
	pub const GPT_PROTECTIVE: MbrOsType = 0xEE;
}

pub const PROTECTIVE_MBR_SIGNATURE: u16 = 0xAA55;

#[repr(C)]
pub struct MasterBootRecord {
	pub bootstrap_code: [u8; 440],
	pub unique_mbr_signature: [u8; 4],
	pub unknown: [u8; 2],
	pub partitions: [MbrPartitionEntry; 4],
	pub signature: u16
}

#[repr(C)]
pub struct MbrPartitionEntry {
	pub boot_indicator: u8,
	pub start_head: u8,
	pub start_sector: u8,
	pub start_track: u8,
//	pub start_chs: [u8; 3],
	pub os_type: u8,
	pub end_head: u8,
	pub end_sector: u8,
	pub end_track: u8,
//	pub end_chs: [u8; 3],
	pub starting_lba: [u8; 4], // Not an u32 because the MasterBootRecord struct needs this struct to NOT be u32 aligned
	pub size_in_lba: [u8; 4]
}

pub fn make_gpt_protective_mbr(_clamped_max_chs: u32) -> MasterBootRecord {
	MasterBootRecord {
		bootstrap_code: [0; 440],
		unique_mbr_signature: [0; 4],
		unknown: [0; 2],
		partitions: [
			make_gpt_protective_partition_entry(_clamped_max_chs),
			unsafe {transmute([0u8; mem::size_of::<MbrPartitionEntry>()])},
			unsafe {transmute([0u8; mem::size_of::<MbrPartitionEntry>()])},
			unsafe {transmute([0u8; mem::size_of::<MbrPartitionEntry>()])}
		],
		signature: PROTECTIVE_MBR_SIGNATURE
	}
}

pub fn make_gpt_protective_partition_entry(_clamped_max_chs: u32) -> MbrPartitionEntry {
	MbrPartitionEntry {
		boot_indicator: 0,
		start_head: 0x00,
		start_sector: 0x02,
		start_track: 0x00,
		os_type: os_types::GPT_PROTECTIVE,
		end_head: 0xFF, // TODO: Properly handle end chs
		end_sector: 0xFF,
		end_track: 0xFF,
		starting_lba: [0x01, 0x00, 0x00, 0x00],
		size_in_lba: [0xFF, 0xFF, 0xFF, 0xFF] // TODO: Properly handle size in lba
	}
}

/// Converts the given LBA to a CHS address clamped to 0xFFFFFF.
#[deprecated]
pub fn lba_to_chs_clamped(_lba: u32) -> u32 {
	// TODO: This is stupid because we would actually need the god damn head and cylinder counts from the actual drive (which also literally don't exist) to calculate the chs
	// TODO: Just put 0xFFFFFF as the end chs of the protective partition entry and be done with it (eventhough *in theory* that's not completely correct as the chs must not lay outside the disk as per uefi spec)
	// TODO: (That won't happen anyway because we don't use 10 MB drives anymore so the chs will literally always be clamped anyway)
	0
}

/// A cylinder-head-sector address.
pub struct Chs {
	
}

impl Chs {
	pub fn max() {
		
	}
}

/// Geometry of a [cylinder-head-sector](self::Chs) addressed disk.
/// Represents the number
pub struct ChsGeo {
	
}

impl ChsGeo {
	
}