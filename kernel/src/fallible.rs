//#![feature(allocator_api)]

//use alloc::alloc::Global;
//
//pub struct FallibleVec<T, A: FallibleAlloc = Global> {
//	alloc: A::Meta,
//}
//impl<T, A: FallibleAlloc> FallibleVec<T, A> {
//	pub fn new_in(alloc_meta: A::Meta) -> A::Result<Self> {
//		
//	}
//}

//use alloc::vec::Vec;
//use core::alloc::Allocator;
//
//trait FallibleVec {
//	
//}
//impl<T, A: Allocator> FallibleVec for Vec<T> {
//	
//}
//
//pub trait FallibleAlloc {
//	type Meta;
//	type Result<T>;
//}
//
////pub trait UnfallibleAlloc: FallibleAlloc {
////	
////}
//
//pub struct GlobalFallibleAlloc {
//	
//}
//impl FallibleAlloc for GlobalFallibleAlloc {
//	
//}
