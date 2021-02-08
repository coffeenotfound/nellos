#![no_std]

#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]

mod _binds;
pub use _binds::*;

mod overrides;
pub use overrides::*;

#[macro_use]
mod osl;

//pub use override::*;

//mod reexport {
//	pub use _binds::*;
//}
//pub use reexport::*;
