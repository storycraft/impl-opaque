#![no_std]
#![doc = include_str!("../README.md")]

#[doc(hidden)]
pub mod __private {
    /// Marker struct for prohibiting [`Default`] trait implementation
    #[derive(Clone, Copy)]
    pub struct Opaque;
}

pub use impl_opaque_macro::opaque;
