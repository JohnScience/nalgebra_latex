use thiserror::Error;

/// Out-of-bounds error for indexing into the "vector of unknowns"
/// [`nalgebra_latex::lin_sys::unknowns::Unknowns`][crate::lin_sys::unknowns::Unknowns]
#[derive(Error, Debug)]
#[error("Index of unknown of the linear system is out of bounds")]
pub struct OutOfBoundsError;
