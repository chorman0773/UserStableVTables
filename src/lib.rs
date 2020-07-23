
#![no_std]

#[cfg(feature="alloc")]
extern crate alloc;

pub mod traits;
pub mod ptr;
pub mod refs;

#[cfg(feature="box")]
pub mod boxed;
