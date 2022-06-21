//! Module with types and traits for [`nalgebra_linsys`]

use nalgebra::Matrix;

pub mod unknowns;
pub mod fmt;
pub mod env;
pub mod err;

/// The type representing a linear system, i.e. a system of simultaneous linear equations
/// 
/// # Example
/// 
/// ```
/// use nalgebra::{matrix, Const};
/// use nalgebra_latex::{
///     lin_sys::{
///         LinSys,
///         unknowns::SingleLetterVecOfUnknowns,
///         fmt::CasesLinSysFormatter,
///     },
///     fmt::LatexFormatter,
/// };
///
/// let mut s = String::new();
/// let m = matrix!(
///     1,2,3;
///     4,5,6;
///     7,8,9;
/// );
/// let vec_of_unknowns = SingleLetterVecOfUnknowns {
///     c: 'x',
///     len: Const::<3>,
/// };
/// let ls = LinSys::new(m, vec_of_unknowns);
/// CasesLinSysFormatter::write_latex(&mut s, &ls).unwrap();
/// assert_eq!(s, r"\begin{cases}1x_{0}+2x_{1}+3x_{2}\\4x_{0}+5x_{1}+6x_{2}\\7x_{0}+8x_{1}+9x_{2}\end{cases}");
/// ```
pub struct LinSys<T,R,C,S,U> {
    /// Matrix representation of a linear system. For convenience, the field is of type [`nalgebra::Matrix`]
    pub matrix: Matrix<T,R,C,S>,
    /// The unknowns over which the linear system is defined
    pub unknowns: U,
}

impl<T,R,C,S,U> LinSys<T,R,C,S,U>
{
    pub fn new(mrls: Matrix<T, R, C, S>, unknowns: U) -> Self {
        LinSys {
            matrix: mrls,
            unknowns,
        }
    }
}
