#![cfg_attr(feature = "clippy", feature(plugin))]
#![cfg_attr(feature = "clippy", plugin(clippy))]

extern crate kernel32;
extern crate winapi;
extern crate chrono;
extern crate enum_set;

#[macro_use]
mod macros;

mod constants;
mod util;

pub mod access;
pub mod handle;
pub mod named;
pub mod mapping;
pub mod object;
pub mod overlapped;
pub mod pipe;
pub mod process;
pub mod string;
pub mod sync;
pub mod waitable;

pub mod prelude {
    pub use named::{NamedBuilder, NamedObject};
    pub use object::{Object, ObjectExt};
    pub use string::{ToAnsiString, ToWideString};
    pub use waitable::Waitable;
}
