#![no_std]
#![doc = include_str!("../README.md")]

pub use impl_opaque_macro::opaque;

#[doc(hidden)]
pub mod __private {
    #[derive(Clone, Copy)]
    pub struct Opaque;

    #[macro_export]
    #[doc(hidden)]
    macro_rules! field {
        ($vis:vis $name:ident : $ty:ty = $expr:expr) => {
            $crate::field!()
        };

        () => {
            compile_error!("cannot use field macro outside of impl block")
        };
    }
}
