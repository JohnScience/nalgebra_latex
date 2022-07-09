//! Module with types and traits for [`nalgebra_linsys`]

use nalgebra::Matrix;

pub mod env;
pub mod err;
pub mod fmt;
pub mod numbering;
pub mod unknowns;

/// The type representing a [linear system], i.e. a system of simultaneous linear equations
///
/// # Example
///
/// ```
/// use nalgebra::{matrix, Const};
/// use nalgebra_latex::{
///     lin_sys::{
///         LinSys,
///         unknowns::SingleLetterBoldfaceVecOfUnknowns,
///         numbering::Numbering,
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
/// let vec_of_unknowns = SingleLetterBoldfaceVecOfUnknowns::<_,{Numbering::OneBased}>::new('x', Const::<3>);
/// let ls = LinSys::new(m, vec_of_unknowns);
/// CasesLinSysFormatter::write_latex(&mut s, &ls).unwrap();
/// assert_eq!(s, r"\begin{cases}$1x_{1}+2x_{2}+3x_{3}$\\$4x_{1}+5x_{2}+6x_{3}$\\$7x_{1}+8x_{2}+9x_{3}$\end{cases}");
/// ```
///
/// [linear system]: https://en.wikipedia.org/wiki/System_of_linear_equations
pub struct LinSys<T, R, C, S, U> {
    /// Matrix representation of a [linear system]. For convenience, the field is of type [`nalgebra::Matrix`]
    ///
    /// [linear system]: https://en.wikipedia.org/wiki/System_of_linear_equations
    pub matrix: Matrix<T, R, C, S>,
    /// The unknowns over which the [linear system] is defined
    ///
    /// [linear system]: https://en.wikipedia.org/wiki/System_of_linear_equations
    pub unknowns: U,
}

impl<T, R, C, S, U> LinSys<T, R, C, S, U> {
    pub fn new(mrls: Matrix<T, R, C, S>, unknowns: U) -> Self {
        LinSys {
            matrix: mrls,
            unknowns,
        }
    }
}
