use std::{cmp, error, mem};
use std::fs::File;
use std::hash::Hasher;
use std::io::{Read, Seek, SeekFrom, Write};

use byteorder::{LE, WriteBytesExt};
use crc::{crc32, Hasher32};

use crate::mbr::{self, MasterBootRecord};

pub const DEFAULT_BLOCK_SIZE: usize = 512;

pub type Guid = uuid::Uuid;
pub type Utf16LEChar = u16;

pub mod partition_types {
	use super::Guid;
	
	pub const UNUSED: Guid = Guid::from_bytes([0 as u8; 16]);
	pub const EFI_SYSTEM: Guid = Guid::from_u128(0xC12A7328_F81F_11D2_BA4B_00A0C93EC93B);
}

// DEBUG:
mod tests {
	#[test]
	pub fn uuid_from_u128_endian() {
		assert_eq!(
			super::partition_types::EFI_SYSTEM,
			uuid::Uuid::parse_str("C12A7328-F81F-11D2-BA4B-00A0C93EC93B").unwrap()			
		);
	}
	
	#[test]
	pub fn uuid_to_guid_mixed_endian() {
		assert_eq!(
			super::uuid_to_guid_mixed_endian(super::partition_types::EFI_SYSTEM),
			u128::from_ne_bytes(*b"\x28\x73\x2A\xC1\x1F\xF8\xD2\x11\xBA\x4B\x00\xA0\xC9\x3E\xC9\x3B")
		);
	}
}

pub struct GptDisk {
	block_size: u32,
	disk_size_lba: u64,
	partitions: Vec<GptPartition>,
	primary_header: GptHeader,
	backup_header: GptHeader,
}

impl GptDisk {
	/// Create a new, empty disk
	pub fn new_empty(block_size: u32, disk_size_lba: u64, disk_guid: Option<Guid>) -> GptDisk {
		let real_disk_guid = disk_guid
			.unwrap_or_else(|| Guid::new_v4());
		
		let primary_header_lba = 1;
		let backup_header_lba = disk_size_lba - 1;
		
		// Make primary header
		let mut primary_header = GptHeader {
			signature: 0x5452415020494645,
			revision: 0x00010000,
			header_size: 92,
			header_crc32: 0,
			my_lba: primary_header_lba,
			alternate_lba: backup_header_lba,
			first_usable_lba: 34, // DEBUG: Temp
			last_usable_lba: disk_size_lba - 1 - 33,
			disk_guid: real_disk_guid,
			partition_array_start_lba: 2,
			num_partition_entries: 0,
			partition_entry_size: 128,
			partition_array_crc32: 0,
		};
		primary_header.update_header_crc32();
		
		// Make backup header
		let mut backup_header = primary_header.clone();
		backup_header.my_lba = backup_header_lba;
		backup_header.alternate_lba = primary_header_lba;
		backup_header.partition_array_start_lba = disk_size_lba - 33;
		backup_header.update_header_crc32();
		
		// Make disk object
		GptDisk {
			block_size,
			disk_size_lba,
			primary_header,
			backup_header,
			partitions: Vec::new(),
		}
	}
	
	pub fn create_partition(&mut self, options: CreatePartitionOptions) -> Option<&mut GptPartition> {
		// Find block position
		// DEBUG: For now always start at the first usable lba or the end of the prev partition
//		let start_lba = self.primary_header.first_usable_lba;
		let part_padding = 1; // DEBUG: Test padding in lba between partitions, remove later
		let start_lba = self.partitions
			.last().map_or(self.primary_header.first_usable_lba, |p| p.end_lba_incl + 1 + part_padding);
		let end_lba_incl = start_lba + options.size_in_lba.saturating_sub(1); // TODO: Right now a partition will always have the size of atleast one lba, even if requested size was 0... But what should we do in that case anyways?
		
		// Create partition
		let mut partition = GptPartition::new(
			options.partition_type,
			options.unique_guid,
			options.attributes,
			start_lba,
			end_lba_incl,
		);
		
		partition.partition_name = options.partition_name;
		
		// Add partition to list
		self.partitions.push(partition);
		
		// Update headers
		let num_parts = self.partitions.len() as u32;
		self.primary_header.num_partition_entries = num_parts;
		self.backup_header.num_partition_entries = num_parts;
		
//		// Set layout
//		let layout = GptPartitionLayout {
//			start_lba,
//			end_lba,
//		};
//		partition.disk_layout = layout;
//		
//		// Add partition to list
//		self.partitions.push(partition);
//		
//		// Update partition count
//		let num_parts = self.partitions.len() as u32;
//		self.primary_header.num_partition_entries = num_parts;
//		self.backup_header.num_partition_entries = num_parts;
		
		None
	}
	
	// TODO:
	#[deprecated]
	pub fn check_integrity(&self) {}
	
	pub fn partitions<'a>(&'a self) -> PartitionIter<'a> {
		PartitionIter {
			disk: self,
			front: 0,
		}
	}
	
	pub fn disk_size_lba(&self) -> u64 {
		self.disk_size_lba
	}
	
	pub fn block_size(&self) -> u32 {
		self.block_size
	}
	
	pub fn primary_header(&self) -> &GptHeader {
		&self.primary_header
	}
	
	pub fn backup_header(&self) -> &GptHeader {
		&self.backup_header
	}
	
	pub fn update_crc(&mut self) {
		// Update partition table crcs
		// (Parition table crcs must be updated first,
		//  because they are fed into the gpt header crc!)
		self.primary_header.update_partition_array_crc32(&self.partitions, self.primary_header.partition_entry_size);
		self.backup_header.update_partition_array_crc32(&self.partitions, self.backup_header.partition_entry_size);
		
		// Update header crcs
		self.primary_header.update_header_crc32();
		self.backup_header.update_header_crc32();
	}
	
	pub fn writer<'a>(&'a self, file: File) -> GptDiskWriter<'a> {
		GptDiskWriter {
			disk: self,
			file,
			initialized: false,
		}
	}
}

pub struct CreatePartitionOptions {
	partition_type: Guid,
	unique_guid: Guid,
	size_in_lba: u64,
	attributes: GptPartitionAttribs,
	partition_name: [Utf16LEChar; 36],
}

impl CreatePartitionOptions {
	pub fn new(partition_type: Guid, unique_guid: Option<Guid>, size_in_lba: u64, attributes: GptPartitionAttribs, name: [Utf16LEChar; 36]) -> Self {
		CreatePartitionOptions {
			partition_type,
			unique_guid: unique_guid
				.unwrap_or_else(|| Guid::new_v4()),
			size_in_lba,
			attributes,
			partition_name: name,
		}
	}
}

pub struct PartitionIter<'a> {
	disk: &'a GptDisk,
	front: usize,
}

impl<'a> Iterator for PartitionIter<'a> {
	type Item = &'a GptPartition;
	
	fn next(&mut self) -> Option<Self::Item> {
		if let Some(a) = self.disk.partitions.get(self.front) {
			self.front += 1;
			Some(a)
		} else {
			None
		}
	}
}

//pub struct PartitionBuilder<'a> {
//	disk: &'a mut GptDisk,
//}

pub struct GptDiskWriter<'a> {
	disk: &'a GptDisk,
	file: File,
	initialized: bool,
}

impl<'a> GptDiskWriter<'a> {
	fn ensure_init(&mut self) -> Result<(), Box<dyn error::Error>> {
		if !self.initialized {
			self.initialized = true;
			
			// Resize file
			self.file.set_len(self.disk.disk_size_lba * self.disk.block_size as u64)?;
		}
		
		Ok(())
	}
	
	pub fn write_protective_mbr(&mut self) -> Result<(), Box<dyn error::Error>> {
		// Init
		self.ensure_init()?;
		let file = &mut self.file;
		
		file.seek(SeekFrom::Start(0))?;
		
		// Make protective mbr struct
		let raw_mbr = mbr::make_gpt_protective_mbr(0xFFFFFF);
		
		// Cast to slice
		let mbr_slice = unsafe {mem::transmute::<&MasterBootRecord, &[u8; mem::size_of::<MasterBootRecord>()]>(&raw_mbr)};
		
		// Write to file
		file.write_all(mbr_slice)?;
		
		Ok(())
	}
	
	pub fn write_gpt_header(&mut self, primary: bool) -> Result<(), Box<dyn error::Error>> {
		// Init
		self.ensure_init()?;
		let file = &mut self.file;
		
		let block_size = self.disk.block_size;
		
		{// Serialize header
			let (h, pos_lba) = match primary {
				true => (&self.disk.primary_header, 1),
				false => (&self.disk.backup_header, (self.disk.disk_size_lba - 1)),
			};
			
			// Seek to header pos
			file.seek(SeekFrom::Start(pos_lba * block_size as u64))?;
			
			// Serialize
			file.write_u64::<LE>(h.signature)?;
			file.write_u32::<LE>(h.revision)?;
			file.write_u32::<LE>(h.header_size)?;
			file.write_u32::<LE>(h.header_crc32)?;
			file.write_u32::<LE>(0)?;
			file.write_u64::<LE>(h.my_lba)?;
			file.write_u64::<LE>(h.alternate_lba)?;
			file.write_u64::<LE>(h.first_usable_lba)?;
			file.write_u64::<LE>(h.last_usable_lba)?;
			file.write_u128::<LE>(uuid_to_guid_mixed_endian(h.disk_guid))?;
			file.write_u64::<LE>(h.partition_array_start_lba)?;
			file.write_u32::<LE>(h.num_partition_entries)?;
			file.write_u32::<LE>(h.partition_entry_size)?;
			file.write_u32::<LE>(h.partition_array_crc32)?;
		}
		
		// Serialize primary partition array
		{
			let (h, start_lba) = match primary {
				true => (&self.disk.primary_header, self.disk.primary_header.partition_array_start_lba),
				false => (&self.disk.backup_header, self.disk.backup_header.partition_array_start_lba),
			};
			
			for (i, part) in self.disk.partitions().enumerate() {
				// Seek to entry pos
				file.seek(SeekFrom::Start(start_lba * block_size as u64 + (i as u64 * h.partition_entry_size as u64)))?;
				
				file.write_u128::<LE>(uuid_to_guid_mixed_endian(part.partition_type_guid))?;
				file.write_u128::<LE>(uuid_to_guid_mixed_endian(part.unique_guid))?;
//				file.write_all(part.partition_type_guid.as_bytes())?;
//				file.write_all(part.unique_guid.as_bytes())?;
				file.write_u64::<LE>(part.start_lba)?;
				file.write_u64::<LE>(part.end_lba_incl)?;
				file.write_u64::<LE>(*part.attributes.as_raw())?;
				file.write_all(unsafe {mem::transmute::<&[u16; 36], &[u8; 72]>(&part.partition_name)})?;
			}
		}
		
		Ok(())
	}
	
	pub fn write_partition_content(&mut self, partition: &GptPartition, stream: &mut dyn Read) -> Result<(), Box<dyn error::Error>> {
		// Init
		self.ensure_init()?;
		
		let mut read_buffer = [0 as u8; 128];
		
		// Seek in disk file
		let content_start = partition.start_lba * self.disk.block_size as u64;
		self.file.seek(SeekFrom::Start(content_start))?;
		
//		let layout = partition.disk_layout;
		let mut remaining_size = ((partition.end_lba_incl - partition.start_lba) + 1) * self.disk.block_size as u64;
		loop {
			let bytes_read = stream.read(&mut read_buffer)?;
			
			if bytes_read == 0 {
//				println!("[write_partition_content] Read zero bytes, breaking", );
				break;
			}
			
			let real_size = cmp::min(bytes_read as u64, remaining_size);
			
			// Write to file
			self.file.write_all(&read_buffer[0..real_size as usize])?;
			
			// Subtract read size
			remaining_size -= real_size;
		}
		
		Ok(())
	}
	
	pub fn flush(self) -> File {
		self.file
	}
}

#[derive(Clone)]
pub struct GptHeader {
	pub signature: u64,
	pub revision: u32,
	pub header_size: u32,
	pub header_crc32: u32,
	pub my_lba: u64,
	pub alternate_lba: u64,
	pub first_usable_lba: u64,
	pub last_usable_lba: u64,
	pub disk_guid: Guid,
	pub partition_array_start_lba: u64,
	pub num_partition_entries: u32,
	pub partition_entry_size: u32,
	pub partition_array_crc32: u32,
}

impl GptHeader {
	pub fn update_header_crc32(&mut self) {
		let mut digest = crc32::Digest::new(crc32::IEEE);
		
		digest.write_u64(self.signature);
		digest.write_u32(self.revision);
		digest.write_u32(self.header_size);
		digest.write_u32(0);
		digest.write_u32(0);
		digest.write_u64(self.my_lba);
		digest.write_u64(self.alternate_lba);
		digest.write_u64(self.first_usable_lba);
		digest.write_u64(self.last_usable_lba);
		digest.write_u128(uuid_to_guid_mixed_endian(self.disk_guid));
		digest.write_u64(self.partition_array_start_lba);
		digest.write_u32(self.num_partition_entries);
		digest.write_u32(self.partition_entry_size);
		digest.write_u32(self.partition_array_crc32);
		
		self.header_crc32 = digest.sum32();
	}
	
	pub fn update_partition_array_crc32(&mut self, partitions: &[GptPartition], partition_entry_size: u32) {
		let mut digest = crc32::Digest::new(crc32::IEEE);
		
		for p in partitions.iter() {
//			crc::crc32::Hasher32::write(&mut digest, p.partition_type_guid.as_bytes());
//			crc::crc32::Hasher32::write(&mut digest, p.unique_guid.as_bytes());
			digest.write_u128(uuid_to_guid_mixed_endian(p.partition_type_guid));
			digest.write_u128(uuid_to_guid_mixed_endian(p.unique_guid));
			digest.write_u64(p.start_lba);
			digest.write_u64(p.end_lba_incl);
			digest.write_u64(*p.attributes.as_raw());
			
			for c in p.partition_name.iter().copied() {
				digest.write_u16(c);
			}
			
			// Digest extra padding (if the entry size is bigger than 128 bit)
			for _ in 128..partition_entry_size {
				digest.write_u8(0);
			}
		}
		
		self.partition_array_crc32 = digest.sum32();
	}
}

pub struct GptPartition {
	pub partition_type_guid: Guid,
	pub unique_guid: Guid,
	pub size_in_lba: u64,
	pub attributes: GptPartitionAttribs,
	pub partition_name: [Utf16LEChar; 36],
	pub start_lba: u64,
	pub end_lba_incl: u64,
//	pub disk_layout: GptPartitionLayout,
}

impl GptPartition {
	pub(crate) fn new(partition_type_guid: Guid, unique_guid: Guid, attributes: GptPartitionAttribs, start_lba: u64, end_lba_incl: u64) -> GptPartition {
		GptPartition {
			partition_type_guid,
			unique_guid,
			size_in_lba: (end_lba_incl - start_lba) + 1,
			attributes,
			partition_name: [0x0; 36],
			start_lba,
			end_lba_incl,
		}
	}
	
	pub fn set_name_ascii(&mut self, name_acii: &[u8]) {
		let real_len = cmp::min(name_acii.len(), 36);
		
		let own_name = &mut self.partition_name;
		for i in 0..real_len {
			own_name[i] = name_acii[i] as u16;
		}
		for i in real_len..36 {
			own_name[i] = 0;
		}
	}
}

//#[deprecated]
//pub struct GptPartitionLayout {
//	pub start_lba: u64,
//	pub end_lba: u64,
//}

pub struct GptPartitionAttribs(u64);

impl GptPartitionAttribs {
	pub fn zero() -> Self {
		Self(0)
	}
	
	pub unsafe fn from_raw(raw_bits: u64) -> Self {
		Self(raw_bits)
	}
	
	pub fn as_raw(&self) -> &u64 {
		&self.0
	}
}

pub fn uuid_to_guid_mixed_endian(uuid: uuid::Uuid) -> u128 {
	let src = uuid.to_u128_le().to_ne_bytes();
	let mut buf = [0u8; 16];
	
	buf[0] = src[3];
	buf[1] = src[2];
	buf[2] = src[1];
	buf[3] = src[0];
	
	buf[4] = src[5];
	buf[5] = src[4];
	
	buf[6] = src[7];
	buf[7] = src[6];
	
	buf[6] = src[7];
	buf[7] = src[6];
	
	buf[8] = src[8];
	buf[9] = src[9];
	buf[10] = src[10];
	buf[11] = src[11];
	buf[12] = src[12];
	buf[13] = src[13];
	buf[14] = src[14];
	buf[15] = src[15];
	
//	let (a, b, c, d) = uuid.to_fields_le();
//	
//	(a.swap_bytes() as u128)
//	| ((b.swap_bytes() as u128) << 4)
//	| ((c.swap_bytes() as u128) << 6)
//	| ((u64::from_ne_bytes(*d) as u128) << 8)
	
	u128::from_le_bytes(buf)
}
