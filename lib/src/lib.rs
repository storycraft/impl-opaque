#![no_std]
#![doc = include_str!("../README.md")]

pub use impl_opaque_macro::opaque;

#[doc(hidden)]
pub mod __private {
    #[derive(Clone, Copy)]
    pub struct Opaque;
}
