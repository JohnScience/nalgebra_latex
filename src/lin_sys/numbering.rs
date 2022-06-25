//! Module with temporarily solution for numbering to support formatting entities from [`nalgebra_linsys`].
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

/// The alias for the type of the numbering.
///
/// # Notes
///
/// Check the [module level documentation][self] for the purpose of the type alias.
#[cfg(not(feature = "adt_const_params"))]
pub type NumberingTy = usize;

#[cfg(feature = "adt_const_params")]
pub type NumberingTy = Numbering;

/// A kind of numbering used for [LaTeX] formatting.
///
/// # Notes
///
/// Check the [module level documentation][self] for the purpose of the module.
///
/// [LaTeX]: https://www.overleaf.com/learn/latex/Learn_LaTeX_in_30_minutes#What_is_LaTeX.3F
#[cfg(not(feature = "adt_const_params"))]
#[allow(non_snake_case, non_upper_case_globals)]
pub mod Numbering {
    pub const ZeroBased: usize = 0usize;
    pub const OneBased: usize = 1usize;
}

#[cfg(feature = "adt_const_params")]
#[derive(PartialEq, Eq)]
pub enum Numbering {
    ZeroBased,
    OneBased,
}
