use core::alloc::{GlobalAlloc, Layout};
use core::marker::PhantomData;

pub struct KernelGlobalAlloc {
	_ph: PhantomData<()>,
}

impl KernelGlobalAlloc {
	pub const fn new() -> Self {
		Self {
			_ph: PhantomData,
		}
	}
}

unsafe impl GlobalAlloc for KernelGlobalAlloc {
	unsafe fn alloc(&self, _layout: Layout) -> *mut u8 {
		panic!("Unfallible global allocation is forbidden in kernel code, use fallible alloc")
	}
	
	unsafe fn dealloc(&self, _ptr: *mut u8, _layout: Layout) {
		panic!("Unfallible global (de)allocation is forbidden in kernel code, use fallible alloc")
	}
}
