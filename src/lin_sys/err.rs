//! A module offering [LaTeX] formatting errors for [`nalgebra_linsys`].
//!
//! [LaTeX]: https://www.overleaf.com/learn/latex/Learn_LaTeX_in_30_minutes#What_is_LaTeX.3F

use thiserror::Error;

/// Out-of-bounds error for indexing into the "vector of unknowns"
/// [`nalgebra_latex::lin_sys::unknowns::Unknowns`][crate::lin_sys::unknowns::Unknowns]
#[derive(Error, Debug)]
#[error("Index of unknown of the linear system is out of bounds")]
pub struct OutOfBoundsError;
