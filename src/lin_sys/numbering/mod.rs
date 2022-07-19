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
//! You can also choose to use the `adt_const_params` crate feature to enable the unstable Rust feature-namesake.
//!
//! Note that because the doc is generated with `--all-features`, [`NumberingTy`] and [`Numbering`]
//! are re-exported from [`adt_const_params`]. With `adt_const_params` disabled, the aforementioned types
//! will be re-exported from [`workaround`] instead.
//!
//! [ADT]: https://en.wikipedia.org/wiki/Algebraic_data_type
//! [unstable]: https://github.com/rust-lang/rust/issues/95174

#[cfg(any(doc_cfg, doc))]
#[cfg_attr(doc_cfg, doc(cfg(any(doc_cfg, doc))))]
pub mod adt_const_params;
#[cfg(any(doc_cfg, doc))]
#[cfg_attr(doc_cfg, doc(cfg(any(doc_cfg, doc))))]
pub mod workaround;

#[cfg(not(any(doc_cfg, doc)))]
mod adt_const_params;
#[cfg(not(any(doc_cfg, doc)))]
mod workaround;

#[cfg(feature = "adt_const_params")]
#[cfg_attr(doc_cfg, doc(cfg(feature = "adt_const_params")))]
pub use adt_const_params::{Numbering, NumberingTy};
#[cfg(not(feature = "adt_const_params"))]
#[cfg_attr(doc_cfg, doc(cfg(not(feature = "adt_const_params"))))]
pub use workaround::{Numbering, NumberingTy};
