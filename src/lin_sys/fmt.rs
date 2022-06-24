//! A module offering a number of [LaTeX] formatters for entities from [`nalgebra_linsys`].
//! 
//! 
//! [`nalgebra_linsys`]: https://crates.io/crates/nalgebra_linsys
//! [LaTeX]: https://www.overleaf.com/learn/latex/Learn_LaTeX_in_30_minutes#What_is_LaTeX.3F

use core::fmt::{Write, Display};

use nalgebra::{Dim, RawStorage};
use crate::{
    lin_sys::{
        env::CasesEnvironment,
        unknowns::Unknowns,
        LinSys,
    },
    fmt::LatexFormatter, env::LatexEnvironment,
};

/// Plain ["environment"]-agnostic [LaTeX] formatter for [linear systems], e.g. 
/// `1x_{0}+0x_{1}+3x_{2}\\4x_{0}+5x_{1}+6x_{2}`.
/// 
/// # Example
/// 
/// ```
/// use nalgebra::{matrix, Dynamic};
/// use nalgebra_latex::{
///     lin_sys::{
///         LinSys,
///         unknowns::SingleLetterVecOfUnknowns,
///         fmt::PlainLinSysFormatter,
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
///     // use nalgebra::Const if you can keep the size unchanged
///     len: Dynamic::new(3),
/// };
/// let ls = LinSys::new(m, vec_of_unknowns);
/// PlainLinSysFormatter::write_latex(&mut s, &ls).unwrap();
/// assert_eq!(s, r"1x_{0}+2x_{1}+3x_{2}\\4x_{0}+5x_{1}+6x_{2}\\7x_{0}+8x_{1}+9x_{2}");
/// ```
/// 
/// # Notes
/// 
/// This type can be the foundational block of other formatters for [linear systems].
/// 
/// Zeroes are printed too.
/// 
/// [linear systems]: https://en.wikipedia.org/wiki/System_of_linear_equations
/// [LaTeX]: https://www.overleaf.com/learn/latex/Learn_LaTeX_in_30_minutes#What_is_LaTeX.3F
/// ["environment"]: https://www.overleaf.com/learn/latex/Environments
pub struct PlainLinSysFormatter;

/// Formatter for [linear systems] using [`CasesEnvironment`], which represents `cases` [environment].
/// 
/// [linear systems]: https://en.wikipedia.org/wiki/System_of_linear_equations
/// [environment]: https://www.overleaf.com/learn/latex/Environments
pub struct CasesLinSysFormatter;

impl <T,R,C,S,U> LatexFormatter<LinSys<T,R,C,S,U>> for PlainLinSysFormatter
where
    T: Display,
    R: Dim,
    C: Dim,
    S: RawStorage<T,R,C>,
    U: Unknowns,
{
    fn write_latex<W: Write>(dest: &mut W, input: &LinSys<T,R,C,S,U>) -> Result<(), core::fmt::Error> {
        let nrows = input.matrix.nrows();
        let ncols = input.matrix.ncols();
        for i in 0..nrows {
            for j in 0..ncols {
                write!(dest, "{}", input.matrix[(i,j)])?;
                unsafe { input.unknowns.write_latex_for_ith_unchecked(dest, j) }?;
                if j != ncols - 1 {
                    write!(dest, "+")?;
                }
            }
            if i != nrows - 1 {
                write!(dest, r"\\")?;
            }
        }
        Ok(())
    }
}

impl<T,R,C,S,U> LatexFormatter<LinSys<T,R,C,S,U>> for CasesLinSysFormatter
where
    T: Display,
    R: Dim,
    C: Dim,
    S: RawStorage<T,R,C>,
    U: Unknowns,
{
    fn write_latex<W: core::fmt::Write>(dest: &mut W, input: &LinSys<T,R,C,S,U>) -> Result<(), core::fmt::Error> {
        CasesEnvironment::write_opening_tag(dest)?;
        PlainLinSysFormatter::write_latex(dest, input)?;
        CasesEnvironment::write_closing_tag(dest)
    }
}
