#![feature(try_blocks)]
//#![feature(const_generics)]
#![feature(bool_to_option)]

use std::fs::{File, OpenOptions};
use std::io::{self, Seek, SeekFrom};
use std::path::{Path, PathBuf};
use std::str::FromStr;

use clap::Arg;
use fatfs::{FatType, FsOptions, ReadWriteSeek};
use widestring::{U16Str, U16String};

use crate::gpt::{CreatePartitionOptions, GptDisk, GptPartitionAttribs, Guid};
use crate::memdisk::MemDisk;

pub mod gpt;
pub mod mbr;
pub mod memdisk;

const NELL_BOOTSTASH_PARTITION_GPT_TYPE_GUID: Guid = Guid::from_u128(0x77ffd558_c91d_42e0_b03d_7f1efd959111);

fn main() {
	let matches = clap::App::new("makediskimg")
		.arg(Arg::with_name("bootloaderefi").long("bootloaderefi").takes_value(true))
		.arg(Arg::with_name("kernelelf").long("kernelelf").takes_value(true))
		.get_matches();
	
	let bootloader_efi_path = PathBuf::from_str(matches.value_of("bootloaderefi")
		.unwrap_or("../../bootloader_uefi/target/x86_64-unknown-uefi/debug/bootloader_uefi.efi")).unwrap();
	
	let kernel_path = PathBuf::from_str(matches.value_of("kernelelf").unwrap()).unwrap();
	
//	// DEBUG:
//	simple_logger::SimpleLogger::new()
//		.with_level(log::LevelFilter::Trace)
//		.init().unwrap();
	
	// Partitions:
	// efi_system
	// nell_boot
	// nell_reserve
	// nell_system
	// nell_user
	
	let gpt_block_size = 512;
//	let disk_size_lba = 33_548_800 * 2 / block_size;
	let disk_size_lba = (33_548_800 * 4) / gpt_block_size;
	
	let img_path = PathBuf::from("build/boot.img");
	
	// Create gpt disk
	let mut gpt_disk = GptDisk::new_empty(gpt_block_size as u32, disk_size_lba as u64, None);
	
	// Build efi system parition
	let mut efi_partition = build_efi_partition(&bootloader_efi_path);
	
	gpt_disk.create_partition(CreatePartitionOptions::new(
		gpt::partition_types::EFI_SYSTEM,
		None,
		(efi_partition.memdisk.size() / gpt_block_size) as u64,
		GptPartitionAttribs::zero(),
		utf16_to_array_nul(&U16String::from_str("UEFI System"))
	));
	
	// Build nell bootstash partition
	let mut bootstash_partition = build_bootstash_partition(&kernel_path);
	
	gpt_disk.create_partition(CreatePartitionOptions::new(
		NELL_BOOTSTASH_PARTITION_GPT_TYPE_GUID,
//		Guid::from_u128(0xEBD0A0A2_B9E5_4433_87C0_68B6B72699C7),
//		gpt::partition_types::EFI_SYSTEM,
		Some(Guid::from_u128(0xA4A4A4A4_A4A4_A4A4_A4A4_A4A4A4A4A4A4)),
		(bootstash_partition.memdisk.size() / gpt_block_size) as u64,
		GptPartitionAttribs::zero(),
		utf16_to_array_nul(&U16String::from_str("Nell Boot"))
	));
	
	// Update header crc
	// TODO: THIS IS SUPER STUPID, FIX THIS API, I JUST SPENT AN HOUR DEBUGGING THIS, THE CRC SHOULD NEVER BE ABLE TO BE OUT OF DATE!!
	gpt_disk.update_crc();
	
	let img_file = OpenOptions::new()
		.create(true).write(true).read(true).truncate(true)
		.open(&img_path)
		.unwrap();
	
	let mut writer = gpt_disk.writer(img_file);
	writer.write_protective_mbr().unwrap();
	writer.write_gpt_header(true).unwrap();
	writer.write_gpt_header(false).unwrap();
	
	// Write partition contents
	efi_partition.memdisk.seek(SeekFrom::Start(0)).unwrap();
	writer.write_partition_content(gpt_disk.partitions().nth(0).unwrap(), &mut efi_partition.memdisk)
		.unwrap();
	
	bootstash_partition.memdisk.seek(SeekFrom::Start(0)).unwrap();
	writer.write_partition_content(gpt_disk.partitions().nth(1).unwrap(), &mut bootstash_partition.memdisk)
		.unwrap();
	
	writer.flush().sync_all().unwrap();
	
	/*
	// DEBUG:
	{
		let disk = ::gpt::disk::read_disk(&img_path).unwrap();
		for (k, v) in disk.partitions().iter() {
			println!("Part#{:02}:\n{}", k, v);
			
			let start = v.bytes_start(::gpt::disk::LogicalBlockSize::Lb512).unwrap();
			let len = v.bytes_len(::gpt::disk::LogicalBlockSize::Lb512).unwrap();
			
			let disk2 = File::open(&img_path).unwrap();
			let vfs_slice = fscommon::StreamSlice::new(disk2, start, start+len).unwrap();
			
			let vfs = fatfs::FileSystem::new(vfs_slice, FsOptions::new()).unwrap();
			for f in vfs.root_dir().iter() {
				println!("file \"{}\"", f.unwrap().file_name());
			}
		}
	}
	*/
}

fn build_efi_partition(booloader_efi_path: &Path) -> BuiltPartition {
	let vfs_size_bytes = 33_548_800 + 1032*512;
	// Create mem disk buffer
	let mut vfs_buf = MemDisk::new_fixed_size(vfs_size_bytes);
	
	let vfs_sector_size = 512;
	
	let format_opts = fatfs::FormatVolumeOptions::new()
		.fat_type(FatType::Fat32)
		.bytes_per_sector(vfs_sector_size as u16)
		.total_sectors((vfs_size_bytes / vfs_sector_size) as u32)
		.bytes_per_cluster(vfs_sector_size as u32);
	
	fatfs::format_volume(&mut vfs_buf, format_opts)
		.unwrap();
	
	vfs_buf.seek(SeekFrom::Start(0)).unwrap();
	let mut vfs = fatfs::FileSystem::new(&mut vfs_buf, FsOptions::new()).unwrap();
	
	{// Populate fs
		// "../../bootloader_uefi/target/x86_64-unknown-uefi/debug/bootloader_uefi.efi"
		copy_to_vfs(&mut vfs, booloader_efi_path, "/efi/boot/nell_foo/nellbootx64.efi").unwrap(); // Note that the nell folder can be custom named to allow multiple installations (all using the same efi system partition, as it should)
		copy_to_vfs(&mut vfs, booloader_efi_path, "/efi/boot/bootx64.efi").unwrap(); // Needed for automatic boot instead of getting dumped into the uefi shell
		copy_to_vfs(&mut vfs, booloader_efi_path, "/nellbootx64.efi").unwrap();
	}
	vfs.unmount().unwrap();
	
	BuiltPartition {
		memdisk: vfs_buf,
	}
}

fn build_bootstash_partition(kernel_path: &Path) -> BuiltPartition {
	let vfs_size_bytes = 33_548_800 + 1032*512;
	// Create mem disk buffer
	let mut vfs_buf = MemDisk::new_fixed_size(vfs_size_bytes);
	
	let vfs_sector_size = 512;
	
	let format_opts = fatfs::FormatVolumeOptions::new()
		.fat_type(FatType::Fat32)
		.bytes_per_sector(vfs_sector_size as u16)
		.total_sectors((vfs_size_bytes / vfs_sector_size) as u32)
		.bytes_per_cluster(vfs_sector_size as u32);
	
	fatfs::format_volume(&mut vfs_buf, format_opts)
		.unwrap();
	
	vfs_buf.seek(SeekFrom::Start(0)).unwrap();
	let mut vfs = fatfs::FileSystem::new(&mut vfs_buf, FsOptions::new()).unwrap();
	
	{// Populate fs
		copy_to_vfs(&mut vfs, kernel_path, "/kernel.elf").unwrap();
	}
//	vfs.root_dir().create_file("test.txt").unwrap();
	vfs.unmount().unwrap();
	
	BuiltPartition {
		memdisk: vfs_buf,
	}
}

struct BuiltPartition {
	memdisk: MemDisk,
}

fn copy_to_vfs<T: ReadWriteSeek>(fs: &mut fatfs::FileSystem<T>, src_path: impl AsRef<Path>, vfs_path: &str) -> io::Result<()> {
	let mut src_file = File::open(src_path.as_ref())?;
	
	let (target_dir, file_name) = {
		let segs = vfs_path.split('/').collect::<Vec<_>>();
		
		let mut prev = fs.root_dir();
		for (i, s) in segs.iter().copied().enumerate() {
			if (i > 0 || !s.is_empty()) && i < segs.len()-1 {
				prev = prev.create_dir(s)?;
			}
		}
		(prev, *segs.last().unwrap())
	};
	
	// Create dirs
	let mut vfs_file = target_dir
		.create_file(file_name)?;
	
	io::copy(&mut src_file, &mut vfs_file)?;
	Ok(())
}

//fn utf16_to_array_nul<const N: usize>(ustr: &U16Str) -> [u16; N] {
fn utf16_to_array_nul(ustr: &U16Str) -> [u16; 36] {
	let mut buf = [0u16; 36];
	for (i, c) in ustr.as_slice().iter().copied().take(36usize.saturating_sub(1)).enumerate() {
		buf[i] = c;
	}
	buf[buf.len()-1] = b'\0' as _;
	buf
}

//trait U16ToArray {
//	fn to_array_nul<const N: usize>(&self) -> Option<[u16; N]>;
//}
//impl U16ToArray for U16CString {
//	fn to_array_nul<const N: usize>(&self) -> Option<[u16; N]> {
//		
//		None
//	}
//}
//
