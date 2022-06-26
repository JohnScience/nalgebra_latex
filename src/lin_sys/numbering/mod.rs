//! Module with temporary solution for numbering to support formatting entities from [`nalgebra_linsys`].
//!
//! # Notes
//!
//! At the moment of writing, support for [ADT] constant parameters is [unstable]. This module offers a
//! forward-compatible workaround for this issue.
//!
//! Even in the current version of Rust [`Numbering::ZeroBased`]` : `[`NumberingTy`] can hold if
//! [`Numbering`] is a module, [`ZeroBased`][`Numbering::ZeroBased`] is a constant of type `usize`, and
//! [`NumberingTy`] is an alias for `usize`.
//!
//! Once support is landed, [`Numbering`] will be an enum, [`ZeroBased`][`Numbering::ZeroBased`] will be an enum
//! variant, and [`NumberingTy`] will be a deprecated alias for [`Numbering`].
//!
//! [ADT]: https://en.wikipedia.org/wiki/Algebraic_data_type
//! [unstable]: https://github.com/rust-lang/rust/issues/95174
//!

pub mod workaround;
pub mod adt_const_params;

#[cfg(not(feature = "adt_const_params"))]
pub use workaround::*;
#[cfg(feature = "adt_const_params")]
pub use adt_const_params::*;
