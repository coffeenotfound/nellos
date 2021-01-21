use core::alloc::Allocator;
use alloc::alloc::Global;

pub trait CharRepetoire {
	type Point: CodePoint;
}

pub trait CodePoint = Copy + Eq;

pub struct Unicode(());
impl CharRepetoire for Unicode {
	type Point = char;
}

pub struct Ascii8(());
impl CharRepetoire for Ascii8 {
	type Point = u8;
}

pub trait StrEncoding {
	type Repetoire: CharRepetoire;
}

/// Marker trait to indicate a StrEncoding represents
/// a fixed-length encoding.
pub trait FixedStrEncoding: StrEncoding {}

pub struct Utf8(());
impl StrEncoding for Utf8 {
	type Repetoire = Unicode;
}

pub struct Utf32(());
impl StrEncoding for Utf8 {
	type Repetoire = Unicode;
}

pub struct Ucs2(());
impl StrEncoding for Ucs2 {
	type Repetoire = Unicode;
}
impl FixedStrEncoding for Ucs2 {}

pub type Ucs2String<A: Allocator = Global> = VarString<Ucs2, A>;

pub struct VarString<E: StrEncoding, A: Allocator = Global> {
	
}
