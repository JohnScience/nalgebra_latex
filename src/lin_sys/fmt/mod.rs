//! A module offering a number of [LaTeX] formatters for entities from [`nalgebra_linsys`].
//!
//! [`nalgebra_linsys`]: https://crates.io/crates/nalgebra_linsys
//! [LaTeX]: https://www.overleaf.com/learn/latex/Learn_LaTeX_in_30_minutes#What_is_LaTeX.3F

mod impl_latex_formatter;
mod impl_write_labelled_display_math_block;

/// Plain ["environment"]-agnostic [LaTeX] formatter for [linear systems], e.g.
/// `1x_{0}+0x_{1}=3\\4x_{0}+5x_{1}=6`.
///
/// # Example
///
/// ```
/// use nalgebra::{matrix, Dynamic};
/// use nalgebra_latex::{
///     lin_sys::{
///         LinSys,
///         unknowns::SingleLetterBoldfaceVecOfUnknowns,
///         numbering::Numbering,
///         fmt::PlainLinSysFormatter,
///     },
///     fmt::{write_latex, LatexFormatter},
///     latex_modes::{InlineMathMode, DisplayMathMode, InnerParagraphMode},
/// };
///
/// let mut s = String::new();
/// let m = matrix!(
///     1,2,3;
///     4,5,6;
///     7,8,9;
/// );
/// let vec_of_unknowns = SingleLetterBoldfaceVecOfUnknowns::<_,{Numbering::OneBased}>::new(
///     'x',
///     // use nalgebra::Const if you can keep the size unchanged
///     Dynamic::new(2)
/// );
/// let ls = LinSys::new(m, vec_of_unknowns).unwrap();
/// write_latex::<PlainLinSysFormatter,InnerParagraphMode,DisplayMathMode,_,_>(&mut s, &ls).unwrap();
/// assert_eq!(s, r"1x_{1}+2x_{2}=3\\4x_{1}+5x_{2}=6\\7x_{1}+8x_{2}=9");
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
