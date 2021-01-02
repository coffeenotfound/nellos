use std::cmp;
use std::error;
use std::fmt;
use std::io::{self, SeekFrom};

pub struct MemDisk {
	data: Vec<u8>,
	cursor: usize,
}

impl MemDisk {
	pub fn new_fixed_size(size: usize) -> Self {
		// Allocate buffer
		let mut data = Vec::with_capacity(size);
		data.resize(data.capacity(), 0);
		
		// Make object
		MemDisk {
			data,
			cursor: 0,
		}
	}
	
	pub fn size(&self) -> usize {
		self.data.len()
	}
	
	pub fn cursor(&self) -> usize {
		self.cursor
	}
}

impl io::Read for MemDisk {
	fn read(&mut self, buf: &mut [u8]) -> Result<usize, io::Error> {
		let real_read_size = cmp::min(buf.len(), self.data.len() - self.cursor);
		
		// Copy data into buffer
		unsafe {
			std::ptr::copy_nonoverlapping(self.data.as_ptr().offset(self.cursor as isize), buf.as_mut_ptr(), real_read_size);
		}
		
		// Advance cursor
		self.cursor += real_read_size;
		
		Ok(real_read_size)
	}
}

impl io::Write for MemDisk {
	fn write(&mut self, buf: &[u8]) -> Result<usize, io::Error> {
		let real_write_size = cmp::min(buf.len(), self.data.len() - self.cursor);
		
		// Copy data into our buffer
		unsafe {
			std::ptr::copy_nonoverlapping(buf.as_ptr(), self.data.as_mut_ptr().offset(self.cursor as isize), real_write_size);
		}
		
		// Advance cursor
		self.cursor += real_write_size;
		
		Ok(real_write_size)
	}
	
	fn flush(&mut self) -> Result<(), io::Error> {
		// Do nothing
		Ok(())
	}
}

impl io::Seek for MemDisk {
	fn seek(&mut self, pos: SeekFrom) -> Result<u64, io::Error> {
		// Calc new cursor pos
		let mut new_cursor_pos = match pos {
			SeekFrom::Start(n) => n as usize,
			SeekFrom::Current(n) => {
				if n < 0 && (-n as usize) > self.cursor {
					return Err(io::Error::new(io::ErrorKind::InvalidInput, SimpleError));
				}
				(self.cursor as i64 + n) as usize
			}
			SeekFrom::End(n) => {
				if n < 0 && (-n as usize) > self.data.len() {
					return Err(io::Error::new(io::ErrorKind::InvalidInput, SimpleError));
				}
				(self.data.len() as i64 + n) as usize
			}
		};
		
		// Clamp cursor
		if new_cursor_pos > self.data.len() {
			new_cursor_pos = self.data.len();
		}
		
		// Update cursor
		self.cursor = new_cursor_pos;
		
		Ok(self.cursor as u64)
	}
}

#[derive(Debug)]
pub struct SimpleError;

impl fmt::Display for SimpleError {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "Some error")
	}
}

impl error::Error for SimpleError {
	fn source(&self) -> Option<&(dyn error::Error + 'static)> {
		None
	}
}
